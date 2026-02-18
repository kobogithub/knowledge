use anyhow::{anyhow, Context, Result};
use clap::Subcommand;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Subcommand, Debug)]
pub enum McpCommands {
    /// Add an MCP server to the project configuration
    Add {
        /// Server name or npx package (e.g., 'rust-docs' or '@modelcontextprotocol/server-postgres')
        name: String,

        /// Optional custom command to run the server
        #[arg(short, long)]
        command: Option<String>,

        /// Optional arguments for the server
        #[arg(short, long, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Optional environment variables (key=value format)
        #[arg(short, long)]
        env: Vec<String>,
    },

    /// List configured MCP servers
    List,

    /// Remove an MCP server from configuration
    Remove {
        /// Server name to remove
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct KnConfig {
    project: ProjectConfig,
    #[serde(default)]
    agents: HashMap<String, String>,
    #[serde(default)]
    skills: SkillsConfig,
    #[serde(default)]
    beads: BeadsConfig,
    #[serde(default)]
    mcp: McpConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    #[serde(rename = "type")]
    project_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct SkillsConfig {
    #[serde(default)]
    enabled: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct BeadsConfig {
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default = "default_templates_dir")]
    templates_dir: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct McpConfig {
    #[serde(default)]
    servers: HashMap<String, McpServer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct McpServer {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    env: HashMap<String, String>,
}

fn default_true() -> bool {
    true
}

fn default_templates_dir() -> String {
    ".beads/templates".to_string()
}

pub struct McpHandler;

impl McpHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, command: McpCommands) -> Result<()> {
        match command {
            McpCommands::Add {
                name,
                command,
                args,
                env,
            } => self.add_server(&name, command, args, env),
            McpCommands::List => self.list_servers(),
            McpCommands::Remove { name } => self.remove_server(&name),
        }
    }

    fn add_server(
        &self,
        name: &str,
        custom_command: Option<String>,
        args: Vec<String>,
        env_vars: Vec<String>,
    ) -> Result<()> {
        let config_path = Path::new("kn.toml");

        if !config_path.exists() {
            return Err(anyhow!(
                "kn.toml not found. Run 'kn init' first to initialize the project."
            ));
        }

        // Parse environment variables
        let mut env_map = HashMap::new();
        for env in env_vars {
            let parts: Vec<&str> = env.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(anyhow!(
                    "Invalid environment variable format: '{}'. Use KEY=VALUE",
                    env
                ));
            }
            env_map.insert(parts[0].to_string(), parts[1].to_string());
        }

        // Get server configuration (from preset or custom)
        let server = if let Some(cmd) = custom_command {
            McpServer {
                command: cmd,
                args,
                env: env_map,
            }
        } else {
            self.get_preset_server(name, args, env_map)?
        };

        // Read current config
        let config_content = fs::read_to_string(config_path).context("Failed to read kn.toml")?;

        let mut config: KnConfig =
            toml::from_str(&config_content).context("Failed to parse kn.toml")?;

        // Add server
        if config.mcp.servers.contains_key(name) {
            println!(
                "{}",
                format!("  ⚠ MCP server '{}' already exists, updating...", name).yellow()
            );
        }

        config.mcp.servers.insert(name.to_string(), server.clone());

        // Write back config
        let new_content = toml::to_string_pretty(&config).context("Failed to serialize config")?;

        fs::write(config_path, new_content).context("Failed to write kn.toml")?;

        println!(
            "{}",
            format!("✓ Added MCP server '{}'", name).green().bold()
        );
        println!("\n{}", "Server configuration:".bright_white().bold());
        println!("  Command: {}", server.command.cyan());
        if !server.args.is_empty() {
            println!("  Args: {}", server.args.join(" ").cyan());
        }
        if !server.env.is_empty() {
            println!("  Environment:");
            for (key, value) in &server.env {
                println!("    {} = {}", key.cyan(), value.bright_black());
            }
        }

        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  • Run 'kn mcp list' to see all configured servers");
        println!("  • The server will be available in OpenCode MCP integration");

        Ok(())
    }

    fn list_servers(&self) -> Result<()> {
        let config_path = Path::new("kn.toml");

        if !config_path.exists() {
            return Err(anyhow!(
                "kn.toml not found. Run 'kn init' first to initialize the project."
            ));
        }

        let config_content = fs::read_to_string(config_path).context("Failed to read kn.toml")?;

        let config: KnConfig =
            toml::from_str(&config_content).context("Failed to parse kn.toml")?;

        if config.mcp.servers.is_empty() {
            println!("{}", "No MCP servers configured yet.".yellow());
            println!("\n{}", "Add a server with:".bright_white().bold());
            println!("  kn mcp add <server-name>");
            println!("\n{}", "Available presets:".bright_white().bold());
            self.print_available_presets();
            return Ok(());
        }

        println!("{}", "Configured MCP Servers:".bright_white().bold());
        println!();

        for (name, server) in &config.mcp.servers {
            println!("{}", format!("• {}", name).bright_cyan().bold());
            println!("  Command: {}", server.command.white());
            if !server.args.is_empty() {
                println!("  Args: {}", server.args.join(" ").bright_black());
            }
            if !server.env.is_empty() {
                println!("  Environment:");
                for (key, value) in &server.env {
                    println!("    {} = {}", key.cyan(), value.bright_black());
                }
            }
            println!();
        }

        Ok(())
    }

    fn remove_server(&self, name: &str) -> Result<()> {
        let config_path = Path::new("kn.toml");

        if !config_path.exists() {
            return Err(anyhow!(
                "kn.toml not found. Run 'kn init' first to initialize the project."
            ));
        }

        let config_content = fs::read_to_string(config_path).context("Failed to read kn.toml")?;

        let mut config: KnConfig =
            toml::from_str(&config_content).context("Failed to parse kn.toml")?;

        if !config.mcp.servers.contains_key(name) {
            return Err(anyhow!("MCP server '{}' not found", name));
        }

        config.mcp.servers.remove(name);

        let new_content = toml::to_string_pretty(&config).context("Failed to serialize config")?;

        fs::write(config_path, new_content).context("Failed to write kn.toml")?;

        println!(
            "{}",
            format!("✓ Removed MCP server '{}'", name).green().bold()
        );

        Ok(())
    }

    fn get_preset_server(
        &self,
        name: &str,
        custom_args: Vec<String>,
        custom_env: HashMap<String, String>,
    ) -> Result<McpServer> {
        let presets = self.get_server_presets();

        if let Some(mut preset) = presets.get(name).cloned() {
            // Override with custom args/env if provided
            if !custom_args.is_empty() {
                preset.args = custom_args;
            }
            if !custom_env.is_empty() {
                preset.env.extend(custom_env);
            }
            Ok(preset)
        } else {
            // If not a preset, assume it's an npx package
            let command = if name.starts_with('@') || name.contains('/') {
                "npx".to_string()
            } else {
                return Err(anyhow!(
                    "Unknown MCP server preset: '{}'. Use --command to specify a custom command, or see available presets with 'kn mcp list'",
                    name
                ));
            };

            let mut args = vec!["-y".to_string(), name.to_string()];
            args.extend(custom_args);

            Ok(McpServer {
                command,
                args,
                env: custom_env,
            })
        }
    }

    fn get_server_presets(&self) -> HashMap<String, McpServer> {
        let mut presets = HashMap::new();

        // Filesystem server
        presets.insert(
            "filesystem".to_string(),
            McpServer {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-filesystem".to_string(),
                    ".".to_string(),
                ],
                env: HashMap::new(),
            },
        );

        // PostgreSQL server
        presets.insert(
            "postgres".to_string(),
            McpServer {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-postgres".to_string(),
                ],
                env: {
                    let mut env = HashMap::new();
                    env.insert(
                        "POSTGRES_URL".to_string(),
                        "postgresql://localhost/mydb".to_string(),
                    );
                    env
                },
            },
        );

        // GitHub server
        presets.insert(
            "github".to_string(),
            McpServer {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-github".to_string(),
                ],
                env: {
                    let mut env = HashMap::new();
                    env.insert("GITHUB_TOKEN".to_string(), "your-token-here".to_string());
                    env
                },
            },
        );

        // Brave Search server
        presets.insert(
            "brave-search".to_string(),
            McpServer {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-brave-search".to_string(),
                ],
                env: {
                    let mut env = HashMap::new();
                    env.insert("BRAVE_API_KEY".to_string(), "your-api-key".to_string());
                    env
                },
            },
        );

        // Puppeteer (web scraping) server
        presets.insert(
            "puppeteer".to_string(),
            McpServer {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-puppeteer".to_string(),
                ],
                env: HashMap::new(),
            },
        );

        presets
    }

    fn print_available_presets(&self) {
        println!("  • filesystem    - Access local files");
        println!("  • postgres      - PostgreSQL database access");
        println!("  • github        - GitHub API integration");
        println!("  • brave-search  - Web search via Brave API");
        println!("  • puppeteer     - Web scraping and browser automation");
        println!("\nOr use any npm package with: kn mcp add @scope/package");
    }
}
