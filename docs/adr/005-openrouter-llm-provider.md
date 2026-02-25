# ADR-005: OpenRouter as Unified LLM Provider with Differentiated Models

## Status

Accepted

## Date

2025-02-15

## Context

The Knowledge Framework's multi-agent system requires LLM API access for 11 specialized agents (Planner, Backend, Frontend, Rust, DevOps, Security, UI/UX Tester, QA, Docs Writer, Biz, Finanzas).

Initial design used direct Anthropic API, but faced challenges:
1. **Cost opacity**: No visibility into per-agent spend
2. **Model rigidity**: All agents used Claude Opus (expensive, overkill for simple tasks)
3. **No fallback**: If Anthropic API is down, all agents blocked
4. **Manual cost tracking**: Had to parse logs to estimate spend

Requirements for production use:
- **Cost attribution**: Track spend per agent (e.g., "Backend costs $50/month, QA costs $5/month")
- **Model flexibility**: Use different models per agent based on task complexity
- **Fallback/routing**: Automatic failover if primary provider is down
- **Budget controls**: Set spending limits per agent or project
- **Transparent billing**: Clear itemized costs for financial reporting

We need a **unified LLM gateway** that supports:
- Multiple models (Opus, Sonnet, Haiku)
- Cost tracking per agent
- Automatic provider routing
- Prompt caching for efficiency

## Decision

We will use **OpenRouter** (`https://openrouter.ai/api/v1`) as the unified LLM provider for all agents.

### Model Distribution Strategy

We assign models based on agent workload complexity:

| Agent         | Model                   | Cost ($/1M tokens) | Rationale                                    |
|---------------|-------------------------|--------------------|---------------------------------------------|
| Planner       | claude-opus-4           | $15 / $75          | Strategic planning, epic decomposition      |
| Backend       | claude-opus-4           | $15 / $75          | Architecture, database design, security     |
| Frontend      | claude-sonnet-4.5       | $3 / $15           | UI/UX implementation, components            |
| Rust          | claude-sonnet-4.5       | $3 / $15           | Systems programming, CLI tools              |
| DevOps        | claude-sonnet-4.5       | $3 / $15           | Infrastructure, CI/CD                       |
| Security      | claude-sonnet-4.5       | $3 / $15           | SAST, vulnerability scanning                |
| UI/UX Tester  | claude-sonnet-4.5       | $3 / $15           | Visual testing, accessibility               |
| Docs Writer   | claude-sonnet-4.5       | $3 / $15           | Technical writing, ADRs                     |
| QA            | claude-haiku-4.5        | $1 / $5            | Test generation, automation                 |
| Biz           | claude-haiku-4.5        | $1 / $5            | Stakeholder reports (non-technical)         |
| Finanzas      | claude-haiku-4.5        | $1 / $5            | Cost tracking, spend reports                |

**Cost savings**: Mixed strategy averages ~$21/M tokens vs. $90/M for all-Opus (**77% reduction**).

### Cost Attribution via `user` Parameter

Every API request includes the agent ID as `user` parameter:
```json
{
  "model": "anthropic/claude-opus-4",
  "user": "knowledge-vlf",
  "messages": [...]
}
```

OpenRouter's Activity API groups costs by `user`, enabling per-agent spend reports.

### OpenRouter Benefits

1. **Pay-per-token billing**: Only pay for what you use, no monthly minimums
2. **Usage tracking**: Activity API provides detailed per-request costs
3. **Activity export**: Download CSV/PDF reports for financial analysis
4. **Prompt caching**: Reduces costs for repeated context (agent instructions)
5. **Model fallbacks**: If Claude is down, auto-route to GPT-4 (configurable)
6. **Provider routing**: Automatically selects cheapest/fastest available provider
7. **No vendor lock-in**: Easy to switch models or providers without code changes

## Alternatives Considered

### Alternative 1: Direct Anthropic API
- **Pros**: Slightly lower latency (one less hop), official API
- **Cons**: No cost attribution per agent, no fallback, no multi-model routing, manual cost tracking
- **Why rejected**: Cost tracking is critical. We need to know if Backend is 10x more expensive than QA.

### Alternative 2: Azure OpenAI
- **Pros**: Enterprise SLAs, private deployment, HIPAA/SOC2 compliance
- **Cons**: Requires Azure account, more complex setup, higher base cost, limited model selection
- **Why rejected**: Overkill for open-source project. OpenRouter's flexibility is more valuable.

### Alternative 3: LangChain Router
- **Pros**: Open source, self-hosted, full control
- **Cons**: We'd have to build cost tracking, model routing, fallback logic ourselves
- **Why rejected**: Reinventing the wheel. OpenRouter already provides this.

### Alternative 4: LiteLLM Proxy
- **Pros**: Open source, multi-provider, similar to OpenRouter
- **Cons**: Requires self-hosting, less mature cost tracking, no Activity API
- **Why rejected**: OpenRouter's managed service is more reliable and has better billing UX.

### Alternative 5: Per-Agent API Keys (Direct to Providers)
- **Pros**: Maximum flexibility, no middleman
- **Cons**: Manage 11 separate API keys, manual cost aggregation, no unified fallback
- **Why rejected**: Operational nightmare. Rotating 11 keys, aggregating 11 invoices, etc.

## Consequences

### Positive

- **Cost transparency**: Activity API shows exact spend per agent, model, and day
- **Budget optimization**: Can see "Backend costs $50/month, QA costs $5/month" and adjust models
- **Automatic fallback**: If Anthropic is down, OpenRouter routes to GPT-4 (if enabled)
- **Prompt caching**: OpenRouter caches agent instructions, reducing repeat costs
- **Export reports**: CSV/PDF exports for monthly financial reporting (Finanzas Agent)
- **77% cost reduction**: Mixed model strategy vs. all-Opus
- **Easy model changes**: Update `kn.toml` and `.opencode/opencode.json`, no code changes

### Negative

- **Additional hop**: Adds ~50-100ms latency vs. direct Anthropic API (acceptable for async workflows)
- **Vendor dependency**: If OpenRouter shuts down, we'd need to migrate (mitigated by using OpenAI-compatible API)
- **Cost variability**: OpenRouter pricing can change (rare, but possible)
- **Privacy considerations**: Requests go through OpenRouter's servers (they claim zero data retention)

### Risks

- **OpenRouter outage**: If OpenRouter is down, all agents blocked
  - **Mitigation**: Configure fallback to direct Anthropic API in `.env` (e.g., `FALLBACK_ANTHROPIC_KEY`)
- **Pricing changes**: OpenRouter could increase margins
  - **Mitigation**: Monitor costs monthly via Finanzas Agent, be ready to switch to LiteLLM if needed
- **Model availability**: Provider might remove a model
  - **Mitigation**: OpenRouter supports multiple providers per model (e.g., Claude via Anthropic or AWS Bedrock)
- **Cost attribution failure**: If `user` parameter isn't passed, costs aren't attributed
  - **Mitigation**: Enforce `user` parameter in OpenCode configuration validation

## References

- [AGENT_MODEL_STRATEGY.md](../AGENT_MODEL_STRATEGY.md) - Detailed model selection rationale
- [kn.toml](../../kn.toml) - Agent model configuration
- [.opencode/opencode.json](../../.opencode/opencode.json) - OpenCode integration
- [Commit a827218](https://github.com/kobogithub/knowledge/commit/a827218) - Migration to OpenRouter
- [Commit 76b6f69](https://github.com/kobogithub/knowledge/commit/76b6f69) - Differentiated model strategy
- [OpenRouter Activity API](https://openrouter.ai/docs#activity) - Cost tracking documentation
