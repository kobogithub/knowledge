use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;

use crate::config::WorkspaceStandard;
use crate::core::kn_home;

/// Create symlinks for skills based on workspace standard.
///
/// OpenCode: `.opencode/skills/<name>` (directory symlink).
/// Claude:   `.claude/skills/<name>` (directory symlink) — `skills/<name>/SKILL.md`
///           is already Claude Code's native skill format, so no transformation needed.
pub fn create_skill_symlinks(
    project_root: &Path,
    skill_names: &[String],
    workspace: &WorkspaceStandard,
) -> Result<()> {
    let global_skills = kn_home::skills_dir()?;

    let target_dir = match workspace {
        WorkspaceStandard::OpenCode => project_root.join(".opencode/skills"),
        WorkspaceStandard::Claude => project_root.join(".claude/skills"),
    };
    fs::create_dir_all(&target_dir)?;

    for skill_name in skill_names {
        let source = global_skills.join(skill_name);
        let link = target_dir.join(skill_name);

        if !source.exists() {
            eprintln!("Warning: Skill '{}' not found in ~/.kn/skills/", skill_name);
            continue;
        }

        // Remove existing symlink if present
        if link.exists() || link.is_symlink() {
            fs::remove_file(&link).ok();
        }

        unix_fs::symlink(&source, &link)
            .with_context(|| format!("Failed to create symlink for {}", skill_name))?;
    }

    Ok(())
}

/// Create symlinks for agents based on workspace standard.
///
/// OpenCode: `.opencode/agents/<name>` (directory symlink to `~/.kn/agents/<name>/`).
/// Claude:   `.claude/agents/<name>.md` (file symlink to `~/.kn/agents/<name>/AGENTS.md`)
///           — Claude Code's subagent convention is one flat `.md` file per agent
///           directly under `.claude/agents/`, not a subdirectory.
pub fn create_agent_symlinks(
    project_root: &Path,
    agent_names: &[String],
    workspace: &WorkspaceStandard,
) -> Result<()> {
    let global_agents = kn_home::agents_dir()?;

    match workspace {
        WorkspaceStandard::OpenCode => {
            let target_dir = project_root.join(".opencode/agents");
            fs::create_dir_all(&target_dir)?;

            for agent_name in agent_names {
                let source = global_agents.join(agent_name);
                let link = target_dir.join(agent_name);

                if !source.exists() {
                    eprintln!("Warning: Agent '{}' not found in ~/.kn/agents/", agent_name);
                    continue;
                }

                if link.exists() || link.is_symlink() {
                    fs::remove_dir_all(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }
        }
        WorkspaceStandard::Claude => {
            let target_dir = project_root.join(".claude/agents");
            fs::create_dir_all(&target_dir)?;

            for agent_name in agent_names {
                let source = global_agents.join(agent_name).join("AGENTS.md");
                let link = target_dir.join(format!("{}.md", agent_name));

                if !source.exists() {
                    eprintln!(
                        "Warning: AGENTS.md not found for agent '{}' in ~/.kn/agents/",
                        agent_name
                    );
                    continue;
                }

                if link.exists() || link.is_symlink() {
                    fs::remove_file(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }
        }
    }

    Ok(())
}
