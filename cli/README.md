# kn - Knowledge Framework CLI

A CLI tool for AI-assisted development workflows with a **global ~/.kn/ directory** for managing agents and skills across all your projects. Supports **OpenCode** and **Antigravity** AI assistants simultaneously.

## Installation

```bash
cd cli
cargo build --release
# Binary will be at: ./target/release/kn
```

Add to PATH:
```bash
# Linux/Mac
sudo cp target/release/kn /usr/local/bin/

# Or add to your shell profile:
export PATH="$PATH:/path/to/knowledge/cli/target/release"
```

## Quick Start

```bash
# 1. Install agents to global directory
kn agents install ./agents/rust ./agents/planner

# 2. Install skills to global directory
kn skills install ./skills/rust-best-practices

# 3. Initialize a project (interactive prompts)
cd your-project
kn init

# 4. Sync project symlinks (after config changes)
kn sync
```

## Architecture Overview

**Global Directory (~/.kn/):**
- `~/.kn/agents/` - Installed agents (single source of truth)
- `~/.kn/skills/` - Installed skills (single source of truth)

**Project Workspace:**
- `kn.toml` - Project configuration (agents, skills, workspace standard)
- **OpenCode**: Symlinks in `.opencode/skills/` and `.opencode/agents/`
- **Antigravity**: Symlinks in `.agent/skills/` and `./agents/`
- **Both**: Symlinks in all locations (recommended)

**Benefits:**
- ✅ Install once, use everywhere
- ✅ Consistent versions across projects
- ✅ Easy updates (update in ~/.kn/, run `kn sync`)
- ✅ Support both OpenCode and Antigravity simultaneously

## Usage

### Manage Agents

Install agents to global directory (~/.kn/agents/):
```bash
# Install from local path
kn agents install ./agents/rust

# Install multiple agents
kn agents install ./agents/rust ./agents/planner ./agents/devops

# List installed agents with metadata
kn agents list
```

**Agent Structure:**
```
~/.kn/agents/
└── rust/
    └── AGENTS.md  # Contains YAML frontmatter with metadata
```

**YAML Frontmatter Example:**
```yaml
---
name: rust
id_prefix: r5t
description: Rust development expert
required_skills:
  - rust-best-practices
  - docker-best-practices
recommended_skills:
  - github-actions-best-practices
tags:
  - rust
  - systems
---
```

### Manage Skills

Install skills to global directory (~/.kn/skills/):
```bash
# Install from local path
kn skills install ./skills/rust-best-practices

# Install from URL
kn skills install https://example.com/skills/typescript/SKILL.md

# List installed skills
kn skills list
```

**Skill Structure:**
```
~/.kn/skills/
└── rust-best-practices/
    ├── SKILL.md      # Main skill documentation
    ├── examples/     # (Optional) Code examples
    └── scripts/      # (Optional) Helper scripts
```

### Initialize Project

Initialize a new project with interactive prompts:
```bash
kn init
```

**Interactive Prompts:**
1. **Project name** - Auto-detected from directory name
2. **Workspace standard** - OpenCode only / Antigravity only / Both (recommended)
3. **Select agents** - Multi-select from ~/.kn/agents/
4. **Select recommended skills** - Optional skills suggested by selected agents

**Non-Interactive Mode:**
```bash
# Skip prompts, use defaults (no agents, no skills)
kn init --yes
```

**What Gets Created:**
- `kn.toml` - Project configuration
- `AGENTS.md` - Agent workflow instructions
- Symlinks to selected agents and skills (based on workspace standard)

**Example kn.toml:**
```toml
[project]
name = "my-project"
workspace_standard = "both"

[agents.rust]
id = "my-project-r5t"
role = "rust"
required_skills = ["rust-best-practices", "docker-best-practices"]

[skills]
enabled = ["rust-best-practices", "docker-best-practices", "bd-best-practices"]
```

### Sync Project Symlinks

After modifying `kn.toml`, sync symlinks to match configuration:
```bash
kn sync
```

This command:
- Reads `kn.toml` configuration
- Creates/updates symlinks based on `workspace_standard`
- Removes symlinks for disabled agents/skills

### Check Dependencies

Verify that all required dependencies are installed:
```bash
kn doctor
```

Show detailed output with installation status:
```bash
kn doctor --verbose
```

**Dependencies Checked:**
- ✅ Rust (1.70+) - Required for building kn
- ✅ Cargo - Rust package manager
- ✅ Git - Version control
- ✅ bd (beads) - Issue tracking system
- ⚪ Dolt - Optional, for Beads database
- ✅ Node.js (18.0+) - Required for MCP servers
- ✅ npm - Node package manager

Exit codes:
- `0` - All required dependencies met
- `1` - One or more critical dependencies missing

### Beads Templates

Generate issue templates for Beads workflow:

Print to stdout:
```bash
kn beads template task
kn beads template epic
kn beads template bug
kn beads template feature
kn beads template chore
```

Save to file:
```bash
kn beads template epic -o .beads/templates/epic.md
```

Force overwrite existing file:
```bash
kn beads template task -o task.md --force
```

### MCP Servers

Manage Model Context Protocol (MCP) servers for documentation lookup:

List available presets and configured servers:
```bash
kn mcp list
```

Add a preset server:
```bash
kn mcp add filesystem
kn mcp add postgres
kn mcp add github
```

Add with custom environment variables:
```bash
kn mcp add postgres -e POSTGRES_URL=postgresql://user:pass@localhost/db
```

Remove a server:
```bash
kn mcp remove postgres
```

## Workspace Structure Examples

When you run `kn init`, the following structure is created based on your workspace standard:

**Both (OpenCode + Antigravity):**
```
your-project/
├── kn.toml                    # Project configuration
├── AGENTS.md                  # Agent instructions
├── .opencode/
│   ├── agents/               # Symlinks to ~/.kn/agents/
│   └── skills/               # Symlinks to ~/.kn/skills/
├── .agent/
│   └── skills/               # Symlinks to ~/.kn/skills/
├── agents/                   # Symlinks to ~/.kn/agents/
└── skills/                   # Symlinks to ~/.kn/skills/
```

**OpenCode Only:**
```
your-project/
├── kn.toml
├── AGENTS.md
└── .opencode/
    ├── agents/
    └── skills/
```

**Antigravity Only:**
```
your-project/
├── kn.toml
├── AGENTS.md
├── .agent/
│   └── skills/
├── agents/
└── skills/
```

## Configuration (kn.toml)

```toml
[project]
name = "my-project"
description = ""
workspace_standard = "both"

[agents.rust]
id = "my-project-r5t"
role = "rust"
required_skills = ["rust-best-practices", "docker-best-practices"]

[agents.planner]
id = "my-project-x6e"
role = "planner"
required_skills = ["bd-best-practices"]

[skills]
enabled = [
  "rust-best-practices",
  "docker-best-practices",
  "bd-best-practices"
]
```

## Development

Build:
```bash
cargo build
```

Test:
```bash
cargo test
```

Run:
```bash
cargo run -- init
```

## Architecture

```
cli/
├── src/
│   ├── lib.rs                   # Public library API
│   ├── main.rs                  # CLI entry point
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs              # kn init (interactive project setup)
│   │   ├── agents.rs            # kn agents install/list
│   │   ├── skills.rs            # kn skills install/list
│   │   ├── sync.rs              # kn sync (update symlinks)
│   │   ├── beads.rs             # kn beads template
│   │   ├── mcp.rs               # kn mcp add/list/remove
│   │   └── doctor.rs            # kn doctor (dependency check)
│   ├── config/
│   │   ├── mod.rs
│   │   ├── kn_toml.rs           # KnConfig struct, save/load
│   │   └── workspace.rs         # WorkspaceStandard enum
│   ├── core/
│   │   ├── mod.rs
│   │   ├── kn_home.rs           # ~/.kn/ management functions
│   │   └── symlinks.rs          # Symlink creation logic
│   └── models/
│       ├── mod.rs
│       ├── agent.rs             # AgentMetadata with YAML parser
│       ├── skill.rs             # SkillMetadata with YAML parser
│       └── project.rs           # ProjectConfig
├── Cargo.toml
└── README.md
```

## License

MIT
