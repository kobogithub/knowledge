# Getting Started with Knowledge Framework

Welcome! This guide will walk you through setting up and using the Knowledge Framework CLI (`kn`) for the first time.

## Prerequisites

- **Rust 1.93+** - [Install Rust](https://rustup.rs/)
- **Git** - For version control
- **Beads** (optional) - [Install bd CLI](https://github.com/beadlist/beads) for issue tracking

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

[beads]
enabled = true
templates_dir = ".beads/templates"

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

## Working with Beads (Issue Tracking)

### Setup Beads

```bash
# Initialize beads in your project
bd init

# Create your first issue
bd create "Setup CI/CD pipeline" --type task -p 1 -l devops
```

### Agent Workflow

```bash
# Find available work
bd ready -l rust

# Claim a task
bd update task-id --claim

# Report progress
bd comments add task-id "[Rust Agent] Working on implementation..."

# Complete the task
bd close task-id
```

### Syncing with Git

```bash
# Sync beads to JSONL
bd sync

# Commit changes
git add .beads/issues.jsonl
git commit -m "Update issues"
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

# 4. Setup issue tracking
bd init

# 5. Create templates
mkdir -p .beads/templates
kn beads template epic -o .beads/templates/epic.md
kn beads template task -o .beads/templates/task.md

# 6. Plan your work
kn beads template epic > planning/mvp.md
# Edit planning/mvp.md

# 7. Create issues from your plan
bd create "Build authentication system" --type epic -p 0

# 8. Start coding!
bd ready -l backend
bd update auth-task-id --claim
# ... code ...
bd close auth-task-id

# 9. Sync and commit
bd sync
git add .
git commit -m "Initial setup with Knowledge Framework"
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

[beads]
enabled = true                 # Enable beads integration
templates_dir = ".beads/templates" # Templates location

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
- 🛠️ Check [Development Guide](./DEVELOPMENT.md) to contribute
- 📋 Browse [CLI Reference](../cli/README.md) for all commands

## Get Help

- 📧 [GitHub Issues](https://github.com/kobogithub/knowledge/issues)
- 💬 [GitHub Discussions](https://github.com/kobogithub/knowledge/discussions)
- 📖 [Full Documentation](../README.md)

---

Happy coding! 🚀
