# Agent Instructions

This project uses **bd** (beads) for issue tracking with a multi-agent workflow.

## Agent Roles

Each agent has specialized responsibilities and autonomy to close their own tasks:

- **[Planner Agent](./agents/planner/AGENTS.md)** (`knowledge-x6e`) - Coordinates work, creates epics, assigns tasks
- **[Frontend Agent](./agents/frontend/AGENTS.md)** (`knowledge-4yh`) - UI/UX, React, components, client-side
- **[Backend Agent](./agents/backend/AGENTS.md)** (`knowledge-vlf`) - APIs, databases, business logic, security
- **[Rust Agent](./agents/rust/AGENTS.md)** (`knowledge-r5t`) - CLI tools, libraries, systems programming
- **[DevOps Agent](./agents/devops/AGENTS.md)** (`knowledge-w5p`) - Infrastructure, CI/CD, deployment, monitoring
- **[Security Agent](./agents/security/AGENTS.md)** (`knowledge-s3c`) - AppSec, vulnerability scanning, SAST, secret detection, API security
- **[UI/UX Tester Agent](./agents/uiux-tester/AGENTS.md)** (`knowledge-u7x`) - Visual fidelity, interaction testing, accessibility (WCAG), responsive design
- **[QA Agent](./agents/qa/AGENTS.md)** (`knowledge-pu1`) - Testing, quality assurance, bug hunting, test automation
- **[Docs Writer Agent](./agents/docs-writer/AGENTS.md)** (`knowledge-doc`) - Technical writing, ADRs, documentation, knowledge extraction
- **[Finanzas Agent](./agents/finanzas/AGENTS.md)** (`knowledge-f1n`) - OpenRouter cost tracking, spend reports, budget controls

**Click on your role above for detailed instructions.**

## Quick Reference (All Agents)

```bash
# Find your work
bd ready -l <your-label>        # e.g., bd ready -l frontend
bd list --assignee <agent-id>   # Your assigned tasks

# Claim and start work
bd update <task-id> --claim     # Atomically claim task
bd agent state <agent-id> working

# Report progress
bd comments add <task-id> "[Agent Name] Progress update..."
bd agent heartbeat <agent-id>   # Update activity timestamp

# Complete your work (YOU close your own tasks)
bd comments add <task-id> "[Agent Name] ✓ Completed: details..."
bd close <task-id>
bd agent state <agent-id> done

# Serialize pushes with merge-slot
bd merge-slot acquire
git push
bd merge-slot release

# Sync with git
bd sync
git add .beads/issues.jsonl
git commit -m "<Agent>: description"
git push
```

## Framework de 5 Fases

Todo trabajo significativo sigue 5 fases. Ver skill `bd-best-practices` para detalles completos.

| Fase | Nombre | Comandos Clave |
|------|--------|---------------|
| 1 | Exploracion | `bd create -t decision`, `bd query`, `bd kv`, `bd todo add` |
| 2 | Especificacion | `bd formula list`, `bd cook`, `bd lint`, `bd graph` |
| 3 | Task Planning | `bd mol pour`, `bd swarm`, `bd slot`, `bd count` |
| 4 | Implementacion | `bd agent state`, `bd heartbeat`, `bd merge-slot`, `bd audit` |
| 5 | Verificacion | `bd gate resolve`, `bd preflight`, `bd orphans`, `bd epic close-eligible` |

### Labels por Fase
- `phase:exploration` - Investigacion y discovery
- `phase:specification` - Especificacion y plan
- `phase:planning` - Task planning y asignacion
- `phase:implementation` - Implementacion activa
- `phase:verification` - Verificacion y cierre

## Workflow Principles

1. **Autonomy**: Each agent closes their own tasks when complete
2. **Transparency**: Report progress through comments
3. **Coordination**: Use issue references to coordinate with other agents
4. **Ownership**: You own your tasks from claim to completion
5. **Honesty**: Only close when actually complete and tested

## Git Branching Strategy

All agents MUST follow the branching strategy. See skill `standard-commits` for complete documentation.

### Branch Hierarchy

```
prod (stable releases)
  └─ dev (integration)
      └─ epic/<epic-id> (epic integration branch)
          └─ <epic-id>/<agent-role> (agent work branch)
```

### Branching Rules

| Branch Type    | Pattern                    | Created From | PR Target    |
|----------------|----------------------------|--------------|--------------|
| Production     | `prod`                     | -            | -            |
| Integration    | `dev`                      | `prod`       | `prod`       |
| Epic           | `epic/<epic-id>`           | `dev`        | `dev`        |
| Agent work     | `<epic-id>/<agent-role>`   | `epic/<id>`  | `epic/<id>`  |
| Independent    | `task/<task-id>`           | `dev`        | `dev`        |
| Hotfix         | `hotfix/<issue-id>`        | `prod`       | `prod`+`dev` |
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

```bash
# Reference other agents in comments
bd comments add task-id "[Frontend Agent] @knowledge-x6e Need clarification..."
bd comments add task-id "[Backend Agent] Blocked by devops-task-id"

# Create tasks for other agents
bd create "Setup staging database" \
  -t chore -p 1 -l devops \
  --assignee knowledge-w5p

# Check agent status
bd list -l "gt:agent"           # All agents
bd agent show <agent-id>         # Specific agent
```

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

