# ADR-003: Beads (bd) as Issue Tracking System

## Status

Accepted

## Date

2024-12-10

## Context

The Knowledge Framework requires an issue tracking system that supports:

1. **Git-native workflow**: Issues should live in the repository alongside code
2. **Offline-first**: Developers must be able to create/update issues without internet
3. **AI-friendly format**: Agents need programmatic access to read/write issues
4. **Rich metadata**: Support for labels, priorities, assignees, hierarchies, dependencies
5. **Audit trail**: Track who did what and when
6. **Complex workflows**: Support for epics, molecules (multi-step workflows), gates, and custom issue types
7. **Flexible queries**: Agents need to filter by status, assignee, labels, dates, etc.

Traditional options fall short:
- **GitHub Issues**: Requires internet, not git-native, limited offline support
- **GitLab Issues**: Same limitations as GitHub
- **JIRA**: Heavyweight, expensive, poor CLI, not git-native
- **Plain Markdown files**: No structure, no queries, no workflow support
- **YAML/JSON files**: Manual management, no CLI, no validation

## Decision

We will use **Beads (`bd`)** by Steve Yegge as the issue tracking system.

**Beads** is a git-native issue tracker with:
- **SQLite + JSONL dual storage**: `.beads/issues.jsonl` for git diffs, `.beads/*.db` for queries
- **Rich CLI**: `bd create`, `bd list`, `bd update`, `bd comments add`, `bd close`, etc.
- **Flexible schema**: Support for custom issue types, labels, metadata
- **Hierarchical issues**: Epics → tasks, parent/child relationships
- **Advanced workflows**: Molecules (formulas), gates (approval steps), convoys (release trains)
- **Dolt backend (optional)**: Git-like version control for issue data with branching/merging

### Integration Points

1. **Per-project tracking**: Each repository has `.beads/issues.jsonl`
2. **Agent assignment**: `--assignee knowledge-vlf` assigns issues to specific agents
3. **Formula templates**: `.beads/formulas/mol-feature.formula.json` defines multi-step workflows
4. **5-phase framework**: Labels `phase:exploration`, `phase:implementation`, etc.
5. **Quality gates**: `bd gate create` blocks merges until criteria met

### Example Workflow

```bash
# Planner creates epic
bd create "User authentication" -t epic -p 0 -l authentication

# Planner creates tasks
bd create "JWT token service" -t task -p 1 --assignee knowledge-vlf --parent <epic-id>
bd create "Login UI component" -t task -p 1 --assignee knowledge-4yh --parent <epic-id>

# Backend agent claims and works
bd update <task-id> --claim
bd agent state knowledge-vlf working
# ... do work ...
bd comments add <task-id> "[Backend Agent] ✓ Implemented JWT service with RS256 signing"
bd close <task-id>

# Query progress
bd list --parent <epic-id>
bd list --assignee knowledge-vlf -s closed
```

## Alternatives Considered

### Alternative 1: GitHub Issues
- **Pros**: Familiar, integrated with PRs, web UI, notifications
- **Cons**: Requires internet, not git-native, issues live on GitHub servers (lock-in), poor offline support
- **Why rejected**: We need offline-first and git-native. Agents should be able to create issues even without internet.

### Alternative 2: git-bug
- **Pros**: Git-native, distributed, offline-first
- **Cons**: Limited workflow support (no formulas/molecules), no hierarchical issues, smaller ecosystem, less active development
- **Why rejected**: Too basic. We need epics, molecules, gates, and complex queries.

### Alternative 3: Fossil (integrated issue tracker)
- **Pros**: Built-in wiki + issues + version control in one tool
- **Cons**: Not git (custom VCS), smaller ecosystem, requires replacing git entirely
- **Why rejected**: Git is non-negotiable. We can't ask developers to abandon git.

### Alternative 4: Linear (API-based)
- **Pros**: Excellent UX, great for teams, good API
- **Cons**: Cloud-only, requires subscription, not git-native, vendor lock-in
- **Why rejected**: Not git-native, requires internet, monthly cost.

### Alternative 5: Plain YAML files + custom tooling
- **Pros**: Full control, git-native, simple format
- **Cons**: We'd have to build CLI, queries, workflows, validation from scratch
- **Why rejected**: Reinventing the wheel. Beads already provides this with 10+ years of evolution.

## Consequences

### Positive

- **Git-native**: Issues tracked in `.beads/issues.jsonl`, diffable, mergeable
- **Offline-first**: Agents work without internet connection
- **Rich queries**: `bd list -l frontend --assignee knowledge-4yh -s open`
- **Hierarchical issues**: Epics contain tasks, tasks contain subtasks
- **Formulas (molecules)**: Define multi-step workflows (e.g., `mol-feature.formula.json`)
- **Quality gates**: `bd gate create` blocks epic closure until criteria met
- **Audit trail**: Every action tracked with timestamp and actor
- **Flexible metadata**: Custom labels, priorities, types, fields
- **Agent coordination**: Comments link agents (e.g., `@knowledge-x6e need input`)
- **Proven in production**: Used by Steve Yegge and team for years

### Negative

- **Learning curve**: New tool, not as familiar as GitHub Issues
- **Documentation**: Beads docs are evolving, some features under-documented
- **Ecosystem**: Smaller community than GitHub Issues/JIRA
- **UI**: No web interface (CLI-only, though this is fine for AI agents)
- **Dolt dependency (optional)**: Advanced features require Dolt (Git for data)

### Risks

- **Beads abandonment**: What if Steve stops maintaining it?
  - **Mitigation**: Beads is open source, issues.jsonl is human-readable, we can fork if needed
- **Merge conflicts**: Two agents updating same issue simultaneously
  - **Mitigation**: Beads handles JSON merges well, and we have `bd merge-slot` for push serialization
- **Scalability**: Will Beads handle 10,000+ issues?
  - **Mitigation**: Beads uses SQLite, which scales to millions of rows. Issues.jsonl is compressed on disk.
- **GitHub integration**: Issues live in repo, not on GitHub
  - **Mitigation**: Can sync to GitHub Issues via `bd export` if needed. For now, local-first is a feature, not a bug.

## References

- [Beads GitHub](https://github.com/beadlist/beads) - Official repository
- [bd-best-practices skill](../../.opencode/skills/bd-best-practices/SKILL.md) - 5-phase workflow documentation
- [.beads/formulas/](../../.beads/formulas/) - Workflow formula templates
- [AGENTS.md](../../AGENTS.md) - Agent coordination via Beads
- [Commit c0220b7](https://github.com/kobogithub/knowledge/commit/c0220b7) - 5-phase workflow implementation with Beads
