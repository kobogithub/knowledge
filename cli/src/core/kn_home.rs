use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Get the global kn home directory (~/.kn/)
pub fn kn_home() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(".kn"))
}

/// Get the global skills directory (~/.kn/skills/)
pub fn skills_dir() -> Result<PathBuf> {
    Ok(kn_home()?.join("skills"))
}

/// Get the global agents directory (~/.kn/agents/)
pub fn agents_dir() -> Result<PathBuf> {
    Ok(kn_home()?.join("agents"))
}

/// Ensure the global kn directory structure exists
pub fn ensure_kn_home() -> Result<()> {
    let kn_home = kn_home()?;
    let skills = skills_dir()?;
    let agents = agents_dir()?;

    fs::create_dir_all(&kn_home)
        .with_context(|| format!("Failed to create directory: {}", kn_home.display()))?;

    fs::create_dir_all(&skills)
        .with_context(|| format!("Failed to create directory: {}", skills.display()))?;

    fs::create_dir_all(&agents)
        .with_context(|| format!("Failed to create directory: {}", agents.display()))?;

    Ok(())
}

/// List all installed skills in ~/.kn/skills/
pub fn list_installed_skills() -> Result<Vec<String>> {
    let skills = skills_dir()?;

    if !skills.exists() {
        return Ok(Vec::new());
    }

    let mut skill_names = Vec::new();

    for entry in fs::read_dir(&skills)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Check if it has a SKILL.md file
            if path.join("SKILL.md").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    skill_names.push(name.to_string());
                }
            }
        }
    }

    skill_names.sort();
    Ok(skill_names)
}

/// List all installed agents in ~/.kn/agents/
pub fn list_installed_agents() -> Result<Vec<String>> {
    let agents = agents_dir()?;

    if !agents.exists() {
        return Ok(Vec::new());
    }

    let mut agent_names = Vec::new();

    for entry in fs::read_dir(&agents)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Check if it has an AGENTS.md file
            if path.join("AGENTS.md").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    agent_names.push(name.to_string());
                }
            }
        }
    }

    agent_names.sort();
    Ok(agent_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kn_home_path() {
        let home = kn_home().unwrap();
        assert!(home.ends_with(".kn"));
    }

    #[test]
    fn test_skills_dir_path() {
        let skills = skills_dir().unwrap();
        assert!(skills.ends_with(".kn/skills"));
    }

    #[test]
    fn test_agents_dir_path() {
        let agents = agents_dir().unwrap();
        assert!(agents.ends_with(".kn/agents"));
    }
}
