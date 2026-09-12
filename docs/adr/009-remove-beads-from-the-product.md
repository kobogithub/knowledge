# ADR-009: Remove Beads From the Product, Not Just the Workflow

## Status

Accepted

## Date

2026-09-06

## Context

[ADR-006](./006-adopt-speckit-remove-beads.md) replaced Beads (`bd`) with spec-kit as the
project's workflow. It removed `.beads/`, dropped `bd-best-practices` from `kn.toml` and
every agent definition, and rewrote `AGENTS.md` around the spec-kit cycle.

It stopped at the workflow layer. The **product** kept shipping Beads:

- `kn beads template` — a subcommand in `cli/src/commands/beads.rs`, wired into `main.rs`
- `kn doctor` — checked `bd` as a **required** dependency and `dolt` as optional, so a
  machine without Beads reported a failing dependency count
- `install.sh` — installed `bd` via `cargo install bd`, installing Rust first if needed,
  and **aborted the installation** if it could not

So `kn` obliged every user to install a tool the project itself had stopped using nine
weeks earlier, and failed the install if that tool would not build.

The gap surfaced while sanitising broken references in EPIC-01. The maintainer chose to
retire the documentation first (US-01), which left the docs and the binary deliberately out
of step and recorded as debt. This ADR closes that gap in the other direction.

## Decision

Remove Beads from the product entirely.

1. **Drop the `kn beads` subcommand**: delete `cli/src/commands/beads.rs`, its `mod`
   declaration, and the `Beads` variant and dispatch in `main.rs`. This is a **breaking
   change** to the CLI: a public subcommand disappears.
2. **Drop `bd` and `dolt` from `kn doctor`**, including the `"bd"` install-hint branch.
   The dependency count goes from 7 to 5.
3. **Drop the `bd` install from `install.sh`** and its mentions in `uninstall.sh` and the
   `--skip-deps` help text. Installing `kn` no longer installs, requires, or fails on
   Beads.
4. **Ship it in v0.12.0**, alongside the rest of EPIC-01, flagged as breaking in the
   changelog.

Anyone still using Beads independently installs `bd` themselves; it was never `kn` that
made Beads work, only `kn` that insisted on its presence.

## Alternatives Considered

### Keep the subcommand and re-document it

- Pros: no breaking change; the template generator is small and works.
- Cons: `kn` would keep requiring an unrelated tool at install time, and the docs would
  have to explain a workflow the project abandoned in ADR-006.
- Why rejected: the cost is not the subcommand, it is the mandatory dependency and the
  explanation. A user reading about Beads in `kn`'s docs reasonably concludes the project
  uses Beads.

### Keep the subcommand but make `bd` optional in doctor and the installer

- Pros: no breaking change; the install stops failing.
- Cons: leaves a subcommand nothing documents and nobody in this project uses, which is
  exactly the kind of phantom reference EPIC-01 exists to remove.
- Why rejected: half a removal is the state we are already in, and it is the state that
  produced this ADR.

### Extract `kn beads` into a separate tool

- Pros: preserves it for anyone who wants it.
- Cons: a package to publish and maintain for a generator of five markdown templates, with
  no known users.
- Why rejected: disproportionate. The templates remain in git history if anyone needs them.

## Consequences

### Positive

- Installing `kn` stops installing Rust-plus-Beads for a feature the project does not use,
  and stops aborting when `bd` will not build.
- `kn doctor` reports on dependencies `kn` actually needs — 5, not 7 with one that always
  reads as missing on a clean machine.
- Documentation and binary agree again, closing the debt US-01 opened deliberately.

### Negative

- Breaking change: anyone scripting `kn beads template` loses it in v0.12.0.
- Beads users lose a convenience they may have been relying on, with no replacement
  shipped.

### Risks

- **Someone depends on `kn beads template`.** Mitigation: the project is pre-1.0 with a
  single maintainer and no known external users of that path; the changelog flags it as
  breaking, and v0.11.0 stays downloadable.
- **The install script has CI coverage that assumes `bd`.** Mitigation: run the install
  workflow after the change; it already asserts platform refusal behaviour and will show
  whether the dependency was load-bearing there.

## References

- [ADR-006](./006-adopt-speckit-remove-beads.md) — removed Beads from the workflow; this completes it
- [ADR-008](./008-product-layer-over-speckit.md) — the product layer whose first story surfaced the gap
- [`specs/005-agent-team-projects/tasks.md`](../../specs/005-agent-team-projects/tasks.md) — T041 to T045
- [`docs/product/stories/EPIC-01/US-01.md`](../product/stories/EPIC-01/US-01.md) — the story that retired the documentation first
