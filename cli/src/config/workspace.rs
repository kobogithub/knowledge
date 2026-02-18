use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceStandard {
    OpenCode,
    Antigravity,
    Both,
}

impl fmt::Display for WorkspaceStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceStandard::OpenCode => write!(f, "opencode"),
            WorkspaceStandard::Antigravity => write!(f, "antigravity"),
            WorkspaceStandard::Both => write!(f, "both"),
        }
    }
}

impl WorkspaceStandard {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "opencode" | "1" => Some(Self::OpenCode),
            "antigravity" | "2" => Some(Self::Antigravity),
            "both" | "3" => Some(Self::Both),
            _ => None,
        }
    }
}
