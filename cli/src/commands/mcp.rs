use anyhow::{anyhow, Context, Result};
use clap::Subcommand;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::KnConfig;
use crate::core::kn_home;
use crate::models::mcp::McpMetadata;

// Registry API response structures
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RegistryResponse {
    servers: Vec<ServerEntry>,
    metadata: RegistryMetadata,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ServerEntry {
    server: ServerInfo,
    #[serde(rename = "_meta")]
    meta: ServerMeta,
}

#[derive(Debug, Deserialize)]
struct ServerInfo {
    name: String,
    description: String,
    #[serde(default)]
    title: Option<String>,
    version: String,
    #[serde(default, rename = "websiteUrl")]
    website_url: Option<String>,
    #[serde(default)]
    packages: Vec<PackageInfo>,
    #[serde(default)]
    repository: RepositoryInfo,
}

#[derive(Debug, Deserialize, Default)]
struct PackageInfo {
    #[serde(rename = "registryType")]
    registry_type: String,
    identifier: String,
}

#[derive(Debug, Deserialize, Default)]
struct RepositoryInfo {
    #[serde(default)]
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ServerMeta {
    #[serde(rename = "io.modelcontextprotocol.registry/official")]
    official: OfficialMeta,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OfficialMeta {
    status: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RegistryMetadata {
    count: usize,
    #[serde(default, rename = "nextCursor")]
    next_cursor: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum McpCommands {
    /// Install an MCP server globally to ~/.kn/mcps/
    Install {
        /// MCP name (preset name, npm package, or local path)
        name: String,

        /// Custom MCP name (for npm packages or local paths)
        #[arg(long)]
        as_name: Option<String>,

        /// Custom command executable
        #[arg(short, long)]
        command: Option<String>,

        /// Custom arguments
        #[arg(short, long)]
        args: Vec<String>,

        /// Environment variables (KEY=value)
        #[arg(short, long)]
        env: Vec<String>,

        /// Skip npm package validation
        #[arg(long)]
        skip_validation: bool,
    },

    /// Uninstall an MCP server from ~/.kn/mcps/
    Uninstall {
        /// MCP name to uninstall
        name: String,
    },

    /// List all globally installed MCPs
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Show detailed information about an MCP
    Info {
        /// MCP name
        name: String,
    },

    /// Add an MCP to the current project (enables it in kn.toml)
    Add {
        /// MCP name (must be installed in ~/.kn/mcps/)
        name: String,

        /// Custom arguments for this project
        #[arg(short, long)]
        args: Vec<String>,

        /// Environment variables (KEY=value)
        #[arg(short, long)]
        env: Vec<String>,
    },

    /// Remove an MCP from the current project
    Remove {
        /// MCP name to remove
        name: String,
    },

    /// Enable an MCP in the current project
    Enable {
        /// MCP name to enable
        name: String,
    },

    /// Disable an MCP in the current project
    Disable {
        /// MCP name to disable
        name: String,
    },

    /// List available MCP presets
    Presets,

    /// Search for MCP servers in the official registry
    Search {
        /// Search query (searches in name and description)
        query: String,

        /// Maximum number of results to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Interactively select and install an MCP server
        #[arg(short, long)]
        install: bool,
    },
}

// Project-level MCP configuration in kn.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMcpConfig {
    #[serde(default)]
    pub enabled: Vec<String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub config: HashMap<String, McpProjectOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpProjectOverride {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,

    #[serde(default)]
    pub enabled: bool,
}

pub struct McpHandler;

impl Default for McpHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl McpHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, command: McpCommands) -> Result<()> {
        match command {
            McpCommands::Install {
                name,
                as_name,
                command,
                args,
                env,
                skip_validation,
            } => self.install_mcp(&name, as_name, command, args, env, skip_validation),
            McpCommands::Uninstall { name } => self.uninstall_mcp(&name),
            McpCommands::List { detailed } => self.list_mcps(detailed),
            McpCommands::Info { name } => self.show_mcp_info(&name),
            McpCommands::Add { name, args, env } => self.add_to_project(&name, args, env),
            McpCommands::Remove { name } => self.remove_from_project(&name),
            McpCommands::Enable { name } => self.enable_in_project(&name),
            McpCommands::Disable { name } => self.disable_in_project(&name),
            McpCommands::Presets => self.list_presets(),
            McpCommands::Search {
                query,
                limit,
                install,
            } => self.search_registry(&query, limit, install),
        }
    }

    /// Install an MCP globally to ~/.kn/mcps/
    fn install_mcp(
        &self,
        name: &str,
        as_name: Option<String>,
        custom_command: Option<String>,
        custom_args: Vec<String>,
        env_vars: Vec<String>,
        skip_validation: bool,
    ) -> Result<()> {
        kn_home::ensure_kn_home()?;

        let mcp_name = as_name.as_ref().unwrap_or(&name.to_string()).clone();
        let mcps_dir = kn_home::mcps_dir()?;
        let mcp_dir = mcps_dir.join(&mcp_name);

        // Parse environment variables
        let mut env_map = HashMap::new();
        for env in env_vars {
            let parts: Vec<&str> = env.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid env format: '{}'. Use KEY=VALUE", env));
            }
            env_map.insert(parts[0].to_string(), parts[1].to_string());
        }

        // Get or create MCP metadata
        let metadata = if let Some(preset) = McpMetadata::from_preset(name) {
            // Use preset as base, override if custom values provided
            let mut meta = preset;

            if let Some(cmd) = custom_command {
                meta.command.executable = cmd;
            }
            if !custom_args.is_empty() {
                meta.command.args = custom_args;
            }
            if !env_map.is_empty() {
                meta.environment.extend(env_map);
            }

            meta
        } else if let Some(cmd) = custom_command {
            // Custom MCP configuration
            self.create_custom_mcp(&mcp_name, &cmd, custom_args, env_map)?
        } else if name.starts_with('@') || name.contains('/') {
            // Assume it's an npm package
            self.create_npm_mcp(&mcp_name, name, env_map, skip_validation)?
        } else {
            return Err(anyhow!(
                "Unknown MCP preset: '{}'. Use --command for custom MCPs or see 'kn mcp presets'",
                name
            ));
        };

        // Create MCP directory
        fs::create_dir_all(&mcp_dir)
            .with_context(|| format!("Failed to create {}", mcp_dir.display()))?;

        // Save mcp.toml
        let mcp_toml_path = mcp_dir.join("mcp.toml");
        metadata.save(&mcp_toml_path)?;

        println!(
            "{}",
            format!("✓ Installed MCP '{}'", mcp_name).green().bold()
        );
        println!("\n{}", "MCP Configuration:".bright_white().bold());
        println!("  Name: {}", metadata.mcp.name.cyan());
        println!("  Description: {}", metadata.mcp.description);
        println!(
            "  Command: {}",
            format!(
                "{} {}",
                metadata.command.executable,
                metadata.command.args.join(" ")
            )
            .cyan()
        );

        if !metadata.environment.is_empty() {
            println!("  Environment:");
            for (key, value) in &metadata.environment {
                println!("    {} = {}", key.cyan(), value.bright_black());
            }
        }

        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  • Add to project: kn mcp add {}", mcp_name);
        println!("  • View details: kn mcp info {}", mcp_name);

        Ok(())
    }

    /// Uninstall an MCP from ~/.kn/mcps/
    fn uninstall_mcp(&self, name: &str) -> Result<()> {
        let mcps_dir = kn_home::mcps_dir()?;
        let mcp_dir = mcps_dir.join(name);

        if !mcp_dir.exists() {
            return Err(anyhow!("MCP '{}' is not installed", name));
        }

        fs::remove_dir_all(&mcp_dir)
            .with_context(|| format!("Failed to remove {}", mcp_dir.display()))?;

        println!("{}", format!("✓ Uninstalled MCP '{}'", name).green().bold());
        Ok(())
    }

    /// List all globally installed MCPs
    fn list_mcps(&self, detailed: bool) -> Result<()> {
        let mcps = kn_home::list_installed_mcps_with_metadata()?;

        if mcps.is_empty() {
            println!("{}", "No MCPs installed yet.".yellow());
            println!("\n{}", "Install an MCP with:".bright_white().bold());
            println!("  kn mcp install <preset-name>");
            println!("\n{}", "See available presets:".bright_white().bold());
            println!("  kn mcp presets");
            return Ok(());
        }

        println!(
            "{}",
            format!("Installed MCPs ({}):", mcps.len())
                .bright_white()
                .bold()
        );
        println!();

        for mcp in mcps {
            println!("{}", format!("• {}", mcp.mcp.name).bright_cyan().bold());
            println!("  {}", mcp.mcp.description.dimmed());

            if detailed {
                println!(
                    "  Command: {}",
                    format!("{} {}", mcp.command.executable, mcp.command.args.join(" ")).white()
                );

                if !mcp.environment.is_empty() {
                    println!("  Environment:");
                    for key in mcp.environment.keys() {
                        println!("    {} (configured)", key.cyan());
                    }
                }

                if let Some(pkg) = &mcp.mcp.package {
                    println!("  Package: {}", pkg.bright_black());
                }
            }

            println!();
        }

        Ok(())
    }

    /// Show detailed information about an MCP
    fn show_mcp_info(&self, name: &str) -> Result<()> {
        let mcps_dir = kn_home::mcps_dir()?;
        let mcp_toml = mcps_dir.join(name).join("mcp.toml");

        if !mcp_toml.exists() {
            return Err(anyhow!("MCP '{}' is not installed", name));
        }

        let metadata = McpMetadata::from_file(&mcp_toml)?;

        println!(
            "{}",
            format!("MCP: {}", metadata.mcp.name).bright_cyan().bold()
        );
        println!("{}", metadata.mcp.description);
        println!();

        println!("{}", "Configuration:".bright_white().bold());
        println!("  Type: {}", metadata.command.command_type);
        println!("  Executable: {}", metadata.command.executable.cyan());
        println!("  Args: {}", metadata.command.args.join(" ").cyan());

        if let Some(pkg) = &metadata.mcp.package {
            println!("  Package: {}", pkg);
        }

        println!();
        println!("{}", "Options:".bright_white().bold());
        println!(
            "  Custom args: {}",
            if metadata.config.supports_custom_args {
                "yes".green()
            } else {
                "no".red()
            }
        );

        if !metadata.config.default_args.is_empty() {
            println!("  Default args: {}", metadata.config.default_args.join(" "));
        }

        if !metadata.config.required_env.is_empty() {
            println!(
                "  Required env: {}",
                metadata.config.required_env.join(", ").yellow()
            );
        }

        if !metadata.environment.is_empty() {
            println!();
            println!("{}", "Environment:".bright_white().bold());
            for (key, value) in &metadata.environment {
                println!("  {} = {}", key.cyan(), value.bright_black());
            }
        }

        Ok(())
    }

    /// Add an MCP to the current project
    fn add_to_project(&self, name: &str, args: Vec<String>, env_vars: Vec<String>) -> Result<()> {
        // Verify MCP is installed
        let mcps_dir = kn_home::mcps_dir()?;
        let mcp_dir = mcps_dir.join(name);

        if !mcp_dir.exists() {
            return Err(anyhow!(
                "MCP '{}' is not installed. Install it first with: kn mcp install {}",
                name,
                name
            ));
        }

        // Load project config
        let config_path = Path::new("kn.toml");
        if !config_path.exists() {
            return Err(anyhow!("kn.toml not found. Run 'kn init' first."));
        }

        // Parse environment variables
        let mut env_map = HashMap::new();
        for env in env_vars {
            let parts: Vec<&str> = env.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid env format: '{}'. Use KEY=VALUE", env));
            }
            env_map.insert(parts[0].to_string(), parts[1].to_string());
        }

        // Read and update config
        let content = fs::read_to_string(config_path)?;
        let mut config: KnConfig = toml::from_str(&content)?;

        // Initialize mcp section if needed
        if config.mcp.is_none() {
            config.mcp = Some(ProjectMcpConfig::default());
        }

        let mcp_config = config.mcp.as_mut().unwrap();

        // Add to enabled list if not already there
        if !mcp_config.enabled.contains(&name.to_string()) {
            mcp_config.enabled.push(name.to_string());
        }

        // Add project-specific configuration if provided
        if !args.is_empty() || !env_map.is_empty() {
            mcp_config.config.insert(
                name.to_string(),
                McpProjectOverride {
                    args,
                    environment: env_map,
                    enabled: true,
                },
            );
        }

        // Save updated config
        let new_content = toml::to_string_pretty(&config)?;
        fs::write(config_path, new_content)?;

        println!(
            "{}",
            format!("✓ Added MCP '{}' to project", name).green().bold()
        );
        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  • Run 'kn sync' to generate .opencode/opencode.json");
        println!("  • Configure in .opencode/opencode.json if needed");

        Ok(())
    }

    /// Remove an MCP from the current project
    fn remove_from_project(&self, name: &str) -> Result<()> {
        let config_path = Path::new("kn.toml");
        if !config_path.exists() {
            return Err(anyhow!("kn.toml not found"));
        }

        let content = fs::read_to_string(config_path)?;
        let mut config: KnConfig = toml::from_str(&content)?;

        if let Some(mcp_config) = config.mcp.as_mut() {
            mcp_config.enabled.retain(|n| n != name);
            mcp_config.config.remove(name);
        }

        let new_content = toml::to_string_pretty(&config)?;
        fs::write(config_path, new_content)?;

        println!(
            "{}",
            format!("✓ Removed MCP '{}' from project", name)
                .green()
                .bold()
        );
        Ok(())
    }

    /// Enable an MCP in the project (if disabled)
    fn enable_in_project(&self, name: &str) -> Result<()> {
        self.set_mcp_enabled(name, true)
    }

    /// Disable an MCP in the project
    fn disable_in_project(&self, name: &str) -> Result<()> {
        self.set_mcp_enabled(name, false)
    }

    fn set_mcp_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        let config_path = Path::new("kn.toml");
        if !config_path.exists() {
            return Err(anyhow!("kn.toml not found"));
        }

        let content = fs::read_to_string(config_path)?;
        let mut config: KnConfig = toml::from_str(&content)?;

        if let Some(mcp_config) = config.mcp.as_mut() {
            if let Some(override_config) = mcp_config.config.get_mut(name) {
                override_config.enabled = enabled;
            } else if enabled {
                // If enabling and no config exists, just add to enabled list
                if !mcp_config.enabled.contains(&name.to_string()) {
                    mcp_config.enabled.push(name.to_string());
                }
            } else {
                // If disabling, remove from enabled list
                mcp_config.enabled.retain(|n| n != name);
            }
        }

        let new_content = toml::to_string_pretty(&config)?;
        fs::write(config_path, new_content)?;

        let status = if enabled { "enabled" } else { "disabled" };
        println!("{}", format!("✓ MCP '{}' {}", name, status).green().bold());
        Ok(())
    }

    /// List available MCP presets
    fn list_presets(&self) -> Result<()> {
        println!("{}", "Available MCP Presets:".bright_white().bold());
        println!();

        let presets = McpMetadata::list_presets();
        for preset_name in presets {
            if let Some(preset) = McpMetadata::from_preset(&preset_name) {
                println!("{}", format!("• {}", preset.mcp.name).bright_cyan().bold());
                println!("  {}", preset.mcp.description.dimmed());

                if let Some(pkg) = preset.mcp.package {
                    println!("  Package: {}", pkg.bright_black());
                }

                if !preset.config.required_env.is_empty() {
                    println!(
                        "  Required env: {}",
                        preset.config.required_env.join(", ").yellow()
                    );
                }

                println!();
            }
        }

        println!("{}", "Install a preset:".bright_white().bold());
        println!("  kn mcp install <preset-name>");
        println!();
        println!("{}", "Or install any npm package:".bright_white().bold());
        println!("  kn mcp install @scope/package --as-name my-mcp");

        Ok(())
    }

    /// Search for MCP servers in the official registry
    fn search_registry(&self, query: &str, limit: usize, interactive_install: bool) -> Result<()> {
        println!(
            "{}",
            format!("🔍 Searching MCP Registry for '{}'...", query)
                .bright_cyan()
                .bold()
        );
        println!();

        // Fetch servers from registry
        let url = format!(
            "https://registry.modelcontextprotocol.io/v0/servers?limit={}",
            limit * 2 // Fetch more to account for filtering
        );

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .context("Failed to connect to MCP Registry")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Registry API returned error: {} {}",
                response.status(),
                response.text().unwrap_or_default()
            );
        }

        let registry_data: RegistryResponse = response
            .json()
            .context("Failed to parse registry response")?;

        // Filter results by query (case-insensitive search in name and description)
        let query_lower = query.to_lowercase();
        let results: Vec<&ServerEntry> = registry_data
            .servers
            .iter()
            .filter(|entry| {
                let name_match = entry.server.name.to_lowercase().contains(&query_lower);
                let desc_match = entry
                    .server
                    .description
                    .to_lowercase()
                    .contains(&query_lower);
                let title_match = entry
                    .server
                    .title
                    .as_ref()
                    .map(|t| t.to_lowercase().contains(&query_lower))
                    .unwrap_or(false);

                name_match || desc_match || title_match
            })
            .take(limit)
            .collect();

        if results.is_empty() {
            println!(
                "{}",
                format!("No MCP servers found matching '{}'", query).yellow()
            );
            println!();
            println!("{}", "Try:".bright_white());
            println!("  • A different search term");
            println!("  • kn mcp presets  (for built-in presets)");
            println!("  • Browse https://github.com/mcp");
            return Ok(());
        }

        println!(
            "{}",
            format!("Found {} matching server(s):", results.len())
                .bright_white()
                .bold()
        );
        println!();

        // Display results
        for (idx, entry) in results.iter().enumerate() {
            let title = entry.server.title.as_ref().unwrap_or(&entry.server.name);

            println!(
                "{} {}",
                format!("{}.", idx + 1).bright_black(),
                title.bright_cyan().bold()
            );
            println!("   Name: {}", entry.server.name.dimmed());
            println!("   Version: {}", entry.server.version.dimmed());
            println!("   {}", entry.server.description);

            if let Some(url) = &entry.server.website_url {
                println!("   Website: {}", url.bright_blue().underline());
            }

            if let Some(repo) = &entry.server.repository.url {
                println!("   Repository: {}", repo.bright_black());
            }

            // Show package type if available
            if let Some(pkg) = entry.server.packages.first() {
                println!(
                    "   Package: {} ({})",
                    pkg.identifier.bright_black(),
                    pkg.registry_type
                );
            }

            println!();
        }

        // Interactive installation
        if interactive_install && !results.is_empty() {
            use dialoguer::{theme::ColorfulTheme, Select};

            let selections: Vec<String> = results
                .iter()
                .enumerate()
                .map(|(idx, entry)| {
                    format!(
                        "{}. {} - {}",
                        idx + 1,
                        entry.server.title.as_ref().unwrap_or(&entry.server.name),
                        entry.server.description
                    )
                })
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an MCP server to install (or ESC to cancel)")
                .items(&selections)
                .default(0)
                .interact_opt()?;

            if let Some(idx) = selection {
                let selected = results[idx];
                self.install_from_registry(selected)?;
            } else {
                println!("{}", "Installation cancelled.".yellow());
            }
        } else if !interactive_install {
            println!("{}", "To install an MCP server:".bright_white().bold());
            println!("  kn mcp search <query> --install");
            println!("  kn mcp install <npm-package> --as-name <name>");
        }

        Ok(())
    }

    /// Install an MCP server from registry entry
    fn install_from_registry(&self, entry: &ServerEntry) -> Result<()> {
        println!();
        println!(
            "{}",
            format!(
                "Installing '{}' (v{})...",
                entry.server.name, entry.server.version
            )
            .bright_green()
            .bold()
        );

        // Extract package identifier
        let package = entry
            .server
            .packages
            .first()
            .ok_or_else(|| anyhow!("No package information available for this server"))?;

        // For now, only support npm packages
        if package.registry_type == "npm" {
            // Extract package name from identifier
            let npm_package = package.identifier.clone();

            // Use the server name as the MCP name (simplified, without version)
            let mcp_name = entry
                .server
                .name
                .split('/')
                .next_back()
                .unwrap_or(&entry.server.name)
                .to_string();

            println!(
                "{}",
                format!("  Installing npm package: {}", npm_package).cyan()
            );

            self.install_mcp(
                &npm_package,
                Some(mcp_name),
                None,
                vec![],
                vec![],
                false, // Don't skip validation
            )?;
        } else {
            anyhow::bail!(
                "Unsupported package type '{}'. Currently only npm packages are supported.",
                package.registry_type
            );
        }

        Ok(())
    }

    // Helper functions

    fn create_custom_mcp(
        &self,
        name: &str,
        command: &str,
        args: Vec<String>,
        env: HashMap<String, String>,
    ) -> Result<McpMetadata> {
        Ok(McpMetadata {
            mcp: crate::models::mcp::McpInfo {
                name: name.to_string(),
                description: format!("Custom MCP server: {}", name),
                package: None,
                version: None,
            },
            command: crate::models::mcp::CommandConfig {
                command_type: "local".to_string(),
                executable: command.to_string(),
                args,
            },
            environment: env,
            config: crate::models::mcp::McpConfigOptions {
                supports_custom_args: true,
                default_args: vec![],
                required_env: vec![],
            },
        })
    }

    /// Validate that an npm package exists
    fn validate_npm_package(&self, package: &str, skip_validation: bool) -> Result<()> {
        if skip_validation {
            return Ok(());
        }

        println!(
            "{}",
            format!("Validating npm package '{}'...", package).cyan()
        );

        // Use npm view to check if package exists
        let output = std::process::Command::new("npm")
            .args(["view", package, "version", "--json"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let version_output = String::from_utf8_lossy(&output.stdout);
                println!("{}", format!("✓ Package '{}' found", package).green());
                // Try to parse version for extra confirmation
                if !version_output.trim().is_empty() {
                    println!("  Version: {}", version_output.trim().bright_black());
                }
                Ok(())
            }
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow!(
                    "npm package '{}' not found.\n\nError: {}\n\nPlease check:\n  • Package name is correct (check npmjs.com)\n  • You have internet connection\n  • npm is properly configured",
                    package,
                    error_msg.trim()
                ))
            }
            Err(e) => {
                Err(anyhow!(
                    "Failed to validate npm package (npm command failed): {}\n\nTip: You can skip validation with --skip-validation flag",
                    e
                ))
            }
        }
    }

    fn create_npm_mcp(
        &self,
        name: &str,
        package: &str,
        env: HashMap<String, String>,
        skip_validation: bool,
    ) -> Result<McpMetadata> {
        // Validate npm package exists before creating metadata
        self.validate_npm_package(package, skip_validation)?;

        Ok(McpMetadata {
            mcp: crate::models::mcp::McpInfo {
                name: name.to_string(),
                description: format!("npm package: {}", package),
                package: Some(package.to_string()),
                version: None,
            },
            command: crate::models::mcp::CommandConfig {
                command_type: "local".to_string(),
                executable: "npx".to_string(),
                args: vec!["-y".to_string(), package.to_string()],
            },
            environment: env,
            config: crate::models::mcp::McpConfigOptions {
                supports_custom_args: false,
                default_args: vec![],
                required_env: vec![],
            },
        })
    }
}
