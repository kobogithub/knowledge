use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::config::{KnConfig, OpenCodeConfig, WorkspaceStandard};
use crate::core::{kn_home, symlinks};

#[derive(Args)]
pub struct SyncCommand {
    /// Project directory (default: current directory)
    #[arg(short, long)]
    project: Option<PathBuf>,
}

impl SyncCommand {
    pub fn execute(&self) -> Result<()> {
        let project_root = if let Some(ref path) = self.project {
            path.clone()
        } else {
            env::current_dir().context("Failed to get current directory")?
        };

        println!(
            "{}",
            "🔄 Syncing skills and agents from ~/.kn/..."
                .bright_cyan()
                .bold()
        );

        // Check if kn.toml exists
        let config_path = project_root.join("kn.toml");
        if !config_path.exists() {
            println!(
                "{}",
                "⚠ kn.toml not found. Run 'kn init' first to configure this project.".yellow()
            );
            return Ok(());
        }

        // Load config
        let config = KnConfig::load(&config_path).context("Failed to load kn.toml")?;

        println!("  Project: {}", config.project.name.bright_white().bold());
        println!("  Workspace: {}", config.project.workspace_standard.cyan());

        let workspace =
            WorkspaceStandard::parse(&config.project.workspace_standard).unwrap_or_default();

        // Sync skills
        if !config.skills.enabled.is_empty() {
            println!("\n{}", "Skills:".bright_white().bold());

            // Verify all skills exist in ~/.kn/skills/
            let skills_dir = kn_home::skills_dir()?;
            let mut missing_skills = Vec::new();
            let mut existing_skills = Vec::new();

            for skill_name in &config.skills.enabled {
                let skill_path = skills_dir.join(skill_name);
                if skill_path.exists() {
                    existing_skills.push(skill_name.clone());
                    println!("  {} {}", "✓".green(), skill_name);
                } else {
                    missing_skills.push(skill_name.clone());
                    println!(
                        "  {} {} {}",
                        "✗".red(),
                        skill_name,
                        "(not found in ~/.kn/skills/)".dimmed()
                    );
                }
            }

            if !missing_skills.is_empty() {
                println!(
                    "\n{}",
                    format!(
                        "⚠ {} skills missing. Install them with:",
                        missing_skills.len()
                    )
                    .yellow()
                );
                for skill in &missing_skills {
                    println!("  kn skills install ./skills/{}", skill);
                }
            }

            // Create symlinks for existing skills
            if !existing_skills.is_empty() {
                symlinks::create_skill_symlinks(&project_root, &existing_skills, &workspace)?;
                println!(
                    "{}",
                    format!("  ✓ Created {} skill symlinks", existing_skills.len()).green()
                );
            }
        } else {
            println!("\n{}", "No skills enabled in kn.toml".dimmed());
        }

        // Sync agents
        if !config.agents.is_empty() {
            println!("\n{}", "Agents:".bright_white().bold());

            let agents_dir = kn_home::agents_dir()?;
            let mut missing_agents = Vec::new();
            let mut existing_agents = Vec::new();

            for (agent_name, agent_config) in &config.agents {
                let agent_path = agents_dir.join(agent_name);
                if agent_path.exists() {
                    existing_agents.push(agent_name.clone());
                    println!(
                        "  {} {} {}",
                        "✓".green(),
                        agent_name,
                        format!("({})", agent_config.id).dimmed()
                    );
                } else {
                    missing_agents.push(agent_name.clone());
                    println!(
                        "  {} {} {}",
                        "✗".red(),
                        agent_name,
                        "(not found in ~/.kn/agents/)".dimmed()
                    );
                }
            }

            if !missing_agents.is_empty() {
                println!(
                    "\n{}",
                    format!(
                        "⚠ {} agents missing. Install them with:",
                        missing_agents.len()
                    )
                    .yellow()
                );
                for agent in &missing_agents {
                    println!("  kn agents install ./agents/{}", agent);
                }
            }

            // Create symlinks for existing agents
            if !existing_agents.is_empty() {
                symlinks::create_agent_symlinks(&project_root, &existing_agents, &workspace)?;
                println!(
                    "{}",
                    format!("  ✓ Created {} agent symlinks", existing_agents.len()).green()
                );
            }
        } else {
            println!("\n{}", "No agents enabled in kn.toml".dimmed());
        }

        // Sync formulas
        println!("\n{}", "Formulas:".bright_white().bold());
        let formulas_source = kn_home::formulas_dir()?;
        let formulas_target = project_root.join(".beads/formulas");

        if formulas_source.exists() {
            fs::create_dir_all(&formulas_target)?;
            let mut formula_count = 0;

            for entry in fs::read_dir(&formulas_source)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json")
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".formula.json"))
                {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    let dest = formulas_target.join(&name);
                    if !dest.exists() {
                        fs::copy(&path, &dest)?;
                        println!("  {} {} {}", "✓".green(), name, "(new)".dimmed());
                        formula_count += 1;
                    } else {
                        println!("  {} {}", "✓".green(), name);
                    }
                }
            }

            if formula_count > 0 {
                println!(
                    "{}",
                    format!("  ✓ Installed {} new formula(s)", formula_count).green()
                );
            }
        } else {
            println!(
                "  {} {}",
                "ℹ".bright_blue(),
                "No formulas in ~/.kn/formulas/ (install with: kn update)".dimmed()
            );
        }

        // Sync MCPs (generate .opencode/opencode.json)
        if let Some(mcp_config) = &config.mcp {
            if !mcp_config.enabled.is_empty() {
                println!("\n{}", "MCPs:".bright_white().bold());

                // Check which MCPs are installed
                let mcps_dir = kn_home::mcps_dir()?;
                let mut missing_mcps = Vec::new();
                let mut existing_mcps = Vec::new();

                for mcp_name in &mcp_config.enabled {
                    let mcp_dir = mcps_dir.join(mcp_name);
                    if mcp_dir.exists() {
                        existing_mcps.push(mcp_name.clone());

                        // Check if disabled in project config
                        let enabled = mcp_config
                            .config
                            .get(mcp_name)
                            .map(|cfg| cfg.enabled)
                            .unwrap_or(true);

                        if enabled {
                            println!("  {} {}", "✓".green(), mcp_name);
                        } else {
                            println!("  {} {} {}", "○".dimmed(), mcp_name, "(disabled)".dimmed());
                        }
                    } else {
                        missing_mcps.push(mcp_name.clone());
                        println!(
                            "  {} {} {}",
                            "✗".red(),
                            mcp_name,
                            "(not found in ~/.kn/mcps/)".dimmed()
                        );
                    }
                }

                if !missing_mcps.is_empty() {
                    println!(
                        "\n{}",
                        format!("⚠ {} MCPs missing. Install them with:", missing_mcps.len())
                            .yellow()
                    );
                    for mcp in &missing_mcps {
                        println!("  kn mcp install {}", mcp);
                    }
                }

                // Generate .opencode/opencode.json
                let opencode_dir = project_root.join(".opencode");
                let opencode_json = opencode_dir.join("opencode.json");

                // Load existing config if present to preserve other settings
                let mut opencode_config = if opencode_json.exists() {
                    match OpenCodeConfig::from_file(&opencode_json) {
                        Ok(cfg) => cfg,
                        Err(_) => {
                            println!(
                                "  {} Could not parse existing opencode.json, creating new one",
                                "⚠".yellow()
                            );
                            OpenCodeConfig::new()
                        }
                    }
                } else {
                    OpenCodeConfig::new()
                };

                // Generate MCP configuration
                match OpenCodeConfig::generate_from_project(mcp_config) {
                    Ok(generated) => {
                        opencode_config.mcp = generated.mcp;

                        // Save to file
                        match opencode_config.save(&opencode_json) {
                            Ok(_) => {
                                println!(
                                    "{}",
                                    format!(
                                        "  ✓ Generated .opencode/opencode.json with {} MCPs",
                                        opencode_config.mcp.as_ref().map(|m| m.len()).unwrap_or(0)
                                    )
                                    .green()
                                );
                            }
                            Err(e) => {
                                println!(
                                    "  {} Failed to save .opencode/opencode.json: {}",
                                    "✗".red(),
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("  {} Failed to generate MCP config: {}", "✗".red(), e);
                    }
                }
            }
        } else {
            println!("\n{}", "No MCPs enabled in kn.toml".dimmed());
        }

        println!("\n{}", "✓ Sync completed!".bright_green().bold());

        Ok(())
    }
}
