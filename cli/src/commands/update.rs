use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Args)]
pub struct UpdateCommand {
    /// Show available version without updating
    #[arg(long)]
    check: bool,

    /// Force update even if already on latest version
    #[arg(long)]
    force: bool,
}

const GITHUB_REPO: &str = "kobogithub/knowledge";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl UpdateCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", "🔍 Checking for updates...".bright_cyan().bold());
        println!("  Current version: {}", CURRENT_VERSION.bright_yellow());

        // Get latest release from GitHub
        let latest_version = self.get_latest_version()?;
        println!("  Latest version:  {}", latest_version.bright_green());

        // Compare versions
        if !self.force && latest_version == CURRENT_VERSION {
            println!(
                "\n{}",
                "✓ You are already on the latest version!".bright_green()
            );
            return Ok(());
        }

        if self.check {
            if latest_version != CURRENT_VERSION {
                println!(
                    "\n{}",
                    format!(
                        "⚠ Update available: {} → {}",
                        CURRENT_VERSION, latest_version
                    )
                    .bright_yellow()
                );
            }
            return Ok(());
        }

        // Perform update
        println!(
            "\n{}",
            format!("📥 Updating to version {}...", latest_version)
                .bright_cyan()
                .bold()
        );

        self.update_cli(&latest_version)?;

        println!("\n{}", "✓ Update complete!".bright_green().bold());
        println!(
            "  Run {} to verify the new version.",
            "kn --version".bright_white()
        );

        Ok(())
    }

    fn get_latest_version(&self) -> Result<String> {
        // Use gh CLI to get latest release
        let output = Command::new("gh")
            .args([
                "release",
                "view",
                "--repo",
                GITHUB_REPO,
                "--json",
                "tagName",
                "--jq",
                ".tagName",
            ])
            .output()
            .context("Failed to execute 'gh' command. Is GitHub CLI installed?")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to fetch latest release: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let tag = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in gh output")?
            .trim()
            .to_string();

        // Remove 'v' prefix if present
        Ok(tag.strip_prefix('v').unwrap_or(&tag).to_string())
    }

    fn update_cli(&self, version: &str) -> Result<()> {
        // Get current executable path
        let current_exe = env::current_exe().context("Failed to get current executable path")?;
        println!(
            "  Installing to: {}",
            current_exe.display().to_string().bright_white()
        );

        // Determine platform
        let (os, arch) = self.get_platform()?;
        let asset_name = format!("kn-{}-{}", os, arch);
        println!("  Platform: {}", asset_name.bright_white());

        // Download tarball from GitHub release
        let download_url = format!(
            "https://github.com/{}/releases/download/v{}/{}.tar.gz",
            GITHUB_REPO, version, asset_name
        );

        println!("  Downloading from GitHub...");

        // Download tarball
        let temp_tarball = self.download_binary(&download_url)?;

        // Extract tarball
        println!("  Extracting archive...");
        let temp_dir = std::env::temp_dir().join("kn-update-extract");
        fs::create_dir_all(&temp_dir)?;

        let output = Command::new("tar")
            .args([
                "xzf",
                temp_tarball.to_str().unwrap(),
                "-C",
                temp_dir.to_str().unwrap(),
            ])
            .output()
            .context("Failed to extract tarball")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to extract tarball: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let extracted_binary = temp_dir.join("kn");

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&extracted_binary)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&extracted_binary, perms)?;
        }

        // Replace current binary
        println!("  Installing new binary...");
        fs::rename(&extracted_binary, &current_exe)
            .context("Failed to replace current binary. Try running with sudo.")?;

        // Clean up
        let _ = fs::remove_file(&temp_tarball);
        let _ = fs::remove_dir_all(&temp_dir);

        Ok(())
    }

    fn get_platform(&self) -> Result<(&'static str, &'static str)> {
        let os = if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            anyhow::bail!("Unsupported operating system");
        };

        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            anyhow::bail!("Unsupported architecture");
        };

        Ok((os, arch))
    }

    fn download_binary(&self, url: &str) -> Result<PathBuf> {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("kn-update");

        let output = Command::new("curl")
            .args(["-L", "-f", "-o", temp_file.to_str().unwrap(), url])
            .output()
            .context("Failed to execute 'curl' command. Is curl installed?")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to download binary: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(temp_file)
    }
}
