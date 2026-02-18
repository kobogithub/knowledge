use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use std::env;
use std::path::PathBuf;

use crate::config::{KnConfig, WorkspaceStandard};
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

        let workspace = WorkspaceStandard::from_str(&config.project.workspace_standard)
            .unwrap_or(WorkspaceStandard::Both);

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

        println!("\n{}", "✓ Sync completed!".bright_green().bold());

        Ok(())
    }
}
