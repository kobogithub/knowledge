use anyhow::{bail, Result};
use clap::{Args, Subcommand};
use colored::*;

use crate::core::kn_home;
use crate::core::stack::{self, StackPreset};

#[derive(Args)]
pub struct StackCommand {
    #[command(subcommand)]
    command: StackSubcommand,
}

#[derive(Subcommand)]
enum StackSubcommand {
    /// List available stack-presets and the skills they bundle
    List,
    /// Show a single stack-preset's details
    Show {
        /// Preset name (see `kn stack list`)
        name: String,
    },
}

impl StackCommand {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            StackSubcommand::List => list_presets(),
            StackSubcommand::Show { name } => show_preset(name),
        }
    }
}

fn list_presets() -> Result<()> {
    let presets = stack::list_presets()?;

    if presets.is_empty() {
        println!(
            "{}",
            "No stack-presets available in ~/.kn/stacks/.".yellow()
        );
        println!(
            "  {} Reinstall kn to stage the shipped presets, or add your own \
             ~/.kn/stacks/<name>.toml",
            "ℹ".bright_blue()
        );
        return Ok(());
    }

    println!("{}", "Available stack-presets:".bright_cyan().bold());
    let skills_dir = kn_home::skills_dir()?;
    for preset in &presets {
        println!(
            "\n  {}  {}",
            preset.name.bright_white().bold(),
            preset.description.dimmed()
        );
        let missing = preset.missing_skills_in(&skills_dir);
        for skill in &preset.skills {
            if missing.contains(skill) {
                println!("    {} {} {}", "-".dimmed(), skill, "(missing)".red());
            } else {
                println!("    {} {}", "-".dimmed(), skill);
            }
        }
    }
    Ok(())
}

fn show_preset(name: &str) -> Result<()> {
    let preset = stack::load_preset_for_use(name)?;

    print_preset(&preset)?;
    Ok(())
}

fn print_preset(preset: &StackPreset) -> Result<()> {
    if preset.skills.is_empty() {
        bail!("Preset '{}' has no skills", preset.name);
    }
    println!("{}", preset.name.bright_white().bold());
    if !preset.description.is_empty() {
        println!("  {}", preset.description.dimmed());
    }
    let skills_dir = kn_home::skills_dir()?;
    let missing = preset.missing_skills_in(&skills_dir);
    println!("  {}", "skills:".bright_cyan());
    for skill in &preset.skills {
        if missing.contains(skill) {
            println!("    {} {}", skill, "(missing)".red());
        } else {
            println!("    {}", skill);
        }
    }
    Ok(())
}
