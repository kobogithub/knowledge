use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;

use crate::config::WorkspaceStandard;
use crate::core::kn_home;

/// Create symlinks for skills based on workspace standard.
///
/// Currently only OpenCode workspace is supported. The workspace parameter
/// is preserved for future re-enablement of Antigravity support.
pub fn create_skill_symlinks(
    project_root: &Path,
    skill_names: &[String],
    _workspace: &WorkspaceStandard,
) -> Result<()> {
    let global_skills = kn_home::skills_dir()?;

    // OpenCode: Create .opencode/skills/ symlinks → ~/.kn/skills/*
    let target_dir = project_root.join(".opencode/skills");
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
/// Currently only OpenCode workspace is supported. The workspace parameter
/// is preserved for future re-enablement of Antigravity support.
pub fn create_agent_symlinks(
    project_root: &Path,
    agent_names: &[String],
    _workspace: &WorkspaceStandard,
) -> Result<()> {
    let global_agents = kn_home::agents_dir()?;

    // OpenCode: Create .opencode/agents/ symlinks → ~/.kn/agents/*
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

    Ok(())
}
