use anyhow::Result;
use clap::Args;
use colored::*;
use std::process::Command;

#[derive(Args)]
pub struct DoctorCommand {
    /// Show detailed output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug)]
struct Dependency {
    name: &'static str,
    command: &'static str,
    version_flag: &'static str,
    required: bool,
    min_version: Option<&'static str>,
}

#[derive(Debug)]
struct CheckResult {
    name: String,
    installed: bool,
    version: Option<String>,
    required: bool,
    status: DependencyStatus,
}

#[derive(Debug, PartialEq)]
enum DependencyStatus {
    Ok,
    Missing,
    VersionTooOld,
    Optional,
}

impl DoctorCommand {
    pub fn execute(&self) -> Result<()> {
        println!("{}", "🔍 Checking dependencies...\n".bright_cyan().bold());

        let dependencies = vec![
            Dependency {
                name: "Rust",
                command: "rustc",
                version_flag: "--version",
                required: true,
                min_version: Some("1.70"),
            },
            Dependency {
                name: "Cargo",
                command: "cargo",
                version_flag: "--version",
                required: true,
                min_version: None,
            },
            Dependency {
                name: "Git",
                command: "git",
                version_flag: "--version",
                required: true,
                min_version: None,
            },
            Dependency {
                name: "bd",
                command: "bd",
                version_flag: "--version",
                required: true,
                min_version: None,
            },
            Dependency {
                name: "Dolt",
                command: "dolt",
                version_flag: "version",
                required: false,
                min_version: None,
            },
            Dependency {
                name: "Node.js",
                command: "node",
                version_flag: "--version",
                required: true,
                min_version: Some("18.0"),
            },
            Dependency {
                name: "npm",
                command: "npm",
                version_flag: "--version",
                required: true,
                min_version: None,
            },
        ];

        let mut results = Vec::new();
        let mut critical_missing = 0;

        for dep in dependencies {
            let result = self.check_dependency(&dep);

            if result.required && result.status == DependencyStatus::Missing {
                critical_missing += 1;
            }

            results.push(result);
        }

        // Print results
        self.print_results(&results);

        // Print summary
        println!();
        let total = results.len();
        let ok_count = results
            .iter()
            .filter(|r| r.status == DependencyStatus::Ok)
            .count();
        let optional_count = results
            .iter()
            .filter(|r| r.status == DependencyStatus::Optional)
            .count();

        if critical_missing == 0 {
            println!(
                "{}",
                format!("✓ Overall: {}/{} dependencies met", ok_count, total)
                    .bright_green()
                    .bold()
            );

            if optional_count > 0 {
                println!(
                    "{}",
                    format!("  ({} optional dependencies not installed)", optional_count)
                        .bright_black()
                );
            }

            println!("{}", "\n🎉 Ready to use!".bright_green().bold());
        } else {
            println!(
                "{}",
                format!(
                    "⚠ Overall: {}/{} required dependencies met",
                    ok_count - optional_count,
                    total - optional_count
                )
                .bright_yellow()
                .bold()
            );
            println!(
                "{}",
                format!("  {} critical dependencies missing", critical_missing).bright_red()
            );

            println!("\n{}", "Installation Instructions:".bright_white().bold());
            self.print_installation_instructions(&results);

            std::process::exit(1);
        }

        Ok(())
    }

    fn check_dependency(&self, dep: &Dependency) -> CheckResult {
        let output = Command::new(dep.command).arg(dep.version_flag).output();

        match output {
            Ok(output) if output.status.success() => {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = self.extract_version(&version_str);

                let status = if !dep.required {
                    DependencyStatus::Ok
                } else if let (Some(min), Some(current)) = (dep.min_version, &version) {
                    if self.version_meets_minimum(current, min) {
                        DependencyStatus::Ok
                    } else {
                        DependencyStatus::VersionTooOld
                    }
                } else {
                    DependencyStatus::Ok
                };

                CheckResult {
                    name: dep.name.to_string(),
                    installed: true,
                    version,
                    required: dep.required,
                    status,
                }
            }
            _ => {
                let status = if dep.required {
                    DependencyStatus::Missing
                } else {
                    DependencyStatus::Optional
                };

                CheckResult {
                    name: dep.name.to_string(),
                    installed: false,
                    version: None,
                    required: dep.required,
                    status,
                }
            }
        }
    }

    fn extract_version(&self, output: &str) -> Option<String> {
        // Extract version from command output
        // Most tools output "tool x.y.z" format
        let words: Vec<&str> = output.split_whitespace().collect();

        for word in words {
            // Look for version pattern (e.g., "1.93.0", "v20.10.0")
            if word.chars().any(|c| c.is_ascii_digit()) && word.contains('.') {
                return Some(word.trim_start_matches('v').to_string());
            }
        }

        None
    }

    fn version_meets_minimum(&self, current: &str, minimum: &str) -> bool {
        let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();

        let min_parts: Vec<u32> = minimum.split('.').filter_map(|s| s.parse().ok()).collect();

        for i in 0..min_parts.len().min(current_parts.len()) {
            if current_parts[i] > min_parts[i] {
                return true;
            }
            if current_parts[i] < min_parts[i] {
                return false;
            }
        }

        true
    }

    fn print_results(&self, results: &[CheckResult]) {
        for result in results {
            let symbol = match result.status {
                DependencyStatus::Ok => "✓".bright_green(),
                DependencyStatus::Missing => "✗".bright_red(),
                DependencyStatus::VersionTooOld => "⚠".bright_yellow(),
                DependencyStatus::Optional => "○".bright_black(),
            };

            let name = format!("{:<12}", result.name);

            let info = if let Some(ref version) = result.version {
                if self.verbose {
                    format!("{:<15} (installed)", version)
                } else {
                    version.clone()
                }
            } else if result.status == DependencyStatus::Optional {
                "(optional - not found)".bright_black().to_string()
            } else {
                "(not found)".bright_red().to_string()
            };

            println!("{} {} {}", symbol, name, info);
        }
    }

    fn print_installation_instructions(&self, results: &[CheckResult]) {
        for result in results {
            if result.required && !result.installed {
                println!("\n{}", format!("  • {}", result.name).bright_white().bold());

                match result.name.as_str() {
                    "Rust" => {
                        println!(
                            "    {}",
                            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                                .bright_cyan()
                        );
                        println!("    {}", "https://rustup.rs/".bright_black());
                    }
                    "Git" => {
                        println!(
                            "    {}",
                            "Ubuntu/Debian: sudo apt install git".bright_cyan()
                        );
                        println!("    {}", "macOS: brew install git".bright_cyan());
                        println!("    {}", "https://git-scm.com/downloads".bright_black());
                    }
                    "bd" => {
                        println!("    {}", "cargo install bd".bright_cyan());
                        println!(
                            "    {}",
                            "or visit: https://github.com/your-org/bd".bright_black()
                        );
                    }
                    "Node.js" => {
                        println!(
                            "    {}",
                            "Ubuntu/Debian: sudo apt install nodejs npm".bright_cyan()
                        );
                        println!("    {}", "macOS: brew install node".bright_cyan());
                        println!(
                            "    {}",
                            "Or use nvm: https://github.com/nvm-sh/nvm".bright_cyan()
                        );
                        println!("    {}", "https://nodejs.org/".bright_black());
                    }
                    "npm" => {
                        println!("    {}", "(comes with Node.js installation)".bright_black());
                    }
                    _ => {}
                }
            }
        }
    }
}
