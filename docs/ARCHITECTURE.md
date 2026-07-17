# Knowledge Framework Architecture

This document describes the technical architecture, design decisions, and implementation details of the Knowledge Framework.

## Overview

Knowledge Framework is a **meta-framework** that automates the setup and management of AI-assisted development workflows. It consists of three main components:

1. **CLI Tool (`kn`)** - Rust binary for project management
2. **Agent System** - Multi-agent AI workflow coordination
3. **Skills & Templates** - Reusable knowledge modules

## System Architecture

```
┌─────────────────────────────────────────────────────┐
│                   User / Developer                   │
└─────────────────────┬───────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────┐
│              CLI Tool (kn binary)                    │
│  ┌──────────┬──────────┬──────────┬──────────────┐ │
│  │   init   │  skills  │  beads   │  mcp (soon)  │ │
│  └──────────┴──────────┴──────────┴──────────────┘ │
└─────────────────────┬───────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
        ▼             ▼             ▼
┌──────────┐   ┌──────────┐   ┌──────────┐
│  Skills  │   │  Beads   │   │   MCP    │
│  Repo    │   │  Issues  │   │ Servers  │
└──────────┘   └──────────┘   └──────────┘
        │             │             │
        └─────────────┼─────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────┐
│              AI Agents (Claude, etc.)                │
│  ┌──────────┬──────────┬──────────┬──────────────┐ │
│  │ Planner  │ Frontend │ Backend  │   Rust/etc   │ │
│  └──────────┴──────────┴──────────┴──────────────┘ │
└─────────────────────────────────────────────────────┘
```

## Core Components

### 1. CLI Tool (Rust)

**Location**: `cli/`

**Purpose**: Command-line interface for all operations

**Technology Stack**:
- **clap** - Command-line argument parsing with derive API
- **reqwest** - HTTP client for downloading skills
- **serde** - Serialization/deserialization (JSON, YAML, TOML)
- **anyhow** - Error handling
- **colored** - Terminal output formatting

**Commands**:
```
kn
├── init         # Project initialization
├── skills       # Skills management
│   ├── install  # Install skills
│   └── list     # List installed skills
├── beads        # Beads utilities
│   └── template # Generate issue templates
└── mcp          # MCP server config (planned)
    ├── add
    └── list
```

**Design Decisions**:

1. **Why Rust?**
   - Single binary distribution (no runtime dependencies)
   - Excellent error handling with `Result<T, E>`
   - Fast compilation and execution
   - Cross-platform support
   - Strong type system prevents bugs

2. **Blocking vs Async**
   - Uses `reqwest::blocking` for simplicity
   - CLI operations are short-lived
   - No need for async complexity

3. **Configuration Format**
   - TOML for `kn.toml` (human-readable, Rust-friendly)
   - YAML for skill frontmatter (standard in ecosystem)
   - JSON for MCP config (compatibility)

### 2. Agent System

**Location**: `AGENTS*.md` files

**Purpose**: Coordinate multi-agent AI workflows with specialized roles

**Components**:
- **AGENTS.md** - Main coordination document
- **AGENTS_PLANNER.md** - Planning and orchestration
- **AGENTS_FRONTEND.md** - UI/UX implementation
- **AGENTS_BACKEND.md** - Server-side logic
- **AGENTS_RUST.md** - Systems programming
- **AGENTS_DEVOPS.md** - Infrastructure management

**Agent Architecture**:

```
┌────────────────────────────────────────┐
│         Planner Agent (x6e)            │
│  - Creates epics                       │
│  - Assigns tasks                       │
│  - Coordinates agents                  │
└───────────┬────────────────────────────┘
            │
    ┌───────┼───────┬───────┬───────┐
    ▼       ▼       ▼       ▼       ▼
┌────────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐
│Frontend│ │Back│ │Rust│ │Dev │ │... │
│  (4yh) │ │(vlf│ │(r5t│ │Ops │ │    │
└────────┘ └────┘ └────┘ └────┘ └────┘
```

**Workflow Principles**:
1. **Autonomy** - Each agent closes their own tasks
2. **Transparency** - Progress reported via comments
3. **Coordination** - Cross-agent references in issues
4. **Ownership** - Agent owns task from claim to completion

### 3. Skills System

**Location**: `skills/` (per-project)

**Purpose**: Reusable knowledge modules for AI agents

**Format**:
```markdown
---
name: skill-name
scope: programming|frontend|backend|devops
auto_invoke: true|false
description: Brief description
---

# Skill Content

Markdown content with instructions, examples, best practices.
```

**Resolution Strategy**:
1. **By Name**: Resolves to `https://raw.githubusercontent.com/agentskills/skills/main/{name}/SKILL.md`
2. **By URL**: Direct download from provided URL
3. **By Path**: Copy from local filesystem

**Storage**:
- Skills installed to `skills/{skill-name}/SKILL.md`
- Tracked in `kn.toml` under `[skills].enabled`

### 4. Beads Integration (deprecated — CLI utility only)

> **Deprecated for the agent workflow.** As of [ADR-006](./adr/006-adopt-speckit-remove-beads.md),
> the multi-agent workflow no longer uses `bd`/`.beads/` for coordination or issue tracking —
> see "Spec-Kit Integration" below for the current workflow. The `kn beads template` CLI
> subcommand documented here is still present in the Rust source as a standalone markdown
> template generator; removing it from the CLI is tracked as a separate follow-up task, not
> yet done, so this section is left for reference until that code is removed.

**Location**: `.beads/` (per-project, no longer created by the agent workflow)

**Purpose**: Git-like issue tracking with Dolt backend

**Data Format**:
```
.beads/
├── issues.jsonl       # Human-readable JSONL
└── *.db              # Dolt database files
```

**Template System** (still available via `kn beads template`, code not yet removed):
- 5 template types: epic, task, bug, feature, chore
- Embeded in CLI as const strings
- Structured sections for consistency
- Markdown format with checklists

### 4.5. Spec-Kit Integration

**Location**: `.specify/` (framework config) and `specs/NNN-feature-name/` (per initiative)

**Purpose**: Spec-first development — one folder per feature/initiative, generated and
driven by the `specify-cli` tool via Claude Code skills (`.claude/skills/speckit-*`)

**Data Format**:
```
.specify/
├── memory/constitution.md   # Project governing principles
├── templates/                # spec/plan/tasks/checklist templates
├── scripts/                  # bash helpers (create-new-feature.sh, etc.)
└── workflows/speckit/        # workflow registry

specs/
└── NNN-feature-name/
    ├── spec.md    # What to build (from /speckit-specify)
    ├── plan.md    # Technical design (from /speckit-plan)
    └── tasks.md   # Dependency-ordered checkboxes (from /speckit-tasks)
```

**Why Spec-Kit?**
- Plain markdown, git-diffable, no companion database or binary required
- Portable across AI coding agents (Claude, Copilot, Codex, etc.) via `--integration`
- Cycle enforced by convention (`/speckit-specify` → `/speckit-plan` → `/speckit-tasks` →
  `/speckit-implement`), not by a CLI-enforced gate — see `AGENTS.md` for how the 11 agent
  roles use it

**Coordination model**: no atomic claiming or locking (unlike the old `bd merge-slot`).
The Planner agent splits `tasks.md` into sections per role; each agent works its own git
branch. See [ADR-006](./adr/006-adopt-speckit-remove-beads.md) for the full rationale and
accepted risks.

### 5. MCP (Model Context Protocol) Integration

**Status**: Planned (Phase 2)

**Purpose**: Connect AI agents to documentation servers

**Design**:
```toml
[mcp]
servers = [
  "rust-docs",
  "mdn-web-docs",
  "custom-api-docs"
]
```

**Implementation**:
- Generate MCP config files for OpenCode/Claude
- Validate server availability
- List available servers
- Custom server definitions

## Data Flow

### Project Initialization

```
User runs: kn init
    │
    ├─ Detect project type (Cargo.toml, package.json, etc.)
    ├─ Generate AGENTS.md from template
    ├─ Create kn.toml with detected settings
    └─ Output next steps
```

### Skill Installation

```
User runs: kn skills install typescript
    │
    ├─ Resolve source (name → URL)
    ├─ Download SKILL.md content
    ├─ Parse YAML frontmatter
    ├─ Validate metadata (name required)
    ├─ Create skills/typescript/ directory
    ├─ Write SKILL.md file
    ├─ Update kn.toml [skills].enabled
    └─ Output success message
```

### Template Generation

```
User runs: kn beads template epic -o epic.md
    │
    ├─ Select template const string
    ├─ Check if output file exists
    ├─ Create parent directories if needed
    ├─ Write template to file
    └─ Output confirmation
```

## Design Decisions

### 1. Why Markdown for Templates?

**Pros**:
- Universal format (readable anywhere)
- Git-friendly (diffs, merges)
- Supports code blocks, checklists
- No parsing overhead for humans

**Cons**:
- No structured validation
- Free-form content

**Decision**: Markdown with structured sections is best balance

### 2. Why TOML for Configuration?

**Pros**:
- Human-readable and editable
- Strong typing
- Comments supported
- Rust-native (serde support)

**Cons**:
- Less popular than JSON/YAML
- Indentation-sensitive

**Decision**: TOML provides best UX for config files

### 3. Why Embedded Templates?

**Pros**:
- Single binary distribution
- No runtime file dependencies
- Fast (no I/O)
- Versioned with CLI

**Cons**:
- Templates not easily customizable
- Binary size increase (minimal: ~5KB)

**Decision**: Embedded for v1, extensible in future

### 4. Why Blocking HTTP?

**Pros**:
- Simpler code
- Sufficient performance for CLI
- No async runtime overhead

**Cons**:
- Can't parallelize downloads
- Blocks during network calls

**Decision**: Simplicity over async complexity for CLI

## Performance Characteristics

### CLI Startup Time
- **Target**: < 100ms
- **Current**: ~50ms (Rust binary)
- **Bottleneck**: None (statically linked)

### Skill Installation
- **Target**: < 2s per skill
- **Current**: ~500ms (network dependent)
- **Bottleneck**: HTTP download time

### Template Generation
- **Target**: < 10ms
- **Current**: ~5ms
- **Bottleneck**: File I/O

### Build Time
- **Clean build**: ~20s (with dependencies)
- **Incremental**: ~3s
- **Release build**: ~20s (with optimizations)

## Security Considerations

### 1. Skill Downloads
- **Threat**: Malicious code in skills
- **Mitigation**: Skills are markdown (passive content)
- **Future**: Signature verification, allowlist

### 2. HTTP Downloads
- **Threat**: Man-in-the-middle attacks
- **Mitigation**: HTTPS only (reqwest enforces)
- **Validation**: URL parsing with `url` crate

### 3. File Operations
- **Threat**: Path traversal attacks
- **Mitigation**: Validate paths, no `.` or `..` in skill names
- **Permissions**: Respect filesystem permissions

### 4. Configuration Parsing
- **Threat**: Malicious TOML/YAML
- **Mitigation**: Schema validation, bounded parsing
- **Error Handling**: Fail fast on invalid config

## Testing Strategy

### Current State
- Manual testing during development
- Integration tests via CLI invocation
- No unit tests yet (Phase 2)

### Planned Tests

**Unit Tests**:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_skill_metadata() { }
    
    #[test]
    fn test_detect_project_type() { }
    
    #[test]
    fn test_template_generation() { }
}
```

**Integration Tests**:
```rust
// tests/cli_tests.rs
use assert_cmd::Command;

#[test]
fn test_init_command() {
    let mut cmd = Command::cargo_bin("kn").unwrap();
    cmd.arg("init").arg("--help")
        .assert()
        .success();
}
```

## Future Architecture

### Plugin System (Phase 3)

```
kn plugins/
├── install <name>
├── list
└── uninstall <name>

~/.kn/plugins/
├── custom-skill-source/
└── additional-templates/
```

### Multi-Project Sync (Phase 3)

```
kn sync
├── pull         # Sync from central repo
├── push         # Push local changes
└── status       # Show sync status
```

### Web Dashboard (Phase 4)

```
kn serve --port 8080
# Opens web UI at http://localhost:8080
# Shows: projects, agents, tasks, metrics
```

## Dependencies

### Production
- `clap = 4.5` - CLI parsing
- `serde = 1.0` - Serialization
- `serde_json = 1.0` - JSON support
- `serde_yaml = 0.9` - YAML parsing
- `toml = 0.8` - TOML parsing
- `anyhow = 1.0` - Error handling
- `colored = 2.1` - Terminal colors
- `reqwest = 0.11` - HTTP client
- `url = 2.5` - URL parsing

### Development
- `assert_cmd` - CLI testing
- `tempfile` - Temp files for tests

## Build Artifacts

### Binary Size
- **Debug**: ~15MB (unoptimized)
- **Release**: ~2.3MB (stripped, optimized)
- **Release + LTO**: ~1.8MB (max optimization)

### Optimization Settings

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit
strip = true         # Strip symbols
```

---

**Last Updated**: 2024-02-13  
**Version**: 0.1.0  
**Status**: In Active Development (57% complete)
