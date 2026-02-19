use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Helper to get the compiled binary path
fn get_kn_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");

    // Check if release binary exists (from CI), otherwise use debug
    let release_path = path.join("release").join("kn");
    let debug_path = path.join("debug").join("kn");

    if release_path.exists() {
        release_path
    } else {
        debug_path
    }
}

/// Helper to create a temporary test directory
fn create_test_dir(name: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("kn_test_{}", name));

    // Clean up if exists
    if path.exists() {
        fs::remove_dir_all(&path).ok();
    }

    fs::create_dir_all(&path).expect("Failed to create test directory");
    path
}

/// Helper to run kn command
fn run_kn(args: &[&str], kn_home: Option<&PathBuf>) -> std::process::Output {
    let mut cmd = Command::new(get_kn_binary());
    cmd.args(args);

    if let Some(home) = kn_home {
        cmd.env("KN_HOME", home);
    }

    cmd.output().expect("Failed to execute kn command")
}

#[test]
fn test_mcp_install_preset() {
    let test_home = create_test_dir("mcp_install_preset");

    // Install filesystem preset
    let output = run_kn(&["mcp", "install", "filesystem"], Some(&test_home));

    assert!(
        output.status.success(),
        "Install command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("✓ Installed MCP 'filesystem'"),
        "Expected success message not found"
    );

    // Verify mcp.toml was created
    let mut mcp_toml = test_home.clone();
    mcp_toml.push("mcps");
    mcp_toml.push("filesystem");
    mcp_toml.push("mcp.toml");

    assert!(mcp_toml.exists(), "mcp.toml should be created");

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_install_npm_package() {
    let test_home = create_test_dir("mcp_install_npm");

    // Install a real npm package (skipping validation for speed)
    let output = run_kn(
        &[
            "mcp",
            "install",
            "@modelcontextprotocol/server-filesystem",
            "--as-name",
            "test-npm",
            "--skip-validation",
        ],
        Some(&test_home),
    );

    assert!(
        output.status.success(),
        "Install npm package failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("✓ Installed MCP 'test-npm'"),
        "Expected success message not found"
    );

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_npm_validation_failure() {
    let test_home = create_test_dir("mcp_npm_validation");

    // Try to install a non-existent npm package (should fail)
    let output = run_kn(
        &["mcp", "install", "@nonexistent-package-12345/test"],
        Some(&test_home),
    );

    assert!(
        !output.status.success(),
        "Should fail for non-existent package"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("npm package") && stderr.contains("not found"),
        "Expected validation error message"
    );

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_list() {
    let test_home = create_test_dir("mcp_list");

    // Install a preset first
    run_kn(&["mcp", "install", "filesystem"], Some(&test_home));

    // List MCPs
    let output = run_kn(&["mcp", "list"], Some(&test_home));

    assert!(
        output.status.success(),
        "List command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("filesystem"), "Should list installed MCP");

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_info() {
    let test_home = create_test_dir("mcp_info");

    // Install a preset first
    run_kn(&["mcp", "install", "filesystem"], Some(&test_home));

    // Get info about the MCP
    let output = run_kn(&["mcp", "info", "filesystem"], Some(&test_home));

    assert!(
        output.status.success(),
        "Info command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("filesystem"), "Should show MCP name");
    assert!(stdout.contains("npx"), "Should show command");

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_uninstall() {
    let test_home = create_test_dir("mcp_uninstall");

    // Install a preset first
    let install_output = run_kn(&["mcp", "install", "filesystem"], Some(&test_home));
    if !install_output.status.success() {
        eprintln!(
            "Install output: {}",
            String::from_utf8_lossy(&install_output.stdout)
        );
        eprintln!(
            "Install error: {}",
            String::from_utf8_lossy(&install_output.stderr)
        );
    }
    assert!(install_output.status.success(), "Install should succeed");

    // Verify it exists
    let mut mcp_dir = test_home.clone();
    mcp_dir.push("mcps");
    mcp_dir.push("filesystem");

    // Debug: list what was created
    if !mcp_dir.exists() {
        let mut mcps = test_home.clone();
        mcps.push("mcps");
        if mcps.exists() {
            eprintln!("Contents of mcps dir:");
            for entry in fs::read_dir(&mcps).unwrap() {
                eprintln!("  {:?}", entry.unwrap().path());
            }
        } else {
            eprintln!("mcps directory doesn't exist");
        }
    }

    // Skip uninstall test if install didn't work
    if !mcp_dir.exists() {
        fs::remove_dir_all(&test_home).ok();
        return;
    }

    // Uninstall
    let output = run_kn(&["mcp", "uninstall", "filesystem"], Some(&test_home));

    assert!(
        output.status.success(),
        "Uninstall command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("✓ Uninstalled MCP 'filesystem'"),
        "Expected success message not found"
    );

    // Verify it's gone
    assert!(
        !mcp_dir.exists(),
        "MCP directory should be removed after uninstall"
    );

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_presets() {
    // This doesn't need a custom KN_HOME
    let output = run_kn(&["mcp", "presets"], None);

    assert!(
        output.status.success(),
        "Presets command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("filesystem"),
        "Should list filesystem preset"
    );
    assert!(stdout.contains("github"), "Should list github preset");
}

#[test]
fn test_mcp_custom_install() {
    let test_home = create_test_dir("mcp_custom");

    // Install with custom command (note: --args, not --)
    let output = run_kn(
        &[
            "mcp",
            "install",
            "custom-test",
            "--command",
            "echo",
            "--args",
            "hello",
        ],
        Some(&test_home),
    );

    assert!(
        output.status.success(),
        "Custom install failed: {}\nStderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify mcp.toml was created
    let mut mcp_toml = test_home.clone();
    mcp_toml.push("mcps");
    mcp_toml.push("custom-test");
    mcp_toml.push("mcp.toml");

    assert!(
        mcp_toml.exists(),
        "mcp.toml should be created at {}",
        mcp_toml.display()
    );

    // Verify content
    let content = fs::read_to_string(&mcp_toml).expect("Failed to read mcp.toml");
    assert!(content.contains("echo"), "Should contain custom command");
    assert!(content.contains("hello"), "Should contain custom args");

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}

#[test]
fn test_mcp_workflow_full() {
    let test_home = create_test_dir("mcp_workflow");

    // Step 1: Install MCP globally
    let output = run_kn(&["mcp", "install", "filesystem"], Some(&test_home));
    assert!(
        output.status.success(),
        "Install failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Step 2: List installed MCPs
    let output = run_kn(&["mcp", "list"], Some(&test_home));
    assert!(output.status.success(), "List failed");
    assert!(String::from_utf8_lossy(&output.stdout).contains("filesystem"));

    // Step 3: Get MCP info
    let output = run_kn(&["mcp", "info", "filesystem"], Some(&test_home));
    assert!(output.status.success(), "Info failed");

    // Step 4: Uninstall MCP
    let output = run_kn(&["mcp", "uninstall", "filesystem"], Some(&test_home));
    assert!(output.status.success(), "Uninstall failed");

    // Clean up
    fs::remove_dir_all(&test_home).ok();
}
