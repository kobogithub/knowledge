use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::core::kn_home;

/// A named bundle of catalog skills, loaded from `~/.kn/stacks/<name>.toml`.
///
/// A stack-preset lets the user activate a whole set of skills in one step
/// (e.g. `kn init --stack web-astro`) instead of enabling each skill by hand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackPreset {
    /// Unique preset identifier (should match the file stem).
    pub name: String,
    /// One-line human description, shown by `kn stack list`.
    #[serde(default)]
    pub description: String,
    /// Ordered list of catalog skill names this preset bundles.
    pub skills: Vec<String>,
}

impl StackPreset {
    /// Skills referenced by this preset that are missing from the given skills
    /// directory (no `<skill>/SKILL.md`).
    pub fn missing_skills_in(&self, skills_dir: &Path) -> Vec<String> {
        self.skills
            .iter()
            .filter(|s| !skills_dir.join(s).join("SKILL.md").exists())
            .cloned()
            .collect()
    }

    /// Skills referenced by this preset that are missing from `~/.kn/skills/`.
    pub fn missing_skills(&self) -> Result<Vec<String>> {
        Ok(self.missing_skills_in(&kn_home::skills_dir()?))
    }
}

/// Load a single preset by name from `~/.kn/stacks/`.
pub fn load_preset(name: &str) -> Result<StackPreset> {
    load_preset_from(&kn_home::stacks_dir()?, name)
}

/// Load a single preset by name from an explicit stacks directory.
pub fn load_preset_from(stacks_dir: &Path, name: &str) -> Result<StackPreset> {
    let path = stacks_dir.join(format!("{name}.toml"));
    if !path.exists() {
        bail!("Unknown stack preset '{name}'");
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read preset {}", path.display()))?;
    let preset: StackPreset = toml::from_str(&content)
        .with_context(|| format!("Failed to parse preset {}", path.display()))?;
    Ok(preset)
}

/// List all presets in `~/.kn/stacks/`, sorted by name.
pub fn list_presets() -> Result<Vec<StackPreset>> {
    list_presets_from(&kn_home::stacks_dir()?)
}

/// List all presets in an explicit stacks directory, sorted by name.
///
/// Malformed TOML files are skipped with a warning to stderr rather than
/// aborting the whole listing.
pub fn list_presets_from(stacks_dir: &Path) -> Result<Vec<StackPreset>> {
    if !stacks_dir.exists() {
        return Ok(Vec::new());
    }

    let mut presets = Vec::new();
    for entry in std::fs::read_dir(stacks_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        match std::fs::read_to_string(&path)
            .ok()
            .and_then(|c| toml::from_str::<StackPreset>(&c).ok())
        {
            Some(preset) => presets.push(preset),
            None => eprintln!("Warning: skipping malformed preset {}", path.display()),
        }
    }

    presets.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(presets)
}

/// Names of all presets available in `~/.kn/stacks/` (for error messages).
pub fn list_preset_names() -> Result<Vec<String>> {
    Ok(list_presets()?.into_iter().map(|p| p.name).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_preset(dir: &Path, name: &str, body: &str) {
        fs::write(dir.join(format!("{name}.toml")), body).unwrap();
    }

    fn make_skill(skills_dir: &Path, name: &str) {
        let d = skills_dir.join(name);
        fs::create_dir_all(&d).unwrap();
        fs::write(d.join("SKILL.md"), "# skill").unwrap();
    }

    #[test]
    fn parses_a_preset_file() {
        let tmp = std::env::temp_dir().join(format!("kn_stack_parse_{}", std::process::id()));
        fs::create_dir_all(&tmp).unwrap();
        write_preset(
            &tmp,
            "web-astro",
            "name = \"web-astro\"\ndescription = \"web\"\nskills = [\"astro-best-practices\", \"htmx-best-practices\"]\n",
        );

        let p = load_preset_from(&tmp, "web-astro").unwrap();
        assert_eq!(p.name, "web-astro");
        assert_eq!(p.description, "web");
        assert_eq!(
            p.skills,
            vec!["astro-best-practices", "htmx-best-practices"]
        );

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn unknown_preset_errors() {
        let tmp = std::env::temp_dir().join(format!("kn_stack_unknown_{}", std::process::id()));
        fs::create_dir_all(&tmp).unwrap();
        assert!(load_preset_from(&tmp, "does-not-exist").is_err());
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn lists_presets_sorted_and_skips_malformed() {
        let tmp = std::env::temp_dir().join(format!("kn_stack_list_{}", std::process::id()));
        fs::create_dir_all(&tmp).unwrap();
        write_preset(&tmp, "b-stack", "name = \"b-stack\"\nskills = [\"x\"]\n");
        write_preset(&tmp, "a-stack", "name = \"a-stack\"\nskills = [\"y\"]\n");
        write_preset(&tmp, "broken", "this is = not valid toml [[[");

        let presets = list_presets_from(&tmp).unwrap();
        let names: Vec<_> = presets.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["a-stack", "b-stack"]);

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn empty_dir_lists_nothing() {
        let tmp = std::env::temp_dir().join(format!("kn_stack_empty_{}", std::process::id()));
        // Do not create it — exercise the missing-dir path.
        let presets = list_presets_from(&tmp).unwrap();
        assert!(presets.is_empty());
    }

    #[test]
    fn missing_skills_detected() {
        let tmp = std::env::temp_dir().join(format!("kn_stack_missing_{}", std::process::id()));
        let skills = tmp.join("skills");
        fs::create_dir_all(&skills).unwrap();
        make_skill(&skills, "astro-best-practices");
        // htmx-best-practices intentionally absent.

        let preset = StackPreset {
            name: "web-astro".into(),
            description: String::new(),
            skills: vec!["astro-best-practices".into(), "htmx-best-practices".into()],
        };

        let missing = preset.missing_skills_in(&skills);
        assert_eq!(missing, vec!["htmx-best-practices"]);

        fs::remove_dir_all(&tmp).ok();
    }
}
