use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
use commands::init::InitCommand;

#[derive(Parser)]
#[command(name = "kn")]
#[command(author, version, about = "CLI tool for AI-assisted development workflows", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize AI agent workflow in current project
    Init(InitCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(cmd) => cmd.execute()?,
    }

    Ok(())
}
