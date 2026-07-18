use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::commands::mcp::ProjectMcpConfig;
use crate::core::kn_home;
use crate::models::mcp::McpMetadata;

/// Claude Code project MCP configuration (`.mcp.json`), matching the
/// `{"mcpServers": {...}}` shape Claude Code reads at the project root.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeMcpConfig {
    #[serde(rename = "mcpServers", skip_serializing_if = "Option::is_none")]
    pub mcp_servers: Option<HashMap<String, ClaudeMcpServer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeMcpServer {
    pub command: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,
}

impl Default for ClaudeMcpConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ClaudeMcpConfig {
    /// Create a new, empty Claude MCP config
    pub fn new() -> Self {
        Self { mcp_servers: None }
    }

    /// Load existing config from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let config: ClaudeMcpConfig = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize Claude MCP config")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))?;

        Ok(())
    }

    /// Generate Claude Code MCP configuration from project's kn.toml
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

            // Build args: base args from metadata + optional project-specific args
            let mut args = mcp_metadata.command.args.clone();
            let additional_args = project_override
                .map(|o| &o.args)
                .filter(|a| !a.is_empty())
                .unwrap_or(&mcp_metadata.config.default_args);

            if mcp_metadata.config.supports_custom_args {
                args.extend(additional_args.iter().cloned());
            } else if !additional_args.is_empty() {
                eprintln!("Warning: MCP '{}' does not support custom args", mcp_name);
            }

            // Merge environment variables (global + project-specific)
            let mut env = mcp_metadata.environment.clone();
            if let Some(override_cfg) = project_override {
                env.extend(override_cfg.environment.clone());
            }

            let server = ClaudeMcpServer {
                command: mcp_metadata.command.executable.clone(),
                args,
                env,
            };

            mcp_servers.insert(mcp_name.clone(), server);
        }

        if !mcp_servers.is_empty() {
            config.mcp_servers = Some(mcp_servers);
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = ClaudeMcpConfig::new();
        assert!(config.mcp_servers.is_none());
    }

    #[test]
    fn test_serialize_minimal() {
        let config = ClaudeMcpConfig::new();
        let json = serde_json::to_string_pretty(&config).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_serialize_with_mcp() {
        let mut config = ClaudeMcpConfig::new();
        let mut servers = HashMap::new();

        servers.insert(
            "test".to_string(),
            ClaudeMcpServer {
                command: "npx".to_string(),
                args: vec!["-y".to_string(), "test-server".to_string()],
                env: HashMap::new(),
            },
        );

        config.mcp_servers = Some(servers);

        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("\"mcpServers\""));
        assert!(json.contains("\"test\""));
        assert!(json.contains("\"command\": \"npx\""));
        assert!(json.contains("\"args\""));
    }
}
