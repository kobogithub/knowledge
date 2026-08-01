# ADR-007: Personal Stacks & Curated Skill Catalog

## Status

Accepted

## Date

2026-08-01

## Context

`kn` is a personal meta-framework CLI for the maintainer's own projects (see
[ADR-001](./001-rust-cli-tool.md)). Its skill catalog had grown into a generic set of
`*-best-practices` skills — including technologies the maintainer does not use (AWS,
Jsonnet, Kubernetes, Terraform, Notion reporting) — while lacking the ones they do
(FastAPI, HTMX, Go, Railway).

Separately, starting a new project meant enabling each relevant skill by hand. There was
no notion of "the set of skills I use for an Astro web app" or "for a Rust CLI".

A comparison with the sibling project `taligent_coe_framework` (`tali`) — which models
work as a layered `Core → Blueprint → Stack → Plugin` hierarchy with private registries,
SSO, and white-label distribution — confirmed that `kn` should **not** adopt that
machinery. `kn` is a single-user tool; that complexity is unwarranted (see the recorded
decision "no port taligent architecture").

## Decision

Two changes, both preserving `kn`'s flat, personal model:

1. **Curate the skill catalog** to the maintainer's real stack.
   - Removed: `aws-`, `jsonnet-`, `kubernetes-`, `terraform-best-practices`,
     `notion-reporting-standard`.
   - Added: `fastapi-`, `htmx-`, `go-`, `railway-best-practices`.
   - Retained: rust, python, docker, github-actions, bash, supabase-postgres,
     standard-commits, documentation-guide, astro, all `security-*` and `uiux-*`.

2. **Introduce lightweight stack-presets.** A preset is a named TOML file
   (`~/.kn/stacks/<name>.toml`) mapping a stack name to an ordered list of catalog
   skills. Presets are staged from the repo `stacks/` tree by `install.sh`, exactly like
   skills/agents/formulas, and are user-editable.
   - `kn init --stack <name>` resolves a preset, verifies its skills exist, enables
     exactly that bundle (deduped with any manual selections), and records
     `stack = "<name>"` under `[project]` in `kn.toml`.
   - `kn stack list` / `kn stack show <name>` list and inspect presets.
   - Shipped presets: `web-astro`, `api-fastapi`, `cli-rust`, `cli-go`, `data-py`.

Explicitly **out of scope** (rejected to keep `kn` simple): layered Core/Blueprint/Stack
hierarchy, private registries, white-label forks, SSO. A "stack" in `kn` is only a named
skill bundle — nothing more.

## Consequences

- **Positive**: The catalog reflects what the maintainer actually builds; new projects
  bootstrap in one step; presets are plain TOML the user can add/edit without touching
  Rust. Backward compatible — `kn.toml` files without a `stack` key load unchanged, and
  `sync` warns (not crashes) when a project references a skill that left the catalog.
- **Negative / trade-offs**: `kn update` only swaps the binary; refreshing the catalog
  (new skills / presets) still requires re-running `install.sh`. Preset skill sets are
  opinionated defaults that may need occasional tuning.
- **Follow-up noted**: `init.rs`/`sync.rs` still stage beads formulas to `.beads/formulas`
  and the generated `AGENTS.md` references `bd`, despite [ADR-006](./006-adopt-speckit-remove-beads.md).
  This pre-existing inconsistency is unrelated to this ADR and is left for a separate
  cleanup.

## References

- Spec: `specs/001-personal-stacks/`
- [ADR-001: Rust CLI tool](./001-rust-cli-tool.md)
- [ADR-004: 3-tier skill distribution](./004-3-tier-skill-distribution.md)
- [ADR-006: Adopt spec-kit, remove beads](./006-adopt-speckit-remove-beads.md)
