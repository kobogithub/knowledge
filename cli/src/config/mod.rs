// NOTE: Antigravity (Gemini) support is paused. The gemini module is preserved
// in gemini.rs but not compiled. Re-enable by uncommenting the lines below.
// pub mod gemini;
pub mod claude_mcp;
pub mod kn_toml;
pub mod opencode;
pub mod workspace;

// pub use gemini::{GeminiConfig, GeminiMcpServer};
pub use claude_mcp::{ClaudeMcpConfig, ClaudeMcpServer};
pub use kn_toml::KnConfig;
pub use opencode::{OpenCodeConfig, OpenCodeMcpServer};
pub use workspace::WorkspaceStandard;
