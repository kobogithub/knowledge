use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SkillMetadata {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub auto_invoke: bool,
}

impl SkillMetadata {
    /// Parse YAML frontmatter from SKILL.md file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        Self::from_content(&content)
    }

    /// Parse YAML frontmatter from markdown content
    pub fn from_content(content: &str) -> Result<Self> {
        // Check if content starts with YAML frontmatter (---)
        if !content.trim_start().starts_with("---") {
            return Err(anyhow!("Skill file missing YAML frontmatter"));
        }

        // Split by "---" to extract frontmatter
        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() < 3 {
            return Err(anyhow!("Invalid YAML frontmatter format"));
        }

        let yaml_content = parts[1].trim();

        let metadata: SkillMetadata =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML frontmatter")?;

        if metadata.name.is_empty() {
            return Err(anyhow!("Skill must have a 'name' field in frontmatter"));
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill_frontmatter() {
        let content = r#"---
name: rust-best-practices
version: 1.0.0
author: Knowledge Framework
description: Rust programming best practices
tags:
  - rust
  - systems
  - cli
auto_invoke: false
---

# Rust Best Practices

Content here...
"#;

        let metadata = SkillMetadata::from_content(content).unwrap();
        assert_eq!(metadata.name, "rust-best-practices");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.tags.len(), 3);
        assert!(!metadata.auto_invoke);
    }

    #[test]
    fn test_missing_name() {
        let content = r#"---
version: 1.0.0
---
# Skill
"#;
        let result = SkillMetadata::from_content(content);
        assert!(result.is_err());
    }
}
