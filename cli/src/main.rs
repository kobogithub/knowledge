use anyhow::Result;
use clap::{Parser, Subcommand};

use kn::commands::agents::AgentsCommand;
use kn::commands::beads::BeadsCommand;
use kn::commands::doctor::DoctorCommand;
use kn::commands::init::InitCommand;
use kn::commands::mcp::{McpCommands, McpHandler};
use kn::commands::skills::SkillsCommand;
use kn::commands::stack::StackCommand;
use kn::commands::sync::SyncCommand;
use kn::commands::update::UpdateCommand;

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
    /// List and inspect stack-presets (bundles of skills)
    Stack(StackCommand),
    /// Manage agents (install, list)
    Agents(AgentsCommand),
    /// Sync skills and agents from ~/.kn/ to project
    Sync(SyncCommand),
    /// Update kn CLI to latest version
    Update(UpdateCommand),
    /// Beads issue tracking utilities
    Beads(BeadsCommand),
    /// Manage MCP (Model Context Protocol) servers
    #[command(subcommand)]
    Mcp(McpCommands),
    /// Check system dependencies and installation
    Doctor(DoctorCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(cmd) => cmd.execute()?,
        Commands::Skills(cmd) => cmd.execute()?,
        Commands::Stack(cmd) => cmd.execute()?,
        Commands::Agents(cmd) => cmd.execute()?,
        Commands::Sync(cmd) => cmd.execute()?,
        Commands::Update(cmd) => cmd.execute()?,
        Commands::Beads(cmd) => cmd.execute()?,
        Commands::Mcp(cmd) => {
            let handler = McpHandler::new();
            handler.handle(cmd)?;
        }
        Commands::Doctor(cmd) => cmd.execute()?,
    }

    Ok(())
}
