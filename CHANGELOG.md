# Changelog

All notable changes to the Knowledge Framework CLI (`kn`) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0] - 2026-02-25

### Added

#### New Agents (2)
- **Docs Writer Agent** (`knowledge-doc`) - Technical writer for documentation, ADRs, changelogs, and persistent knowledge extraction from bd issues
  - Model: `anthropic/claude-sonnet-4.5`
  - Skills: `documentation-guide`, `bd-best-practices`
  - MCP servers: `github`, `context7`
- **Biz Agent** (`knowledge-biz`) - Stakeholder reporting, Notion dashboards, non-technical progress summaries
  - Model: `anthropic/claude-haiku-4.5` (optimized for cost efficiency)
  - Dual-mode output: Notion via MCP (preferred), Markdown fallback in `docs/reports/`
  - Consumes financial data from Finanzas Agent (no duplication)
  - Skills: `notion-reporting-standard`, `bd-best-practices`

#### New Skills (2)
- `documentation-guide` - Comprehensive guide for writing technical documentation (templates, API docs, code comment conventions)
- `notion-reporting-standard` - Reporting standard for stakeholder dashboards (weekly status, sprint review, executive summary templates)

#### ADR Infrastructure & Historical Records
- Created `docs/adr/` directory with ADR template (`000-template.md`) and index (`README.md`)
- **5 historical ADRs** documenting key architectural decisions:
  - ADR-001: Rust as implementation language for CLI tool
  - ADR-002: Multi-agent architecture with specialized roles
  - ADR-003: Beads (bd) as git-native issue tracking system
  - ADR-004: 3-tier skill distribution system (`skills/` → `~/.kn/skills/` → `.opencode/skills/`)
  - ADR-005: OpenRouter as unified LLM provider with differentiated models

#### Stakeholder Reporting
- Created `docs/reports/` directory for Markdown fallback reports
- First weekly status report (`2026-02-25-weekly-status.md`) — 98% issue completion rate, 11 agents operational

### Changed

#### Agent System
- Total agents increased from 9 to **11** (added Docs Writer, Biz)
- Total skills increased from 21 to **23** (added `documentation-guide`, `notion-reporting-standard`)
- Agent definitions moved from `.opencode/opencode.json` to `kn.toml` (opencode.json retains only MCP configs)

## [0.7.1] - 2026-02-24

### Changed

#### Conventional Commit Types (10 types)
- **Added** `perf` (performance optimization → PATCH bump)
- **Added** `build` (Cargo.toml, Dockerfiles, install scripts → PATCH bump)
- **Added** `ci` (GitHub Actions, workflows → PATCH bump)
- **Removed** `hotfix` (redundant — urgency conveyed by `hotfix/` branch pattern, not commit type; use `fix` instead)

#### Agent Documentation
- All 9 agent AGENTS.md files updated with 10-type commit table and role-specific commit examples
- Root AGENTS.md: fixed residual "Auto-Tagging" section → "Tagging (Manual)" with correct manual workflow

#### `kn init` Console Summary
- Updated types list from 7 to 10 (`feat | fix | refactor | perf | build | ci | chore | docs | style | test`)
- Fixed auto-tagging text → manual tagging instructions

### Removed
- `standard_commits_datalake.md` — Fully superseded by `skills/standard-commits/SKILL.md`

## [0.7.0] - 2026-02-24

### Added

#### Git Branching Strategy & Semantic Versioning
- **Branch hierarchy**: `prod ← dev ← epic/<id> ← <id>/<agent-role>` with clear ownership rules
- **Conventional commits enforcement**: All commit messages must follow `<type>(<scope>): <message>` format
- **SemVer bump rules**: `feat` → MINOR, `fix/refactor/chore/docs/test/style` → PATCH, `!` → MAJOR

#### Standard Commits Skill
- New skill `standard-commits` (`skills/standard-commits/SKILL.md`) — Comprehensive guide covering conventional commits format, SemVer rules, branch hierarchy, tag strategy, PR review workflow, agent commit rules, and CI/CD integration

#### PR Review Workflow
- New formula `mol-review.formula.json` — 6-step PR review cycle: triage → fix-backend → fix-frontend → fix-devops → verify-fixes → merge-gate
- Planner AGENTS.md extended with full PR review workflow section: `gh` CLI commands for reading comments, creating issues from reviews, approving/merging PRs

#### Enhanced `kn init`
- **Step 8.5**: Copies formulas from `~/.kn/formulas/` to `.beads/formulas/` during project initialization
- **Step 8.6**: Creates `dev` branch from current branch if it doesn't exist
- Summary now shows conventional commits reference
- AGENTS.md template includes branching strategy and conventional commits table

#### Distribution Pipeline
- `install.sh` now copies formulas to `~/.kn/formulas/` during installation
- `kn sync` now syncs formulas from `~/.kn/` to the project (non-destructive, only copies missing files)
- New `formulas_dir()` and `list_installed_formulas()` functions in `kn_home.rs`

### Changed

#### Formulas (v2)
- `mol-feature.formula.json` — Added `create-branches` and `create-pr-epic-to-dev` steps for branching workflow
- `mol-bugfix.formula.json` — Added `create-branch` and `create-pr` steps with hotfix support
- `mol-release.formula.json` — Added `create-release-branch` and `create-pr-to-prod` steps

#### Agent Documentation
- All 9 agent AGENTS.md files updated with "Git Branching Strategy & Conventional Commits" section (branch hierarchy, workflow commands, commit type table)
- Root AGENTS.md updated with branching rules table
- Fixed missing YAML frontmatter `---` delimiter in `agents/rust/AGENTS.md`

#### CI/CD
- `ci.yml` updated to trigger on `dev` branch (push and pull_request)

## [0.6.0] - 2026-02-24

### Added

#### 5-Phase Workflow Framework
- **Structured 5-phase lifecycle** for all significant work across 9 agents:
  1. **Exploration** — Investigation, ADRs, proposals (`bd create -t decision`, `bd query`, `bd kv`, `bd todo add`)
  2. **Specification** — Formal plan with epics, formulas, acceptance criteria (`bd formula list`, `bd cook`, `bd lint`, `bd graph`)
  3. **Task Planning** — Decomposition, assignment, dependency DAGs (`bd mol pour`, `bd swarm`, `bd slot`, `bd count`)
  4. **Implementation** — Coordinated execution with monitoring (`bd agent state`, `bd heartbeat`, `bd merge-slot`, `bd audit`)
  5. **Verification** — Testing, approval gates, formal closure (`bd gate resolve`, `bd preflight`, `bd orphans`, `bd epic close-eligible`)

#### Workflow Formula Templates (4)
- `mol-feature.formula.json` — 8-step feature workflow across multiple agents
- `mol-bugfix.formula.json` — 4-step bugfix with root cause analysis and regression testing
- `mol-spike.formula.json` — 3-step time-boxed technical investigation
- `mol-release.formula.json` — 6-step release workflow with quality gates

#### Merge-Slot Infrastructure
- Created `knowledge-merge-slot` bead (`gt:slot`) for serialized push coordination
- Prevents race conditions when multiple agents push concurrently

#### Phase Labels
- Standardized labels: `phase:exploration`, `phase:specification`, `phase:planning`, `phase:implementation`, `phase:verification`

### Changed

#### Agent Documentation
- All 9 agent AGENTS.md files updated with "Protocolo de 5 Fases" section
- Planner AGENTS.md expanded with `mol pour`, `bd kv`, `bd lint`, `bd graph`, `swarm`, `slot`, `count`, `distill`, `agent state`, `gates`, `preflight`, `orphans`
- Root AGENTS.md updated with 5-phase framework table, phase labels, and merge-slot protocol in Quick Reference

#### Skills
- `bd-best-practices` skill completely rewritten from placeholder to comprehensive 5-phase framework manual

#### Documentation
- README.md and README_ES.md updated to reflect 9 agents, 5-phase framework, v0.6.0 status
- Project structure updated with formulas directory, all 11 skills, and 9 agent directories
- Roadmap updated: Phase 3 (Multi-Agent & Workflow) marked as completed

## [0.5.1] - 2026-02-23

### Fixed
- Fixed `kn update` failing with "Text file busy" (ETXTBSY) error on Linux
  - Root cause: `fs::copy()` tried to overwrite the running binary, which Linux blocks
  - Solution: Remove (unlink) the running binary first, then copy the new one
  - The running process keeps its inode open, so this is safe — standard self-update pattern
  - Also ensures correct permissions (0o755) on the newly installed binary

## [0.5.0] - 2026-02-23

### Added

#### New Agents
- **Security Agent** (`knowledge-s3c`) - Application security, vulnerability scanning, SAST
  - Skills: Trivy (container/dependency scanning), Semgrep (SAST), Gitleaks (secret detection), OWASP ZAP (API security)
  - Model: `anthropic/claude-sonnet-4.5`
- **UI/UX Tester Agent** (`knowledge-u7x`) - Visual fidelity, interaction testing, accessibility
  - Skills: Pixelmatch (visual regression), Playwright (browser testing), Axe-core (WCAG accessibility), Viewport Testing (responsive design)
  - Model: `anthropic/claude-sonnet-4.5`
- **Finanzas Agent** (`knowledge-f1n`) - OpenRouter cost tracking, spend reports, budget controls
  - Model: `anthropic/claude-haiku-4.5` (optimized for low-cost operations)

#### New Skills (8 total)
- `security-trivy` - Container and dependency vulnerability scanning
- `security-semgrep` - Static Application Security Testing (SAST)
- `security-gitleaks` - Secret detection and prevention
- `security-owasp-zap` - API security testing
- `uiux-pixelmatch` - Visual regression testing with pixel comparison
- `uiux-playwright` - Browser automation for UI testing
- `uiux-axe-core` - WCAG accessibility compliance testing
- `uiux-viewport-testing` - Responsive design and viewport testing

#### MCP Server Integration
- **6 MCP servers configured** with per-agent scoping:
  - `playwright` - Browser automation (Frontend, QA, UI/UX Tester)
  - `penpot` - Design tool integration (Frontend, UI/UX Tester)
  - `github` - Repository operations (Planner, Backend, DevOps, QA, Security)
  - `postgres` - Database operations (Backend)
  - `context7` - Documentation lookup (Backend, Frontend, Rust)
  - `sentry` - Error monitoring (Backend, DevOps, QA, Security)
- **Per-agent MCP scoping** using OpenCode's `tools` glob pattern system
  - All MCPs disabled globally, enabled selectively per-agent
  - Minimizes context budget usage per agent session
- **`[mcp]` section** added to `kn.toml` configuration

#### Documentation
- `docs/AGENT_MODEL_STRATEGY.md` - Complete rewrite for OpenRouter with pricing, cost tracking strategy, and MCP distribution table
- MCP assignments documented in each agent's `AGENTS.md` frontmatter (`mcp_servers:` field)

### Changed

#### LLM Provider Migration
- **BREAKING**: Migrated from GitHub Copilot to **OpenRouter** as LLM provider
  - `github-copilot/claude-opus-4` → `anthropic/claude-opus-4` (Planner, Backend)
  - `github-copilot/claude-sonnet-4.5` → `anthropic/claude-sonnet-4.5` (Frontend, Rust, DevOps, Security, UI/UX Tester)
  - `github-copilot/claude-haiku-4` → `anthropic/claude-haiku-4.5` (QA, Finanzas)
- All 9 agent AGENTS.md files updated with new model references
- `kn.toml` and `.opencode/opencode.json` migrated to OpenRouter model IDs
- `OPENROUTER_API_KEY` required (added to `.env.example`)
- Per-agent cost tracking via OpenRouter's `user` parameter

#### CLI Simplification
- **BREAKING**: Simplified to **OpenCode-only** workspace standard (Antigravity/Gemini support paused)
  - `WorkspaceStandard` enum reduced to `OpenCode` only
  - `Antigravity` and `Both` values still parse but map to `OpenCode` with deprecation warning
  - `kn init` no longer prompts for workspace standard selection
  - `kn sync` no longer generates Gemini/Antigravity configuration
  - Symlink generation simplified to OpenCode paths only
  - Gemini module (`config/gemini.rs`) preserved but commented out for future re-enablement

#### Agent System
- Total agents increased from 6 to **9** (added Security, UI/UX Tester, Finanzas)
- All agents now have `mcp_servers` field in YAML frontmatter
- Differentiated model strategy: Opus for critical agents, Sonnet for specialized, Haiku for cost-sensitive

### Fixed
- Fixed model string inconsistency: Security and UI/UX Tester had `claude-4.5-sonnet` (reversed) → corrected to `claude-sonnet-4.5`
- Fixed config drift: Security and UI/UX Tester agents were missing from `kn.toml` and `opencode.json`

## [0.4.0] - 2026-02-20

### Changed
- Version bump for differentiated model strategy per agent
- Added model configuration to agent metadata YAML frontmatter
- Configured Notion MCP and models per agent

## [0.3.2] - 2026-02-19

### Fixed
- Fixed `kn update` command failing with "Invalid cross-device link" error
  - Changed from `fs::rename()` to `fs::copy()` to support cross-filesystem updates
  - Now works when `/tmp` and installation directory are on different partitions
  - Issue occurred when temporary directory was on different filesystem than target binary

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

### Fixed
- Fixed GitHub Actions release workflow permissions (added `contents: write` permission)

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

[Unreleased]: https://github.com/kobogithub/knowledge/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/kobogithub/knowledge/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/kobogithub/knowledge/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/kobogithub/knowledge/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/kobogithub/knowledge/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/kobogithub/knowledge/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/kobogithub/knowledge/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/kobogithub/knowledge/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/kobogithub/knowledge/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/kobogithub/knowledge/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/kobogithub/knowledge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/kobogithub/knowledge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/kobogithub/knowledge/releases/tag/v0.1.0
