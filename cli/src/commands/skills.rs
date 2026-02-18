use anyhow::{anyhow, Context, Result};
use clap::{Args, Subcommand};
use colored::*;
use std::fs;
use std::path::Path;

use crate::core::kn_home;
use crate::models::SkillMetadata;

#[derive(Args)]
pub struct SkillsCommand {
    #[command(subcommand)]
    action: SkillsAction,
}

#[derive(Subcommand)]
enum SkillsAction {
    /// Install a skill to ~/.kn/skills/ from local path or URL
    Install(InstallArgs),
    /// List installed skills in ~/.kn/skills/
    List,
}

#[derive(Args)]
struct InstallArgs {
    /// Skill name or local path (e.g., rust-best-practices or ./skills/rust-best-practices)
    source: String,

    /// Force reinstall if already exists
    #[arg(short, long)]
    force: bool,
}

impl SkillsCommand {
    pub fn execute(&self) -> Result<()> {
        match &self.action {
            SkillsAction::Install(args) => install_skill(args),
            SkillsAction::List => list_skills(),
        }
    }
}

fn install_skill(args: &InstallArgs) -> Result<()> {
    println!(
        "{}",
        "📦 Installing skill to ~/.kn/skills/..."
            .bright_cyan()
            .bold()
    );

    // Ensure ~/.kn/skills/ exists
    kn_home::ensure_kn_home()?;
    let global_skills_dir = kn_home::skills_dir()?;

    // Determine source
    let source_path = Path::new(&args.source);

    let skill_content = if source_path.exists() {
        // Load from local path
        let skill_file = if source_path.is_dir() {
            source_path.join("SKILL.md")
        } else {
            source_path.to_path_buf()
        };

        if !skill_file.exists() {
            return Err(anyhow!("SKILL.md not found in {}", source_path.display()));
        }

        println!(
            "  Loading from local path: {}",
            skill_file.display().to_string().bright_yellow()
        );
        fs::read_to_string(&skill_file).context("Failed to read SKILL.md")?
    } else if args.source.starts_with("http://") || args.source.starts_with("https://") {
        // Download from URL
        println!("  Downloading from {}...", args.source.bright_yellow());
        download_skill(&args.source)?
    } else {
        return Err(anyhow!(
            "Source '{}' not found. Provide a valid local path or URL.",
            args.source
        ));
    };

    // Parse metadata
    let metadata = SkillMetadata::from_content(&skill_content)?;
    println!("  Skill: {}", metadata.name.bright_green().bold());
    if !metadata.description.is_empty() {
        println!("  Description: {}", metadata.description.dimmed());
    }

    // Determine target directory
    let skill_dir = global_skills_dir.join(&metadata.name);

    if skill_dir.exists() && !args.force {
        return Err(anyhow!(
            "Skill '{}' already exists in ~/.kn/skills/. Use --force to overwrite.",
            metadata.name
        ));
    }

    // Create skill directory and write SKILL.md
    fs::create_dir_all(&skill_dir).context("Failed to create skill directory")?;

    let skill_file = skill_dir.join("SKILL.md");
    fs::write(&skill_file, &skill_content).context("Failed to write SKILL.md")?;

    println!(
        "{}",
        format!("  ✓ Installed to {}", skill_dir.display()).green()
    );

    println!(
        "\n{}",
        "✓ Skill installed successfully!".bright_green().bold()
    );
    println!("\n{}", "Next steps:".bright_white().bold());
    println!("  1. Run 'kn sync' in your project to create symlinks");
    println!("  2. Or use 'kn init' in a new project and select this skill");

    Ok(())
}

/// Public API for batch installation (used by kn init)
pub fn install_skill_from_repo(skill_name: &str, repo_path: &Path, force: bool) -> Result<()> {
    kn_home::ensure_kn_home()?;
    let global_skills_dir = kn_home::skills_dir()?;

    // Source path in the repo
    let source_dir = repo_path.join("skills").join(skill_name);
    let source_file = source_dir.join("SKILL.md");

    if !source_file.exists() {
        return Err(anyhow!(
            "Skill '{}' not found in repository at {}",
            skill_name,
            source_dir.display()
        ));
    }

    let skill_content = fs::read_to_string(&source_file)?;
    let metadata = SkillMetadata::from_content(&skill_content)?;

    // Target directory in ~/.kn/skills/
    let skill_dir = global_skills_dir.join(&metadata.name);

    if skill_dir.exists() && !force {
        // Already installed, skip
        return Ok(());
    }

    // Create and copy
    fs::create_dir_all(&skill_dir)?;
    let target_file = skill_dir.join("SKILL.md");
    fs::write(&target_file, &skill_content)?;

    println!("{}", format!("  ✓ Installed {}", metadata.name).green());

    Ok(())
}

fn list_skills() -> Result<()> {
    let global_skills_dir = kn_home::skills_dir()?;

    if !global_skills_dir.exists() {
        println!("{}", "No skills installed yet.".yellow());
        println!("Run 'kn skills install <path>' to install your first skill.");
        println!("Example: kn skills install ./skills/rust-best-practices");
        return Ok(());
    }

    println!("{}", "Installed Skills:".bright_white().bold());
    println!(
        "{} {}",
        "Location:".dimmed(),
        global_skills_dir.display().to_string().dimmed()
    );
    println!();

    let skills = kn_home::list_installed_skills()?;

    if skills.is_empty() {
        println!("{}", "  No skills found.".yellow());
        println!("  Run 'kn skills install <path>' to install a skill.");
        return Ok(());
    }

    for skill_name in skills {
        let skill_path = global_skills_dir.join(&skill_name).join("SKILL.md");

        match fs::read_to_string(&skill_path) {
            Ok(content) => match SkillMetadata::from_content(&content) {
                Ok(metadata) => {
                    println!(
                        "  {} {}",
                        "●".bright_green(),
                        metadata.name.bright_white().bold()
                    );
                    if !metadata.description.is_empty() {
                        println!("    {}", metadata.description.dimmed());
                    }
                    if !metadata.tags.is_empty() {
                        println!("    Tags: {}", metadata.tags.join(", ").dimmed());
                    }
                    println!(
                        "    Auto-invoke: {}",
                        if metadata.auto_invoke {
                            "yes".green()
                        } else {
                            "no".dimmed()
                        }
                    );
                    println!();
                }
                Err(_) => {
                    println!(
                        "  {} {} {}",
                        "●".yellow(),
                        skill_name,
                        "(invalid metadata)".dimmed()
                    );
                    println!();
                }
            },
            Err(_) => {
                println!(
                    "  {} {} {}",
                    "●".red(),
                    skill_name,
                    "(error reading)".dimmed()
                );
                println!();
            }
        }
    }

    Ok(())
}

fn download_skill(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url).context("Failed to download skill")?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to download skill: HTTP {}",
            response.status()
        ));
    }

    response.text().context("Failed to read response body")
}
