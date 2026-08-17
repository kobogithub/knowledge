# Agent Model Strategy

This document explains the differentiated model strategy for each agent in the knowledge project.

## Two model identifiers, two runtimes — do not unify them

An agent's model is named in two different places, in two different syntaxes. They look
like the same thing and they are not:

| Where | Syntax | Consumed by |
|---|---|---|
| `model:` in the frontmatter of `agents/*/AGENTS.md` | Tier alias — `opus`, `sonnet`, `haiku` | **Claude Code**, when the agent is invoked |
| `"model"` inside the JSON payloads below | Prefixed id — `anthropic/claude-opus-4` | **OpenRouter's** API namespace |

The frontmatter previously carried the prefixed form, which Claude Code cannot resolve —
every agent failed on invocation with *"There's an issue with the selected model"* (see
issue #6). It now uses tier aliases, which do not break when a new model family ships.

**A find-and-replace across this repository will corrupt one or the other.** The OpenRouter
ids in this document are correct as written; leave them alone.

| Frontmatter alias | OpenRouter id | Agents |
|---|---|---|
| `opus` | `anthropic/claude-opus-4` | planner, backend |
| `sonnet` | `anthropic/claude-sonnet-4.5` | devops, docs-writer, frontend, rust, security, uiux-tester |
| `haiku` | `anthropic/claude-haiku-4.5` | biz, finanzas, qa |

> **Separately worth checking**: the OpenRouter ids below name the Claude 4 family. If
> OpenRouter has since retired those names, the routing config is stale in its own right —
> a different problem from the frontmatter bug, and not fixed here.

## Provider: OpenRouter

All agents use **OpenRouter** (`https://openrouter.ai/api/v1`) as the unified LLM provider. This gives us:

- **Pay-per-token billing** with full cost visibility per request
- **Usage tracking** via the Activity API (grouped by API key, model, or user)
- **Activity Export** as CSV/PDF for financial reporting
- **User Tracking** via the `user` parameter to attribute costs per agent
- **Model fallbacks** and provider routing for high availability
- **Prompt caching** for cost reduction on repeated contexts

API key: `OPENROUTER_API_KEY` (see `.env.example`)

## Model Selection Rationale

We use different Claude models optimized for each agent's specific workload:

### Claude Opus 4 - Maximum Reasoning
**Used by:** Planner, Backend
**Model ID:** `anthropic/claude-opus-4`
**Pricing:** $15.00 / $75.00 per 1M tokens (input/output)
**Characteristics:**
- Highest reasoning capability
- Best for complex decision-making
- Ideal for architecture and strategic planning
- Higher cost, justified by task complexity

**Why:**
- **Planner**: Requires deep reasoning for task decomposition, dependency analysis, and strategic coordination
- **Backend**: Complex architecture decisions, database schema design, security patterns, and business logic

### Claude Sonnet 4.5 - Balanced Performance
**Used by:** Frontend, Rust, DevOps, Security, UI/UX Tester
**Model ID:** `anthropic/claude-sonnet-4.5`
**Pricing:** $3.00 / $15.00 per 1M tokens (input/output)
**Characteristics:**
- Excellent balance of performance and cost
- Strong code generation capabilities
- Good for most development tasks
- Fast response times

**Why:**
- **Frontend**: UI/UX implementation, component development, client-side logic
- **Rust**: Systems programming, CLI tools, type-safe development
- **DevOps**: Infrastructure configuration, pipeline optimization, deployment automation
- **Security**: Vulnerability scanning, SAST analysis, security pattern detection
- **UI/UX Tester**: Visual fidelity testing, accessibility validation, responsive design checks

### Claude Haiku 4.5 - Speed & Efficiency
**Used by:** QA, Finanzas
**Model ID:** `anthropic/claude-haiku-4.5`
**Pricing:** $1.00 / $5.00 per 1M tokens (input/output)
**Characteristics:**
- Fastest response times
- Most cost-effective
- Excellent for repetitive tasks
- Good for test generation and data processing

**Why:**
- **QA**: Automated test generation, test execution, validation scripts, repetitive quality checks
- **Finanzas**: Cost reporting, usage analysis, financial summaries - data-centric work that doesn't need deep reasoning

## Model Distribution

| Agent       | Model                        | Tier   | Use Case                          | Input $/1M  | Output $/1M |
|-------------|------------------------------|--------|-----------------------------------|-------------|-------------|
| Planner     | anthropic/claude-opus-4      | High   | Strategic planning & coordination | $15.00      | $75.00      |
| Backend     | anthropic/claude-opus-4      | High   | Architecture & business logic     | $15.00      | $75.00      |
| Frontend    | anthropic/claude-sonnet-4.5  | Medium | UI/UX implementation              | $3.00       | $15.00      |
| Rust        | anthropic/claude-sonnet-4.5  | Medium | Systems programming               | $3.00       | $15.00      |
| DevOps      | anthropic/claude-sonnet-4.5  | Medium | Infrastructure & CI/CD            | $3.00       | $15.00      |
| Security    | anthropic/claude-sonnet-4.5  | Medium | AppSec & vulnerability scanning   | $3.00       | $15.00      |
| UI/UX Tester| anthropic/claude-sonnet-4.5  | Medium | Visual & accessibility testing    | $3.00       | $15.00      |
| QA          | anthropic/claude-haiku-4.5   | Low    | Test automation                   | $1.00       | $5.00       |
| Finanzas    | anthropic/claude-haiku-4.5   | Low    | Cost tracking & reporting         | $1.00       | $5.00       |

## Configuration Files

The model configuration is defined in three places:

1. **AGENTS.md** (Source of truth)
   ```yaml
   ---
   name: backend
   model: anthropic/claude-opus-4
   reasoning: Complex architecture decisions...
   mcp_servers:
     - name: github
       package: "@modelcontextprotocol/server-github"
       description: GitHub API for PRs, issues, and code reviews
   ---
   ```

2. **kn.toml** (Project configuration)
   ```toml
   [agents.backend]
   model = "anthropic/claude-opus-4"

   [mcp]
   enabled = ["playwright", "penpot", "github", "postgres"]
   ```

3. **.opencode/opencode.json** (OpenCode integration)
   ```json
   {
     "agent": {
       "backend": {
         "model": "anthropic/claude-opus-4",
         "tools": {
           "github_*": true,
           "postgres_*": true,
           "context7_*": true,
           "sentry_*": true
         }
       }
     }
   }
   ```

## MCP Server Assignments

All MCPs are declared globally and disabled by default via `tools: { "mcp_*": false }`.
Each agent enables only the MCPs it needs via per-agent `tools` configuration.

### Available MCPs

| MCP | Type | Package/URL | Description |
|-----|------|-------------|-------------|
| playwright | local | `@playwright/mcp` | Browser automation, screenshots, DOM inspection |
| penpot | local | `penpot-mcp-server` | Design tokens, component specs from Penpot |
| github | local | `@modelcontextprotocol/server-github` | GitHub API (PRs, issues, repos, Actions) |
| postgres | local | `@modelcontextprotocol/server-postgres` | PostgreSQL read-only access, schema inspection |
| context7 | remote | `https://mcp.context7.com/mcp` | Documentation search for frameworks/libraries |
| sentry | remote | `https://mcp.sentry.dev/mcp` | Error tracking and production issue analysis |

### Per-Agent Distribution

| Agent | playwright | penpot | github | postgres | context7 | sentry |
|-------|:----------:|:------:|:------:|:--------:|:--------:|:------:|
| Planner | | | ✅ | | | |
| Backend | | | ✅ | ✅ | ✅ | ✅ |
| Frontend | ✅ | ✅ | | | ✅ | |
| Rust | | | | | ✅ | |
| DevOps | | | ✅ | | | ✅ |
| QA | ✅ | | ✅ | | | ✅ |
| Security | | | ✅ | | | ✅ |
| UI/UX Tester | ✅ | ✅ | | | | |
| Finanzas | | | | | | |

### Design Rationale

- **Context budget**: Each MCP adds tokens to context. Agents with Haiku (QA, Finanzas) get minimal MCPs.
- **Least privilege**: Agents only get MCPs directly relevant to their work.
- **playwright**: Only for agents doing browser/visual testing (Frontend, QA, UI/UX Tester).
- **postgres**: Only Backend — the only agent that inspects DB schemas directly.
- **context7**: For agents that frequently consult framework documentation (Backend, Frontend, Rust).
- **sentry**: For agents that diagnose production errors (Backend, DevOps, QA, Security).

## Cost Tracking with OpenRouter

### Per-Request Usage

Every API response includes a `usage` object:

```json
{
  "usage": {
    "prompt_tokens": 194,
    "completion_tokens": 2,
    "total_tokens": 196,
    "cost": 0.95
  }
}
```

### Agent-Level Tracking

Use the `user` parameter to attribute costs per agent:

```json
{
  "model": "anthropic/claude-opus-4",
  "user": "knowledge-vlf",
  "messages": [...]
}
```

This enables the **Finanzas Agent** to query usage grouped by agent ID via the Activity API.

### Activity Export

Reports can be exported from `https://openrouter.ai/activity` as CSV or PDF, grouped by:
- **API Key** - total project spend
- **Model** - spend per model tier
- **User** - spend per agent (when using `user` parameter)

## Cost Optimization

Estimated cost comparison (per 1M tokens input + output):

| Strategy | Cost |
|----------|------|
| All Opus | $90.00/M |
| Mixed (current) | ~$21.00/M average |
| **Savings** | **~77%** |

Additional optimization via OpenRouter:
- **Prompt caching**: Reduces repeat context costs
- **Provider routing**: Automatically selects cheapest available provider
- **Model fallbacks**: Prevents failed requests from requiring retries

## Adjusting Models

To change a model for an agent:

1. Update `agents/{agent}/AGENTS.md` frontmatter
2. Update `kn.toml` agent configuration
3. Update `.opencode/opencode.json` (if using OpenCode)
4. Commit and push changes

Example:
```bash
# If Frontend needs more complex reasoning
vim agents/frontend/AGENTS.md  # Change to anthropic/claude-opus-4
vim kn.toml                    # Update model
git commit -am "feat: Upgrade Frontend to Opus for complex UI work"
```

## Monitoring & Iteration

We track via the Finanzas Agent:
- **Cost per agent**: Daily/weekly/monthly spend breakdown
- **Token usage**: Input vs output token ratios
- **Model efficiency**: Cost-per-task-completed metrics
- **Budget alerts**: Thresholds for spend notifications

Consider upgrading/downgrading models based on:
- Task complexity changes
- Quality requirements
- Budget constraints
- Performance needs

## Best Practices

1. **Start conservative**: Use Haiku by default, upgrade only when needed
2. **Measure impact**: Track quality differences between models via Finanzas reports
3. **Document reasoning**: Always explain why a specific model was chosen
4. **Review periodically**: Re-evaluate model choices as Claude evolves
5. **Use user tracking**: Always pass agent ID as `user` parameter for cost attribution

---

**Last updated:** 2026-02-23
**Provider:** OpenRouter (`https://openrouter.ai/api/v1`)
**Next review:** When new Claude models are released or significant task changes occur
