# Implementation Plan: Absorb framework_ia

**Branch**: `epic/003-absorb-framework-ia` | **Date**: 2026-08-06 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/003-absorb-framework-ia/spec.md`

## Summary

Absorb the material worth keeping from the private `framework_ia` repository into `knowledge`, then retire the donor. Eight skills with no counterpart are imported and renamed to the catalog's conventions; six existing skills are enriched from seven deeper donor versions; one candidate (`alembic`) is rejected and two (`framework-init`, `beads`) are discarded outright. The donor's beads workflow, agent schema and shell initializer are not adopted.

The same pass repairs the agent↔skill graph that initiative 001 left inconsistent — four agent references point at deleted skills and five skills are referenced by nobody — and adds a repeatable check so the defect cannot recur silently. Stack presets are extended to use the enriched catalog.

Content work dominates: no Rust is touched, and the largest risk is not breakage but *silent loss* — a merge that drops a topic while looking longer. The plan counters that with heading-level verification rather than line counts.

## Technical Context

**Language/Version**: Markdown (skill documents), TOML (stack presets), YAML frontmatter (skills and agents). Untouched: Rust 2021 / `kn` v0.10.0.

**Primary Dependencies**: None added. The donor and the catalog already share an identical `SKILL.md` format (`name`, `description`, `version`, `tags`), which is what makes this an import rather than a format migration.

**Storage**: N/A

**Testing**: Existing `ci.yml` (`cargo fmt`, `clippy -D warnings`, `cargo build`, `cargo test`, `shellcheck`). Plus a new agent↔skill consistency script (FR-013) which is itself the regression gate for the graph repair. `cli/tests/stack_integration.rs` exercises preset resolution and must keep passing untouched.

**Target Platform**: The `kn` skill catalog as consumed by `kn init`, `kn skills`, `kn stack` and the agent definitions.

**Project Type**: CLI tool with a bundled knowledge catalog

**Performance Goals**: N/A

**Constraints**:
- CLI behaviour unchanged; nothing under `cli/` is edited (FR-021). This is load-bearing — it is why R3 rejects splitting the datastore skill.
- Stack presets and skill catalog change only as this feature specifies (FR-022 governs the donor's *other* material, not the catalog itself).
- The donor's beads workflow, agent definitions and `framework-init.sh` are not adopted (FR-022).
- The donor's git history is not rewritten (FR-024).

**Scale/Scope**: 8 skills imported (~5,300 lines), 6 skills merged from 7 donor sources (~5,000 donor lines against ~3,100 existing), 3 agents repaired, ~9 agents/presets rewired, 5 presets extended, 3 donor files de-secreted. Catalog: 21 → 29 skills.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Status: PASS (vacuously) — no constraints to check.**

`.specify/memory/constitution.md` remains the unmodified spec-kit template; every principle is a `[PRINCIPLE_N_NAME]` placeholder. The project has never run `/speckit-constitution`, so there are no ratified principles this feature can violate.

Recorded rather than skipped, and worth noting for this feature specifically: a ratified constitution would very likely constrain *catalog growth* and *what qualifies for inclusion* — exactly the judgement R1 had to make from evidence instead. If the constitution is ever filled, R1's vetting criteria are a candidate for promotion into it.

**Post-Phase 1 re-check**: unchanged. No new runtime dependency, no new project structure, no new abstraction.

## Project Structure

### Documentation (this feature)

```text
specs/003-absorb-framework-ia/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 — 9 decisions, including the full vetting record
├── data-model.md        # Phase 1 — catalog inventory and the graph being repaired
├── quickstart.md        # Phase 1 — verification guide
├── contracts/
│   └── catalog-surface.md   # Phase 1 — what consumers of the catalog may rely on
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2 (/speckit-tasks — NOT created here)
```

### Source Code (repository root)

End state. `←` new, `✎` edited, `✗` not imported.

```text
knowledge/
├── skills/                                   21 → 29
│   ├── tailwind-best-practices/          ←   from donor tailwind (1136)
│   ├── testing-best-practices/           ←   from donor testing (1026)
│   ├── observability-best-practices/     ←   from donor observability (1175)
│   ├── redis-best-practices/             ←   from donor redis (556)
│   ├── object-storage-best-practices/    ←   from donor object-storage (754)
│   ├── bun-best-practices/               ←   from donor bun (734)
│   ├── engram-memory/                    ←   from donor engram (549)
│   ├── context7-docs/                    ←   from donor context7 (306)
│   │
│   ├── astro-best-practices/             ✎   + donor astro (616 vs 467)
│   ├── docker-best-practices/            ✎   + donor docker (948 vs 582)
│   ├── fastapi-best-practices/           ✎   + donor fastapi (769 vs 152 — a stub today)
│   ├── github-actions-best-practices/    ✎   + donor github-actions (1072 vs 763)
│   ├── supabase-postgres-best-practices/ ✎   + donor supabase (988) AND postgresql (747)
│   ├── uiux-playwright/                  ✎   + donor playwright (1017 vs 409)
│   └── (14 others unchanged)
│
├── agents/
│   ├── backend/AGENTS.md                 ✎   drop 2 dangling refs, wire new skills
│   ├── devops/AGENTS.md                  ✎   drop 3 dangling refs, wire new skills
│   ├── biz/AGENTS.md                     ✎   drop 1 dangling ref
│   ├── frontend/AGENTS.md                ✎   wire tailwind, bun, testing
│   ├── qa/AGENTS.md                      ✎   wire testing
│   ├── planner/AGENTS.md                 ✎   wire standard-commits, engram, context7
│   └── (5 others — wired as the orphan sweep requires)
│
├── stacks/*.toml                         ✎   extend all five per research R8
│
├── scripts/check-skill-graph.sh          ←   FR-013 repeatable consistency check
│
├── docs/adr/008-absorb-framework-ia.md   ←   records the import and what was left behind
│
└── cli/  install.sh  kn.toml             (untouched — FR-021)

framework_ia/                             (donor, retired)
├── mcps/opencode.json                    ✎   key → environment variable
├── .opencode/opencode.json               ✎   same
├── template/opencode.json.template       ✎   same
└── README.md                             ✎   retirement notice pointing to knowledge
```

**Structure Decision**: Imported skills become directories under `skills/` named `<topic>-best-practices`, matching the 11 existing skills that already use that form; `engram-memory` and `context7-docs` stay outside that family because they document how to drive a tool rather than practices for a technology (research R2). The datastore topic keeps its single existing skill rather than splitting, because `cli/tests/stack_integration.rs:151` hardcodes the name and `cli/` is out of scope (research R3).

## Implementation Phases

| Phase | Story | Work | Risk |
|---|---|---|---|
| 1 | US1 (P1) | Merge the 7 donor skills into 6 catalog skills | **Silent content loss** |
| 2 | US2 (P2) | Import the 8 accepted skills, renamed; record every decision | Catalog re-inflation |
| 3 | US3 (P3) | Repair dangling refs and orphans; ship the consistency check | Must run **after** 1 and 2 |
| 4 | US4 (P4) | Extend the five stack presets | Preset must resolve |
| 5 | US5 (P5) | De-secret and retire the donor | Reads as remediation when it is not |

**Ordering constraints**:

- **US3 after US1 and US2.** The graph repair asserts that every referenced skill exists and every skill is referenced. Running it before the catalog is final would wire agents to names that are about to change and would report the eight imports as orphans.
- **US1 before US4.** A preset naming a skill mid-merge is fine, but a preset naming a *rejected* skill is a defect; the final skill set must exist first.
- **US5 is independent** and may be done at any point — it touches only the donor repository.

**The one thing to watch**: US1's failure mode is invisible. A merged file can be longer than both originals and still have lost a whole section. Line count proves nothing; the plan verifies at heading level (research R6), and the quickstart automates that comparison.

## Complexity Tracking

> Fill ONLY if Constitution Check has violations that must be justified

No violations — the constitution contains no ratified principles.

Two additions are worth flagging even though no gate forbids them:

| Addition | Why needed | Simpler alternative rejected because |
|---|---|---|
| `scripts/check-skill-graph.sh` | FR-013 requires a repeatable check. The defect being repaired went undetected from initiative 001 until it was found by inspection during this feature's analysis. | A one-time manual repair leaves nothing to catch the next occurrence, and the next occurrence is likely — any future catalog curation can strand agent references again. |
| ADR-008 | The import reverses no earlier decision, but it records what was deliberately **not** taken (`alembic`, `beads`, `framework-init`, the donor's agent schema and workflow). FR-020 requires that record to survive. | Recording it only in `tasks.md` buries it in an initiative folder; ADRs are where "why is it not here?" is answered. |
