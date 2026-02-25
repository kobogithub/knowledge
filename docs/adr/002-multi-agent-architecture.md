# ADR-002: Multi-Agent Architecture with Specialized Roles

## Status

Accepted

## Date

2025-01-10

## Context

AI-assisted development workflows require coordination across multiple domains (frontend, backend, infrastructure, testing, documentation). A single monolithic agent faces several challenges:

1. **Context overload**: One agent handling all responsibilities leads to bloated prompts and diluted expertise
2. **Model cost inefficiency**: Using the same high-cost model (e.g., Claude Opus) for both complex architecture decisions and simple test generation wastes budget
3. **Lack of specialization**: Generic agents produce generic solutions; specialized agents leverage domain-specific best practices
4. **Concurrency bottleneck**: One agent can't work on frontend and backend simultaneously
5. **Attribution difficulty**: Hard to track which work was done by which capability/role

The framework needs a way to:
- Distribute work by domain expertise
- Enable parallel execution across roles
- Optimize model selection per workload complexity
- Provide clear ownership and accountability

## Decision

We will implement a **multi-agent architecture** with **9 specialized agents**, each with:
- **Unique agent ID** (e.g., `knowledge-x6e`) for issue assignment and tracking
- **Domain expertise** defined in `agents/{role}/AGENTS.md`
- **Optimized LLM model** based on workload complexity (Opus/Sonnet/Haiku)
- **Dedicated MCP servers** scoped to the agent's needs
- **Autonomous task ownership** - agents claim, execute, and close their own tasks

### Agent Roster

| Agent          | ID             | Model                   | Responsibilities                                  |
|----------------|----------------|-------------------------|--------------------------------------------------|
| Planner        | knowledge-x6e  | claude-opus-4           | Epic creation, task decomposition, coordination  |
| Backend        | knowledge-vlf  | claude-opus-4           | APIs, databases, business logic, security        |
| Frontend       | knowledge-4yh  | claude-sonnet-4.5       | UI/UX, React, components, client-side           |
| Rust           | knowledge-r5t  | claude-sonnet-4.5       | CLI tools, libraries, systems programming        |
| DevOps         | knowledge-w5p  | claude-sonnet-4.5       | Infrastructure, CI/CD, deployment, monitoring    |
| Security       | knowledge-s3c  | claude-sonnet-4.5       | AppSec, SAST, vulnerability scanning            |
| UI/UX Tester   | knowledge-u7x  | claude-sonnet-4.5       | Visual fidelity, accessibility (WCAG)           |
| QA             | knowledge-pu1  | claude-haiku-4.5        | Testing, quality assurance, test automation      |
| Docs Writer    | knowledge-doc  | claude-sonnet-4.5       | Technical writing, ADRs, documentation          |
| Biz            | knowledge-biz  | claude-haiku-4.5        | Stakeholder reporting, Notion dashboards        |
| Finanzas       | knowledge-f1n  | claude-haiku-4.5        | OpenRouter cost tracking, spend reports         |

### Workflow Coordination

Agents coordinate through **Beads (bd)** issue tracking:
```bash
# Planner creates epic and assigns tasks
bd create "Implement user authentication" -t epic -p 0
bd create "Backend: JWT token service" -t task -p 1 --assignee knowledge-vlf
bd create "Frontend: Login form component" -t task -p 1 --assignee knowledge-4yh
bd create "Security: Audit auth implementation" -t task -p 2 --assignee knowledge-s3c

# Agents claim and complete their work
bd update knowledge-vlf.123 --claim
bd comments add knowledge-vlf.123 "[Backend Agent] ✓ JWT service implemented with RS256"
bd close knowledge-vlf.123
```

### Principles

1. **Autonomy**: Each agent closes their own tasks when complete
2. **Transparency**: Progress reported via `bd comments add`
3. **Coordination**: Cross-agent references in issue comments
4. **Ownership**: Agent owns task from claim to completion
5. **Honesty**: Only close when actually complete and tested

## Alternatives Considered

### Alternative 1: Single Generalist Agent
- **Pros**: Simpler mental model, no coordination overhead, one agent to configure
- **Cons**: Massive context bloat, expensive (Opus for all tasks), no parallelism, generic solutions
- **Why rejected**: Doesn't scale. A 10,000-line system prompt combining frontend + backend + DevOps best practices is unmaintainable and ineffective.

### Alternative 2: Domain-Specific Agents with Human Router
- **Pros**: Specialized expertise, clearer boundaries
- **Cons**: Requires human to manually route tasks to agents - bottleneck for autonomous workflows
- **Why rejected**: We want autonomous operation. The Planner agent automates routing.

### Alternative 3: Dynamic Agent Spawning (LangGraph-style)
- **Pros**: Agents created on-demand, infinite scalability
- **Cons**: No persistent identity, hard to track costs, complex orchestration logic
- **Why rejected**: Overkill for our use case. 9 agents is sufficient, and persistent IDs enable cost tracking via OpenRouter's `user` parameter.

### Alternative 4: Hierarchical Multi-Level Agents
- **Pros**: Mirrors org charts (manager → senior → junior agents)
- **Cons**: Overhead of management layers, slower decision-making, complex handoffs
- **Why rejected**: Flat structure is faster. Planner coordinates but doesn't "manage" - other agents have equal autonomy.

## Consequences

### Positive

- **77% cost reduction**: Mixed model strategy (Opus/Sonnet/Haiku) vs. all-Opus
- **Parallel execution**: Frontend and Backend agents work concurrently on same epic
- **Domain expertise**: Security agent uses specialized skills (Trivy, Semgrep, Gitleaks, OWASP ZAP)
- **Clear accountability**: Agent ID in every commit/comment enables blame and cost attribution
- **Optimized context**: Each agent only loads skills/MCPs relevant to their domain
- **Scalability**: Adding new agents (e.g., "Mobile Agent") is straightforward

### Negative

- **Coordination overhead**: Planner must create well-structured epics with clear task boundaries
- **Learning curve**: New contributors must understand agent roles and assignment protocol
- **Potential conflicts**: Multiple agents modifying same file requires merge conflict resolution
- **Complexity**: 9 AGENTS.md files to maintain vs. 1 monolithic agent

### Risks

- **Task misassignment**: Unclear boundaries (e.g., is Tailwind CSS config frontend or DevOps?)
  - **Mitigation**: Documented in `agents/{role}/AGENTS.md` - "When in doubt, ask Planner"
- **Idle agents**: Some agents (e.g., Finanzas) only work weekly, not daily
  - **Mitigation**: Acceptable - agents are stateless LLM calls, no "idle compute cost"
- **Agent capability drift**: Models improve over time, changing what each tier can handle
  - **Mitigation**: Quarterly review (documented in `AGENT_MODEL_STRATEGY.md`)

## References

- [AGENTS.md](../../AGENTS.md) - Agent coordination documentation
- [AGENT_MODEL_STRATEGY.md](../AGENT_MODEL_STRATEGY.md) - Model selection rationale
- [kn.toml](../../kn.toml) - Agent configuration
- [Commit 76b6f69](https://github.com/kobogithub/knowledge/commit/76b6f69) - Differentiated model strategy implementation
- [Commit c0220b7](https://github.com/kobogithub/knowledge/commit/c0220b7) - 5-phase workflow framework (agent coordination)
