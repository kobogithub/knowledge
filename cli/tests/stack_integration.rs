use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Path to the kn binary built for this test run (set by Cargo).
fn kn_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kn"))
}

/// Repository root (parent of the cli/ crate).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            fs::copy(&path, &target).unwrap();
        }
    }
}

/// Create a fresh temp dir, cleaning any previous run.
fn fresh_dir(name: &str) -> PathBuf {
    let path = env::temp_dir().join(format!("kn_stack_it_{name}"));
    if path.exists() {
        fs::remove_dir_all(&path).ok();
    }
    fs::create_dir_all(&path).unwrap();
    path
}

/// Populate a temp KN_HOME with the repo's real skills, stacks, and agents.
fn setup_home(name: &str) -> PathBuf {
    let home = fresh_dir(&format!("home_{name}"));
    let root = repo_root();
    copy_dir(&root.join("skills"), &home.join("skills"));
    copy_dir(&root.join("stacks"), &home.join("stacks"));
    copy_dir(&root.join("agents"), &home.join("agents"));
    fs::create_dir_all(home.join("formulas")).unwrap();
    fs::create_dir_all(home.join("mcps")).unwrap();
    home
}

fn run(args: &[&str], home: &Path, cwd: &Path) -> Output {
    Command::new(kn_binary())
        .args(args)
        .env("KN_HOME", home)
        .current_dir(cwd)
        .output()
        .expect("failed to run kn")
}

// ---------------------------------------------------------------------------
// US1 — curated catalog
// ---------------------------------------------------------------------------

#[test]
fn us1_catalog_is_curated() {
    let skills = repo_root().join("skills");

    // Removed skills are gone.
    for removed in [
        "aws-best-practices",
        "jsonnet-best-practices",
        "kubernetes-best-practices",
        "terraform-best-practices",
        "notion-reporting-standard",
    ] {
        assert!(
            !skills.join(removed).exists(),
            "removed skill still present: {removed}"
        );
    }

    // New skills are present and installable (have a SKILL.md).
    for added in [
        "fastapi-best-practices",
        "htmx-best-practices",
        "go-best-practices",
        "railway-best-practices",
    ] {
        assert!(
            skills.join(added).join("SKILL.md").exists(),
            "new skill missing SKILL.md: {added}"
        );
    }
}

#[test]
fn every_preset_references_existing_skills() {
    let root = repo_root();
    let skills = root.join("skills");
    let stacks = root.join("stacks");

    for entry in fs::read_dir(&stacks).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let content = fs::read_to_string(&path).unwrap();
        // Cheap parse: every quoted token on a `skills` line names a real skill.
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('"') {
                let skill = line.trim_matches(|c| c == '"' || c == ',' || c == ' ');
                assert!(
                    skills.join(skill).join("SKILL.md").exists(),
                    "{} references missing skill {skill}",
                    path.display()
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// US2 — init from a preset
// ---------------------------------------------------------------------------

#[test]
fn us2_init_with_preset_enables_bundle_and_records_stack() {
    let home = setup_home("init_ok");
    let proj = fresh_dir("proj_ok");

    let out = run(&["init", "-y", "--stack", "web-astro"], &home, &proj);
    assert!(
        out.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let toml = fs::read_to_string(proj.join("kn.toml")).expect("kn.toml written");
    assert!(
        toml.contains("stack = \"web-astro\""),
        "stack not recorded:\n{toml}"
    );
    for skill in [
        "astro-best-practices",
        "htmx-best-practices",
        "supabase-postgres-best-practices",
        "railway-best-practices",
        "github-actions-best-practices",
        "docker-best-practices",
    ] {
        assert!(
            toml.contains(skill),
            "preset skill {skill} not enabled:\n{toml}"
        );
    }

    // No duplicate skill entries.
    let count = toml.matches("docker-best-practices").count();
    assert_eq!(count, 1, "docker-best-practices duplicated:\n{toml}");
}

#[test]
fn us2_unknown_preset_fails_and_writes_nothing() {
    let home = setup_home("init_unknown");
    let proj = fresh_dir("proj_unknown");

    let out = run(&["init", "-y", "--stack", "does-not-exist"], &home, &proj);
    assert!(!out.status.success(), "unknown preset should fail");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Unknown stack preset"),
        "missing helpful error: {stderr}"
    );
    assert!(
        !proj.join("kn.toml").exists(),
        "kn.toml must not be written"
    );
}

#[test]
fn us2_preset_with_missing_skill_fails_and_writes_nothing() {
    let home = setup_home("init_missing");
    let proj = fresh_dir("proj_missing");

    // Remove a skill that web-astro needs.
    fs::remove_dir_all(home.join("skills").join("htmx-best-practices")).unwrap();

    let out = run(&["init", "-y", "--stack", "web-astro"], &home, &proj);
    assert!(!out.status.success(), "missing skill should fail");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("htmx-best-practices"),
        "error should name the missing skill: {stderr}"
    );
    assert!(
        !proj.join("kn.toml").exists(),
        "kn.toml must not be written"
    );
}

#[test]
fn us2_malformed_preset_reports_parse_error_not_unknown() {
    // Fix #1: an existing-but-broken preset must surface a parse error.
    let home = setup_home("init_malformed");
    let proj = fresh_dir("proj_malformed");

    fs::write(
        home.join("stacks").join("broken.toml"),
        "this is = not valid toml [[[",
    )
    .unwrap();

    let out = run(&["init", "-y", "--stack", "broken"], &home, &proj);
    assert!(!out.status.success(), "malformed preset should fail");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Failed to parse"),
        "should surface the parse error: {stderr}"
    );
    assert!(
        !stderr.contains("Unknown stack preset"),
        "parse error must not be masked as unknown: {stderr}"
    );
    assert!(
        !proj.join("kn.toml").exists(),
        "kn.toml must not be written"
    );
}

// ---------------------------------------------------------------------------
// US3 — discover presets
// ---------------------------------------------------------------------------

#[test]
fn us3_stack_list_shows_presets() {
    let home = setup_home("list");
    let cwd = fresh_dir("list_cwd");

    let out = run(&["stack", "list"], &home, &cwd);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    for preset in ["web-astro", "api-fastapi", "cli-rust", "cli-go", "data-py"] {
        assert!(
            stdout.contains(preset),
            "preset {preset} not listed:\n{stdout}"
        );
    }
}

#[test]
fn us3_stack_list_empty_is_graceful() {
    // A KN_HOME with no stacks/ directory.
    let home = fresh_dir("home_empty");
    fs::create_dir_all(home.join("skills")).unwrap();
    let cwd = fresh_dir("empty_cwd");

    let out = run(&["stack", "list"], &home, &cwd);
    assert!(out.status.success(), "empty list should exit 0");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("No stack-presets"),
        "expected graceful empty message:\n{stdout}"
    );
}
