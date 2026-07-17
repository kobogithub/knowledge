# ADR-006: Adopt GitHub Spec Kit, Remove Beads (bd)

## Status

Accepted

## Date

2026-07-17

## Context

The project used Beads (`bd`, see [ADR-003](./003-beads-issue-tracking.md)) as its issue
tracker and as the coordination backbone for the 11-agent multi-agent architecture
described in [ADR-002](./002-multi-agent-architecture.md): atomic task claiming
(`bd update --claim`), agent state machine (`bd agent state`), push serialization
(`bd merge-slot`), dependency graphs, gates, and molecule/formula templates for
multi-step workflows.

The project has decided to move to **spec-first development** using GitHub's open-source
`spec-kit` (`specify-cli`) instead of maintaining a custom issue-tracker-driven workflow.
`spec-kit` provides a standard, portable cycle — `/speckit-constitution` →
`/speckit-specify` → `/speckit-clarify` (optional) → `/speckit-plan` → `/speckit-tasks` →
`/speckit-analyze` (optional) → `/speckit-implement` → `/speckit-converge` — installed as
Claude Code skills under `.claude/skills/speckit-*` and backed by `.specify/` (templates,
scripts, constitution) plus a `specs/NNN-feature-name/` folder per initiative
(`spec.md`, `plan.md`, `tasks.md`).

Removing `bd` and adopting `spec-kit` were evaluated together (not independently) because
they overlap on the same job — tracking and structuring work — and adopting one while
keeping the other would mean maintaining two competing sources of truth for "what work is
there and what state is it in."

## Decision

- Remove `bd` (beads) entirely from the workflow layer: delete `.beads/`, drop
  `bd-best-practices` from `kn.toml` and all 12 agent role definitions, and rewrite the
  root `AGENTS.md` plus all 11 `agents/<role>/AGENTS.md` files to use the spec-kit cycle
  instead of `bd` commands.
- Adopt `specify-cli` (`specify init . --integration claude`) as the spec-first workflow
  tool. It is already installed and initialized in this repo (`.specify/`,
  `.claude/skills/speckit-*`).
- **No replacement for `bd`'s multi-agent coordination primitives.** Atomic task claiming,
  agent state machine, heartbeats, `merge-slot` push serialization, dependency graphs,
  gates, and the KV store are dropped without a substitute. Coordination becomes manual:
  the Planner agent is the single assignment point, splitting `specs/NNN-feature/tasks.md`
  into sections per role (via PR comment or inline note in the file), and each agent works
  its own git branch (`<feature-id>/<role>`, unchanged from the existing branching
  strategy) — collision avoidance now comes from branch isolation, not from locking.
- This pass covers only the workflow/docs layer. The `kn` CLI's own Rust integration with
  `bd` (`cli/src/commands/beads.rs`, the `bd` dependency check in `doctor.rs`, formula
  copying in `init.rs`/`sync.rs`) is explicitly **out of scope** and left for a separate
  follow-up, since it is compiled code with its own tests and shouldn't be mixed with a
  documentation/workflow rewrite.
- `docs/adr/002-multi-agent-architecture.md` and `docs/adr/003-beads-issue-tracking.md`
  are marked `Superseded by ADR-006` rather than edited, per this repo's ADR convention
  (immutable once accepted).

## Alternatives Considered

### Alternative 1: Keep `bd`, add `spec-kit` alongside it
- Pros: keeps atomic claiming/coordination primitives; spec-kit only adds spec-writing structure.
- Cons: two sources of truth for "what work exists" (bd issues vs. `specs/*/tasks.md`);
  every workflow doc would need to explain which tool owns which concern. Rejected because
  the user explicitly wants `bd` gone, not layered.

### Alternative 2: Build a custom coordination replacement (claims, locking) on top of spec-kit
- Pros: would preserve the collision-avoidance guarantees `bd` provided.
- Cons: significant new engineering effort to rebuild what `bd` already did, for a single-
  project personal repo where the actual collision risk is low (branch-per-agent already
  isolates work). Rejected as premature — revisit only if branch-based isolation proves
  insufficient in practice.

### Alternative 3: Full replacement including the `kn` CLI's `bd` integration in this same pass
- Pros: no lingering `bd` references anywhere, including compiled code.
- Cons: mixes a large documentation/workflow rewrite with a Rust code change that has its
  own test surface, increasing risk and review burden in a single pass. Rejected in favor
  of scoping the CLI code removal as a separate follow-up (tracked informally, not yet a
  `specs/` initiative).

## Consequences

### Positive

- Single, portable spec-first workflow (`spec-kit`) instead of a bespoke tracker; specs are
  plain markdown, git-diffable, and don't require a companion tool (`bd`) to be installed.
- Removes duplicate reporting/mapping logic — skills like `notion-reporting-standard` now
  read directly from `tasks.md` checkboxes instead of maintaining a separate mapping from
  `bd` epic/priority fields.

### Negative

- Loss of atomic task claiming, agent state visibility, dependency graphs, gates, and
  audit trail — all now handled informally (PR comments, branch names, manual `tasks.md`
  edits) rather than by a queryable system.
- No automated "ready work" queue (`bd ready -l <role>`) — agents depend on the Planner
  actively splitting and assigning `tasks.md` sections rather than pulling work themselves.

### Risks

- **Push collisions between concurrent agents without `merge-slot` serialization.**
  Mitigation: the existing branch-per-agent strategy (`<feature-id>/<role>`) already
  isolates each agent's pushes to its own branch; collisions can only occur at PR-merge
  time into the shared `epic/<feature-id>` branch, which already goes through PR review.
- **Manual assignment doesn't scale well if the number of concurrent initiatives grows.**
  Mitigation: revisit if this becomes a bottleneck — nothing here prevents building a
  lightweight tracker later; the risk is accepted for the current single-project scope.

## References

- [ADR-002: Multi-Agent Architecture](./002-multi-agent-architecture.md) — superseded by this ADR
- [ADR-003: Beads (bd) as Issue Tracking System](./003-beads-issue-tracking.md) — superseded by this ADR
- [GitHub spec-kit](https://github.com/github/spec-kit)
- `AGENTS.md` (root) — the rewritten spec-kit-based workflow
