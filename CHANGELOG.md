# Changelog

All notable changes to the Knowledge Framework CLI (`kn`) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ⚠️ BREAKING — `kn beads` is gone, and `bd` is no longer installed

[ADR-006](./docs/adr/006-adopt-speckit-remove-beads.md) replaced Beads with spec-kit as
the workflow nine weeks ago but left it in the product. `kn` kept shipping the subcommand,
`kn doctor` kept requiring `bd`, and `install.sh` installed it via cargo — **aborting the
installation if it could not**. So installing `kn` obliged you to install a tool this
project had stopped using. [ADR-009](./docs/adr/009-remove-beads-from-the-product.md)
closes that.

- **Removed `kn beads template`.** If you script it, it disappears in this release;
  v0.11.0 stays downloadable. The templates remain in git history.
- **`kn doctor` no longer checks `bd` or Dolt.** The count goes from 7 dependencies to 5,
  and a clean machine no longer reports a missing required dependency.
- **`install.sh` no longer installs `bd`**, and no longer installs Rust on your behalf to
  do it. Installing `kn` needs Git and Node.js; the binary is pre-compiled.

Using Beads independently? Install `bd` yourself — `kn` never made it work, it only
insisted on its presence.

### Added

- **Product layer above spec-kit** ([ADR-008](./docs/adr/008-product-layer-over-speckit.md)):
  `docs/product/` holds the brief (`PROJECT.md`), the scope contract (`PRD.md`) and user
  stories with Gherkin, each with a signature header the agents verify before deriving the
  next artifact.
- **`analyst` role** (`knowledge-an1`): owns discovery, and only discovery. Its rule is
  that whatever the input does not say goes to open questions rather than into the brief.
- **Four commands**, shipped as skills: `/product-discovery`, `/product-prd`,
  `/product-stories` and `/product-gate`.
- **Signature gates on spec-kit**: `.specify/extensions.yml` hooks `before_specify` and
  `before_plan` through `/product-gate`, without touching the generated `speckit-*` skills.
- **Client approval, separate from the maintainer's signature**, on the brief and the PRD.
- **QA validates PRs against the story's Gherkin**, one line per scenario; a failing
  scenario blocks the merge.

### Fixed

- **`CLAUDE.md` imported a file that only exists after `kn sync`.** Both the repo's own and
  the one the CLI generates now import `@agents/planner/AGENTS.md`, which is tracked, so a
  fresh clone resolves it. A test asserts no import targets `.claude/agents/` again.
- **Six agent definitions required skills removed from the catalog** by ADR-007 —
  `notion-reporting-standard` in biz, plus terraform, aws and kubernetes across devops and
  backend. `docs/reports/README.md` pointed at the removed reporting skill too; the
  three-section standard is now written out there.
- **`.agent/README.md` listed 10 skills and three that no longer exist.** Regenerated from
  `skills/`: 21.
- **The ADR index was missing ADR-007**, which has existed since August.
- **`install.sh` tried to copy formulas from `.beads/`**, deleted by ADR-006, warning on
  every install.


## [0.11.0] - 2026-08-17

### ⚠️ BREAKING — `kn` now supports Apple Silicon macOS only

Support for **Linux, Windows and Intel Macs is removed**. If you are on any of those,
do not upgrade: this and every later version will not install, and `kn update` will
not find an artifact for your platform. v0.10.0 remains downloadable and keeps working.

The version number stays below 1.0.0 deliberately — the project is still evolving, and
pre-1.0 convention puts breaking changes in the minor position.

**Why**: the project is maintained on and for a single Apple Silicon machine. Building
and publishing three artifacts, two of which nobody installed, meant carrying packaging
recipes that were never exercised — the RPM recipe was pinned at `Version: 0.1.0` while
the project shipped `0.10.0`, nine minor versions stale, and had produced no current
package in a long time.

### Added

- **Homebrew is the primary way to install `kn`**:

  ```bash
  brew install kobogithub/knowledge/kn
  ```

- **The Homebrew tap now updates itself.** A `publish-formula` job renders the formula
  from the artifacts a release actually published and pushes it to
  `kobogithub/homebrew-knowledge`. It runs only after the release succeeds, so a failed
  build leaves the tap pointing at the last working version.
- **A weekly tap drift check** compares the published formula against the newest release
  and files an issue if they disagree — independent of the release workflow, so it
  catches divergence that appears later.
- **The formula warns about competing installs.** If a `kn` exists outside the Homebrew
  prefix, the install output names both copies, says which one your PATH will run, and
  gives the command to remove the other.

### Fixed

- **The Homebrew tap was two releases behind.** It served `0.9.0` while the project had
  published `0.10.0`, because updating it was a manual step in the release document that
  was skipped. Anyone installing via Homebrew got the old version. This is what the
  automation above exists to prevent.
- **Release notes never contained their checksums.** The template built them inside a
  quoted heredoc, so every release through v0.10.0 published the literal string
  `$(cat checksums.txt)` instead of the actual hashes.
- **`brew test kn` was broken for anyone without `bd` installed.** The formula's test
  ran `kn doctor`, which exits non-zero when optional tools are missing. It now asserts
  on what the formula installs.
- **Workflow actions too old to run** (`actions/checkout@v3`, `actions/cache@v3`,
  `softprops/action-gh-release@v1`) updated.

### Changed

- `install.sh` refuses to run on anything but Apple Silicon macOS, **before downloading
  anything**, and names the supported platform. Rosetta detection is retained — a native
  Apple Silicon Mac reports `x86_64` under a translated shell, and the installer still
  resolves that to the ARM64 binary.
- Releases build one artifact instead of three. The asset name `kn-macos-arm64.tar.gz`
  is unchanged, because `install.sh` and in-tool `kn update` both parse it.
- CI runs on `macos-latest` only.
- `docs/release/RELEASE.md` no longer documents a manual tap update, and says explicitly
  not to hand-edit the tap.

### Removed

- `kn.spec` (RPM), `debian/` (Debian packaging), `install.ps1` (Windows installer) and
  `docs/packaging/`. All recoverable from git history.
- The "Package Managers (Coming Soon)" section promising APT and DNF packages, and the
  Windows installation section, from both READMEs.

### Repository housekeeping (initiative 002)

These landed on `prod` before this tag was cut, so `0.11.0` is the first release
that contains them.

#### Added

- **MIT `LICENSE`** at the repository root. The repository was public but carried no
  licence, so GitHub reported none and nobody could legally use or fork it. Both READMEs
  already linked to `LICENSE`; that link was dangling until now.
- **`CONTRIBUTING.md`** — branch hierarchy and pull request target per branch type,
  conventional commit format with SemVer impact, and how initiatives are specified
  before implementation. Derived from the rules already in `AGENTS.md`.
- **Structured GitHub issue forms** (`.github/ISSUE_TEMPLATE/`) for bug reports and
  feature requests, with required fields and blank issues disabled, plus a pull request
  template asking for the change, the related initiative, and how it was verified.
- **`docs/README.md`** — an index of every document under `docs/`, grouped by topic.
- **Documentation link checking in CI** — a `lychee` job validates internal Markdown
  links on every push. External URLs are deliberately not checked so a third-party
  outage cannot fail the build.
- **`kn agents` is now documented** in both READMEs. The command shipped but appeared in
  neither.

#### Changed

- **Maintainer process documents moved out of the repository root**, cutting root
  Markdown from 12 files to 5:
  - `HOMEBREW.md`, `DEBIAN.md`, `RPM.md` → `docs/packaging/`
  - `RELEASE.md`, `GITHUB_PAGES.md` → `docs/release/`
  - `SECURITY_AUDIT_REPORT.md` → `docs/security/`

  Inside the built `.deb` and `.rpm`, these now install under
  `/usr/share/doc/kn/docs/packaging/` rather than `/usr/share/doc/kn/`.
- **Both READMEs now lead with installation**, ahead of the feature catalog, so a new
  reader reaches the install command on the first screen. Section structure is identical
  in English and Spanish.
- **Corrected inaccurate README claims**: the skill count read 11 in three places when
  the catalog holds 21, and the agent roster listed 9 of 11 (`docs-writer` and `biz`
  were missing).
- **Beads sections now state their status** — this project's workflow uses spec-kit per
  [ADR-006](docs/adr/006-adopt-speckit-remove-beads.md), while `kn beads` remains
  available for projects using Beads independently.

#### Fixed

- **Five broken documentation links**: `LICENSE` from both READMEs, and three references
  to a `DEVELOPMENT.md` that was never created.
- **Dangling references in ADR-003, ADR-004 and ADR-005** — `cli/src/kn_home.rs` moved to
  `cli/src/core/kn_home.rs`; paths under `.beads/` and `.opencode/` were removed by
  earlier work and are now annotated as such rather than linked.
- **Removed a maintainer-specific absolute path** from
  `specs/001-personal-stacks/quickstart.md`.

#### Internal

- `kn.spec` and `debian/rules` copied `HOMEBREW.md` and `DEBIAN.md` by name from the
  repository root, so the relocation above would have broken both package builds. Those
  lines are removed rather than repointed — each recipe already copies `docs/`
  recursively, and repointing would have shipped every affected document twice.

## [0.10.0] - 2026-08-01

### Added

- **Stack-presets** — a preset is a named bundle of skills you activate together, so a
  new project starts with the right knowledge in one step:
  - `kn init --stack <name>` resolves a preset from `~/.kn/stacks/<name>.toml`, enables
    exactly its skills (deduped with any manual selections), and records
    `stack = "<name>"` under `[project]` in `kn.toml`.
  - `kn stack list` / `kn stack show <name>` list and inspect presets, flagging any
    skill missing from the catalog.
  - Shipped presets: `web-astro`, `api-fastapi`, `cli-rust`, `cli-go`, `data-py`
    (in the repo `stacks/` tree, staged to `~/.kn/stacks/` by `install.sh`).
  - Presets are plain user-editable TOML — drop in your own file to define a new one,
    no code change required.
- **New skills** for the maintainer's stack: `fastapi-best-practices`,
  `htmx-best-practices`, `go-best-practices`, `railway-best-practices`.
- **Planner is the default agent** for generated workspaces — `kn init`/`kn sync` set the
  planner as the default persona (OpenCode `opencode.json`) and write a root `CLAUDE.md`
  for Claude Code projects, so a fresh session opens coordinating, not building.

### Changed

- **Curated the skill catalog** to the maintainer's real stack: removed
  `aws-best-practices`, `jsonnet-best-practices`, `kubernetes-best-practices`,
  `terraform-best-practices`, and `notion-reporting-standard`.
- `kn sync` now warns (instead of pointing at a stale install command) when a project's
  `kn.toml` references a skill no longer in the catalog, and leaves the file untouched.

### Notes

- `kn init`/`kn sync` still stage beads formulas and reference `bd` in the generated
  `AGENTS.md` despite beads removal (ADR-006); flagged for a separate cleanup (ADR-007).

## [0.9.0] - 2026-07-17

### Added

- **`workspace_standard = "claude"`** — `kn init`/`kn sync` can now target Claude Code
  directly instead of only OpenCode:
  - Skills symlink to `.claude/skills/<name>/` (no transformation needed — `SKILL.md` is
    already Claude Code's native skill format).
  - Agents symlink to `.claude/agents/<name>.md` — a flat file per agent pointing at
    `~/.kn/agents/<name>/AGENTS.md`, matching Claude Code's subagent file convention
    (not converted to strict subagent frontmatter in this pass — plain reference file).
  - MCP servers generate to `.mcp.json` (Claude Code's real `{"mcpServers": {...}}`
    format) via a new `ClaudeMcpConfig` generator, instead of
    `.opencode/opencode.json`.
  - `kn init`'s workspace prompt is now a real selection (OpenCode / Claude Code)
    instead of being hardcoded to OpenCode.

## [0.8.1] - 2026-07-17

### Added

- Published `kn` on Homebrew via a custom tap
  ([kobogithub/homebrew-knowledge](https://github.com/kobogithub/homebrew-knowledge)):
  `brew tap kobogithub/knowledge && brew install kobogithub/knowledge/kn`. The formula
  downloads precompiled per-arch binaries (no Rust toolchain required) and stages
  skills/agents from the tagged source archive. Note: `kn` already exists in
  `homebrew-core` (Knative client) — the fully-qualified tap name is required.

### Changed

- **Migrated the multi-agent workflow from Beads (`bd`) to GitHub Spec Kit** (`specify-cli`).
  See [ADR-006](docs/adr/006-adopt-speckit-remove-beads.md) for the full rationale.
  - Removed `.beads/` and the `bd-best-practices` skill; dropped `bd-best-practices` from
    `kn.toml` and all 12 agent role definitions.
  - Rewrote `AGENTS.md` (root) and all 11 `agents/<role>/AGENTS.md` to use the spec-kit
    cycle (`/speckit-specify` → `/speckit-plan` → `/speckit-tasks` → `/speckit-implement`)
    instead of `bd` claim/comment/close commands.
  - Remapped `notion-reporting-standard`, `security-gitleaks`, and `uiux-playwright`
    skills off of Beads-specific reporting formats.
  - Marked [ADR-002](docs/adr/002-multi-agent-architecture.md) and
    [ADR-003](docs/adr/003-beads-issue-tracking.md) as superseded by ADR-006.
  - **Not included in this change**: the `kn` CLI's own Rust integration with `bd`
    (`kn beads` subcommand, the `bd` dependency check in `kn doctor`, formula copying in
    `init`/`sync`) — tracked as a separate follow-up, since it's compiled code with its
    own test surface.

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
