# Getting Started with Knowledge Framework

Welcome! This guide will walk you through setting up and using the Knowledge Framework CLI (`kn`) for the first time.

## Prerequisites

- **Rust 1.93+** - [Install Rust](https://rustup.rs/)
- **Git** - For version control
- **uv** + **specify-cli** (optional) - [Install spec-kit](https://github.com/github/spec-kit) for the spec-driven workflow (`specify init . --integration claude`)

## Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/kobogithub/knowledge.git
cd knowledge

# Build the CLI
cd cli
cargo build --release

# The binary will be at: ./target/release/kn
```

### Option 2: Install Globally

```bash
# After building, copy to system path
sudo cp target/release/kn /usr/local/bin/

# Verify installation
kn --version
```

### Option 3: Add to PATH

```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$PATH:/path/to/knowledge/cli/target/release"

# Reload shell
source ~/.bashrc  # or ~/.zshrc
```

## Your First Project

### Step 1: Initialize a New Project

```bash
# Navigate to your project
cd ~/projects/my-awesome-app

# Initialize with Knowledge Framework
kn init

# Or skip prompts with -y
kn init -y
```

**What gets created:**
- `AGENTS.md` - Agent workflow instructions
- `kn.toml` - Project configuration file

### Step 2: Review Generated Files

```bash
# Check the AGENTS.md file
cat AGENTS.md

# Review configuration
cat kn.toml
```

### Step 3: Customize Your Setup

Edit `kn.toml` to add your agent IDs:

```toml
[project]
name = "my-awesome-app"
type = "Node"

[agents]
planner = "myproject-abc"
implementation = "myproject-xyz"

[skills]
enabled = ["typescript", "react-19"]

[mcp]
servers = ["rust-docs", "mdn-web-docs"]
```

## Working with Skills

### Installing Skills

Skills are reusable knowledge modules for AI agents.

```bash
# Install from agentskills.io (by name)
kn skills install typescript

# Install from URL
kn skills install https://raw.githubusercontent.com/agentskills/skills/main/typescript/SKILL.md

# Install from local file
kn skills install /path/to/custom/SKILL.md

# Force reinstall
kn skills install typescript --force
```

### Listing Skills

```bash
kn skills list
```

Output example:
```
📚 Installed Skills

  ● typescript
    Scope: programming
    TypeScript best practices and patterns
    Auto-invoke: yes

  ● react-19
    Scope: frontend
    React 19 features and hooks
    Auto-invoke: no
```

### Creating Custom Skills

Create a `SKILL.md` file with YAML frontmatter:

```markdown
---
name: my-custom-skill
scope: backend
auto_invoke: true
description: Custom patterns for my team
---

# My Custom Skill

## Best Practices

1. Always use TypeScript
2. Write tests first
3. Document public APIs

## Code Examples

\`\`\`typescript
// Example code here
\`\`\`
```

Install it:
```bash
kn skills install ./my-custom-skill/SKILL.md
```

## Generating Issue Templates

> These `kn beads template` commands generate standalone markdown templates and are
> independent of the spec-kit workflow below — they are not part of the current agent
> coordination model (see [ADR-006](./adr/006-adopt-speckit-remove-beads.md)) and are kept
> here only because the CLI subcommand still exists.

### Creating Templates

```bash
# Create templates directory
mkdir -p .beads/templates

# Generate templates
kn beads template epic -o .beads/templates/epic.md
kn beads template task -o .beads/templates/task.md
kn beads template bug -o .beads/templates/bug.md
kn beads template feature -o .beads/templates/feature.md
kn beads template chore -o .beads/templates/chore.md
```

### Using Templates

```bash
# Print to stdout
kn beads template task

# Pipe to file
kn beads template epic > planning/mvp-epic.md

# Direct output
kn beads template bug -o bugs/login-issue.md
```

### Available Template Types

1. **Epic** - Large features spanning multiple tasks
2. **Task** - Discrete units of work
3. **Bug** - Issue reports with reproduction steps
4. **Feature** - New functionality with requirements
5. **Chore** - Maintenance, refactoring, technical work

## Configuring MCP Servers

Model Context Protocol (MCP) servers provide external capabilities to AI agents like database access, web search, and file system operations.

### Adding Preset Servers

```bash
# Filesystem access (recommended for all projects)
kn mcp add filesystem

# Database access
kn mcp add postgres -e POSTGRES_URL=postgresql://user:pass@localhost/mydb

# GitHub integration
kn mcp add github -e GITHUB_TOKEN=ghp_your_token_here

# Web search
kn mcp add brave-search -e BRAVE_API_KEY=your_api_key

# Web scraping
kn mcp add puppeteer
```

### Adding Custom Servers

```bash
# Add any npm package
kn mcp add @modelcontextprotocol/server-slack

# Add custom command
kn mcp add my-server --command python -a "-m" -a "my_mcp_module"
```

### Managing Servers

```bash
# List all configured servers
kn mcp list

# Remove a server
kn mcp remove postgres
```

### Available Preset Servers

| Server | Description | Environment Variables |
|--------|-------------|----------------------|
| `filesystem` | Local file access | None |
| `postgres` | PostgreSQL database | `POSTGRES_URL` |
| `github` | GitHub API integration | `GITHUB_TOKEN` |
| `brave-search` | Web search via Brave | `BRAVE_API_KEY` |
| `puppeteer` | Browser automation | None |

### Configuration in kn.toml

MCP servers are stored in your `kn.toml`:

```toml
[mcp.servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "."]

[mcp.servers.postgres]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-postgres"]

[mcp.servers.postgres.env]
POSTGRES_URL = "postgresql://localhost/mydb"
```

## Working with Spec-Kit (Spec-Driven Workflow)

### Setup Spec-Kit

```bash
# Install specify-cli (requires uv)
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git

# Initialize spec-kit in your project (Claude Code integration)
specify init . --integration claude

# Verify
specify check
```

This creates `.specify/` (templates, scripts, constitution) and installs the
`/speckit-*` skills under `.claude/skills/`.

### Agent Workflow

```bash
# Create a new initiative
/speckit-specify Build authentication system with JWT

# Plan and break down into tasks
/speckit-plan
/speckit-tasks

# Implement
/speckit-implement
```

Each initiative lives in `specs/NNN-feature-name/` (`spec.md`, `plan.md`, `tasks.md`).
There is no atomic task claiming — the Planner agent splits `tasks.md` by role, and each
agent marks its own checkboxes as it completes them. See `AGENTS.md` for the full
multi-agent workflow.

### Syncing with Git

```bash
git add specs/
git commit -m "docs(spec): add authentication system spec"
git push
```

## Example Workflow

Here's a complete workflow from start to finish:

```bash
# 1. Create new project
mkdir my-app && cd my-app
git init

# 2. Initialize Knowledge Framework
kn init -y

# 3. Install relevant skills
kn skills install typescript
kn skills install react-19
kn skills install fastapi-best-practices

# 4. Setup spec-kit
specify init . --integration claude

# 5. Create your first spec
/speckit-specify Build authentication system with JWT

# 6. Plan and break into tasks
/speckit-plan
/speckit-tasks

# 7. Review the generated specs/001-authentication-system/tasks.md

# 8. Start coding!
# Work your assigned section of tasks.md on your own branch
git checkout -b 001-authentication-system/backend
# ... code, marking checkboxes in tasks.md as you go ...

# 9. Commit and push
git add .
git commit -m "feat(auth): implement JWT authentication"
git push
```

## Project Types

Knowledge Framework auto-detects these project types:

- **Rust** - Detects `Cargo.toml`
- **Node** - Detects `package.json`
- **Python** - Detects `pyproject.toml` or `setup.py`
- **Go** - Detects `go.mod`
- **Monorepo** - Detects `pnpm-workspace.yaml` or `lerna.json`

Override auto-detection:
```bash
kn init --project-type rust
```

## Configuration Reference

### kn.toml Structure

```toml
[project]
name = "project-name"        # Auto-detected from directory
type = "Node"                 # Auto-detected or specified

[agents]
planner = "agent-id-1"        # Planner agent ID
implementation = "agent-id-2" # Implementation agent ID
# Add more agents as needed

[skills]
enabled = ["skill1", "skill2"] # Installed skills list

[mcp]
servers = ["server1", "server2"] # MCP servers for docs
```

## Troubleshooting

### kn command not found

```bash
# Check if cargo bin is in PATH
echo $PATH | grep cargo

# Add to PATH if missing
export PATH="$PATH:$HOME/.cargo/bin"
```

### Skills installation fails

```bash
# Check network connection
curl -I https://raw.githubusercontent.com

# Try local installation instead
kn skills install /path/to/SKILL.md
```

### Permission denied when writing templates

```bash
# Create directory first
mkdir -p .beads/templates

# Check permissions
ls -la .beads/
```

## Next Steps

- 📖 Read [Architecture](./ARCHITECTURE.md) to understand system design
- 🤖 Review [AGENTS.md](../AGENTS.md) for agent workflow details
- 🛠️ Check [AGENTS.md](../AGENTS.md) to contribute
- 📋 Browse [CLI Reference](../cli/README.md) for all commands

## Get Help

- 📧 [GitHub Issues](https://github.com/kobogithub/knowledge/issues)
- 💬 [GitHub Discussions](https://github.com/kobogithub/knowledge/discussions)
- 📖 [Full Documentation](../README.md)

---

Happy coding! 🚀
