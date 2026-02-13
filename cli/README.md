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

Skip auto-installation of recommended skills:
```bash
kn init -y --no-skills
```

Specify project type:
```bash
kn init --project-type rust
```

**Skills Auto-Installation:**

`kn init` automatically installs recommended skills based on detected project type:

- **Rust**: `rust-best-practices`, `docker-best-practices`, `bash-best-practices`
- **Python**: `python-best-practices`, `docker-best-practices`, `bash-best-practices`
- **Node**: `docker-best-practices`, `bash-best-practices`
- **Go**: `docker-best-practices`, `bash-best-practices`
- **Monorepo**: `docker-best-practices`, `bash-best-practices`
- **Unknown**: `bash-best-practices`

Use `--no-skills` flag to skip auto-installation.

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

Generate all templates:
```bash
mkdir -p .beads/templates
kn beads template epic -o .beads/templates/epic.md
kn beads template task -o .beads/templates/task.md
kn beads template bug -o .beads/templates/bug.md
kn beads template feature -o .beads/templates/feature.md
kn beads template chore -o .beads/templates/chore.md
```

### MCP Servers

Manage Model Context Protocol (MCP) servers for documentation lookup and external tool integration:

List available presets and configured servers:
```bash
kn mcp list
```

Add a preset server:
```bash
kn mcp add filesystem
kn mcp add postgres
kn mcp add github
kn mcp add brave-search
kn mcp add puppeteer
```

Add with custom environment variables:
```bash
kn mcp add postgres -e POSTGRES_URL=postgresql://user:pass@localhost/db
kn mcp add github -e GITHUB_TOKEN=ghp_your_token_here
```

Add a custom npm package:
```bash
kn mcp add @modelcontextprotocol/server-slack
kn mcp add @myorg/custom-mcp-server
```

Add a custom command:
```bash
kn mcp add my-server --command python -a "-m" -a "my_mcp_server"
```

Remove a server:
```bash
kn mcp remove postgres
```

#### Available Presets

- **filesystem** - Access local files and directories
- **postgres** - PostgreSQL database queries and schema inspection
- **github** - GitHub API integration (issues, PRs, repos)
- **brave-search** - Web search via Brave Search API
- **puppeteer** - Web scraping and browser automation

### What Gets Created

- **AGENTS.md** - Agent workflow instructions and quick reference
- **kn.toml** - Configuration file for agents, skills, and MCP servers
- **skills/** - Directory with auto-installed skills (based on project type)

### Supported Project Types

**Base Languages:**
- **Rust** - Detects `Cargo.toml`
- **Node** - Detects `package.json`
- **Python** - Detects `pyproject.toml` or `setup.py`
- **Go** - Detects `go.mod`

**Framework Detection:**
- **Rust**: Actix, Axum, Tauri, CLI tools (clap)
- **Node**: Next.js, Astro, React, Express, NestJS
- **Python**: FastAPI, Django, Flask

**Workspace/Monorepo Detection:**
- **Cargo workspaces** - Detects `[workspace]` in `Cargo.toml`
- **pnpm workspaces** - Detects `pnpm-workspace.yaml`
- **npm/yarn workspaces** - Detects `"workspaces"` in `package.json`
- **Lerna** - Detects `lerna.json`

The detected information is displayed during `kn init` and written to `kn.toml` and `AGENTS.md`.

## Configuration (kn.toml)

```toml
[project]
name = "my-project"
type = "Python (FastAPI)"

[agents]
planner = "project-x6e"
implementation = "project-4yh"

[skills]
enabled = ["typescript", "react-19"]

[beads]
enabled = true
templates_dir = ".beads/templates"

[mcp.servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "."]

[mcp.servers.postgres]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-postgres"]

[mcp.servers.postgres.env]
POSTGRES_URL = "postgresql://localhost/mydb"
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
│       ├── skills.rs         # Skills management (install, list)
│       └── beads.rs          # Beads templates (epic, task, bug, feature, chore)
├── Cargo.toml
└── README.md
```

## Next Steps (Phase 1 Roadmap)

- [x] `kn skills install <name>` - Install skills from agentskills.io ✅
- [x] `kn skills list` - List available/installed skills ✅
- [x] `kn beads template <type>` - Generate issue templates ✅
- [ ] `kn mcp add <server>` - Configure MCP servers
- [ ] Cross-platform symlink handling for Windows

## License

MIT
