use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::models::agent::AgentMetadata;
use crate::models::mcp::McpMetadata;

/// Get the global kn home directory (~/.kn/ or $KN_HOME)
pub fn kn_home() -> Result<PathBuf> {
    // Check for KN_HOME environment variable first
    if let Ok(kn_home_env) = std::env::var("KN_HOME") {
        return Ok(PathBuf::from(kn_home_env));
    }

    // Fall back to ~/.kn
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

/// Get the global mcps directory (~/.kn/mcps/)
pub fn mcps_dir() -> Result<PathBuf> {
    Ok(kn_home()?.join("mcps"))
}

/// Get the global formulas directory (~/.kn/formulas/)
pub fn formulas_dir() -> Result<PathBuf> {
    Ok(kn_home()?.join("formulas"))
}

/// Get the global stacks directory (~/.kn/stacks/)
pub fn stacks_dir() -> Result<PathBuf> {
    Ok(kn_home()?.join("stacks"))
}

/// Ensure the global kn directory structure exists
pub fn ensure_kn_home() -> Result<()> {
    let kn_home = kn_home()?;
    let skills = skills_dir()?;
    let agents = agents_dir()?;
    let mcps = mcps_dir()?;
    let formulas = formulas_dir()?;
    let stacks = stacks_dir()?;

    fs::create_dir_all(&kn_home)
        .with_context(|| format!("Failed to create directory: {}", kn_home.display()))?;

    fs::create_dir_all(&skills)
        .with_context(|| format!("Failed to create directory: {}", skills.display()))?;

    fs::create_dir_all(&agents)
        .with_context(|| format!("Failed to create directory: {}", agents.display()))?;

    fs::create_dir_all(&mcps)
        .with_context(|| format!("Failed to create directory: {}", mcps.display()))?;

    fs::create_dir_all(&formulas)
        .with_context(|| format!("Failed to create directory: {}", formulas.display()))?;

    fs::create_dir_all(&stacks)
        .with_context(|| format!("Failed to create directory: {}", stacks.display()))?;

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

/// List all installed formulas in ~/.kn/formulas/ (returns filenames of .formula.json files)
pub fn list_installed_formulas() -> Result<Vec<String>> {
    let formulas = formulas_dir()?;

    if !formulas.exists() {
        return Ok(Vec::new());
    }

    let mut formula_names = Vec::new();

    for entry in fs::read_dir(&formulas)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".formula.json") {
                    formula_names.push(name.to_string());
                }
            }
        }
    }

    formula_names.sort();
    Ok(formula_names)
}

/// List all installed agents in ~/.kn/agents/ (returns just names)
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

/// List all installed agents in ~/.kn/agents/ with full metadata
pub fn list_installed_agents_with_metadata() -> Result<Vec<AgentMetadata>> {
    let agents = agents_dir()?;

    if !agents.exists() {
        return Ok(Vec::new());
    }

    let mut agent_metadata = Vec::new();

    for entry in fs::read_dir(&agents)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let agents_md = path.join("AGENTS.md");
            if agents_md.exists() {
                match AgentMetadata::from_file(&agents_md) {
                    Ok(metadata) => agent_metadata.push(metadata),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", agents_md.display(), e);
                    }
                }
            }
        }
    }

    // Sort by name
    agent_metadata.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(agent_metadata)
}

/// List all installed MCPs in ~/.kn/mcps/
pub fn list_installed_mcps() -> Result<Vec<String>> {
    let mcps = mcps_dir()?;

    if !mcps.exists() {
        return Ok(Vec::new());
    }

    let mut mcp_names = Vec::new();

    for entry in fs::read_dir(&mcps)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Check if it has an mcp.toml file
            if path.join("mcp.toml").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    mcp_names.push(name.to_string());
                }
            }
        }
    }

    mcp_names.sort();
    Ok(mcp_names)
}

/// List all installed MCPs in ~/.kn/mcps/ with full metadata
pub fn list_installed_mcps_with_metadata() -> Result<Vec<McpMetadata>> {
    let mcps = mcps_dir()?;

    if !mcps.exists() {
        return Ok(Vec::new());
    }

    let mut mcp_metadata = Vec::new();

    for entry in fs::read_dir(&mcps)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mcp_toml = path.join("mcp.toml");
            if mcp_toml.exists() {
                match McpMetadata::from_file(&mcp_toml) {
                    Ok(metadata) => mcp_metadata.push(metadata),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", mcp_toml.display(), e);
                    }
                }
            }
        }
    }

    // Sort by name
    mcp_metadata.sort_by(|a, b| a.mcp.name.cmp(&b.mcp.name));
    Ok(mcp_metadata)
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

    #[test]
    fn test_mcps_dir_path() {
        let mcps = mcps_dir().unwrap();
        assert!(mcps.ends_with(".kn/mcps"));
    }

    #[test]
    fn test_formulas_dir_path() {
        let formulas = formulas_dir().unwrap();
        assert!(formulas.ends_with(".kn/formulas"));
    }

    #[test]
    fn test_stacks_dir_path() {
        let stacks = stacks_dir().unwrap();
        assert!(stacks.ends_with(".kn/stacks"));
    }
}
