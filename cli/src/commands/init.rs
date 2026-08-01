use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::collections::HashSet;
use std::fs;

use crate::config::{ClaudeMcpConfig, KnConfig, OpenCodeConfig, WorkspaceStandard};
use crate::core::{kn_home, symlinks, write_claude_md_if_missing};
use crate::models::agent::AgentMetadata;
use std::process::Command;

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

        // 2. Prompt: Workspace standard (OpenCode or Claude Code)
        let workspace_options = ["OpenCode", "Claude Code"];
        let workspace_standard = if self.yes {
            WorkspaceStandard::OpenCode
        } else {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Workspace standard")
                .items(&workspace_options)
                .default(0)
                .interact()?;
            match selection {
                1 => WorkspaceStandard::Claude,
                _ => WorkspaceStandard::OpenCode,
            }
        };
        println!(
            "  {} Workspace: {}",
            "ℹ".bright_blue(),
            match workspace_standard {
                WorkspaceStandard::OpenCode => "OpenCode",
                WorkspaceStandard::Claude => "Claude Code",
            }
            .bright_white()
            .bold()
        );

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

        // 5. Verify required skills are available
        if !required_skills.is_empty() {
            println!("\n{}", "📦 Checking required skills...".bright_cyan());

            let mut missing_skills = Vec::new();
            for skill in &required_skills {
                // Check if already installed in ~/.kn/skills/
                let skill_path = kn_home::skills_dir()?.join(skill);
                if skill_path.exists() {
                    println!("  {} {}", "✓".green(), skill.bright_white());
                } else {
                    println!("  {} {} (missing)", "⚠".yellow(), skill.bright_white());
                    missing_skills.push(skill.as_str());
                }
            }

            if !missing_skills.is_empty() {
                println!(
                    "\n  {} {} skills missing from ~/.kn/skills/",
                    "⚠".yellow(),
                    missing_skills.len()
                );
                println!(
                    "  {} These skills should have been installed automatically.",
                    "ℹ".bright_blue()
                );
                println!(
                    "  {} Try running: {}",
                    "💡".bright_yellow(),
                    "kn update".yellow().bold()
                );
                println!(
                    "  {} Or reinstall kn using the latest install.sh",
                    "💡".bright_yellow()
                );
            }
        }

        // 6. Prompt: Optional recommended skills
        let selected_recommended = if self.yes || recommended_skills.is_empty() {
            HashSet::new()
        } else {
            let rec_vec: Vec<&String> = recommended_skills.iter().collect();
            let rec_names: Vec<String> = rec_vec.iter().map(|s| s.to_string()).collect();

            if !rec_names.is_empty() {
                println!("\n{}", "📚 Optional recommended skills:".bright_cyan());
                println!(
                    "  {} These are optional but may improve agent performance",
                    "ℹ".bright_blue()
                );

                let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select skills to add (Space to select, Enter to skip)")
                    .items(&rec_names)
                    .interact()?;

                selections.into_iter().map(|i| rec_vec[i].clone()).collect()
            } else {
                HashSet::new()
            }
        };

        // 7. Generate kn.toml
        println!("\n{}", "📝 Generating kn.toml...".bright_cyan());

        let mut config = KnConfig::new(&project_name, workspace_standard);

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

        // 8. Create symlinks (OpenCode only)
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

        // 8.5. Copy formulas from ~/.kn/formulas/ to .beads/formulas/
        println!("\n{}", "📜 Installing formulas...".bright_cyan());
        let formulas_source = kn_home::formulas_dir()?;
        let formulas_target = current_dir.join(".beads/formulas");
        if formulas_source.exists() {
            fs::create_dir_all(&formulas_target)?;
            let mut count = 0;
            for entry in fs::read_dir(&formulas_source)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json")
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".formula.json"))
                {
                    let dest = formulas_target.join(path.file_name().unwrap());
                    if !dest.exists() {
                        fs::copy(&path, &dest)?;
                        count += 1;
                    }
                }
            }
            if count > 0 {
                println!("  {} Installed {} formula(s)", "✓".green(), count);
            } else {
                println!(
                    "  {} Formulas already installed or none available",
                    "ℹ".bright_blue()
                );
            }
        } else {
            println!(
                "  {} No formulas found in ~/.kn/formulas/",
                "ℹ".bright_blue()
            );
        }

        // 8.6. Create dev branch if it doesn't exist
        println!("\n{}", "🌿 Setting up branching strategy...".bright_cyan());

        let is_git = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if is_git {
            let dev_exists = Command::new("git")
                .args(["rev-parse", "--verify", "dev"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if !dev_exists {
                let create_result = Command::new("git").args(["branch", "dev"]).output();

                match create_result {
                    Ok(output) if output.status.success() => {
                        println!("  {} Created 'dev' branch from current branch", "✓".green());
                    }
                    _ => {
                        println!(
                            "  {} Could not create 'dev' branch (no commits yet?)",
                            "⚠".yellow()
                        );
                    }
                }
            } else {
                println!("  {} 'dev' branch already exists", "✓".green());
            }
        } else {
            println!(
                "  {} Not a git repository, skipping branch setup",
                "ℹ".bright_blue()
            );
        }

        // 9. Generate MCP config: .opencode/opencode.json (OpenCode) or .mcp.json (Claude)
        match workspace_standard {
            WorkspaceStandard::OpenCode => {
                println!(
                    "\n{}",
                    "📄 Generating .opencode/opencode.json...".bright_cyan()
                );

                let opencode_dir = current_dir.join(".opencode");
                let opencode_json = opencode_dir.join("opencode.json");

                fs::create_dir_all(&opencode_dir)
                    .context("Failed to create .opencode directory")?;

                let mut opencode_config = if let Some(mcp_config) = &config.mcp {
                    match OpenCodeConfig::generate_from_project(mcp_config) {
                        Ok(cfg) => cfg,
                        Err(e) => {
                            println!("  {} Failed to generate MCP config: {}", "⚠".yellow(), e);
                            OpenCodeConfig::new()
                        }
                    }
                } else {
                    OpenCodeConfig::new()
                };

                if selected_agents.iter().any(|a| a.name == "planner") {
                    opencode_config
                        .set_planner_as_default(".opencode/agents/planner/AGENTS.md");
                    println!(
                        "  {} Planner set as default agent (opens coordinating, not building)",
                        "✓".green()
                    );
                }

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
            WorkspaceStandard::Claude => {
                println!("\n{}", "📄 Generating .mcp.json...".bright_cyan());

                let claude_mcp_json = current_dir.join(".mcp.json");

                let claude_config = if let Some(mcp_config) = &config.mcp {
                    match ClaudeMcpConfig::generate_from_project(mcp_config) {
                        Ok(cfg) => cfg,
                        Err(e) => {
                            println!("  {} Failed to generate MCP config: {}", "⚠".yellow(), e);
                            ClaudeMcpConfig::new()
                        }
                    }
                } else {
                    ClaudeMcpConfig::new()
                };

                claude_config
                    .save(&claude_mcp_json)
                    .context("Failed to save .mcp.json")?;

                if let Some(mcp) = &claude_config.mcp_servers {
                    println!(
                        "  {} Created .mcp.json with {} MCPs",
                        "✓".green(),
                        mcp.len()
                    );
                } else {
                    println!("  {} Created .mcp.json (no MCPs)", "✓".green());
                }
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

        // 10.5. Claude Code doesn't read AGENTS.md natively — generate a root
        // CLAUDE.md that imports it and makes the planner the default persona,
        // so a fresh `claude` session opens already coordinating.
        if workspace_standard == WorkspaceStandard::Claude
            && selected_agents.iter().any(|a| a.name == "planner")
        {
            match write_claude_md_if_missing(&current_dir, &project_name) {
                Ok(true) => println!(
                    "  {} Created CLAUDE.md (planner is the default agent)",
                    "✓".green()
                ),
                Ok(false) => println!("  {} CLAUDE.md already exists", "⚠".yellow()),
                Err(e) => println!("  {} Failed to write CLAUDE.md: {}", "⚠".yellow(), e),
            }
        }

        // 11. Summary
        println!("\n{}", "✓ Initialization complete!".bright_green().bold());
        println!("\n{}", "Summary:".bright_white().bold());
        println!("  Project: {}", project_name.bright_yellow());
        println!(
            "  Workspace: {}",
            match workspace_standard {
                WorkspaceStandard::OpenCode => "OpenCode",
                WorkspaceStandard::Claude => "Claude Code",
            }
        );
        println!("  Agents: {}", selected_agents.len());
        println!("  Skills configured: {}", all_skills.len());

        // Check how many skills are actually installed
        let skills_dir = kn_home::skills_dir()?;
        let installed_count = all_skills
            .iter()
            .filter(|s| skills_dir.join(s).exists())
            .count();
        let missing_count = all_skills.len() - installed_count;

        if missing_count > 0 {
            println!(
                "  {} Skills missing: {} (should be auto-installed)",
                "⚠".yellow(),
                missing_count
            );
        }

        println!("\n{}", "Next steps:".bright_white().bold());
        println!("  1. Review kn.toml and customize as needed");

        if missing_count > 0 {
            println!("  2. Reinstall kn to get missing skills:");
            println!("     curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash");
            println!("     or run: kn update");
        }

        let mcp_config_file = match workspace_standard {
            WorkspaceStandard::OpenCode => ".opencode/opencode.json",
            WorkspaceStandard::Claude => ".mcp.json",
        };
        println!(
            "  {}. Review {} for MCP configuration",
            if missing_count > 0 { 3 } else { 2 },
            mcp_config_file
        );

        let next_step = if missing_count > 0 { 4 } else { 3 };
        println!(
            "  {}. Add MCPs: kn mcp install <name> && kn mcp add <name>",
            next_step
        );
        println!(
            "  {}. Run: kn sync (to update symlinks and configs)",
            next_step + 1
        );

        println!("\n{}", "📋 Conventional Commits:".bright_white().bold());
        println!("  Format: <type>(<scope>): <message>");
        println!(
            "  Types: feat | fix | refactor | perf | build | ci | chore | docs | style | test"
        );
        println!("  Breaking changes: Add '!' before ':' (e.g., feat!: breaking change)");
        println!("  Tagging: manual — git tag -a vX.Y.Z -m \"message\" && git push origin vX.Y.Z");

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

## Git Branching Strategy

```
prod (stable releases)
  └─ dev (integration)
      └─ epic/<epic-id> (epic integration branch)
          └─ <epic-id>/<agent-role> (agent work branch)
```

### Conventional Commits (MANDATORY)

All commit messages MUST use: `<type>(<scope>): <message>`

| Type | Bump | Type | Bump |
|------|------|------|------|
| `feat` | MINOR | `fix` | PATCH |
| `refactor` | PATCH | `perf` | PATCH |
| `build` | PATCH | `ci` | PATCH |
| `chore` | PATCH | `docs` | PATCH |
| `style` | PATCH | `test` | PATCH |
| `any!` (breaking) | MAJOR | | |

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
