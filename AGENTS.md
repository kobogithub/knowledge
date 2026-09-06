# Agent Instructions

This project uses **spec-kit** (`specify-cli`, installed in `.specify/` and `.claude/skills/speckit-*`) for spec-driven development with a multi-agent workflow. Every initiative lives in a `specs/NNN-feature-name/` folder (`spec.md`, `plan.md`, `tasks.md`).

## Agent Roles

Each agent has specialized responsibilities and autonomy to close their own tasks:

- **[Analyst Agent](./agents/analyst/AGENTS.md)** (`knowledge-an1`) - Discovery: turns a client input into `docs/product/PROJECT.md` (the brief), without inventing what the client did not say
- **[Planner Agent](./agents/planner/AGENTS.md)** (`knowledge-x6e`) - Coordinates work, runs `/speckit-specify` and `/speckit-plan`, assigns sections of `tasks.md`
- **[Frontend Agent](./agents/frontend/AGENTS.md)** (`knowledge-4yh`) - UI/UX, React, components, client-side
- **[Backend Agent](./agents/backend/AGENTS.md)** (`knowledge-vlf`) - APIs, databases, business logic, security
- **[Rust Agent](./agents/rust/AGENTS.md)** (`knowledge-r5t`) - CLI tools, libraries, systems programming
- **[DevOps Agent](./agents/devops/AGENTS.md)** (`knowledge-w5p`) - Infrastructure, CI/CD, deployment, monitoring
- **[Security Agent](./agents/security/AGENTS.md)** (`knowledge-s3c`) - AppSec, vulnerability scanning, SAST, secret detection, API security
- **[UI/UX Tester Agent](./agents/uiux-tester/AGENTS.md)** (`knowledge-u7x`) - Visual fidelity, interaction testing, accessibility (WCAG), responsive design
- **[QA Agent](./agents/qa/AGENTS.md)** (`knowledge-pu1`) - Testing, quality assurance, bug hunting, test automation
- **[Docs Writer Agent](./agents/docs-writer/AGENTS.md)** (`knowledge-doc`) - Technical writing, ADRs, documentation, knowledge extraction
- **[Biz Agent](./agents/biz/AGENTS.md)** (`knowledge-biz`) - Stakeholder reporting, Notion dashboards, non-technical progress summaries
- **[Finanzas Agent](./agents/finanzas/AGENTS.md)** (`knowledge-f1n`) - OpenRouter cost tracking, spend reports, budget controls

**Click on your role above for detailed instructions.**

> **Coordination model**: there is no atomic task-claiming or locking mechanism anymore
> (that was a `bd`/beads feature, removed — see
> [ADR-006](./docs/adr/006-adopt-speckit-remove-beads.md)). The Planner is the single
> assignment point: it splits `tasks.md` into sections per role/label and hands them out
> via PR comment or directly in the file. Each agent works its own branch (see Git
> Branching Strategy below) so collisions are avoided by branch isolation, not by locking.

## Quick Reference (All Agents)

```bash
# New initiative (Planner runs these; see agents/planner/AGENTS.md)
/speckit-specify <feature description>   # creates specs/NNN-feature-name/spec.md
/speckit-clarify                          # optional, resolves ambiguity before planning
/speckit-plan                             # creates plan.md
/speckit-tasks                            # creates tasks.md (dependency-ordered checkboxes)

# Find your work
# Planner assigns you a section of specs/NNN-feature-name/tasks.md (by role/label,
# via PR comment or inline note) — there is no `bd ready`/`--claim` equivalent.

# Implement your assigned tasks
/speckit-implement                        # executes tasks.md items in order

# Report progress
# Mark your checkboxes [x] in tasks.md as you complete them, and leave progress
# notes as PR comments — there is no separate comments/heartbeat command.

# Push your work (no merge-slot serialization anymore — rely on your own branch)
git push
```

## Framework de 5 Fases

Todo trabajo significativo sigue 5 fases, mapeadas a los comandos de spec-kit
(`.claude/skills/speckit-*`):

| Fase | Nombre | Comandos Clave |
|------|--------|---------------|
| 1 | Exploracion | `/speckit-constitution` (una vez por proyecto), `/speckit-clarify` |
| 2 | Especificacion | `/speckit-specify` |
| 3 | Task Planning | `/speckit-plan`, `/speckit-tasks` |
| 4 | Implementacion | `/speckit-implement` |
| 5 | Verificacion | `/speckit-analyze`, `/speckit-checklist` |

### Convencion de Fase por Rama/PR

Ya no hay labels `phase:*` en un tracker — usar el nombre de rama y la seccion de
`tasks.md` para señalar en que fase esta el trabajo (ej. rama `003-oauth2/backend`
trabajando la seccion "US1" de `specs/003-oauth2/tasks.md`).

## Workflow Principles

1. **Autonomy**: Each agent marks its own checkboxes done in `tasks.md` when complete
2. **Transparency**: Report progress through PR comments
3. **Coordination**: Reference the spec folder (`specs/NNN-feature-name/`) to coordinate with other agents
4. **Ownership**: You own your assigned `tasks.md` section from assignment to completion
5. **Honesty**: Only check off a task when actually complete and tested

## Git Branching Strategy

All agents MUST follow the branching strategy. See skill `standard-commits` for complete documentation.

### Branch Hierarchy

```
prod (stable releases)
  └─ dev (integration)
      └─ epic/<feature-id> (feature integration branch — <feature-id> = specs/NNN-feature-name folder)
          └─ <feature-id>/<agent-role> (agent work branch)
```

### Branching Rules

| Branch Type    | Pattern                    | Created From | PR Target    |
|----------------|----------------------------|--------------|--------------|
| Production     | `prod`                     | -            | -            |
| Integration    | `dev`                      | `prod`       | `prod`       |
| Feature        | `epic/<feature-id>`        | `dev`        | `dev`        |
| Agent work     | `<feature-id>/<agent-role>`| `epic/<id>`  | `epic/<id>`  |
| Independent    | `task/<short-desc>`        | `dev`        | `dev`        |
| Hotfix         | `hotfix/<short-desc>`      | `prod`       | `prod`+`dev` |
| Release        | `release/v<version>`       | `dev`        | `prod`       |

### Conventional Commits (MANDATORY)

All commit messages MUST use format: `<type>(<scope>): <message>`

| Type | Bump | Type | Bump |
|------|------|------|------|
| `feat` | MINOR | `fix` | PATCH |
| `refactor` | PATCH | `perf` | PATCH |
| `build` | PATCH | `ci` | PATCH |
| `chore` | PATCH | `docs` | PATCH |
| `style` | PATCH | `test` | PATCH |
| `any!` (breaking) | MAJOR | | |

### Tagging (Manual)

- Tags are created **manually** after merging to `prod`: `git tag -a vX.Y.Z -m "message" && git push origin vX.Y.Z`
- Pushing a `v*` tag triggers the `release.yml` workflow (binary builds, GitHub Release)

## Agent Coordination

```text
# Reference other agents in PR comments
[Frontend Agent] @knowledge-x6e Need clarification on specs/003-oauth2/spec.md...
[Backend Agent] Blocked by devops — see specs/003-oauth2/tasks.md US2

# Ask the Planner for new work items
# The Planner adds a new checkbox/section to the relevant tasks.md and
# notifies the assigned role by PR comment or direct handoff note.
```

There is no automatic agent-status registry anymore. If you need to know what another
agent is doing, check their open PRs/branches or ask directly — there is no `bd agent
show`/`bd list -l "gt:agent"` equivalent.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **Note remaining work** - Add unchecked items to the relevant `tasks.md`, or leave a PR comment for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update tasks.md** - Check off finished items, leave in-progress ones unchecked
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session (PR comment or session notes)

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
