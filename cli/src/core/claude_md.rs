use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Build the project-root `CLAUDE.md` content that makes the planner the
/// default persona for the main Claude Code thread.
///
/// Claude Code does not read `AGENTS.md` natively — only `CLAUDE.md` is
/// loaded automatically at session start. `@path` imports are expanded at
/// load time, so this pulls in the team overview and the planner's own
/// instructions without duplicating their content.
pub fn generate_claude_md(project_name: &str) -> String {
    format!(
        r#"# CLAUDE.md

@AGENTS.md

## Default Role: Planner Agent

Unless the user explicitly asks you to work as a different specialized role
(frontend, backend, rust, devops, security, qa, uiux-tester, docs-writer, biz,
finanzas — see AGENTS.md), you act as the **Planner Agent** for {project_name}
from the start of the session: analyze the request, run the spec-kit workflow
(`/speckit-specify` → `/speckit-plan` → `/speckit-tasks`), and assign work
before any implementation begins.

@.claude/agents/planner.md
"#,
        project_name = project_name
    )
}

/// Write `CLAUDE.md` at the project root if it doesn't already exist.
/// Returns `true` if the file was created, `false` if it was left alone.
pub fn write_claude_md_if_missing(project_root: &Path, project_name: &str) -> Result<bool> {
    let path = project_root.join("CLAUDE.md");
    if path.exists() {
        return Ok(false);
    }

    let content = generate_claude_md(project_name);
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Unique scratch dir per test invocation (no tempfile crate dependency).
    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("kn-test-claude-md-{}-{}", name, nanos));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_generate_claude_md_imports_agents_and_planner() {
        let content = generate_claude_md("my-project");
        assert!(content.contains("@AGENTS.md"));
        assert!(content.contains("@.claude/agents/planner.md"));
        assert!(content.contains("Planner Agent"));
        assert!(content.contains("my-project"));
    }

    #[test]
    fn test_write_claude_md_if_missing_creates_file() {
        let dir = scratch_dir("create");
        let created = write_claude_md_if_missing(&dir, "demo").unwrap();
        assert!(created);
        assert!(dir.join("CLAUDE.md").exists());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_write_claude_md_if_missing_skips_existing() {
        let dir = scratch_dir("skip");
        fs::write(dir.join("CLAUDE.md"), "custom content").unwrap();

        let created = write_claude_md_if_missing(&dir, "demo").unwrap();
        assert!(!created);
        assert_eq!(
            fs::read_to_string(dir.join("CLAUDE.md")).unwrap(),
            "custom content"
        );
        fs::remove_dir_all(&dir).ok();
    }
}
