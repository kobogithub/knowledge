---
description: "Task list for 003-absorb-framework-ia"
---

# Tasks: Absorb framework_ia

**Input**: Design documents from `specs/003-absorb-framework-ia/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: No automated test tasks. The spec requested none and this feature adds no executable code beyond one shell script. Verification runs through the numbered checks in [quickstart.md](./quickstart.md) (referenced below as V1–V10), which are the acceptance gate.

**Organization**: Grouped by user story so each can be implemented, verified and merged independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on an incomplete task)
- **[Story]**: US1–US5, mapping to the spec's user stories
- Every task names its exact file path

## Path Conventions

Content work under `skills/`, `agents/`, `stacks/`, plus one script and one ADR. **Nothing under `cli/` may be edited** (FR-021) — this is load-bearing, not decorative: `cli/tests/stack_integration.rs:151` hardcodes `supabase-postgres-best-practices`, which is why research R3 rejected splitting that skill.

The donor is referenced as `$DONOR` = `/Users/kobo/Github/personal/framework_ia`.

## Assignment by role

| Phase | Story | Owner |
|---|---|---|
| 1 | Setup | QA Agent (`knowledge-pu1`) |
| 3 | US1 — Merge deeper versions | Docs Writer (`knowledge-doc`), with Backend (`knowledge-vlf`) reviewing the FastAPI/Supabase merges |
| 4 | US2 — Import vetted skills | Docs Writer |
| 5 | US3 — Repair the graph | Planner (`knowledge-x6e`) for wiring, DevOps (`knowledge-w5p`) for the check script |
| 6 | US4 — Stack presets | Planner |
| 7 | US5 — Retire the donor | DevOps + Security (`knowledge-s3c`) for the credential |
| 8 | Polish | Planner (ADR) + QA (final sweep) |

---

## Phase 1: Setup

- [ ] T001 [P] Confirm the donor is reachable and unchanged: `git -C $DONOR log --oneline -1` returns `dfa78fd`, and `ls $DONOR/skills | wc -l` returns 18
- [ ] T002 [P] Record the baseline for later comparison: `ls skills/ | wc -l` returns 21, and the graph check reports **6 dangling references and 5 orphans** (quickstart V3's inline form). This is the "before" that US3 must drive to zero
- [ ] T003 [P] Confirm the pre-existing quality gates are green before any change (quickstart V10). Any failure here is pre-existing — report it, do not absorb it into this feature

---

## Phase 2: Foundational

**None required.** US1 and US2 both write under `skills/` but never to the same file — US1 edits six existing skills, US2 creates eight new directories. US3 depends on both, which the Dependencies section sequences rather than blocking here.

---

## Phase 3: User Story 1 — One catalog, the best version of each topic (Priority: P1) 🎯 MVP

**Goal**: For every topic covered by both repositories, the catalog holds one skill that is no less complete than the better original.

**Independent test**: quickstart V5 reports no missing donor heading except the one recorded drop, and V6 finds no duplicated guidance.

> **The failure mode here is invisible.** A merged file can be longer than both originals and still have lost a section. Do not use line count as evidence — V5 compares at heading level, which is why it exists.

- [ ] T004 [US1] Merge `$DONOR/skills/fastapi/SKILL.md` (769 lines) into `skills/fastapi-best-practices/SKILL.md` (152 lines). This is effectively a replacement — the existing skill is a stub. Keep the catalog's frontmatter (`name: fastapi-best-practices`) and its existing guidance where it does not conflict
- [ ] T005 [P] [US1] Merge `$DONOR/skills/github-actions/SKILL.md` (1072) into `skills/github-actions-best-practices/SKILL.md` (763)
- [ ] T006 [P] [US1] Merge `$DONOR/skills/docker/SKILL.md` (948) into `skills/docker-best-practices/SKILL.md` (582)
- [ ] T007 [P] [US1] Merge `$DONOR/skills/astro/SKILL.md` (616) into `skills/astro-best-practices/SKILL.md` (467)
- [ ] T008 [US1] Merge `$DONOR/skills/playwright/SKILL.md` (1017) into `skills/uiux-playwright/SKILL.md` (409). The donor version is a superset — it adds Playwright Test for CI, accessibility testing, debugging patterns and the "MCP for exploration, Playwright Test for CI" split. Keep the `uiux-playwright` name (research R4)
- [ ] T009 [US1] Merge **both** `$DONOR/skills/supabase/SKILL.md` (988, platform: CLI, local dev, RLS, Auth, Storage, Realtime, Edge Functions) **and** `$DONOR/skills/postgresql/SKILL.md` (747, engine: naming, indexing, JSONB, arrays, triggers, transactions, performance) into the single `skills/supabase-postgres-best-practices/SKILL.md`. Organise with clear top-level sections so platform and engine material stay separately navigable — the result is the catalog's largest skill at roughly 1,400 lines
- [ ] T010 [US1] While merging T009, **drop** the donor `postgresql` skill's "Migrations (Alembic/SQLAlchemy)" section. `alembic` was rejected (research R1), and keeping the section would recommend a tool the catalog says is unused while contradicting the Supabase CLI migration guidance in the same file (research R5, FR-003). This is the single recorded content drop required by FR-005
- [ ] T011 [US1] Resolve the object-storage overlap in T009's result: the Supabase skill owns Supabase-specific Storage APIs; generic S3/MinIO patterns belong to `object-storage-best-practices` (created in US2). Cross-link rather than duplicate (research R4)
- [ ] T012 [US1] Verify quickstart V5 — heading-level comparison for all seven donor→catalog pairs. Every missing heading must either be merged in or added to the recorded-drops list. Read each reported heading before accepting it as a drop; headings may legitimately have been reworded
- [ ] T013 [US1] Verify quickstart V6 — no duplicated guidance across the three resolved overlaps. **Requires manual judgement**: automation finds duplication but not contradiction. Where both sources advised on the same task, confirm the result states a single recommendation (FR-003)

**Checkpoint**: every shared topic has one skill, no less complete than the better original. `fastapi-best-practices` is no longer a stub.

---

## Phase 4: User Story 2 — Only skills that match the maintainer's real work (Priority: P2)

**Goal**: Eight vetted skills enter the catalog, each renamed to its conventions; nothing enters without a recorded decision.

**Independent test**: quickstart V1, V2 and V7 pass — 29 skills, every directory name matching its frontmatter, and nine recorded decisions.

> Every import must be **renamed in two places**: the directory and the frontmatter `name` field. The donor's names all differ (`tailwind-astro`, `testing-qa`, `redis-caching`, `bun-runtime`, `observability-logging`, `context7-documentation`, `engram-memory`). Copying without renaming is the most likely slip, and quickstart V2 exists to catch it.

- [ ] T014 [P] [US2] Import `$DONOR/skills/tailwind/` → `skills/tailwind-best-practices/`, setting frontmatter `name: tailwind-best-practices`
- [ ] T015 [P] [US2] Import `$DONOR/skills/testing/` → `skills/testing-best-practices/`, setting `name: testing-best-practices`
- [ ] T016 [P] [US2] Import `$DONOR/skills/observability/` → `skills/observability-best-practices/`, setting `name: observability-best-practices`
- [ ] T017 [P] [US2] Import `$DONOR/skills/redis/` → `skills/redis-best-practices/`, setting `name: redis-best-practices`
- [ ] T018 [P] [US2] Import `$DONOR/skills/object-storage/` → `skills/object-storage-best-practices/`, setting `name: object-storage-best-practices`
- [ ] T019 [P] [US2] Import `$DONOR/skills/bun/` → `skills/bun-best-practices/`, setting `name: bun-best-practices`
- [ ] T020 [P] [US2] Import `$DONOR/skills/engram/` → `skills/engram-memory/`, keeping `name: engram-memory`. Deliberately outside the `-best-practices` family: it documents how to drive a tool, like `documentation-guide` and `standard-commits` (research R2)
- [ ] T021 [P] [US2] Import `$DONOR/skills/context7/` → `skills/context7-docs/`, setting `name: context7-docs`. Same reasoning as T020
- [ ] T022 [US2] In `skills/testing-best-practices/SKILL.md`, replace any Playwright **setup** instructions with a cross-reference to `uiux-playwright`. The E2E section stays; the setup must live in one place or the two will drift (research R4)
- [ ] T023 [US2] Confirm `alembic`, `framework-init` and `beads` were **not** imported (quickstart V1). `alembic` was rejected for having zero catalog footprint; the other two are discarded by FR-008
- [ ] T024 [US2] Verify quickstart V2 — every skill directory name equals its frontmatter `name`. This catches an unrenamed import, which would load under a name its own document denies
- [ ] T025 [US2] Verify quickstart V7 — the vetting record in [research.md](./research.md) covers all nine candidates with a reason each, so the catalog's growth from 21 to 29 is fully accounted for (FR-009)

**Checkpoint**: catalog holds 29 skills, every one accounted for by a recorded decision.

---

## Phase 5: User Story 3 — Agents and skills that actually match (Priority: P3)

**Goal**: Every referenced skill exists, and every skill is reachable from an agent or preset.

**Independent test**: `scripts/check-skill-graph.sh` exits zero (quickstart V3).

> **Must run after US1 and US2.** Wiring agents before the catalog is final would point them at names about to change, and would report the eight imports as orphans.

- [ ] T026 [P] [US3] In `agents/backend/AGENTS.md`, remove the dangling `recommended_skills` entries `kubernetes-best-practices` and `terraform-best-practices`. These were deleted by initiative 001 as outside the maintainer's stacks — the repair is removal, not recreation
- [ ] T027 [P] [US3] In `agents/devops/AGENTS.md`, remove dangling `terraform-best-practices` (required) and `aws-best-practices`, `kubernetes-best-practices` (recommended)
- [ ] T028 [US3] In `agents/biz/AGENTS.md`, remove the dangling required `notion-reporting-standard`. **This leaves `biz` with zero required skills** — either give it a valid one or record explicitly that the role legitimately requires none. Do not leave the question unanswered
- [ ] T029 [US3] Resolve the five orphans by assigning each to the agent that owns its subject: `fastapi-best-practices` → `backend`; `go-best-practices` → a Go-capable role; `htmx-best-practices` → `frontend`; `railway-best-practices` → `devops`; `standard-commits` → `planner`
- [ ] T030 [US3] **`agents/planner/AGENTS.md` declares no skills at all** — neither required nor recommended. That is why `standard-commits` is orphaned: the role that most obviously owns it has an empty list. Populate it with at least `standard-commits`, `engram-memory` and `context7-docs`
- [ ] T031 [P] [US3] Wire the newly imported technology skills to their owning agents so none arrives orphaned (FR-011): `tailwind-best-practices` and `bun-best-practices` → `frontend`; `testing-best-practices` → `qa`; `observability-best-practices`, `redis-best-practices`, `object-storage-best-practices` → `backend` and/or `devops`
- [ ] T032 [US3] **DevOps**: Write `scripts/check-skill-graph.sh` reporting both directions — every referenced skill exists, and every skill is referenced by at least one agent or preset. Exit non-zero on either violation. This is FR-013's repeatable check: the defect being repaired went undetected from initiative 001 until this feature's analysis found it by inspection
- [ ] T033 [US3] Verify quickstart V3 — the script reports zero dangling and zero orphans, down from 6 and 5

**Checkpoint**: the reference graph holds in both directions, and a check exists to keep it that way.

---

## Phase 6: User Story 4 — Stack presets that use the enriched catalog (Priority: P4)

**Goal**: Each preset resolves completely and includes the newly accepted skills relevant to its stack.

**Independent test**: quickstart V4 — all five presets resolve with zero missing skills.

- [ ] T034 [P] [US4] In `stacks/web-astro.toml`, add `tailwind-best-practices`, `testing-best-practices`, `bun-best-practices` to the existing six (astro, htmx, supabase-postgres, railway, github-actions, docker) → 9
- [ ] T035 [P] [US4] In `stacks/api-fastapi.toml`, add `testing-best-practices`, `observability-best-practices`, `redis-best-practices`, `object-storage-best-practices` to the existing six → 10
- [ ] T036 [P] [US4] In `stacks/data-py.toml`, add `testing-best-practices`, `observability-best-practices` to the existing three (python, supabase-postgres, docker) → 5
- [ ] T037 [P] [US4] In `stacks/cli-rust.toml`, add `testing-best-practices` to the existing four → 5
- [ ] T038 [P] [US4] In `stacks/cli-go.toml`, add `testing-best-practices` to the existing four → 5
- [ ] T039 [US4] Confirm no preset references `engram-memory` or `context7-docs` — these are agent tooling, not project technology, and belong on agents only (research R8). A preset describes what a *project* is built from
- [ ] T040 [US4] Verify quickstart V4 — `kn stack show` reports no missing skill for all five presets, and `cargo test --manifest-path cli/Cargo.toml` still passes with `cli/tests/` untouched

**Checkpoint**: presets deliver the enriched catalog.

---

## Phase 7: User Story 5 — framework_ia retired without leaving a live secret (Priority: P5)

**Goal**: The donor carries no plaintext credential and announces its own retirement.

**Independent test**: quickstart V9 passes.

> This phase touches only the donor repository. It is independent of US1–US4 and may be done at any point.

- [ ] T041 [P] [US5] **Security**: In `$DONOR/mcps/opencode.json:17`, replace the plaintext `CONTEXT7_API_KEY` value with an environment variable reference
- [ ] T042 [P] [US5] **Security**: Same replacement in `$DONOR/.opencode/opencode.json:17`
- [ ] T043 [P] [US5] **Security**: Same replacement in `$DONOR/template/opencode.json.template`, so new projects scaffolded from the template never receive the literal key
- [ ] T044 [US5] Make the required environment variable evident in the donor's configuration, so someone using the repo after the change knows what to supply rather than hitting a silent failure (FR-018)
- [ ] T045 [US5] Add a retirement notice to the top of `$DONOR/README.md`: the repository is retired, `knowledge` is the successor, and these were deliberately left behind — `alembic`, `framework-init`, `beads`, the donor's agent definitions, `framework-init.sh`, and the beads workflow (FR-019, FR-020)
- [ ] T046 [US5] In that same notice, state plainly that **the Context7 key remains in commits `3a2a39f` and `4593dd2` and still requires rotation at the provider**. Removing it from the working tree stops propagation; it does not invalidate the credential. FR-020 requires this so the work is not mistaken for remediation
- [ ] T047 [US5] Verify quickstart V9 — no `ctx7sk-` in the donor working tree, the config reads from an environment variable, the README reads as retired within the first screen, and `git log -S'ctx7sk-'` still shows both commits **unchanged** (FR-024)

**Checkpoint**: the donor is safe to leave alone and impossible to mistake for current.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [ ] T048 Write `docs/adr/008-absorb-framework-ia.md` recording the one-directional import, the three incompatibilities that ruled out a bidirectional merge (agent frontmatter schema, beads vs spec-kit per ADR-006, model namespaces), and what was deliberately not taken. FR-020 requires the record of what was left behind to survive beyond an initiative folder
- [ ] T049 [P] Add the eight new skills to `docs/README.md`'s index if it lists skills individually, so the catalog stays discoverable. **Requires initiative 002 to have merged first** — `docs/README.md` is created there, and this branch was cut from `dev` before it landed. Do not create a competing file here
- [ ] T050 [P] Add a `CHANGELOG.md` entry under `[Unreleased]` covering the import, the merges, the rejected candidate and the graph repair. Add only — do not edit existing entries. Initiative 002 also writes to `[Unreleased]`; if it has already merged, append below its entry rather than replacing the section
- [ ] T051 Verify quickstart V8 — nothing out of scope was touched: `git diff --stat dev...HEAD -- cli/` is empty, `skills/` contains no `beads` or `framework-init`, and `agents/` gained no file (FR-021, FR-022)
- [ ] T052 Verify quickstart V10 — existing quality gates still pass, unchanged from the T003 baseline, with `scripts/check-skill-graph.sh` included in the shellcheck sweep
- [ ] T053 Final sweep: run quickstart V1–V10 and record the outcome of each, including anything recorded as unverified with its reason

---

## Dependencies

```text
Phase 1 (Setup: T001–T003)
   │
   ├─▶ Phase 3 US1 — merge      (T004–T013) ──┐
   │      T004–T011 ─▶ T012–T013              │
   │                                          ├─▶ Phase 5 US3 (T026–T033)
   ├─▶ Phase 4 US2 — import     (T014–T025) ──┘        │
   │      T014–T021 [P] ─▶ T022–T025                   │
   │                                                    ├─▶ Phase 6 US4 (T034–T040)
   └─▶ Phase 7 US5 — donor      (T041–T047)             │
          independent of everything else                │
                                              Phase 8 (T048–T053) ─▶ last
```

**Cross-story constraints**:

- **US3 after US1 and US2** — the graph repair asserts the catalog is final. Run earlier and it wires agents to names about to change, and reports the eight imports as orphans.
- **US4 after US2** — a preset naming a skill that was rejected is a defect; the final skill set must exist first.
- **T011 (US1) depends on T018 (US2)** — resolving the object-storage overlap needs `object-storage-best-practices` to exist. This is the only cross-story task dependency; sequence T018 early if US1 and US2 run in parallel.
- **T022 (US2) depends on T008 (US1)** — cross-referencing `uiux-playwright` from `testing-best-practices` needs the merged version in place.
- **US5 is fully independent** and touches only the donor.

---

## Parallel execution examples

**Within US2** — eight imports, eight new directories, no interdependency:

```text
T014 tailwind   T015 testing   T016 observability   T017 redis
T018 object-storage   T019 bun   T020 engram   T021 context7
```

**Within US1** — after T004 sets the pattern, four merges touch four different files:

```text
T005 github-actions   T006 docker   T007 astro
(T008 playwright and T009 supabase+postgresql are larger; sequence them)
```

**Within US4** — five presets, five files:

```text
T034 web-astro   T035 api-fastapi   T036 data-py   T037 cli-rust   T038 cli-go
```

**Across stories** — three agents on separate branches:

```text
Docs Writer  ─▶ US1 (merges)
Docs Writer  ─▶ US2 (imports)
DevOps       ─▶ US5 (donor retirement — fully independent)
```

---

## Implementation strategy

**MVP = User Story 1 alone**, with one caveat: T011 (resolving the object-storage overlap) needs `object-storage-best-practices` from US2, so shipping US1 by itself leaves that one task open. Everything else in US1 stands alone. Merging the seven donor skills already delivers the substance: the catalog stops being the thinner of two sources on every shared topic, and `fastapi-best-practices` stops being a 152-line stub against a 769-line alternative.

**Recommended increments**:

1. **US1** → merge. The catalog is now the best available on shared topics.
2. **US2** → merge. Eight vetted skills, catalog at 29.
3. **US3** → merge. The graph holds, and a check keeps it holding.
4. **US4** + **US5** + Phase 8 → merge. Presets, donor retirement, ADR.

Each increment is a separate PR into `epic/003-absorb-framework-ia`, which then targets `dev`.

**What to watch**: US1's failure mode leaves no trace. A merged file that grew by 300 lines while losing a 40-line section looks like a success by every cheap measure. T012's heading-level comparison is the only check that catches it — treat a reported missing heading as a finding to investigate, not noise to dismiss.

---

## Task summary

| Phase | Story | Tasks | Count |
|---|---|---|---|
| 1 | Setup | T001–T003 | 3 |
| 2 | Foundational | — | 0 |
| 3 | US1 — Merge (P1) | T004–T013 | 10 |
| 4 | US2 — Import (P2) | T014–T025 | 12 |
| 5 | US3 — Graph repair (P3) | T026–T033 | 8 |
| 6 | US4 — Presets (P4) | T034–T040 | 7 |
| 7 | US5 — Donor retirement (P5) | T041–T047 | 7 |
| 8 | Polish | T048–T053 | 6 |
| | **Total** | | **53** |
