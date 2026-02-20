# Agent Model Strategy

This document explains the differentiated model strategy for each agent in the knowledge project.

## 🎯 Model Selection Rationale

We use different Claude models optimized for each agent's specific workload:

### Claude Opus 4 - Maximum Reasoning
**Used by:** Planner, Backend
**Characteristics:**
- Highest reasoning capability
- Best for complex decision-making
- Ideal for architecture and strategic planning
- Higher cost, justified by task complexity

**Why:**
- **Planner**: Requires deep reasoning for task decomposition, dependency analysis, and strategic coordination
- **Backend**: Complex architecture decisions, database schema design, security patterns, and business logic

### Claude Sonnet 4.5 - Balanced Performance
**Used by:** Frontend, Rust, DevOps
**Characteristics:**
- Excellent balance of performance and cost
- Strong code generation capabilities
- Good for most development tasks
- Fast response times

**Why:**
- **Frontend**: UI/UX implementation, component development, client-side logic
- **Rust**: Systems programming, CLI tools, type-safe development
- **DevOps**: Infrastructure configuration, pipeline optimization, deployment automation

### Claude Haiku 4 - Speed & Efficiency
**Used by:** QA
**Characteristics:**
- Fastest response times
- Most cost-effective
- Excellent for repetitive tasks
- Good for test generation

**Why:**
- **QA**: Automated test generation, test execution, validation scripts, repetitive quality checks

## 📊 Model Distribution

| Agent    | Model                           | Complexity | Use Case                          |
|----------|---------------------------------|------------|-----------------------------------|
| Planner  | github-copilot/claude-opus-4    | High       | Strategic planning & coordination |
| Backend  | github-copilot/claude-opus-4    | High       | Architecture & business logic     |
| Frontend | github-copilot/claude-sonnet-4.5| Medium     | UI/UX implementation              |
| Rust     | github-copilot/claude-sonnet-4.5| Medium     | Systems programming               |
| DevOps   | github-copilot/claude-sonnet-4.5| Medium     | Infrastructure & CI/CD            |
| QA       | github-copilot/claude-haiku-4   | Low        | Test automation                   |

## 🔧 Configuration Files

The model configuration is defined in three places:

1. **AGENTS.md** (Source of truth)
   ```yaml
   ---
   name: backend
   model: github-copilot/claude-opus-4
   reasoning: Complex architecture decisions...
   ---
   ```

2. **kn.toml** (Project configuration)
   ```toml
   [agents.backend]
   model = "github-copilot/claude-opus-4"
   ```

3. **.opencode/opencode.json** (OpenCode integration)
   ```json
   {
     "agent": {
       "backend": {
         "model": "github-copilot/claude-opus-4"
       }
     }
   }
   ```

## 💰 Cost Optimization

Estimated cost savings compared to using Opus for all agents: **~60%**

- Opus (2 agents): High-value strategic and architecture work
- Sonnet (3 agents): Bulk of development work
- Haiku (1 agent): High-volume, repetitive tasks

## 🔄 Adjusting Models

To change a model for an agent:

1. Update `agents/{agent}/AGENTS.md` frontmatter
2. Update `kn.toml` agent configuration
3. Update `.opencode/opencode.json` (if using OpenCode)
4. Commit and push changes

Example:
```bash
# If Frontend needs more complex reasoning
vim agents/frontend/AGENTS.md  # Change to opus-4
vim kn.toml                    # Update model
git commit -am "feat: Upgrade Frontend to Opus for complex UI work"
```

## 📈 Monitoring & Iteration

We should track:
- **Quality**: Does the model produce correct, high-quality output?
- **Speed**: Is response time acceptable for the workflow?
- **Cost**: Is the model cost-justified for the task?

Consider upgrading/downgrading models based on:
- Task complexity changes
- Quality requirements
- Budget constraints
- Performance needs

## 🎓 Best Practices

1. **Start conservative**: Use Haiku by default, upgrade only when needed
2. **Measure impact**: Track quality differences between models
3. **Document reasoning**: Always explain why a specific model was chosen
4. **Review periodically**: Re-evaluate model choices as Claude evolves
5. **A/B test**: Try different models for same task and compare results

---

**Last updated:** 2026-02-20
**Next review:** When new Claude models are released or significant task changes occur
