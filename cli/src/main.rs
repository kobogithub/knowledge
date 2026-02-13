use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
use commands::beads::BeadsCommand;
use commands::init::InitCommand;
use commands::mcp::{McpCommands, McpHandler};
use commands::skills::SkillsCommand;

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
    /// Manage skills (install, list, update)
    Skills(SkillsCommand),
    /// Beads issue tracking utilities
    Beads(BeadsCommand),
    /// Manage MCP (Model Context Protocol) servers
    #[command(subcommand)]
    Mcp(McpCommands),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(cmd) => cmd.execute()?,
        Commands::Skills(cmd) => cmd.execute()?,
        Commands::Beads(cmd) => cmd.execute()?,
        Commands::Mcp(cmd) => {
            let handler = McpHandler::new();
            handler.handle(cmd)?;
        }
    }

    Ok(())
}
