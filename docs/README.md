# Documentation

Everything beyond the [README](../README.md). If you are new to `kn`, start with [Getting Started](./GETTING_STARTED.md).

## Using kn

| Document | What it covers |
|---|---|
| [Getting Started](./GETTING_STARTED.md) | First steps after installing `kn` |
| [Environment Variables](./ENVIRONMENT_VARIABLES.md) | Every variable `kn` reads, and how `kn sync` expands them |
| [Architecture](./ARCHITECTURE.md) | How the framework fits together |

## Agents and MCP

| Document | What it covers |
|---|---|
| [MCP Architecture](./MCP_ARCHITECTURE.md) | How MCP servers are managed |
| [Agent Model Strategy](./AGENT_MODEL_STRATEGY.md) | Which model each agent role uses, and why |
| [Gemini MCP Config](./GEMINI_MCP_CONFIG.md) | MCP setup for Gemini Code Assist |

## Release

| Document | What it covers |
|---|---|
| [Release Process](./release/RELEASE.md) | Cutting a release, tagging, and publishing artifacts |
| [GitHub Pages](./release/GITHUB_PAGES.md) | The install redirect, the retired custom domain, and its planned replacement |

## Security

| Document | What it covers |
|---|---|
| [Security Audit Report](./security/SECURITY_AUDIT_REPORT.md) | Findings and remediation |

## Decisions and reports

- **[Architecture Decision Records](./adr/README.md)** — why the project is built the way it is. Start with [ADR-001](./adr/001-rust-cli-tool.md) for the choice of Rust, and [ADR-006](./adr/006-adopt-speckit-remove-beads.md) for the move from Beads to spec-kit.
- **[Stakeholder Reports](./reports/README.md)** — periodic status summaries.

## Product layer and status

- **[Product layer](./product/README.md)** — what comes *before* a spec: discovery input, `PROJECT.md`, `PRD.md` with epics, and user stories with Gherkin, each with a sign-off gate.
- **[STATUS.md](./STATUS.md)** — the PM view: stories per sprint, specs without issues, issues without PRs, open risks and questions.

## Initiatives

Feature work is specified before it is built. An initiative starts as an epic in [`docs/product/PRD.md`](./product/PRD.md) and, once signed, lives in [`specs/`](../specs/) as a `spec.md`, `plan.md` and `tasks.md`. The workflow is documented in [AGENTS.md](../AGENTS.md).
