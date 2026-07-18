use serde::{Deserialize, Serialize};
use std::fmt;

/// Workspace standard for AI assistant configuration.
///
/// OpenCode and Claude Code are supported. Antigravity (Google Gemini) support
/// is paused and may be re-enabled in a future version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceStandard {
    #[default]
    OpenCode,
    Claude,
    // NOTE: Antigravity support is paused. These variants are preserved
    // for backward compatibility when deserializing existing kn.toml files,
    // but they are treated as OpenCode at runtime.
    // Antigravity,
    // Both,
}

impl fmt::Display for WorkspaceStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceStandard::OpenCode => write!(f, "opencode"),
            WorkspaceStandard::Claude => write!(f, "claude"),
        }
    }
}

impl WorkspaceStandard {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "opencode" | "1" => Some(Self::OpenCode),
            "claude" | "4" => Some(Self::Claude),
            // Backward compatibility: treat legacy values as OpenCode
            "antigravity" | "2" | "both" | "3" => {
                eprintln!(
                    "Warning: workspace_standard '{}' is deprecated, using 'opencode' instead",
                    s
                );
                Some(Self::OpenCode)
            }
            _ => None,
        }
    }
}
