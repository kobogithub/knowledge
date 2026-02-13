use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct SkillsCommand {
    #[command(subcommand)]
    action: SkillsAction,
}

#[derive(Subcommand)]
enum SkillsAction {
    /// Install a skill from URL or local path
    Install(InstallArgs),
    /// List installed skills
    List,
}

#[derive(Args)]
struct InstallArgs {
    /// Skill name, URL, or local path
    source: String,

    /// Force reinstall if already exists
    #[arg(short, long)]
    force: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct SkillMetadata {
    name: String,
    #[serde(default)]
    scope: String,
    #[serde(default)]
    auto_invoke: bool,
    #[serde(default)]
    description: String,
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
    println!("{}", "📦 Installing skill...".bright_cyan().bold());

    let current_dir = std::env::current_dir()?;
    let skills_dir = current_dir.join("skills");

    // Create skills directory if it doesn't exist
    if !skills_dir.exists() {
        fs::create_dir_all(&skills_dir)
            .context("Failed to create skills directory")?;
        println!("{}", "  ✓ Created skills/ directory".green());
    }

    // Determine source type (URL, path, or name)
    let skill_content = if args.source.starts_with("http://") || args.source.starts_with("https://") {
        // Download from URL
        println!("  Downloading from {}...", args.source.bright_yellow());
        download_skill(&args.source)?
    } else if Path::new(&args.source).exists() {
        // Load from local path
        println!("  Loading from local path: {}", args.source.bright_yellow());
        fs::read_to_string(&args.source)
            .context("Failed to read skill file")?
    } else {
        // Try to resolve as skill name from agentskills.io
        let url = format!("https://raw.githubusercontent.com/agentskills/skills/main/{}/SKILL.md", args.source);
        println!("  Resolving skill '{}' from agentskills.io...", args.source.bright_yellow());
        download_skill(&url)?
    };

    // Parse and validate skill metadata
    let metadata = parse_skill_metadata(&skill_content)?;
    println!("  Skill: {} ({})", metadata.name.bright_green(), metadata.scope);

    // Determine target directory
    let skill_dir = skills_dir.join(&metadata.name);
    
    if skill_dir.exists() && !args.force {
        return Err(anyhow!(
            "Skill '{}' already exists. Use --force to overwrite.",
            metadata.name
        ));
    }

    // Create skill directory and write SKILL.md
    fs::create_dir_all(&skill_dir)
        .context("Failed to create skill directory")?;
    
    let skill_file = skill_dir.join("SKILL.md");
    fs::write(&skill_file, &skill_content)
        .context("Failed to write SKILL.md")?;

    println!("{}", format!("  ✓ Installed to {}", skill_dir.display()).green());

    // Update kn.toml
    update_config_with_skill(&current_dir, &metadata.name)?;

    println!("\n{}", "✓ Skill installed successfully!".bright_green().bold());
    println!("\n{}", "Next steps:".bright_white().bold());
    println!("  1. Review skills/{}/SKILL.md", metadata.name);
    println!("  2. Update AGENTS.md to reference the skill");

    Ok(())
}

// Public API for use by other commands (e.g., init)
pub fn install_skill_from_path(source: &str, silent: bool) -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let skills_dir = current_dir.join("skills");

    // Create skills directory if it doesn't exist
    if !skills_dir.exists() {
        fs::create_dir_all(&skills_dir)
            .context("Failed to create skills directory")?;
    }

    // Determine source type and download/load
    let skill_content = if source.starts_with("http://") || source.starts_with("https://") {
        download_skill(source)?
    } else if Path::new(source).exists() {
        fs::read_to_string(source)
            .context("Failed to read skill file")?
    } else {
        // Try from local skills/ directory first (for bundled skills)
        let local_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("skills")
            .join(source)
            .join("SKILL.md");
        
        if local_path.exists() {
            fs::read_to_string(&local_path)
                .context("Failed to read local skill")?
        } else {
            // Try agentskills.io
            let url = format!("https://raw.githubusercontent.com/agentskills/skills/main/{}/SKILL.md", source);
            download_skill(&url)?
        }
    };

    // Parse metadata
    let metadata = parse_skill_metadata(&skill_content)?;
    let skill_dir = skills_dir.join(&metadata.name);
    
    // Skip if already exists
    if skill_dir.exists() {
        if !silent {
            println!("{}", format!("  ⚠ Skill '{}' already exists, skipping", metadata.name).yellow());
        }
        return Ok(metadata.name);
    }

    // Create skill directory and write SKILL.md
    fs::create_dir_all(&skill_dir)
        .context("Failed to create skill directory")?;
    
    let skill_file = skill_dir.join("SKILL.md");
    fs::write(&skill_file, &skill_content)
        .context("Failed to write SKILL.md")?;

    if !silent {
        println!("{}", format!("  ✓ Installed {}", metadata.name).green());
    }

    // Update kn.toml
    let _ = update_config_with_skill(&current_dir, &metadata.name);

    Ok(metadata.name)
}

fn list_skills() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let skills_dir = current_dir.join("skills");

    if !skills_dir.exists() {
        println!("{}", "No skills directory found.".yellow());
        println!("Run 'kn skills install <skill-name>' to install your first skill.");
        return Ok(());
    }

    println!("{}", "Installed Skills:".bright_white().bold());
    println!();

    let mut found_skills = false;

    for entry in fs::read_dir(&skills_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let skill_file = path.join("SKILL.md");
            if skill_file.exists() {
                found_skills = true;
                match fs::read_to_string(&skill_file) {
                    Ok(content) => {
                        match parse_skill_metadata(&content) {
                            Ok(metadata) => {
                                println!("  {} {}", "●".bright_green(), metadata.name.bright_white().bold());
                                if !metadata.scope.is_empty() {
                                    println!("    Scope: {}", metadata.scope.dimmed());
                                }
                                if !metadata.description.is_empty() {
                                    println!("    {}", metadata.description.dimmed());
                                }
                                println!("    Auto-invoke: {}", if metadata.auto_invoke { "yes".green() } else { "no".dimmed() });
                                println!();
                            }
                            Err(_) => {
                                println!("  {} {} {}", "●".yellow(), entry.file_name().to_string_lossy(), "(invalid metadata)".dimmed());
                                println!();
                            }
                        }
                    }
                    Err(_) => {
                        println!("  {} {} {}", "●".red(), entry.file_name().to_string_lossy(), "(error reading)".dimmed());
                        println!();
                    }
                }
            }
        }
    }

    if !found_skills {
        println!("{}", "  No skills found.".yellow());
        println!("  Run 'kn skills install <name>' to install a skill.");
    }

    Ok(())
}

fn download_skill(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)
        .context("Failed to download skill")?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to download skill: HTTP {}",
            response.status()
        ));
    }

    response.text()
        .context("Failed to read response body")
}

fn parse_skill_metadata(content: &str) -> Result<SkillMetadata> {
    // Look for YAML frontmatter between --- markers
    if !content.starts_with("---") {
        return Err(anyhow!("Skill file missing YAML frontmatter"));
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow!("Invalid YAML frontmatter format"));
    }

    let yaml_str = parts[1].trim();
    let metadata: SkillMetadata = serde_yaml::from_str(yaml_str)
        .context("Failed to parse YAML frontmatter")?;

    if metadata.name.is_empty() {
        return Err(anyhow!("Skill must have a 'name' field in frontmatter"));
    }

    Ok(metadata)
}

fn update_config_with_skill(project_dir: &Path, skill_name: &str) -> Result<()> {
    let config_path = project_dir.join("kn.toml");

    if !config_path.exists() {
        println!("{}", "  ⚠ kn.toml not found, skipping config update".yellow());
        return Ok(());
    }

    let content = fs::read_to_string(&config_path)?;
    let mut config: toml::Value = toml::from_str(&content)?;

    // Add skill to [skills.enabled] array if not already present
    if let Some(skills_table) = config.get_mut("skills") {
        if let Some(skills_table) = skills_table.as_table_mut() {
            let enabled = skills_table
                .entry("enabled")
                .or_insert(toml::Value::Array(vec![]));

            if let Some(enabled_array) = enabled.as_array_mut() {
                let skill_value = toml::Value::String(skill_name.to_string());
                if !enabled_array.contains(&skill_value) {
                    enabled_array.push(skill_value);
                    fs::write(&config_path, toml::to_string_pretty(&config)?)?;
                    println!("{}", "  ✓ Updated kn.toml".green());
                }
            }
        }
    }

    Ok(())
}
