# Agent Model Strategy

This document explains the differentiated model strategy for each agent in the knowledge project.

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
   ---
   ```

2. **kn.toml** (Project configuration)
   ```toml
   [agents.backend]
   model = "anthropic/claude-opus-4"
   ```

3. **.opencode/opencode.json** (OpenCode integration)
   ```json
   {
     "agent": {
       "backend": {
         "model": "anthropic/claude-opus-4"
       }
     }
   }
   ```

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
