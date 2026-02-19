use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Metadata for an MCP server stored in ~/.kn/mcps/<name>/mcp.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMetadata {
    pub mcp: McpInfo,
    pub command: CommandConfig,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    #[serde(default)]
    pub config: McpConfigOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpInfo {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub package: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    #[serde(rename = "type")]
    pub command_type: String, // "local", "remote", etc.
    pub executable: String, // "npx", "docker", etc.
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McpConfigOptions {
    #[serde(default)]
    pub supports_custom_args: bool,
    #[serde(default)]
    pub default_args: Vec<String>,
    #[serde(default)]
    pub required_env: Vec<String>,
}

impl McpMetadata {
    /// Load MCP metadata from mcp.toml file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let metadata: McpMetadata = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML in {}", path.display()))?;

        Ok(metadata)
    }

    /// Save MCP metadata to mcp.toml file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize MCP metadata")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))?;

        Ok(())
    }

    /// Generate OpenCode-compatible command array
    /// Combines executable + args, with optional project-specific args
    pub fn generate_command(&self, additional_args: &[String]) -> Vec<String> {
        let mut command = vec![self.command.executable.clone()];
        command.extend(self.command.args.clone());

        if self.config.supports_custom_args {
            command.extend(additional_args.iter().cloned());
        } else if !additional_args.is_empty() {
            eprintln!(
                "Warning: MCP '{}' does not support custom args",
                self.mcp.name
            );
        }

        command
    }

    /// Validate that required environment variables are provided
    pub fn validate_environment(&self, env: &HashMap<String, String>) -> Result<()> {
        for required_var in &self.config.required_env {
            if !env.contains_key(required_var) {
                anyhow::bail!(
                    "Missing required environment variable '{}' for MCP '{}'",
                    required_var,
                    self.mcp.name
                );
            }
        }
        Ok(())
    }

    /// Create from preset configuration
    pub fn from_preset(name: &str) -> Option<Self> {
        match name {
            "filesystem" => Some(Self {
                mcp: McpInfo {
                    name: "filesystem".to_string(),
                    description: "Access local files and directories".to_string(),
                    package: Some("@modelcontextprotocol/server-filesystem".to_string()),
                    version: None,
                },
                command: CommandConfig {
                    command_type: "local".to_string(),
                    executable: "npx".to_string(),
                    args: vec![
                        "-y".to_string(),
                        "@modelcontextprotocol/server-filesystem".to_string(),
                    ],
                },
                environment: HashMap::new(),
                config: McpConfigOptions {
                    supports_custom_args: true,
                    default_args: vec![".".to_string()],
                    required_env: vec![],
                },
            }),

            "github" => Some(Self {
                mcp: McpInfo {
                    name: "github".to_string(),
                    description: "GitHub API integration".to_string(),
                    package: Some("@modelcontextprotocol/server-github".to_string()),
                    version: None,
                },
                command: CommandConfig {
                    command_type: "local".to_string(),
                    executable: "npx".to_string(),
                    args: vec![
                        "-y".to_string(),
                        "@modelcontextprotocol/server-github".to_string(),
                    ],
                },
                environment: {
                    let mut env = HashMap::new();
                    env.insert(
                        "GITHUB_PERSONAL_ACCESS_TOKEN".to_string(),
                        "${GITHUB_TOKEN}".to_string(),
                    );
                    env
                },
                config: McpConfigOptions {
                    supports_custom_args: false,
                    default_args: vec![],
                    required_env: vec!["GITHUB_PERSONAL_ACCESS_TOKEN".to_string()],
                },
            }),

            "postgres" => Some(Self {
                mcp: McpInfo {
                    name: "postgres".to_string(),
                    description: "PostgreSQL database access".to_string(),
                    package: Some("@modelcontextprotocol/server-postgres".to_string()),
                    version: None,
                },
                command: CommandConfig {
                    command_type: "local".to_string(),
                    executable: "npx".to_string(),
                    args: vec![
                        "-y".to_string(),
                        "@modelcontextprotocol/server-postgres".to_string(),
                    ],
                },
                environment: {
                    let mut env = HashMap::new();
                    env.insert(
                        "POSTGRES_URL".to_string(),
                        "postgresql://localhost/mydb".to_string(),
                    );
                    env
                },
                config: McpConfigOptions {
                    supports_custom_args: false,
                    default_args: vec![],
                    required_env: vec!["POSTGRES_URL".to_string()],
                },
            }),

            "brave-search" => Some(Self {
                mcp: McpInfo {
                    name: "brave-search".to_string(),
                    description: "Web search via Brave Search API".to_string(),
                    package: Some("@modelcontextprotocol/server-brave-search".to_string()),
                    version: None,
                },
                command: CommandConfig {
                    command_type: "local".to_string(),
                    executable: "npx".to_string(),
                    args: vec![
                        "-y".to_string(),
                        "@modelcontextprotocol/server-brave-search".to_string(),
                    ],
                },
                environment: {
                    let mut env = HashMap::new();
                    env.insert("BRAVE_API_KEY".to_string(), "your-api-key".to_string());
                    env
                },
                config: McpConfigOptions {
                    supports_custom_args: false,
                    default_args: vec![],
                    required_env: vec!["BRAVE_API_KEY".to_string()],
                },
            }),

            "puppeteer" => Some(Self {
                mcp: McpInfo {
                    name: "puppeteer".to_string(),
                    description: "Web scraping and browser automation".to_string(),
                    package: Some("@modelcontextprotocol/server-puppeteer".to_string()),
                    version: None,
                },
                command: CommandConfig {
                    command_type: "local".to_string(),
                    executable: "npx".to_string(),
                    args: vec![
                        "-y".to_string(),
                        "@modelcontextprotocol/server-puppeteer".to_string(),
                    ],
                },
                environment: HashMap::new(),
                config: McpConfigOptions {
                    supports_custom_args: false,
                    default_args: vec![],
                    required_env: vec![],
                },
            }),

            _ => None,
        }
    }

    /// List all available presets
    pub fn list_presets() -> Vec<String> {
        vec![
            "filesystem".to_string(),
            "github".to_string(),
            "postgres".to_string(),
            "brave-search".to_string(),
            "puppeteer".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_preset() {
        let mcp = McpMetadata::from_preset("filesystem").unwrap();
        assert_eq!(mcp.mcp.name, "filesystem");
        assert!(mcp.config.supports_custom_args);
    }

    #[test]
    fn test_generate_command() {
        let mcp = McpMetadata::from_preset("filesystem").unwrap();
        let cmd = mcp.generate_command(&["/custom/path".to_string()]);

        assert_eq!(cmd[0], "npx");
        assert_eq!(cmd[1], "-y");
        assert!(cmd.contains(&"/custom/path".to_string()));
    }

    #[test]
    fn test_validate_environment() {
        let mcp = McpMetadata::from_preset("github").unwrap();
        let mut env = HashMap::new();

        // Should fail without required env var
        assert!(mcp.validate_environment(&env).is_err());

        // Should succeed with required env var
        env.insert(
            "GITHUB_PERSONAL_ACCESS_TOKEN".to_string(),
            "token".to_string(),
        );
        assert!(mcp.validate_environment(&env).is_ok());
    }
}
