# Changelog

All notable changes to the Knowledge Framework CLI (`kn`) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2026-02-19

### Added
- `kn mcp search` command for searching the official MCP Registry
  - Search across 81+ MCP servers from the community
  - Filter by query (searches name, title, and description)
  - `--limit` flag to control number of results (default: 10)
  - `--install` flag for interactive installation
  - Displays formatted results with version, description, website, and repository
  - Graceful fallback with helpful suggestions when no results found
  - API endpoint: `https://registry.modelcontextprotocol.io/v0/servers`

### Examples
```bash
kn mcp search postgres --limit 5
kn mcp search github --install
kn mcp search "AI assistant"
```

## [0.3.0] - 2026-02-19

### Added
- `kn update` command for self-updating CLI from GitHub releases
  - `kn update --check` to check for updates without updating
  - `kn update --force` to force update even if on latest
  - Automatic download and installation of latest binary
  - Cross-platform support (Linux, macOS, Windows)

## [0.2.0] - 2026-02-18

### Changed - MAJOR REFACTOR

**Global ~/.kn/ Directory Architecture:**
- **BREAKING**: Complete refactor to use `~/.kn/` as global directory for agents and skills
- `~/.kn/agents/` - Single source of truth for all agents
- `~/.kn/skills/` - Single source of truth for all skills
- Projects now use symlinks to global directory (install once, use everywhere)

**New Commands:**
- `kn agents install <path>` - Install agents to ~/.kn/agents/
- `kn agents list` - List installed agents with full metadata
- `kn sync` - Sync project symlinks from kn.toml configuration

**Completely Refactored `kn init`:**
- Interactive prompts using `dialoguer`:
  - Project name (auto-detected from directory)
  - Workspace standard selection (OpenCode/Antigravity/Both)
  - Multi-select agent selection from ~/.kn/agents/
  - Optional recommended skills selection
- Auto-detection of `required_skills` from agent YAML frontmatter
- Generates kn.toml with selected agents and skills
- Creates workspace-specific symlinks (`.opencode/`, `.agent/`, etc.)
- Simplified to single `--yes` flag (non-interactive mode)

**Agent Metadata System:**
- YAML frontmatter in agents/*/AGENTS.md files
- Metadata includes: name, id_prefix, description, required_skills, recommended_skills, tags
- Auto-generation of agent IDs based on project name + id_prefix

**New Rust CLI Architecture:**
- `lib.rs` - Public library API
- `config/` - Configuration management (KnConfig, WorkspaceStandard)
- `core/` - Core functionality (kn_home, symlinks)
- `models/` - Data models (AgentMetadata, SkillMetadata)
- `commands/` - Command handlers (agents, skills, init, sync, etc.)

**Benefits:**
- ✅ Install once, use everywhere
- ✅ Consistent versions across all projects
- ✅ Easy updates (update in ~/.kn/, run `kn sync`)
- ✅ Support both OpenCode and Antigravity simultaneously
- ✅ Workspace-agnostic agent and skill management

### Added
- CI/CD workflows for automated testing and releases
- GitHub Actions for multi-platform binary builds
- Automated Debian and RPM package building
- **Dual workspace standard support (OpenCode + Antigravity)**
  - Automatic generation of `.opencode/opencode.json` for OpenCode
  - Skills installed to `.opencode/skills/<name>/` (standard location)
  - Automatic generation of `.agent/skills/` for Antigravity
  - Symlinks from `.opencode/skills/` to `.agent/skills/` for compatibility
  - `--workspace-standard` flag (opencode|antigravity|both)
  - **Interactive prompt to choose workspace standard in `kn init`**
  - Support for project-specific workspace environments
  - Auto-detection of installed skills for both standards
  - Complete documentation in `.opencode/README.md` and `.agent/README.md`

### Changed
- **BREAKING**: Skills now installed to `.opencode/skills/` instead of `./skills/`
  - Aligns with OpenCode standard (`.opencode/skills/<name>/SKILL.md`)
  - Antigravity accesses via symlinks in `.agent/skills/`
  - Relative paths in `opencode.json` for portability

## [0.1.0] - 2026-02-18

### Added

#### Core CLI Commands
- **`kn init`** - Project initialization with auto-detection
  - Detects project type (Rust, Python, Node, Go, etc.)
  - Detects frameworks (Astro, FastAPI, etc.)
  - Detects workspace/monorepo structure
  - Creates `AGENTS.md` with workflow instructions
  - Generates `kn.toml` configuration
  - Auto-installs recommended skills based on project

- **`kn skills install <name>`** - Install skills
  - Install from agentskills.io registry
  - Install from URL
  - Install from local file path
  - Supports multiple skills at once

- **`kn skills list`** - List available and installed skills
  - Shows installed skills with metadata
  - Indicates skill source (local/remote)

- **`kn beads template <type>`** - Generate issue templates
  - Supports types: epic, task, bug, feature, chore
  - Output to file or stdout
  - Force overwrite with `--force`
  - Compatible with Beads issue tracker

- **`kn mcp add <server>`** - Configure MCP servers
  - 5 preset servers: filesystem, postgres, github, brave-search, puppeteer
  - Support for custom commands
  - Environment variable configuration
  - Stores config in `kn.toml`

- **`kn mcp list`** - List configured MCP servers

- **`kn mcp remove <server>`** - Remove MCP server configuration

- **`kn doctor`** - Verify dependencies
  - Checks: Rust, Cargo, Git, bd, Dolt (optional), Node.js, npm
  - Validates minimum versions
  - Provides installation instructions
  - Colored output with symbols
  - `--verbose` flag for detailed info
  - Exit codes: 0 (OK), 1 (missing deps)

#### Installation System

- **`install.sh`** - Automated installer for Linux/macOS
  - Detects OS and package manager (apt/dnf/yum/pacman/brew)
  - Installs dependencies: Rust, Git, Node.js, bd
  - Clones repository and builds kn from source
  - Installs to `~/.local/bin` or `/usr/local/bin`
  - Verifies installation with `kn doctor`
  - Options: `--help`, `--skip-deps`, `--no-confirm`
  - Error handling, colored output, cleanup with trap

- **`install.ps1`** - Windows installer (manual instructions)

#### Package Installers

- **Homebrew formula** (`Formula/kn.rb`)
  - For macOS via Homebrew
  - Install with: `brew tap kobogithub/knowledge && brew install kn`
  - Includes skills and agent templates
  - Complete documentation in `HOMEBREW.md`

- **Debian packages** (`debian/`)
  - For Ubuntu/Debian via APT
  - Install with: `apt install ./kn.deb`
  - debhelper compatibility level 13
  - Complete documentation in `DEBIAN.md`

- **RPM packages** (`kn.spec`)
  - For Fedora, RHEL, Rocky, AlmaLinux, openSUSE
  - Install with: `dnf install ./kn.rpm`
  - Multi-distribution support
  - Complete documentation in `RPM.md`

#### Skills Included

Pre-installed skills for common workflows:
- `astro-best-practices` - Astro framework development
- `bash-best-practices` - Shell scripting standards
- `docker-best-practices` - Container best practices
- `python-best-practices` - Python development standards
- `rust-best-practices` - Rust development standards
- `supabase-postgres-best-practices` - Supabase/PostgreSQL
- `bd-best-practices` - Beads issue tracking workflow

#### Multi-Agent Support

Agent templates for specialized workflows:
- **Planner Agent** (`knowledge-x6e`) - Work coordination
- **Frontend Agent** (`knowledge-4yh`) - UI/UX, React, client-side
- **Backend Agent** (`knowledge-vlf`) - APIs, databases, business logic
- **Rust Agent** (`knowledge-r5t`) - CLI tools, systems programming
- **DevOps Agent** (`knowledge-w5p`) - Infrastructure, CI/CD
- **QA Agent** (`knowledge-pu1`) - Testing, quality assurance

Each agent has:
- Specialized instructions in `agents/<agent>/AGENTS.md`
- Autonomous task closure authority
- Transparent progress reporting
- Inter-agent coordination capabilities

#### Documentation

- `README.md` - Main documentation with quick start
- `README_ES.md` - Spanish documentation
- `AGENTS.md` - Multi-agent workflow guide
- `HOMEBREW.md` - Homebrew formula guide
- `DEBIAN.md` - Debian package building guide
- `RPM.md` - RPM package building guide
- `RELEASE.md` - Release process documentation
- `cli/README.md` - Complete CLI reference

#### Configuration

- `kn.toml` - Project configuration file
  - Project metadata
  - Skills configuration
  - MCP servers configuration
  - Auto-generated by `kn init`

### Changed
- Improved project detection to support workspaces/monorepos
- Enhanced error messages with colored output
- Optimized binary size with release profile

### Fixed
- Various bug fixes in project detection
- Improved cross-platform compatibility

---

## Release Notes

### v0.1.0 - Initial Release

This is the first release of the Knowledge Framework CLI. It provides:

✅ **Instant Project Setup** - Initialize AI-assisted development in seconds
✅ **Skills Management** - Install and manage AI agent skills
✅ **Issue Templates** - Generate structured Beads templates
✅ **MCP Integration** - Configure Model Context Protocol servers
✅ **Multi-Agent Workflows** - Coordinate work across specialized agents
✅ **Dependency Verification** - Check all required tools with `kn doctor`
✅ **Multi-Platform Installation** - Works on Linux, macOS, Windows

**Installation:**
```bash
# Automated (Linux/macOS)
curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash

# macOS (Homebrew)
brew tap kobogithub/knowledge
brew install kn

# Ubuntu/Debian
wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn_0.1.0-1_amd64.deb
sudo apt install ./kn_0.1.0-1_amd64.deb

# Fedora/RHEL
wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn-0.1.0-1.el9.x86_64.rpm
sudo dnf install ./kn-0.1.0-1.el9.x86_64.rpm
```

**Quick Start:**
```bash
cd your-project/
kn init
kn doctor
kn skills list
```

---

[Unreleased]: https://github.com/kobogithub/knowledge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kobogithub/knowledge/releases/tag/v0.1.0
