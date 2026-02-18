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
    language: Language,
    framework: Option<Framework>,
    is_workspace: bool,
    root_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Language {
    Rust,
    Node,
    Python,
    Go,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Framework {
    // Rust frameworks
    Actix,
    Axum,
    Tauri,
    RustCli,

    // Node frameworks
    NextJs,
    Astro,
    React,
    Express,
    NestJs,

    // Python frameworks
    FastAPI,
    Django,
    Flask,
}

impl InitCommand {
    pub fn execute(&self) -> Result<()> {
        println!(
            "{}",
            "🚀 Initializing AI agent workflow...".bright_cyan().bold()
        );

        let current_dir = std::env::current_dir()?;
        let project_info = self.detect_project(&current_dir)?;

        println!("\n{}", "Detected project:".bright_white().bold());
        println!("  Name: {}", project_info.name.bright_yellow());
        println!("  Language: {:?}", project_info.language);
        if let Some(ref framework) = project_info.framework {
            println!("  Framework: {:?}", framework);
        }
        if project_info.is_workspace {
            println!("  Type: Workspace/Monorepo");
        }
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

        // Create OpenCode configuration
        self.create_opencode_config(&project_info)?;

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

        // Manual override if specified
        if let Some(ref pt) = self.project_type {
            let language = match pt.as_str() {
                "rust" => Language::Rust,
                "node" => Language::Node,
                "python" => Language::Python,
                "go" => Language::Go,
                _ => Language::Unknown,
            };

            return Ok(ProjectInfo {
                name,
                language,
                framework: None,
                is_workspace: false,
                root_dir: dir.to_path_buf(),
            });
        }

        // Auto-detect language
        let language = if dir.join("Cargo.toml").exists() {
            Language::Rust
        } else if dir.join("package.json").exists() {
            Language::Node
        } else if dir.join("pyproject.toml").exists() || dir.join("setup.py").exists() {
            Language::Python
        } else if dir.join("go.mod").exists() {
            Language::Go
        } else {
            Language::Unknown
        };

        // Detect framework based on language
        let framework = match language {
            Language::Rust => self.detect_rust_framework(dir),
            Language::Node => self.detect_node_framework(dir),
            Language::Python => self.detect_python_framework(dir),
            _ => None,
        };

        // Detect workspace/monorepo
        let is_workspace = match language {
            Language::Rust => self.is_cargo_workspace(dir),
            Language::Node => self.is_npm_workspace(dir),
            _ => false,
        };

        Ok(ProjectInfo {
            name,
            language,
            framework,
            is_workspace,
            root_dir: dir.to_path_buf(),
        })
    }

    fn detect_rust_framework(&self, dir: &Path) -> Option<Framework> {
        let cargo_toml_path = dir.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return None;
        }

        if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
            // Check dependencies for frameworks
            if content.contains("actix-web") {
                return Some(Framework::Actix);
            }
            if content.contains("axum") {
                return Some(Framework::Axum);
            }
            if content.contains("tauri") {
                return Some(Framework::Tauri);
            }
            // Check if it's a CLI tool
            if content.contains("[[bin]]")
                || content.contains("[package]") && content.contains("clap")
            {
                return Some(Framework::RustCli);
            }
        }
        None
    }

    fn detect_node_framework(&self, dir: &Path) -> Option<Framework> {
        let package_json_path = dir.join("package.json");
        if !package_json_path.exists() {
            return None;
        }

        if let Ok(content) = fs::read_to_string(&package_json_path) {
            // Check for Next.js
            if content.contains("\"next\"") {
                return Some(Framework::NextJs);
            }
            // Check for Astro
            if content.contains("\"astro\"") || dir.join("astro.config.mjs").exists() {
                return Some(Framework::Astro);
            }
            // Check for NestJS
            if content.contains("\"@nestjs/core\"") {
                return Some(Framework::NestJs);
            }
            // Check for Express
            if content.contains("\"express\"") {
                return Some(Framework::Express);
            }
            // Check for React (not Next.js)
            if content.contains("\"react\"") && !content.contains("\"next\"") {
                return Some(Framework::React);
            }
        }
        None
    }

    fn detect_python_framework(&self, dir: &Path) -> Option<Framework> {
        // Check pyproject.toml
        let pyproject_path = dir.join("pyproject.toml");
        if pyproject_path.exists() {
            if let Ok(content) = fs::read_to_string(&pyproject_path) {
                if content.contains("fastapi") {
                    return Some(Framework::FastAPI);
                }
                if content.contains("django") {
                    return Some(Framework::Django);
                }
                if content.contains("flask") {
                    return Some(Framework::Flask);
                }
            }
        }

        // Check requirements.txt as fallback
        let requirements_path = dir.join("requirements.txt");
        if requirements_path.exists() {
            if let Ok(content) = fs::read_to_string(&requirements_path) {
                if content.contains("fastapi") {
                    return Some(Framework::FastAPI);
                }
                if content.contains("django") || content.contains("Django") {
                    return Some(Framework::Django);
                }
                if content.contains("flask") || content.contains("Flask") {
                    return Some(Framework::Flask);
                }
            }
        }

        None
    }

    fn is_cargo_workspace(&self, dir: &Path) -> bool {
        let cargo_toml_path = dir.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return false;
        }

        if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
            return content.contains("[workspace]");
        }
        false
    }

    fn is_npm_workspace(&self, dir: &Path) -> bool {
        // Check for pnpm workspace
        if dir.join("pnpm-workspace.yaml").exists() {
            return true;
        }

        // Check for lerna
        if dir.join("lerna.json").exists() {
            return true;
        }

        // Check for npm/yarn workspaces in package.json
        let package_json_path = dir.join("package.json");
        if package_json_path.exists() {
            if let Ok(content) = fs::read_to_string(&package_json_path) {
                return content.contains("\"workspaces\"");
            }
        }

        false
    }

    fn create_agents_file(&self, project: &ProjectInfo) -> Result<()> {
        let agents_path = project.root_dir.join("AGENTS.md");

        if agents_path.exists() {
            println!("{}", "  ⚠ AGENTS.md already exists, skipping...".yellow());
            return Ok(());
        }

        let template = self.get_agents_template(project);
        fs::write(&agents_path, template).context("Failed to write AGENTS.md")?;

        println!("{}", "  ✓ Created AGENTS.md".green());
        Ok(())
    }

    fn get_agents_template(&self, project: &ProjectInfo) -> String {
        let project_description = self.get_project_description(project);
        format!(
            r#"# Agent Instructions

This project uses **bd** (beads) for issue tracking with a multi-agent workflow.

## Project: {}

**Type:** {}

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
"#,
            project.name, project_description
        )
    }

    fn get_project_description(&self, project: &ProjectInfo) -> String {
        let mut desc = format!("{:?}", project.language);

        if let Some(ref framework) = project.framework {
            desc.push_str(&format!(" ({:?})", framework));
        }

        if project.is_workspace {
            desc.push_str(" Workspace/Monorepo");
        }

        desc
    }

    fn create_config_file(&self, project: &ProjectInfo) -> Result<()> {
        let config_path = project.root_dir.join("kn.toml");

        if config_path.exists() {
            println!("{}", "  ⚠ kn.toml already exists, skipping...".yellow());
            return Ok(());
        }

        let project_description = self.get_project_description(project);
        let config = format!(
            r#"[project]
name = "{}"
type = "{}"

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
"#,
            project.name, project_description
        );

        fs::write(&config_path, config).context("Failed to write kn.toml")?;

        println!("{}", "  ✓ Created kn.toml".green());
        Ok(())
    }

    fn init_beads(&self, project: &ProjectInfo) -> Result<()> {
        let beads_dir = project.root_dir.join(".beads");

        if beads_dir.exists() {
            println!(
                "{}",
                "  ⚠ .beads already exists, skipping beads init...".yellow()
            );
            return Ok(());
        }

        println!(
            "{}",
            "  ℹ Run 'bd init' to initialize beads issue tracking".bright_blue()
        );
        Ok(())
    }

    fn install_recommended_skills(&self, project: &ProjectInfo) -> Result<()> {
        println!(
            "\n{}",
            "📦 Installing recommended skills...".bright_cyan().bold()
        );

        let mut skills: Vec<&str> = Vec::new();

        // Base skills by language
        match project.language {
            Language::Rust => {
                skills.push("rust-best-practices");
                skills.push("docker-best-practices");
                skills.push("bash-best-practices");
            }
            Language::Node => {
                skills.push("docker-best-practices");
                skills.push("bash-best-practices");
            }
            Language::Python => {
                skills.push("python-best-practices");
                skills.push("docker-best-practices");
                skills.push("bash-best-practices");
            }
            Language::Go => {
                skills.push("docker-best-practices");
                skills.push("bash-best-practices");
            }
            Language::Unknown => {
                skills.push("bash-best-practices");
            }
        }

        // Add framework-specific skills
        if let Some(ref framework) = project.framework {
            match framework {
                Framework::Astro => {
                    skills.push("astro-best-practices");
                }
                Framework::FastAPI => {
                    // FastAPI-specific skills could be added here
                }
                _ => {
                    // Other frameworks can be added as needed
                }
            }
        }

        if skills.is_empty() {
            println!(
                "{}",
                "  ℹ No skills to install for this project type".bright_blue()
            );
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
                    println!(
                        "{}",
                        format!("  ⚠ Failed to install {}: {}", skill_name, e).yellow()
                    );
                }
            }
        }

        println!(
            "\n{}",
            format!(
                "✓ Skills: {} installed, {} already present",
                installed_count, skipped_count
            )
            .bright_green()
        );

        Ok(())
    }

    fn create_opencode_config(&self, project: &ProjectInfo) -> Result<()> {
        let opencode_dir = project.root_dir.join(".opencode");
        let opencode_config_path = opencode_dir.join("opencode.json");

        // Create .opencode directory if it doesn't exist
        if !opencode_dir.exists() {
            fs::create_dir_all(&opencode_dir).context("Failed to create .opencode directory")?;
        }

        if opencode_config_path.exists() {
            println!(
                "{}",
                "  ⚠ .opencode/opencode.json already exists, skipping...".yellow()
            );
            return Ok(());
        }

        // Get absolute path for the project root
        let root_path = project
            .root_dir
            .canonicalize()
            .unwrap_or_else(|_| project.root_dir.clone());
        let root_path_str = root_path.to_string_lossy();

        // Determine project type string
        let project_type = match project.language {
            Language::Rust => {
                if let Some(Framework::RustCli) = project.framework {
                    "rust-cli"
                } else {
                    "rust"
                }
            }
            Language::Node => "node",
            Language::Python => "python",
            Language::Go => "go",
            Language::Unknown => "unknown",
        };

        // Build skills array based on installed skills
        let skills_dir = project.root_dir.join("skills");
        let mut skills_json = String::from("  \"skills\": [\n");

        if skills_dir.exists() {
            if let Ok(entries) = fs::read_dir(&skills_dir) {
                let mut skill_entries: Vec<String> = Vec::new();
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(skill_name) = path.file_name().and_then(|n| n.to_str()) {
                            let skill_path = path.to_string_lossy();
                            skill_entries.push(format!(
                                "    {{\n      \"name\": \"{}\",\n      \"path\": \"{}\"\n    }}",
                                skill_name, skill_path
                            ));
                        }
                    }
                }
                skills_json.push_str(&skill_entries.join(",\n"));
            }
        }
        skills_json.push_str("\n  ]");

        // Create OpenCode configuration
        let config = format!(
            r#"{{
  "mcpServers": {{
    "filesystem": {{
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-filesystem",
        "{root_path}"
      ]
    }},
    "github": {{
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-github"
      ],
      "env": {{
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${{GITHUB_TOKEN}}"
      }}
    }}
  }},
{skills},
  "agents": {{
    "planner": {{
      "id": "{project_name}-planner",
      "role": "planner",
      "instructionsPath": "{root_path}/agents/planner/AGENTS.md"
    }},
    "implementation": {{
      "id": "{project_name}-impl",
      "role": "implementation",
      "instructionsPath": "{root_path}/agents/implementation/AGENTS.md"
    }},
    "review": {{
      "id": "{project_name}-review",
      "role": "review",
      "instructionsPath": "{root_path}/agents/review/AGENTS.md"
    }}
  }},
  "workspace": {{
    "name": "{project_name}",
    "type": "{project_type}",
    "rootPath": "{root_path}",
    "beadsEnabled": true,
    "beadsPath": "{root_path}/.beads"
  }}
}}
"#,
            root_path = root_path_str,
            skills = skills_json,
            project_name = project.name,
            project_type = project_type
        );

        fs::write(&opencode_config_path, config)
            .context("Failed to write .opencode/opencode.json")?;

        println!("{}", "  ✓ Created .opencode/opencode.json".green());
        Ok(())
    }
}
