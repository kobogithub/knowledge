use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::skills::install_skill_from_path;

#[derive(Args)]
pub struct InitCommand {
    /// Project type (auto-detected if not specified)
    #[arg(short, long)]
    project_type: Option<String>,

    /// Skip interactive prompts
    #[arg(short = 'y', long)]
    yes: bool,

    /// Skip auto-installation of recommended skills
    #[arg(long)]
    no_skills: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectInfo {
    name: String,
    project_type: ProjectType,
    root_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProjectType {
    Rust,
    Node,
    Python,
    Go,
    Monorepo,
    Unknown,
}

impl InitCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", "🚀 Initializing AI agent workflow...".bright_cyan().bold());

        let current_dir = std::env::current_dir()?;
        let project_info = self.detect_project(&current_dir)?;

        println!("\n{}", "Detected project:".bright_white().bold());
        println!("  Name: {}", project_info.name.bright_yellow());
        println!("  Type: {:?}", project_info.project_type);
        println!("  Root: {}", project_info.root_dir.display());

        if !self.yes {
            println!("\n{}", "Continue? (y/n): ".bright_white());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("{}", "Aborted.".yellow());
                return Ok(());
            }
        }

        // Create AGENTS.md
        self.create_agents_file(&project_info)?;
        
        // Create kn.toml
        self.create_config_file(&project_info)?;

        // Auto-install recommended skills (unless --no-skills is specified)
        if !self.no_skills {
            self.install_recommended_skills(&project_info)?;
        }

        // Initialize beads if not present
        self.init_beads(&project_info)?;

        println!("\n{}", "✓ Initialization complete!".bright_green().bold());
        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  1. Review and customize AGENTS.md");
        println!("  2. Run: bd init (if not already initialized)");
        if !self.no_skills {
            println!("  3. Review installed skills: kn skills list");
        } else {
            println!("  3. Install skills: kn skills install <skill-name>");
        }

        Ok(())
    }

    fn detect_project(&self, dir: &Path) -> Result<ProjectInfo> {
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();

        let project_type = if let Some(ref pt) = self.project_type {
            match pt.as_str() {
                "rust" => ProjectType::Rust,
                "node" => ProjectType::Node,
                "python" => ProjectType::Python,
                "go" => ProjectType::Go,
                "monorepo" => ProjectType::Monorepo,
                _ => ProjectType::Unknown,
            }
        } else {
            // Auto-detect
            if dir.join("Cargo.toml").exists() {
                ProjectType::Rust
            } else if dir.join("package.json").exists() {
                ProjectType::Node
            } else if dir.join("pyproject.toml").exists() || dir.join("setup.py").exists() {
                ProjectType::Python
            } else if dir.join("go.mod").exists() {
                ProjectType::Go
            } else if dir.join("pnpm-workspace.yaml").exists() || dir.join("lerna.json").exists() {
                ProjectType::Monorepo
            } else {
                ProjectType::Unknown
            }
        };

        Ok(ProjectInfo {
            name,
            project_type,
            root_dir: dir.to_path_buf(),
        })
    }

    fn create_agents_file(&self, project: &ProjectInfo) -> Result<()> {
        let agents_path = project.root_dir.join("AGENTS.md");
        
        if agents_path.exists() {
            println!("{}", "  ⚠ AGENTS.md already exists, skipping...".yellow());
            return Ok(());
        }

        let template = self.get_agents_template(project);
        fs::write(&agents_path, template)
            .context("Failed to write AGENTS.md")?;

        println!("{}", "  ✓ Created AGENTS.md".green());
        Ok(())
    }

    fn get_agents_template(&self, project: &ProjectInfo) -> String {
        format!(r#"# Agent Instructions

This project uses **bd** (beads) for issue tracking with a multi-agent workflow.

## Project: {}

**Type:** {:?}

## Agent Roles

Each agent has specialized responsibilities and autonomy to close their own tasks:

- **Planner Agent** - Coordinates work, creates epics, assigns tasks
- **Implementation Agent** - Writes code, fixes bugs, implements features
- **Review Agent** - Code review, quality assurance, documentation

## Quick Reference

```bash
# Find your work
bd ready -l <your-label>
bd list --assignee <agent-id>

# Claim and start work
bd update <task-id> --claim
bd agent state <agent-id> working

# Report progress
bd comments add <task-id> "[Agent Name] Progress update..."

# Complete your work
bd comments add <task-id> "[Agent Name] ✓ Completed: details..."
bd close <task-id>
bd agent state <agent-id> done

# Sync with git
bd sync
git add .beads/issues.jsonl
git commit -m "<Agent>: description"
git push
```

## Workflow Principles

1. **Autonomy**: Each agent closes their own tasks when complete
2. **Transparency**: Report progress through comments
3. **Coordination**: Use issue references to coordinate with other agents
4. **Ownership**: You own your tasks from claim to completion
5. **Honesty**: Only close when actually complete and tested

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps:

1. **File issues for remaining work** - Create issues for anything incomplete
2. **Run quality gates** - Tests, linters, builds
3. **Update issue status** - Close finished work
4. **PUSH TO REMOTE**:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Verify** - All changes committed AND pushed

**CRITICAL**: Work is NOT complete until `git push` succeeds.
"#, project.name, project.project_type)
    }

    fn create_config_file(&self, project: &ProjectInfo) -> Result<()> {
        let config_path = project.root_dir.join("kn.toml");
        
        if config_path.exists() {
            println!("{}", "  ⚠ kn.toml already exists, skipping...".yellow());
            return Ok(());
        }

        let config = format!(r#"[project]
name = "{}"
type = "{:?}"

[agents]
# Define your agent IDs and roles
# planner = "project-abc"
# implementation = "project-xyz"

[skills]
# Skills to auto-install
# enabled = ["typescript", "react-19", "beads-workflow"]

[beads]
# Beads integration settings
enabled = true
templates_dir = ".beads/templates"

[mcp]
# MCP servers for documentation lookup
# servers = ["rust-docs", "mdn-web-docs"]
"#, project.name, project.project_type);

        fs::write(&config_path, config)
            .context("Failed to write kn.toml")?;

        println!("{}", "  ✓ Created kn.toml".green());
        Ok(())
    }

    fn init_beads(&self, project: &ProjectInfo) -> Result<()> {
        let beads_dir = project.root_dir.join(".beads");
        
        if beads_dir.exists() {
            println!("{}", "  ⚠ .beads already exists, skipping beads init...".yellow());
            return Ok(());
        }

        println!("{}", "  ℹ Run 'bd init' to initialize beads issue tracking".bright_blue());
        Ok(())
    }

    fn install_recommended_skills(&self, project: &ProjectInfo) -> Result<()> {
        println!("\n{}", "📦 Installing recommended skills...".bright_cyan().bold());
        
        // Map project type to recommended skills
        let skills: Vec<&str> = match project.project_type {
            ProjectType::Rust => vec![
                "rust-best-practices",
                "docker-best-practices",
                "bash-best-practices",
            ],
            ProjectType::Node => vec![
                "docker-best-practices",
                "bash-best-practices",
            ],
            ProjectType::Python => vec![
                "python-best-practices",
                "docker-best-practices",
                "bash-best-practices",
            ],
            ProjectType::Go => vec![
                "docker-best-practices",
                "bash-best-practices",
            ],
            ProjectType::Monorepo => vec![
                "docker-best-practices",
                "bash-best-practices",
            ],
            ProjectType::Unknown => vec![
                "bash-best-practices",
            ],
        };

        if skills.is_empty() {
            println!("{}", "  ℹ No skills to install for this project type".bright_blue());
            return Ok(());
        }

        let mut installed_count = 0;
        let mut skipped_count = 0;

        for skill_name in skills {
            // Check if skill already exists
            let skill_dir = project.root_dir.join("skills").join(skill_name);
            
            if skill_dir.exists() {
                skipped_count += 1;
                continue;
            }
            
            // Install from bundled skills - just pass the skill name
            match install_skill_from_path(skill_name, true) {
                Ok(_) => {
                    println!("{}", format!("  ✓ Installed {}", skill_name).green());
                    installed_count += 1;
                }
                Err(e) => {
                    println!("{}", format!("  ⚠ Failed to install {}: {}", skill_name, e).yellow());
                }
            }
        }

        println!("\n{}", format!("✓ Skills: {} installed, {} already present", 
            installed_count, skipped_count).bright_green());
        
        Ok(())
    }
}
