# Implementation Plan: Personal Stacks & Curated Catalog

**Branch**: `epic/001-personal-stacks` | **Date**: 2026-08-01 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-personal-stacks/spec.md`

## Summary

Two changes to the `kn` CLI, kept within its existing flat/personal model:

1. **Curate the skill catalog** — remove `aws-best-practices`, `jsonnet-best-practices`,
   `kubernetes-best-practices`, `terraform-best-practices`, `notion-reporting-standard`
   from the repo `skills/` tree (they stop being staged to `~/.kn/skills/`), and add four
   new skills: `fastapi-best-practices`, `htmx-best-practices`, `go-best-practices`,
   `railway-best-practices`, each a standard `<name>/SKILL.md` directory.

2. **Introduce stack-presets** — a preset is a named TOML file mapping a stack name to an
   ordered list of skill names. Presets ship in a new repo `stacks/` tree, are staged to
   `~/.kn/stacks/` by `install.sh` and `kn update`, and are user-editable there. A new
   `kn stack list` command lists them; `kn init --stack <name>` resolves a preset and
   enables exactly its skills. `kn.toml` gains an optional `[project].stack` field.

No new architecture layers, no registries, no white-label — this reuses the same staging
pattern already used for agents/skills/formulas.

## Technical Context

**Language/Version**: Rust 2021/2024 (existing `kn` CLI crate under `cli/`)

**Primary Dependencies**: `clap` (derive subcommands), `serde` + `toml` (config &
preset parsing), `dialoguer` (interactive select in `init`), `colored` (output),
`anyhow` (errors) — all already in use.

**Storage**: Filesystem only. Presets in `~/.kn/stacks/<name>.toml` (staged from repo
`stacks/`). Skills in `~/.kn/skills/<name>/` (staged from repo `skills/`). Project config
in `./kn.toml`.

**Testing**: `cargo test` (unit + integration with `tempfile`), the crate's existing
harness. Preset resolution and curation get unit tests; `init --stack` gets an
integration test driving a temp `KN_HOME`.

**Target Platform**: Linux x86_64, macOS x86_64/ARM64 (existing release matrix).

**Project Type**: Single-project CLI tool.

**Performance Goals**: N/A (local, sub-second operations).

**Constraints**: Must not break existing projects whose `kn.toml` references a removed
skill (warn, don't crash). Preset selection must be idempotent with manual skill flags.

**Scale/Scope**: Single user; ~21 skills post-curation; 5 shipped presets.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

The project constitution (`.specify/memory/constitution.md`) is an unfilled template, so
there are no ratified principles to gate against. Applying the project's documented
working principles instead (AGENTS.md / CLAUDE.md + the recorded decisions):

- **Flat & personal model preserved** — ✅ no layered Core/Blueprint/Stack/Plugin, no
  registries, no white-label. A "stack-preset" here is only a named skill bundle, not
  taligent's blueprint→stack hierarchy.
- **Conventional commits & branch strategy** — ✅ work lands on `epic/001-personal-stacks`
  then agent branches, per CLAUDE.md.
- **No new heavy dependencies** — ✅ reuses `serde`/`toml`/`clap`.

**Gate result: PASS.** No violations; Complexity Tracking left empty.

## Project Structure

### Documentation (this feature)

```text
specs/001-personal-stacks/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── cli-stack.md     # CLI command + preset-file contract
├── checklists/
│   └── requirements.md  # Spec quality checklist (from /speckit-specify)
└── tasks.md             # Phase 2 output (/speckit-tasks — NOT created here)
```

### Source Code (repository root)

```text
skills/                              # curated catalog (repo source of truth)
├── fastapi-best-practices/SKILL.md  # NEW
├── htmx-best-practices/SKILL.md     # NEW
├── go-best-practices/SKILL.md       # NEW
├── railway-best-practices/SKILL.md  # NEW
└── (removed: aws-, jsonnet-, kubernetes-, terraform-best-practices, notion-reporting-standard)

stacks/                              # NEW — shipped preset definitions
├── web-astro.toml
├── api-fastapi.toml
├── cli-rust.toml
├── cli-go.toml
└── data-py.toml

cli/src/
├── main.rs                          # register `Stack` subcommand
├── commands/
│   ├── mod.rs                       # add `pub mod stack;`
│   ├── stack.rs                     # NEW — `kn stack list` (+ show)
│   └── init.rs                      # add `--stack <name>` flag + preset resolution
├── config/
│   └── kn_toml.rs                   # add optional `[project].stack` field
├── core/
│   ├── kn_home.rs                   # add `stacks_dir()` + `list_installed_stacks()`
│   ├── mod.rs                       # re-export new helpers
│   └── stack.rs                     # NEW — StackPreset model + load/resolve
└── models/                          # (StackPreset may live here or in core/stack.rs)

install.sh                           # stage repo stacks/ → ~/.kn/stacks/
cli/src/commands/update.rs           # `kn update` also refreshes ~/.kn/stacks/
```

**Structure Decision**: Single-project CLI. New logic is one command module
(`commands/stack.rs`), one core module (`core/stack.rs`) for the preset model+resolution,
plus additive edits to `init.rs`, `kn_toml.rs`, `kn_home.rs`, `install.sh`, and
`update.rs`. Catalog curation is filesystem changes under `skills/` and matching test
fixtures.

## Complexity Tracking

> No constitution violations; section intentionally empty.
