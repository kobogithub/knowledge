use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::commands::mcp::ProjectMcpConfig;
use crate::core::kn_home;
use crate::models::mcp::McpMetadata;

/// Gemini Code Assist MCP configuration structure
/// Format: .gemini/antigravity/mcp_config.json
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, GeminiMcpServer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiMcpServer {
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

impl GeminiConfig {
    /// Create a new empty Gemini config
    pub fn new() -> Self {
        Self {
            mcp_servers: HashMap::new(),
        }
    }

    /// Load existing config from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let config: GeminiConfig = serde_json::from_str(&content)
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
            serde_json::to_string_pretty(self).context("Failed to serialize Gemini config")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))?;

        Ok(())
    }

    /// Generate Gemini MCP configuration from project's kn.toml
    pub fn generate_from_project(project_mcp: &ProjectMcpConfig) -> Result<Self> {
        let mut config = Self::new();

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

            let full_command = mcp_metadata.generate_command(additional_args);

            // Split into command (first element) and args (rest)
            let command = full_command.first().unwrap_or(&"".to_string()).clone();
            let args = full_command.into_iter().skip(1).collect();

            // Merge environment variables (global + project-specific)
            let mut environment = mcp_metadata.environment.clone();
            if let Some(override_cfg) = project_override {
                environment.extend(override_cfg.environment.clone());
            }

            // Create Gemini MCP server config
            let server = GeminiMcpServer {
                command,
                args,
                env: environment,
            };

            // Use a naming convention for Gemini (append -mcp-server if not present)
            let server_name = if mcp_name.ends_with("-mcp-server") {
                mcp_name.clone()
            } else {
                format!("{}-mcp-server", mcp_name)
            };

            config.mcp_servers.insert(server_name, server);
        }

        Ok(config)
    }

    /// Merge with existing config
    pub fn merge_with_existing(&mut self, existing: &GeminiConfig) {
        // Preserve servers not managed by kn
        for (name, server) in &existing.mcp_servers {
            if !self.mcp_servers.contains_key(name) {
                self.mcp_servers.insert(name.clone(), server.clone());
            }
        }
    }
}

impl Clone for GeminiMcpServer {
    fn clone(&self) -> Self {
        Self {
            command: self.command.clone(),
            args: self.args.clone(),
            env: self.env.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = GeminiConfig::new();
        assert!(config.mcp_servers.is_empty());
    }

    #[test]
    fn test_serialize_minimal() {
        let config = GeminiConfig::new();
        let json = serde_json::to_string_pretty(&config).unwrap();

        assert!(json.contains("mcpServers"));
    }

    #[test]
    fn test_serialize_with_server() {
        let mut config = GeminiConfig::new();

        config.mcp_servers.insert(
            "github-mcp-server".to_string(),
            GeminiMcpServer {
                command: "docker".to_string(),
                args: vec!["run".to_string(), "-i".to_string()],
                env: HashMap::new(),
            },
        );

        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("\"github-mcp-server\""));
        assert!(json.contains("\"command\": \"docker\""));
        assert!(json.contains("\"args\""));
    }
}
