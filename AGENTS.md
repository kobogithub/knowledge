# Agent Instructions

This project uses **bd** (beads) for issue tracking with a multi-agent workflow.

## Agent Roles

Each agent has specialized responsibilities and autonomy to close their own tasks:

- **[Planner Agent](./AGENTS_PLANNER.md)** (`knowledge-x6e`) - Coordinates work, creates epics, assigns tasks
- **[Frontend Agent](./AGENTS_FRONTEND.md)** (`knowledge-4yh`) - UI/UX, React, components, client-side
- **[Backend Agent](./AGENTS_BACKEND.md)** (`knowledge-vlf`) - APIs, databases, business logic, security
- **[Rust Agent](./AGENTS_RUST.md)** (`knowledge-r5t`) - CLI tools, libraries, systems programming
- **[DevOps Agent](./AGENTS_DEVOPS.md)** (`knowledge-w5p`) - Infrastructure, CI/CD, deployment, monitoring

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

# Complete your work (YOU close your own tasks)
bd comments add <task-id> "[Agent Name] ✓ Completed: details..."
bd close <task-id>
bd agent state <agent-id> done

# Sync with git
bd sync
git add .beads/issues.jsonl
git commit -m "<Agent>: description"
git push
```

## Workflow Principles

1. **Autonomy**: Each agent closes their own tasks when complete
2. **Transparency**: Report progress through comments
3. **Coordination**: Use issue references to coordinate with other agents
4. **Ownership**: You own your tasks from claim to completion
5. **Honesty**: Only close when actually complete and tested

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

