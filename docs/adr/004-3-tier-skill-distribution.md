# ADR-004: 3-Tier Skill Distribution System

## Status

Accepted

## Date

2025-01-20

## Context

The Knowledge Framework uses "skills" - reusable knowledge modules (Markdown files) that provide domain-specific instructions to AI agents. Examples: `rust-best-practices`, `security-trivy`, `docker-best-practices`.

Skills need to be:
1. **Globally accessible**: Installed once, reused across all projects
2. **Project-specific**: Each project selects which skills to enable
3. **Version-controlled**: Tracked in git for reproducibility
4. **Fast to access**: No network calls during agent execution
5. **Workspace-agnostic**: Support both OpenCode and Antigravity standards

Initial design (v0.1-v0.2) installed skills directly in each project:
```
my-project/
└── skills/
    ├── rust-best-practices/SKILL.md
    └── docker-best-practices/SKILL.md
```

**Problems with per-project installation**:
- **Duplication**: Same skill copied to 10 projects = 10x disk space
- **Stale skills**: Updating a skill requires manual sync to all projects
- **Large git repos**: Skills add 500KB-2MB per project
- **Slow `kn init`**: Every project downloads all skills from internet

We need a global skill cache that projects reference, not duplicate.

## Decision

We will implement a **3-tier skill distribution system**:

### Tier 1: Global Skill Library (`~/.kn/skills/`)

Source of truth for all skills, installed once:
```
~/.kn/skills/
├── bash-best-practices/SKILL.md
├── docker-best-practices/SKILL.md
├── rust-best-practices/SKILL.md
├── security-trivy/SKILL.md
└── ...
```

Installed via:
- **Initial install**: `install.sh` populates `~/.kn/skills/` from repository
- **Updates**: `kn update` refreshes skills from GitHub
- **Manual**: `kn skills install <name>` adds new skills

### Tier 2: Project Skill Links (`.opencode/skills/`)

Per-project symlinks to global skills (OpenCode standard):
```
my-project/
└── .opencode/skills/
    ├── rust-best-practices -> ~/.kn/skills/rust-best-practices
    ├── docker-best-practices -> ~/.kn/skills/docker-best-practices
    └── ...
```

Created by:
- **`kn init`**: Auto-detects project type, symlinks recommended skills
- **`kn sync`**: Syncs missing skills from `kn.toml` to `.opencode/skills/`

### Tier 3: Antigravity Compatibility (`.agent/skills/`)

Antigravity workspace standard expects skills in `.agent/skills/`. We create **double-symlinks**:
```
my-project/
├── .opencode/skills/
│   └── rust-best-practices -> ~/.kn/skills/rust-best-practices
└── .agent/skills/
    └── rust-best-practices -> ../.opencode/skills/rust-best-practices
```

This means:
- `.agent/skills/rust-best-practices` → `.opencode/skills/rust-best-practices` → `~/.kn/skills/rust-best-practices`
- **Single source of truth**: `~/.kn/skills/` is the only copy on disk
- **Both standards satisfied**: OpenCode and Antigravity agents find skills in expected locations

### Workflow

```bash
# Install kn (one-time setup)
curl -fsSL https://kn.foxlabar.online/install | bash
# → Populates ~/.kn/skills/ with all available skills

# Initialize project
cd my-project/
kn init -y --workspace-standard both
# → Creates .opencode/skills/ with symlinks to ~/.kn/skills/
# → Creates .agent/skills/ with symlinks to .opencode/skills/
# → Creates kn.toml with enabled skills

# Sync skills after editing kn.toml
vim kn.toml  # Add: enabled = ["security-semgrep"]
kn sync
# → Creates .opencode/skills/security-semgrep -> ~/.kn/skills/security-semgrep
# → Creates .agent/skills/security-semgrep -> ../.opencode/skills/security-semgrep
```

## Alternatives Considered

### Alternative 1: Per-Project Skill Copies (v0.1 design)
- **Pros**: Simple, no symlinks, fully self-contained projects
- **Cons**: Massive duplication (10 projects × 2MB skills = 20MB), stale skills, slow `kn init`
- **Why rejected**: Doesn't scale. A developer with 50 projects would have 100MB of duplicated skills.

### Alternative 2: Git Submodules
- **Pros**: Git-native, version-pinning
- **Cons**: Complex to manage, slower clones, update friction, still duplicates across projects
- **Why rejected**: Submodules are notoriously painful for non-experts. Symlinks are simpler.

### Alternative 3: NPM-style `node_modules/`
- **Pros**: Familiar to JS developers, version resolution built-in
- **Cons**: Requires npm, heavy (package.json, lock files), skills aren't JS packages
- **Why rejected**: Overkill. Skills are static Markdown files, not code dependencies.

### Alternative 4: Docker Volumes
- **Pros**: Containerized consistency
- **Cons**: Requires Docker, slower I/O, overkill for Markdown files
- **Why rejected**: Skills are accessed by AI agents running on the host, not in containers.

### Alternative 5: HTTP Fetch on Demand
- **Pros**: No local storage, always fresh
- **Cons**: Requires internet, slow (network latency), unreliable (GitHub downtime breaks agents)
- **Why rejected**: Offline-first is a core principle. Agents must work without internet.

## Consequences

### Positive

- **Zero duplication**: 100 projects share one copy of each skill in `~/.kn/skills/`
- **Instant updates**: Updating a skill globally (`kn update`) affects all projects immediately
- **Fast `kn init`**: Symlinks are instant (vs. downloading/copying files)
- **Small git repos**: Projects only commit symlinks, not Markdown files
- **Workspace agnostic**: Supports OpenCode (`.opencode/skills/`) and Antigravity (`.agent/skills/`)
- **Offline-first**: Skills cached locally in `~/.kn/`, no network calls needed

### Negative

- **Symlink support**: Requires filesystem that supports symlinks (Linux, macOS, Windows 10+ with Developer Mode)
- **Cross-platform friction**: Windows users must enable Developer Mode or run as admin
- **Version pinning limitations**: All projects share the same skill version (can't pin different versions per project)
- **`.opencode/` in git**: Symlinks committed to git, can break if `~/.kn/` doesn't exist (fresh clone on new machine)

### Risks

- **Windows symlink issues**: Windows 10+ supports symlinks, but requires Developer Mode or admin privileges
  - **Mitigation**: `install.ps1` detects and warns users, docs explain setup
- **Broken symlinks after fresh clone**: Cloning on a new machine without `~/.kn/` breaks symlinks
  - **Mitigation**: `kn init` or `kn sync` detects broken symlinks and recreates them
- **Global skill corruption**: If `~/.kn/skills/` gets corrupted, all projects affected
  - **Mitigation**: `kn doctor` validates skills, `kn update --force` re-downloads

## References

- [Commit 09b6834](https://github.com/kobogithub/knowledge/commit/09b6834) - Global `~/.kn/` architecture implementation
- [Commit 1009192](https://github.com/kobogithub/knowledge/commit/1009192) - Release v0.2.0 with global skills
- [install.sh](../../install.sh) - Installation script populating `~/.kn/skills/`
- [kn_home.rs](../../cli/src/kn_home.rs) - Global `~/.kn/` management code
- [README.md](../../README.md#-quick-start) - User-facing docs on skill installation
