use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

use crate::config::WorkspaceStandard;
use crate::core::kn_home;

/// Create symlinks for skills based on workspace standard
pub fn create_skill_symlinks(
    project_root: &Path,
    skill_names: &[String],
    workspace: &WorkspaceStandard,
) -> Result<()> {
    let global_skills = kn_home::skills_dir()?;

    match workspace {
        WorkspaceStandard::OpenCode => {
            // Create .opencode/skills/ symlinks → ~/.kn/skills/*
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
        }
        WorkspaceStandard::Antigravity => {
            // Create .agent/skills/ symlinks → ~/.kn/skills/*
            let target_dir = project_root.join(".agent/skills");
            fs::create_dir_all(&target_dir)?;

            for skill_name in skill_names {
                let source = global_skills.join(skill_name);
                let link = target_dir.join(skill_name);

                if !source.exists() {
                    eprintln!("Warning: Skill '{}' not found in ~/.kn/skills/", skill_name);
                    continue;
                }

                if link.exists() || link.is_symlink() {
                    fs::remove_file(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }
        }
        WorkspaceStandard::Both => {
            // Create ./skills/ symlinks → ~/.kn/skills/* (shared source)
            let shared_skills_dir = project_root.join("skills");
            fs::create_dir_all(&shared_skills_dir)?;

            for skill_name in skill_names {
                let source = global_skills.join(skill_name);
                let link = shared_skills_dir.join(skill_name);

                if !source.exists() {
                    eprintln!("Warning: Skill '{}' not found in ~/.kn/skills/", skill_name);
                    continue;
                }

                if link.exists() || link.is_symlink() {
                    fs::remove_file(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }

            // Then create .opencode/skills/ → ../skills/*
            let opencode_dir = project_root.join(".opencode/skills");
            fs::create_dir_all(&opencode_dir)?;

            for skill_name in skill_names {
                let source = PathBuf::from("../../skills").join(skill_name);
                let link = opencode_dir.join(skill_name);

                if link.exists() || link.is_symlink() {
                    fs::remove_file(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }

            // And .agent/skills/ → ../skills/*
            let agent_dir = project_root.join(".agent/skills");
            fs::create_dir_all(&agent_dir)?;

            for skill_name in skill_names {
                let source = PathBuf::from("../../skills").join(skill_name);
                let link = agent_dir.join(skill_name);

                if link.exists() || link.is_symlink() {
                    fs::remove_file(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }
        }
    }

    Ok(())
}

/// Create symlinks for agents based on workspace standard
pub fn create_agent_symlinks(
    project_root: &Path,
    agent_names: &[String],
    workspace: &WorkspaceStandard,
) -> Result<()> {
    let global_agents = kn_home::agents_dir()?;

    match workspace {
        WorkspaceStandard::OpenCode => {
            // Create .opencode/agents/ symlinks → ~/.kn/agents/*
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
        WorkspaceStandard::Antigravity => {
            // Create ./agents/ symlinks → ~/.kn/agents/*
            let target_dir = project_root.join("agents");
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
        WorkspaceStandard::Both => {
            // Create ./agents/ symlinks → ~/.kn/agents/* (shared source)
            let shared_agents_dir = project_root.join("agents");
            fs::create_dir_all(&shared_agents_dir)?;

            for agent_name in agent_names {
                let source = global_agents.join(agent_name);
                let link = shared_agents_dir.join(agent_name);

                if !source.exists() {
                    eprintln!("Warning: Agent '{}' not found in ~/.kn/agents/", agent_name);
                    continue;
                }

                if link.exists() || link.is_symlink() {
                    fs::remove_dir_all(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }

            // Then create .opencode/agents/ → ../agents/*
            let opencode_dir = project_root.join(".opencode/agents");
            fs::create_dir_all(&opencode_dir)?;

            for agent_name in agent_names {
                let source = PathBuf::from("../../agents").join(agent_name);
                let link = opencode_dir.join(agent_name);

                if link.exists() || link.is_symlink() {
                    fs::remove_dir_all(&link).ok();
                }

                unix_fs::symlink(&source, &link)?;
            }
        }
    }

    Ok(())
}
