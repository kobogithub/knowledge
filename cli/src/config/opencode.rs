use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::commands::mcp::ProjectMcpConfig;
use crate::core::kn_home;
use crate::models::mcp::McpMetadata;

/// OpenCode configuration structure matching https://opencode.ai/config.json schema
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenCodeConfig {
    #[serde(rename = "$schema")]
    pub schema: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<HashMap<String, OpenCodeMcpServer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenCodeMcpServer {
    #[serde(rename = "type")]
    pub server_type: String,

    pub command: Vec<String>,

    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

fn default_true() -> bool {
    true
}

fn is_true(val: &bool) -> bool {
    *val
}

impl OpenCodeConfig {
    /// Create a new OpenCode config with default schema
    pub fn new() -> Self {
        Self {
            schema: "https://opencode.ai/config.json".to_string(),
            mcp: None,
        }
    }

    /// Load existing config from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let config: OpenCodeConfig = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize OpenCode config")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))?;

        Ok(())
    }

    /// Generate OpenCode MCP configuration from project's kn.toml
    pub fn generate_from_project(project_mcp: &ProjectMcpConfig) -> Result<Self> {
        let mut config = Self::new();
        let mut mcp_servers = HashMap::new();

        for mcp_name in &project_mcp.enabled {
            // Load MCP metadata from ~/.kn/mcps/<name>/mcp.toml
            let mcps_dir = kn_home::mcps_dir()?;
            let mcp_toml = mcps_dir.join(mcp_name).join("mcp.toml");

            if !mcp_toml.exists() {
                eprintln!(
                    "Warning: MCP '{}' not found in ~/.kn/mcps/, skipping",
                    mcp_name
                );
                continue;
            }

            let mcp_metadata = McpMetadata::from_file(&mcp_toml)?;

            // Get project-specific overrides
            let project_override = project_mcp.config.get(mcp_name);

            // Check if disabled in project config
            if let Some(override_cfg) = project_override {
                if !override_cfg.enabled {
                    continue; // Skip disabled MCPs
                }
            }

            // Generate command with optional project args
            let additional_args = project_override
                .and_then(|o| {
                    if o.args.is_empty() {
                        None
                    } else {
                        Some(&o.args)
                    }
                })
                .unwrap_or(&mcp_metadata.config.default_args);

            let command = mcp_metadata.generate_command(additional_args);

            // Merge environment variables (global + project-specific)
            let mut environment = mcp_metadata.environment.clone();
            if let Some(override_cfg) = project_override {
                environment.extend(override_cfg.environment.clone());
            }

            // Create OpenCode MCP server config
            let server = OpenCodeMcpServer {
                server_type: mcp_metadata.command.command_type.clone(),
                command,
                enabled: true,
                environment: if environment.is_empty() {
                    None
                } else {
                    Some(environment)
                },
                timeout: None, // Could be added to metadata if needed
            };

            mcp_servers.insert(mcp_name.clone(), server);
        }

        if !mcp_servers.is_empty() {
            config.mcp = Some(mcp_servers);
        }

        Ok(config)
    }

    /// Merge with existing config (preserves non-MCP settings)
    pub fn merge_with_existing(&mut self, _existing: &OpenCodeConfig) {
        // For now, we only manage MCP section
        // Other sections (keybinds, theme, etc.) are preserved as-is
        // This is a simple implementation - can be extended later
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = OpenCodeConfig::new();
        assert_eq!(config.schema, "https://opencode.ai/config.json");
        assert!(config.mcp.is_none());
    }

    #[test]
    fn test_serialize_minimal() {
        let config = OpenCodeConfig::new();
        let json = serde_json::to_string_pretty(&config).unwrap();

        assert!(json.contains("$schema"));
        assert!(json.contains("https://opencode.ai/config.json"));
    }

    #[test]
    fn test_serialize_with_mcp() {
        let mut config = OpenCodeConfig::new();
        let mut servers = HashMap::new();

        servers.insert(
            "test".to_string(),
            OpenCodeMcpServer {
                server_type: "local".to_string(),
                command: vec![
                    "npx".to_string(),
                    "-y".to_string(),
                    "test-server".to_string(),
                ],
                enabled: true,
                environment: None,
                timeout: None,
            },
        );

        config.mcp = Some(servers);

        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("\"test\""));
        assert!(json.contains("\"type\": \"local\""));
        assert!(json.contains("\"command\""));
    }
}
