use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentMetadata {
    pub name: String,
    pub id_prefix: String,
    pub description: String,
    #[serde(default)]
    pub required_skills: Vec<String>,
    #[serde(default)]
    pub recommended_skills: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Default for AgentMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            id_prefix: String::new(),
            description: String::new(),
            required_skills: Vec::new(),
            recommended_skills: Vec::new(),
            tags: Vec::new(),
        }
    }
}

impl AgentMetadata {
    /// Parse YAML frontmatter from AGENTS.md file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        Self::from_content(&content)
    }

    /// Parse YAML frontmatter from markdown content
    pub fn from_content(content: &str) -> Result<Self> {
        // Check if content starts with YAML frontmatter (---)
        if !content.trim_start().starts_with("---") {
            anyhow::bail!("No YAML frontmatter found in content");
        }

        // Split by "---" to extract frontmatter
        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() < 3 {
            anyhow::bail!("Invalid YAML frontmatter format");
        }

        // parts[0] is empty (before first ---)
        // parts[1] is the YAML content
        // parts[2] is the markdown content
        let yaml_content = parts[1].trim();

        let metadata: AgentMetadata =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML frontmatter")?;

        Ok(metadata)
    }

    /// Generate agent ID for a project
    pub fn generate_id(&self, project_name: &str) -> String {
        format!("{}-{}", project_name, self.id_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
name: rust
id_prefix: r5t
description: Rust development expert
required_skills:
  - rust-best-practices
  - docker-best-practices
recommended_skills:
  - github-actions-best-practices
tags:
  - rust
  - systems
---

# Rust Agent Instructions

Content here...
"#;

        let metadata = AgentMetadata::from_content(content).unwrap();
        assert_eq!(metadata.name, "rust");
        assert_eq!(metadata.id_prefix, "r5t");
        assert_eq!(metadata.required_skills.len(), 2);
        assert_eq!(metadata.recommended_skills.len(), 1);
        assert_eq!(metadata.tags.len(), 2);
    }

    #[test]
    fn test_generate_id() {
        let metadata = AgentMetadata {
            name: "rust".to_string(),
            id_prefix: "r5t".to_string(),
            description: "Rust expert".to_string(),
            required_skills: vec![],
            recommended_skills: vec![],
            tags: vec![],
        };

        let id = metadata.generate_id("my-project");
        assert_eq!(id, "my-project-r5t");
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Agent Instructions\n\nNo frontmatter here";
        let result = AgentMetadata::from_content(content);
        assert!(result.is_err());
    }
}
