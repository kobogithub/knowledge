use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::workspace::WorkspaceStandard;
use crate::commands::mcp::ProjectMcpConfig;
use crate::models::AgentMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnConfig {
    pub project: ProjectSection,
    #[serde(default)]
    pub skills: SkillsSection,
    #[serde(default)]
    pub agents: HashMap<String, AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<ProjectMcpConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSection {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub workspace_standard: String,
    /// Stack-preset the project was initialized from, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillsSection {
    #[serde(default)]
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub role: String,
    #[serde(default)]
    pub required_skills: Vec<String>,
}

impl KnConfig {
    pub fn new(project_name: &str, workspace: WorkspaceStandard) -> Self {
        Self {
            project: ProjectSection {
                name: project_name.to_string(),
                description: String::new(),
                workspace_standard: workspace.to_string(),
                stack: None,
            },
            skills: SkillsSection::default(),
            agents: HashMap::new(),
            mcp: None,
        }
    }

    pub fn add_agent(&mut self, agent_meta: &AgentMetadata, project_name: &str) {
        let agent_config = AgentConfig {
            id: agent_meta.generate_id(project_name),
            role: agent_meta.name.clone(),
            required_skills: agent_meta.required_skills.clone(),
        };

        self.agents.insert(agent_meta.name.clone(), agent_config);
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        if !self.skills.enabled.contains(&skill_name.to_string()) {
            self.skills.enabled.push(skill_name.to_string());
        }
    }

    /// Record the stack-preset this project was initialized from.
    pub fn set_stack(&mut self, stack_name: &str) {
        self.project.stack = Some(stack_name.to_string());
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let config: KnConfig = toml::from_str(&content).context("Failed to parse TOML")?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = KnConfig::new("my-project", WorkspaceStandard::OpenCode);
        assert_eq!(config.project.name, "my-project");
        assert_eq!(config.project.workspace_standard, "opencode");
    }

    #[test]
    fn test_add_skill() {
        let mut config = KnConfig::new("test", WorkspaceStandard::OpenCode);
        config.add_skill("rust-best-practices");
        assert_eq!(config.skills.enabled.len(), 1);
        assert!(config
            .skills
            .enabled
            .contains(&"rust-best-practices".to_string()));
    }

    #[test]
    fn test_add_skill_is_idempotent() {
        let mut config = KnConfig::new("test", WorkspaceStandard::OpenCode);
        config.add_skill("rust-best-practices");
        config.add_skill("rust-best-practices");
        assert_eq!(config.skills.enabled.len(), 1);
    }

    #[test]
    fn test_new_config_has_no_stack() {
        let config = KnConfig::new("test", WorkspaceStandard::OpenCode);
        assert!(config.project.stack.is_none());
    }

    #[test]
    fn test_stack_roundtrip_when_set() {
        let mut config = KnConfig::new("test", WorkspaceStandard::Claude);
        config.set_stack("web-astro");
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(serialized.contains("stack = \"web-astro\""));

        let parsed: KnConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed.project.stack.as_deref(), Some("web-astro"));
    }

    #[test]
    fn test_stack_absent_is_not_serialized() {
        let config = KnConfig::new("test", WorkspaceStandard::OpenCode);
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(!serialized.contains("stack"));
    }

    #[test]
    fn test_legacy_kn_toml_without_stack_loads() {
        // A kn.toml created before this feature has no `stack` key.
        let legacy = "[project]\nname = \"old\"\nworkspace_standard = \"opencode\"\n";
        let parsed: KnConfig = toml::from_str(legacy).unwrap();
        assert_eq!(parsed.project.name, "old");
        assert!(parsed.project.stack.is_none());
    }
}
