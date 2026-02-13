# Knowledge Framework

> **A meta-framework CLI for AI-assisted development workflows**

Knowledge Framework (`kn`) is a command-line tool that solves the "cold start problem" in AI-assisted development by automating project setup with agents, skills, and standardized workflows.

[![Rust](https://img.shields.io/badge/rust-1.93+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## 🎯 What Problem Does This Solve?

Every time you start a new project with AI agents, you need to:
- ❌ Manually configure agent roles and workflows
- ❌ Set up issue tracking from scratch
- ❌ Install and configure skills for each project
- ❌ Configure MCP servers for documentation
- ❌ Create standard templates for planning

**Knowledge Framework automates all of this in seconds.**

---

## ✨ Features

### 🚀 Instant Project Initialization
```bash
cd my-project/
kn init
# → Auto-detects project type
# → Creates AGENTS.md with workflow instructions
# → Generates kn.toml configuration
```

### 📚 Skills Management
```bash
kn skills install typescript        # From agentskills.io
kn skills install https://...       # From URL
kn skills install ./SKILL.md        # From local path
kn skills list                      # View installed skills
```

### 📋 Beads Templates
```bash
kn beads template epic -o epic.md   # Generate structured issue templates
kn beads template task              # Print to stdout for piping
kn beads template bug --force       # Overwrite existing files
```

### 🔌 MCP Integration (Coming Soon)
```bash
kn mcp add rust-docs                # Connect documentation servers
kn mcp list                         # View configured servers
```

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/kobogithub/knowledge.git
cd knowledge

# Build the CLI
cd cli
cargo build --release

# Install globally (optional)
sudo cp target/release/kn /usr/local/bin/
```

### Initialize Your First Project

```bash
cd ~/your-project/
kn init

# Follow prompts or use -y for defaults
kn init -y
```

### Install Skills

```bash
# Install a skill
kn skills install typescript

# View installed skills
kn skills list
```

### Generate Issue Templates

```bash
# Create templates directory
mkdir -p .beads/templates

# Generate all templates
kn beads template epic -o .beads/templates/epic.md
kn beads template task -o .beads/templates/task.md
kn beads template bug -o .beads/templates/bug.md
kn beads template feature -o .beads/templates/feature.md
kn beads template chore -o .beads/templates/chore.md
```

---

## 🏗️ Project Structure

```
knowledge/
├── cli/                      # Rust CLI tool
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   └── commands/
│   │       ├── init.rs       # Project initialization
│   │       ├── skills.rs     # Skills management
│   │       └── beads.rs      # Issue templates
│   ├── Cargo.toml
│   └── README.md
├── .beads/                   # Issue tracking (Beads + Dolt)
│   └── issues.jsonl          # Issue database
├── AGENTS.md                 # Main agent coordination
├── AGENTS_PLANNER.md         # Planner agent instructions
├── AGENTS_FRONTEND.md        # Frontend agent instructions
├── AGENTS_BACKEND.md         # Backend agent instructions
├── AGENTS_RUST.md            # Rust agent instructions
├── AGENTS_DEVOPS.md          # DevOps agent instructions
└── README.md                 # This file
```

---

## 🤖 Multi-Agent Workflow

Knowledge Framework uses [Beads](https://github.com/beadlist/beads) for issue tracking with specialized AI agents:

| Agent | ID | Responsibilities |
|-------|-----|-----------------|
| **Planner** | `knowledge-x6e` | Coordinates work, creates epics, assigns tasks |
| **Frontend** | `knowledge-4yh` | UI/UX, React, components, client-side |
| **Backend** | `knowledge-vlf` | APIs, databases, business logic, security |
| **Rust** | `knowledge-r5t` | CLI tools, libraries, systems programming |
| **DevOps** | `knowledge-w5p` | Infrastructure, CI/CD, deployment, monitoring |

Each agent:
- ✅ Has specialized knowledge and tools
- ✅ Closes their own tasks autonomously
- ✅ Reports progress transparently
- ✅ Coordinates with other agents

See [AGENTS.md](./AGENTS.md) for detailed instructions.

---

## 📚 Documentation

- [CLI Documentation](./cli/README.md) - Complete CLI reference
- [Agent Instructions](./AGENTS.md) - Multi-agent workflow guide
- [Getting Started](./docs/GETTING_STARTED.md) - Step-by-step tutorial
- [Architecture](./docs/ARCHITECTURE.md) - System design and decisions
- [Development Guide](./docs/DEVELOPMENT.md) - Contributing guidelines

---

## 🛣️ Roadmap

### ✅ Phase 1: Core CLI (Completed - 57%)
- [x] `kn init` - Project initialization with auto-detection
- [x] `kn skills install/list` - Skills management
- [x] `kn beads template` - Issue template generation

### 🔄 Phase 2: Integration (In Progress - 43%)
- [ ] `kn mcp add/list` - MCP server configuration
- [ ] Enhanced project detection (workspaces, frameworks)
- [ ] Cross-platform support (Windows symlinks)

### 🔮 Phase 3: Advanced Features
- [ ] `kn agent create` - Custom agent generation
- [ ] `kn workflow init` - Workflow templates
- [ ] `kn sync` - Multi-project synchronization
- [ ] Plugin system for extensibility

### 🌟 Phase 4: Ecosystem
- [ ] Public skills repository (agentskills.io integration)
- [ ] Web dashboard for project overview
- [ ] Team collaboration features
- [ ] Analytics and insights

---

## 🔧 Technology Stack

- **CLI**: Rust (clap, reqwest, serde)
- **Issue Tracking**: Beads + Dolt
- **Skills Format**: Markdown + YAML frontmatter
- **Config**: TOML
- **Agents**: Multi-agent AI workflow (Claude, GPT-4, etc.)

---

## 🎯 Use Cases

### 1. New Project Onboarding
```bash
cd new-project/
kn init
# → Instant agent setup, ready to code
```

### 2. Standardized Workflows
```bash
kn skills install company-standards
# → All projects follow same patterns
```

### 3. Issue Planning
```bash
kn beads template epic > planning/mvp.md
# → Structured planning templates
```

### 4. Documentation Access
```bash
kn mcp add rust-docs mdn-web-docs
# → AI agents have instant doc access
```

---

## 🤝 Contributing

We welcome contributions! See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for guidelines.

### Development Setup

```bash
# Clone and build
git clone https://github.com/kobogithub/knowledge.git
cd knowledge/cli
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check
cargo clippy
```

### Creating Issues

Use Beads for issue tracking:

```bash
# List issues
bd list

# Create a new issue
bd create "Title" --type task -p 1 -l rust

# View issue details
bd show task-id
```

---

## 📊 Current Status

**CLI Development**: 57% complete (4/7 tasks)
- ✅ Project initialization
- ✅ Skills management (install, list)
- ✅ Beads templates (epic, task, bug, feature, chore)
- 🔄 MCP integration (next)
- 🔄 Enhanced detection
- 🔄 Windows support

**Active Development**: Rust Agent implementing MCP integration

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- **[Beads](https://github.com/beadlist/beads)** by Steve Yegge - Issue tracking framework
- **[Dolt](https://doltdb.com)** - Git-like version control for data
- **[agentskills.io](https://agentskills.io)** - Skills repository standard
- **Rust Community** - Amazing tooling and ecosystem

---

## 💬 Support

- 📧 Issues: [GitHub Issues](https://github.com/kobogithub/knowledge/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/kobogithub/knowledge/discussions)
- 📖 Docs: [Documentation](./docs/)

---

**Built with ❤️ by the Knowledge Framework contributors**
