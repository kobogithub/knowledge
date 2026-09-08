# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records (ADRs) for the Knowledge project.

## What is an ADR?

An Architecture Decision Record captures an important architectural decision made along with its context and consequences. ADRs are immutable once accepted — if a decision is reversed, a new ADR supersedes the old one.

## ADR Index

| # | Title | Status | Date |
|---|-------|--------|------|
| [001](./001-rust-cli-tool.md) | Rust as Implementation Language for CLI Tool | Accepted | 2024-12-15 |
| [002](./002-multi-agent-architecture.md) | Multi-Agent Architecture with Specialized Roles | Superseded by [006](./006-adopt-speckit-remove-beads.md) | 2025-01-10 |
| [003](./003-beads-issue-tracking.md) | Beads (bd) as Issue Tracking System | Superseded by [006](./006-adopt-speckit-remove-beads.md) | 2024-12-10 |
| [004](./004-3-tier-skill-distribution.md) | 3-Tier Skill Distribution System | Accepted | 2025-01-20 |
| [005](./005-openrouter-llm-provider.md) | OpenRouter as Unified LLM Provider with Differentiated Models | Accepted | 2025-02-15 |
| [006](./006-adopt-speckit-remove-beads.md) | Adopt GitHub Spec Kit, Remove Beads (bd) | Accepted | 2026-07-17 |
| [007](./007-personal-stacks-and-curated-catalog.md) | Personal Stacks & Curated Skill Catalog | Accepted | 2026-08-01 |
| [008](./008-product-layer-over-speckit.md) | A Product Layer Above spec-kit, With Signature Gates | Accepted | 2026-09-06 |
| [009](./009-remove-beads-from-the-product.md) | Remove Beads From the Product, Not Just the Workflow | Accepted | 2026-09-06 |

<!-- Add new ADRs above this line -->

## ADR Lifecycle

```
Proposed → Accepted → [Deprecated | Superseded by ADR-NNN]
```

- **Proposed**: Under discussion, not yet decided
- **Accepted**: Decision made and in effect
- **Deprecated**: No longer relevant (technology removed, feature sunset)
- **Superseded**: Replaced by a newer ADR (always reference the new one)

## Creating a New ADR

1. Copy the template: `cp docs/adr/000-template.md docs/adr/NNN-title.md`
2. Fill in all sections
3. Update this index
4. Create a PR or commit with: `docs(adr): add ADR-NNN title`

## Template

See [000-template.md](./000-template.md) for the standard ADR template.

## References

- [ADR GitHub Organization](https://adr.github.io/)
- [Michael Nygard's original article](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- `specs/NNN-feature-name/` initiatives — ADRs are often extracted from decisions surfaced during `/speckit-plan` or `/speckit-clarify`
