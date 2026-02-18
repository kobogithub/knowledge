# Knowledge Framework

> **A meta-framework CLI for AI-assisted development workflows**

**[🇪🇸 Leer en Español](./README_ES.md)** | **[🇬🇧 Read in English](./README.md)**

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
# → Prompts for workspace standard (OpenCode/Antigravity/Both)
# → Auto-installs recommended skills
```

**Workspace Standards Supported:**
- ✅ **OpenCode** (`.opencode/skills/<name>/SKILL.md`)
- ✅ **Antigravity** (`.agent/skills/<name>/SKILL.md` symlinked to `.opencode/skills/`)
- ✅ **Both** (default, recommended for maximum compatibility)

**Skills Location:**
- Single source: `.opencode/skills/<skill-name>/SKILL.md`
- Antigravity access: `.agent/skills/<skill-name>` → symlink to `.opencode/skills/`

**Interactive Mode:**
```bash
kn init              # Prompts to choose workspace standard
```

**Non-Interactive Mode:**
```bash
kn init -y                                  # Uses default (both)
kn init -y --workspace-standard opencode    # OpenCode only
kn init -y --workspace-standard antigravity # Antigravity only
kn init -y --workspace-standard both        # Both (explicit)
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

### 🔌 MCP Server Management
```bash
kn mcp add filesystem               # Add preset MCP servers
kn mcp add postgres -e POSTGRES_URL=... # With environment variables
kn mcp list                         # View configured servers
```

---

## 🚀 Quick Start

### Installation

#### 🚀 Automated Installation (Recommended)

The easiest way to install `kn` is using our automated installer. It downloads pre-compiled binaries from GitHub releases:

```bash
# One-line install (recommended)
curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash

# Or download and inspect first
wget https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh
chmod +x install.sh
./install.sh
```

**Smart Features:**
- ✅ **Version Check**: Skips reinstallation if you already have the target version
- ✅ **Auto PATH Setup**: Automatically adds `~/.local/bin` to your shell config (bash, zsh, fish)
- ✅ **Rosetta Detection**: On macOS, uses ARM64 binaries even in Rosetta terminal for better performance
- ✅ **Visual Progress**: Shows download progress with a clean progress bar
- ✅ **Pre-compiled Binaries**: No compilation needed - fast installation

**Available Options:**
```bash
./install.sh --help              # Show all options
./install.sh --version v0.2.0    # Install specific version
./install.sh --skip-deps         # Skip dependency installation (Git, Node.js, bd)
./install.sh --no-confirm        # Non-interactive mode
./install.sh --no-modify-path    # Don't modify shell config files
```

**Supported Platforms:**
- Linux x86_64
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)

**Updates:**
```bash
kn update                        # Update to latest version
```

---

#### 🔨 Manual Installation (From Source)

If you prefer to build from source or need to customize the installation:

```bash
# 1. Install dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Rust
cargo install bd                                                  # Beads
# Install Node.js from https://nodejs.org/ or your package manager

# 2. Clone and build
git clone https://github.com/kobogithub/knowledge.git
cd knowledge/cli
cargo build --release

# 3. Install globally
sudo cp target/release/kn /usr/local/bin/
# or to user directory
mkdir -p ~/.local/bin
cp target/release/kn ~/.local/bin/

# 4. Add to PATH (if not already)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc

# 5. Verify
kn doctor
```

---

#### 🪟 Windows Installation

```powershell
# Download and run the PowerShell script
irm https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.ps1 | iex

# Or follow manual instructions in install.ps1
```

**Note**: Windows automated installation is in development. See `install.ps1` for manual steps.

---

#### 📦 Package Managers (Coming Soon)

We're working on official packages for popular package managers:

- **Homebrew** (macOS): `brew install kobogithub/knowledge/kn`
- **APT** (Ubuntu/Debian): `apt install kn`
- **DNF/YUM** (Fedora/RHEL): `dnf install kn`

For now, please use the automated installer above.

---

#### 📚 Platform-Specific Guides

For detailed platform-specific installation instructions, see:
- **[HOMEBREW.md](./HOMEBREW.md)** - Homebrew formula and tap setup
- **[DEBIAN.md](./DEBIAN.md)** - Building and publishing Debian packages
- **[RPM.md](./RPM.md)** - Building and publishing RPM packages for Fedora/RHEL

### Verify Installation

After installation, verify that all dependencies are met:

```bash
kn doctor
```

**Expected output:**
```
🔍 Checking dependencies...

✓ Rust         1.93.1
✓ Cargo        1.93.1
✓ Git          2.53.0
✓ bd           0.49.6
○ Dolt         (optional - not found)
✓ Node.js      25.6.0
✓ npm          11.8.0

✓ Overall: 7/7 dependencies met

🎉 Ready to use!
```

**Dependencies:**
- ✅ **Rust** (1.70+) - For building kn
- ✅ **Cargo** - Rust package manager
- ✅ **Git** - Version control
- ✅ **bd** (beads) - Issue tracking
- ⚪ **Dolt** - Optional, for Beads database
- ✅ **Node.js** (18.0+) - For MCP servers
- ✅ **npm** - Node package manager

### Initialize Your First Project

```bash
cd ~/your-project/
kn init

# Follow prompts or use -y for defaults
kn init -y

# Skip auto-installation of recommended skills
kn init -y --no-skills
```

**Skills are auto-installed based on detected project:**
- **Base Language + Framework Detection**:
  - Rust projects → rust-best-practices + docker + bash
  - Python projects → python-best-practices + docker + bash
  - Node projects → docker + bash
  - Go projects → docker + bash
- **Framework-specific skills**:
  - Astro detected → astro-best-practices
  - FastAPI detected → python-best-practices
  - More frameworks coming soon!
- **Workspace/Monorepo**: Automatically detected for Cargo, pnpm, npm workspaces

### Manage Skills

```bash
# View installed skills
kn skills list

# Install additional skills
kn skills install <skill-name>

# Install from URL or local path
kn skills install https://example.com/skill/SKILL.md
kn skills install ./local/skill/SKILL.md
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
│   │       ├── beads.rs      # Issue templates
│   │       └── mcp.rs        # MCP server management
│   ├── Cargo.toml
│   └── README.md
├── agents/                   # Agent-specific instructions
│   ├── planner/
│   │   └── AGENTS.md         # Planner agent guide
│   ├── frontend/
│   │   └── AGENTS.md         # Frontend agent guide
│   ├── backend/
│   │   └── AGENTS.md         # Backend agent guide
│   ├── rust/
│   │   └── AGENTS.md         # Rust agent guide
│   └── devops/
│       └── AGENTS.md         # DevOps agent guide
├── skills/                   # Installed AI agent skills
│   ├── astro-best-practices/
│   ├── bash-best-practices/
│   ├── docker-best-practices/
│   ├── python-best-practices/
│   ├── rust-best-practices/
│   ├── supabase-postgres-best-practices/
│   └── README.md
├── docs/                     # Documentation
│   ├── GETTING_STARTED.md
│   └── ARCHITECTURE.md
├── .beads/                   # Issue tracking (Beads + Dolt)
│   └── issues.jsonl          # Issue database
├── AGENTS.md                 # Main agent coordination
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

### ✅ Phase 1: Core CLI (Completed - 100%)
- [x] `kn init` - Project initialization with auto-detection
- [x] `kn skills install/list` - Skills management
- [x] `kn beads template` - Issue template generation
- [x] `kn mcp add/list/remove` - MCP server configuration

### ✅ Phase 2: Enhancement (50%)
- [x] Enhanced project detection (workspaces, frameworks)
- [ ] Cross-platform support (Windows symlinks)
- [ ] Test suite for all commands

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
