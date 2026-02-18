use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use colored::*;
use std::fs;
use std::path::Path;

use crate::core::kn_home;
use crate::models::AgentMetadata;

#[derive(Args)]
pub struct AgentsCommand {
    #[command(subcommand)]
    action: AgentsAction,
}

#[derive(Subcommand)]
enum AgentsAction {
    /// Install an agent to ~/.kn/agents/ from local path
    Install(InstallArgs),
    /// List installed agents in ~/.kn/agents/
    List,
}

#[derive(Args)]
struct InstallArgs {
    /// Agent name or local path (e.g., rust or ./agents/rust)
    source: Vec<String>,

    /// Force reinstall if already exists
    #[arg(short, long)]
    force: bool,
}

impl AgentsCommand {
    pub fn execute(&self) -> Result<()> {
        match &self.action {
            AgentsAction::Install(args) => install_agents(args),
            AgentsAction::List => list_agents(),
        }
    }
}

fn install_agents(args: &InstallArgs) -> Result<()> {
    println!(
        "{}",
        "📦 Installing agents to ~/.kn/agents/..."
            .bright_cyan()
            .bold()
    );

    // Ensure ~/.kn/agents/ exists
    kn_home::ensure_kn_home()?;
    let global_agents_dir = kn_home::agents_dir()?;

    let mut installed_count = 0;
    let mut skipped_count = 0;

    for source in &args.source {
        match install_single_agent(source, &global_agents_dir, args.force) {
            Ok(installed) => {
                if installed {
                    installed_count += 1;
                } else {
                    skipped_count += 1;
                }
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("  ⚠ Failed to install {}: {}", source, e).yellow()
                );
            }
        }
    }

    println!(
        "\n{}",
        format!(
            "✓ Installed {} agents, skipped {} (already exists)",
            installed_count, skipped_count
        )
        .bright_green()
        .bold()
    );

    if installed_count > 0 {
        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  1. Run 'kn init' in a project and select these agents");
        println!("  2. Or use 'kn sync' to update existing projects");
    }

    Ok(())
}

fn install_single_agent(source: &str, global_agents_dir: &Path, force: bool) -> Result<bool> {
    let source_path = Path::new(source);

    let agent_dir = if source_path.exists() {
        // Load from local path
        if source_path.is_dir() {
            source_path.to_path_buf()
        } else {
            return Err(anyhow!(
                "Agent source must be a directory, got file: {}",
                source
            ));
        }
    } else {
        return Err(anyhow!("Agent directory not found: {}", source));
    };

    let agent_file = agent_dir.join("AGENTS.md");
    if !agent_file.exists() {
        return Err(anyhow!("AGENTS.md not found in {}", agent_dir.display()));
    }

    // Parse metadata
    let metadata = AgentMetadata::from_file(&agent_file)?;

    println!("  Agent: {}", metadata.name.bright_green().bold());
    if !metadata.description.is_empty() {
        println!("    {}", metadata.description.dimmed());
    }
    println!(
        "    Required skills: {}",
        if metadata.required_skills.is_empty() {
            "none".dimmed().to_string()
        } else {
            metadata.required_skills.join(", ").cyan().to_string()
        }
    );

    // Target directory in ~/.kn/agents/
    let target_dir = global_agents_dir.join(&metadata.name);

    if target_dir.exists() && !force {
        println!(
            "{}",
            format!("  ⚠ Agent '{}' already exists, skipping", metadata.name).yellow()
        );
        return Ok(false);
    }

    // Copy entire agent directory
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }

    copy_dir_recursive(&agent_dir, &target_dir)?;

    println!(
        "{}",
        format!("  ✓ Installed to {}", target_dir.display()).green()
    );

    Ok(true)
}

fn list_agents() -> Result<()> {
    let global_agents_dir = kn_home::agents_dir()?;

    if !global_agents_dir.exists() {
        println!("{}", "No agents installed yet.".yellow());
        println!("Run 'kn agents install <path>' to install your first agent.");
        println!("Example: kn agents install ./agents/rust");
        return Ok(());
    }

    println!("{}", "Installed Agents:".bright_white().bold());
    println!(
        "{} {}",
        "Location:".dimmed(),
        global_agents_dir.display().to_string().dimmed()
    );
    println!();

    let agents = kn_home::list_installed_agents()?;

    if agents.is_empty() {
        println!("{}", "  No agents found.".yellow());
        println!("  Run 'kn agents install <path>' to install an agent.");
        return Ok(());
    }

    for agent_name in agents {
        let agent_path = global_agents_dir.join(&agent_name).join("AGENTS.md");

        match AgentMetadata::from_file(&agent_path) {
            Ok(metadata) => {
                println!(
                    "  {} {} {}",
                    "●".bright_green(),
                    metadata.name.bright_white().bold(),
                    format!("({})", metadata.id_prefix).dimmed()
                );
                if !metadata.description.is_empty() {
                    println!("    {}", metadata.description.dimmed());
                }
                if !metadata.required_skills.is_empty() {
                    println!(
                        "    Required skills: {}",
                        metadata.required_skills.join(", ").cyan()
                    );
                }
                if !metadata.tags.is_empty() {
                    println!("    Tags: {}", metadata.tags.join(", ").dimmed());
                }
                println!();
            }
            Err(_) => {
                println!(
                    "  {} {} {}",
                    "●".yellow(),
                    agent_name,
                    "(invalid metadata)".dimmed()
                );
                println!();
            }
        }
    }

    Ok(())
}

/// Recursively copy directory contents
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// Public API for batch installation (used by other commands)
pub fn install_agent_from_repo(agent_name: &str, repo_path: &Path, force: bool) -> Result<()> {
    kn_home::ensure_kn_home()?;
    let global_agents_dir = kn_home::agents_dir()?;

    let source_dir = repo_path.join("agents").join(agent_name);
    let source_file = source_dir.join("AGENTS.md");

    if !source_file.exists() {
        return Err(anyhow!(
            "Agent '{}' not found in repository at {}",
            agent_name,
            source_dir.display()
        ));
    }

    let target_dir = global_agents_dir.join(agent_name);

    if target_dir.exists() && !force {
        // Already installed
        return Ok(());
    }

    // Copy directory
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }

    copy_dir_recursive(&source_dir, &target_dir)?;

    Ok(())
}
