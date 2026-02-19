use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::collections::HashSet;
use std::fs;

use crate::config::{GeminiConfig, KnConfig, OpenCodeConfig, WorkspaceStandard};
use crate::core::{kn_home, symlinks};
use crate::models::agent::AgentMetadata;

#[derive(Args)]
pub struct InitCommand {
    /// Skip interactive prompts (use defaults)
    #[arg(short = 'y', long)]
    yes: bool,
}

impl InitCommand {
    pub fn execute(&self) -> Result<()> {
        println!(
            "{}",
            "🚀 Initializing Knowledge Framework workspace..."
                .bright_cyan()
                .bold()
        );

        let current_dir = std::env::current_dir()?;

        // Check if kn.toml already exists
        let config_path = current_dir.join("kn.toml");
        if config_path.exists() {
            println!("{}", "  ⚠ kn.toml already exists!".yellow());
            let overwrite = if self.yes {
                false
            } else {
                Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Overwrite existing configuration?")
                    .default(false)
                    .interact()?
            };

            if !overwrite {
                println!("{}", "  Aborted.".yellow());
                return Ok(());
            }
        }

        // 1. Prompt: Project name
        let default_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project");

        let project_name = if self.yes {
            default_name.to_string()
        } else {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .default(default_name.to_string())
                .interact_text()?
        };

        // 2. Prompt: Workspace standard
        let workspace_standard = if self.yes {
            WorkspaceStandard::Both
        } else {
            let standards = vec!["OpenCode only", "Antigravity only", "Both (recommended)"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose workspace standard for AI assistants")
                .items(&standards)
                .default(2) // "Both" is recommended
                .interact()?;

            match selection {
                0 => WorkspaceStandard::OpenCode,
                1 => WorkspaceStandard::Antigravity,
                2 => WorkspaceStandard::Both,
                _ => WorkspaceStandard::Both,
            }
        };

        // 3. Prompt: Agent selection (multi-select)
        let available_agents = kn_home::list_installed_agents_with_metadata()?;

        let selected_agents = if self.yes {
            // In non-interactive mode, use planner only
            available_agents
                .iter()
                .filter(|a| a.name == "planner")
                .collect()
        } else if available_agents.is_empty() {
            println!("{}", "  ⚠ No agents found in ~/.kn/agents/".yellow());
            println!("  Install agents first: kn agents install <path>");
            Vec::new()
        } else {
            // Separate planner from other agents
            let planner_agent = available_agents.iter().find(|a| a.name == "planner");
            let other_agents: Vec<&AgentMetadata> = available_agents
                .iter()
                .filter(|a| a.name != "planner")
                .collect();

            if other_agents.is_empty() {
                // Only planner available
                if let Some(planner) = planner_agent {
                    println!(
                        "{}",
                        "  ✓ Planner agent will be included (mandatory)".green()
                    );
                    vec![planner]
                } else {
                    println!(
                        "{}",
                        "  ⚠ Planner agent not found in ~/.kn/agents/".yellow()
                    );
                    Vec::new()
                }
            } else {
                // Show selection for other agents
                let agent_names: Vec<String> = other_agents
                    .iter()
                    .map(|a| format!("{} - {}", a.name, a.description))
                    .collect();

                println!(
                    "{}",
                    "  ℹ Planner agent will be included automatically (mandatory)".bright_blue()
                );

                let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select additional agents (Space to select, Enter to confirm)")
                    .items(&agent_names)
                    .interact()?;

                let mut selected: Vec<&AgentMetadata> =
                    selections.into_iter().map(|i| other_agents[i]).collect();

                // Always include planner at the beginning
                if let Some(planner) = planner_agent {
                    selected.insert(0, planner);
                }

                selected
            }
        };

        // 4. Auto-detect required skills from selected agents
        let mut required_skills = HashSet::new();
        let mut recommended_skills = HashSet::new();

        for agent in &selected_agents {
            for skill in &agent.required_skills {
                required_skills.insert(skill.clone());
            }
            for skill in &agent.recommended_skills {
                recommended_skills.insert(skill.clone());
            }
        }

        // 5. Auto-install required skills
        if !required_skills.is_empty() {
            println!(
                "\n{}",
                format!(
                    "📦 Auto-installing {} required skills...",
                    required_skills.len()
                )
                .bright_cyan()
            );

            for skill in &required_skills {
                // Check if already installed in ~/.kn/skills/
                let skill_path = kn_home::skills_dir()?.join(skill);
                if skill_path.exists() {
                    println!("  {} {}", "✓".green(), skill.bright_white());
                } else {
                    println!(
                        "  {} {} (not found in ~/.kn/skills/)",
                        "⚠".yellow(),
                        skill.yellow()
                    );
                }
            }
        }

        // 6. Prompt: Optional recommended skills
        let selected_recommended = if self.yes || recommended_skills.is_empty() {
            HashSet::new()
        } else {
            let rec_vec: Vec<&String> = recommended_skills.iter().collect();
            let rec_names: Vec<String> = rec_vec.iter().map(|s| s.to_string()).collect();

            if !rec_names.is_empty() {
                let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "Select optional recommended skills (Space to select, Enter to confirm)",
                    )
                    .items(&rec_names)
                    .interact()?;

                selections.into_iter().map(|i| rec_vec[i].clone()).collect()
            } else {
                HashSet::new()
            }
        };

        // 7. Generate kn.toml
        println!("\n{}", "📝 Generating kn.toml...".bright_cyan());

        let mut config = KnConfig::new(&project_name, workspace_standard.clone());

        // Add selected agents
        for agent in &selected_agents {
            config.add_agent(agent, &project_name);
        }

        // Add required + selected recommended skills
        for skill in required_skills.iter().chain(selected_recommended.iter()) {
            config.add_skill(skill);
        }

        config.save(&config_path)?;
        println!("  {} Created kn.toml", "✓".green());

        // 8. Create symlinks based on workspace standard
        println!("\n{}", "🔗 Creating symlinks...".bright_cyan());

        // Create symlinks for all enabled skills
        let all_skills: Vec<String> = required_skills
            .iter()
            .chain(selected_recommended.iter())
            .cloned()
            .collect();

        symlinks::create_skill_symlinks(&current_dir, &all_skills, &workspace_standard)?;

        // Create agent symlinks
        let agent_names: Vec<String> = selected_agents.iter().map(|a| a.name.clone()).collect();
        symlinks::create_agent_symlinks(&current_dir, &agent_names, &workspace_standard)?;

        println!("  {} Symlinks created", "✓".green());

        // 9. Generate .opencode/opencode.json for OpenCode workspace
        if matches!(
            workspace_standard,
            WorkspaceStandard::OpenCode | WorkspaceStandard::Both
        ) {
            println!(
                "\n{}",
                "📄 Generating .opencode/opencode.json...".bright_cyan()
            );

            let opencode_dir = current_dir.join(".opencode");
            let opencode_json = opencode_dir.join("opencode.json");

            // Create .opencode directory
            fs::create_dir_all(&opencode_dir).context("Failed to create .opencode directory")?;

            // Generate OpenCode config
            let opencode_config = if let Some(mcp_config) = &config.mcp {
                // If project has MCPs configured, generate with them
                match OpenCodeConfig::generate_from_project(mcp_config) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        println!("  {} Failed to generate MCP config: {}", "⚠".yellow(), e);
                        OpenCodeConfig::new()
                    }
                }
            } else {
                // Empty config with just schema
                OpenCodeConfig::new()
            };

            // Save config
            opencode_config
                .save(&opencode_json)
                .context("Failed to save .opencode/opencode.json")?;

            if let Some(mcp) = &opencode_config.mcp {
                println!(
                    "  {} Created .opencode/opencode.json with {} MCPs",
                    "✓".green(),
                    mcp.len()
                );
            } else {
                println!(
                    "  {} Created .opencode/opencode.json (no MCPs)",
                    "✓".green()
                );
            }
        }

        // 9b. Generate .gemini/antigravity/mcp_config.json for Antigravity workspace
        if matches!(
            workspace_standard,
            WorkspaceStandard::Antigravity | WorkspaceStandard::Both
        ) {
            println!(
                "\n{}",
                "📄 Generating .gemini/antigravity/mcp_config.json...".bright_cyan()
            );

            let gemini_dir = current_dir.join(".gemini").join("antigravity");
            let gemini_json = gemini_dir.join("mcp_config.json");

            // Create .gemini/antigravity directory
            fs::create_dir_all(&gemini_dir)
                .context("Failed to create .gemini/antigravity directory")?;

            // Generate Gemini config
            let gemini_config = if let Some(mcp_config) = &config.mcp {
                // If project has MCPs configured, generate with them
                match GeminiConfig::generate_from_project(mcp_config) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        println!("  {} Failed to generate MCP config: {}", "⚠".yellow(), e);
                        GeminiConfig::new()
                    }
                }
            } else {
                // Empty config
                GeminiConfig::new()
            };

            // Save config
            gemini_config
                .save(&gemini_json)
                .context("Failed to save .gemini/antigravity/mcp_config.json")?;

            if !gemini_config.mcp_servers.is_empty() {
                println!(
                    "  {} Created .gemini/antigravity/mcp_config.json with {} MCPs",
                    "✓".green(),
                    gemini_config.mcp_servers.len()
                );
            } else {
                println!(
                    "  {} Created .gemini/antigravity/mcp_config.json (no MCPs)",
                    "✓".green()
                );
            }
        }

        // 10. Create AGENTS.md if it doesn't exist
        let agents_md_path = current_dir.join("AGENTS.md");
        if !agents_md_path.exists() {
            let agents_md_content = self.generate_agents_md(&project_name, &selected_agents);
            fs::write(&agents_md_path, agents_md_content).context("Failed to write AGENTS.md")?;
            println!("  {} Created AGENTS.md", "✓".green());
        } else {
            println!("  {} AGENTS.md already exists", "⚠".yellow());
        }

        // 11. Summary
        println!("\n{}", "✓ Initialization complete!".bright_green().bold());
        println!("\n{}", "Summary:".bright_white().bold());
        println!("  Project: {}", project_name.bright_yellow());
        println!("  Workspace: {:?}", workspace_standard);
        println!("  Agents: {}", selected_agents.len());
        println!("  Skills: {}", all_skills.len());

        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  1. Review kn.toml and customize as needed");

        if matches!(
            workspace_standard,
            WorkspaceStandard::OpenCode | WorkspaceStandard::Both
        ) {
            println!("  2. Review .opencode/opencode.json for MCP configuration");
        }

        if matches!(
            workspace_standard,
            WorkspaceStandard::Antigravity | WorkspaceStandard::Both
        ) {
            println!("  3. Review .gemini/antigravity/mcp_config.json for MCP configuration");
        }

        println!("  4. Add MCPs: kn mcp install <name> && kn mcp add <name>");
        println!("  5. Run: kn sync (to update symlinks and configs)");

        if !all_skills.is_empty() {
            println!("  6. Ensure skills are installed: kn skills list");
        }

        Ok(())
    }

    fn generate_agents_md(&self, project_name: &str, agents: &[&AgentMetadata]) -> String {
        let agent_list = if agents.is_empty() {
            "- **No agents configured yet**".to_string()
        } else {
            agents
                .iter()
                .map(|a| {
                    format!(
                        "- **{}** (`{}`) - {}",
                        a.name,
                        a.generate_id(project_name),
                        a.description
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        };

        format!(
            r#"# Agent Instructions

This project uses **bd** (beads) for issue tracking with the Knowledge Framework.

## Project: {}

## Enabled Agents

{}

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
            project_name, agent_list
        )
    }
}
