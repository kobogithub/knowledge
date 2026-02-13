# kn - Knowledge CLI

A CLI tool for AI-assisted development workflows, automating setup of agent configurations, skills, and issue tracking.

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

## Usage

### Initialize Project

Auto-detect project type and create configuration:
```bash
kn init
```

Skip interactive prompts:
```bash
kn init -y
```

Specify project type:
```bash
kn init --project-type rust
```

### Manage Skills

Install a skill from local path:
```bash
kn skills install /path/to/SKILL.md
```

Install from URL:
```bash
kn skills install https://example.com/skills/typescript/SKILL.md
```

Install by name (from agentskills.io):
```bash
kn skills install typescript
```

Force reinstall:
```bash
kn skills install typescript --force
```

List installed skills:
```bash
kn skills list
```

### What Gets Created

- **AGENTS.md** - Agent workflow instructions and quick reference
- **kn.toml** - Configuration file for agents, skills, and MCP servers

### Supported Project Types

- **Rust** - Detects `Cargo.toml`
- **Node** - Detects `package.json`
- **Python** - Detects `pyproject.toml` or `setup.py`
- **Go** - Detects `go.mod`
- **Monorepo** - Detects `pnpm-workspace.yaml` or `lerna.json`

## Configuration (kn.toml)

```toml
[project]
name = "my-project"
type = "Node"

[agents]
planner = "project-x6e"
implementation = "project-4yh"

[skills]
enabled = ["typescript", "react-19"]

[beads]
enabled = true
templates_dir = ".beads/templates"

[mcp]
servers = ["rust-docs", "mdn-web-docs"]
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
│   ├── main.rs              # CLI entry point, argument parsing
│   └── commands/
│       ├── mod.rs            # Command module exports
│       ├── init.rs           # Project initialization command
│       └── skills.rs         # Skills management (install, list)
├── Cargo.toml
└── README.md
```

## Next Steps (Phase 1 Roadmap)

- [x] `kn skills install <name>` - Install skills from agentskills.io ✅
- [x] `kn skills list` - List available/installed skills ✅
- [ ] `kn beads template <type>` - Generate issue templates
- [ ] `kn mcp add <server>` - Configure MCP servers
- [ ] Cross-platform symlink handling for Windows

## License

MIT
