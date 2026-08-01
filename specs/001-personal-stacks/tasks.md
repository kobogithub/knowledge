# Tasks: Personal Stacks & Curated Catalog

**Feature**: `specs/001-personal-stacks/` | **Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

**Branch strategy**: work lands on `epic/001-personal-stacks`; each role works its own
`001-personal-stacks/<role>` branch (see CLAUDE.md).

**Tests**: Included — the spec's acceptance scenarios and quickstart define concrete
behaviors worth locking with `cargo test`.

**Assignment legend**: → Rust Agent (`knowledge-r5t`) for CLI/Rust; → Docs Writer
(`knowledge-doc`) for SKILL.md authoring; → DevOps (`knowledge-w5p`) for install/update
staging; → QA (`knowledge-pu1`) for test tasks.

---

## Phase 1: Setup

- [ ] T001 Create `epic/001-personal-stacks` branch from `dev` and push it → Rust Agent
- [ ] T002 Create repo `stacks/` directory at project root as the home for shipped preset TOMLs → Rust Agent

---

## Phase 2: Foundational (blocking prerequisites)

**These must complete before US1/US2/US3 — they define the preset model, storage, and catalog changes everything else builds on.**

- [ ] T003 [P] Add `stacks_dir()` (→ `~/.kn/stacks/`) and `list_installed_stacks()` to `cli/src/core/kn_home.rs`, and ensure `ensure_kn_home()` creates `~/.kn/stacks/` → Rust Agent
- [ ] T004 [P] Create `cli/src/core/stack.rs` with the `StackPreset` struct (`name`, `description`, `skills`) plus `load_preset(name)`, `list_presets()`, and `resolve_skills()` that validates each skill exists in `~/.kn/skills/` → Rust Agent
- [ ] T005 Wire `pub mod stack;` and re-exports into `cli/src/core/mod.rs` → Rust Agent
- [ ] T006 Add optional `stack: Option<String>` to `ProjectSection` in `cli/src/config/kn_toml.rs` with `#[serde(default, skip_serializing_if = "Option::is_none")]`, plus a `set_stack()` helper → Rust Agent
- [ ] T007 [P] Unit tests for `StackPreset` parse/validate + `kn.toml` round-trip with and without `stack` field in `cli/src/core/stack.rs` and `cli/src/config/kn_toml.rs` → QA Agent

---

## Phase 3: User Story 1 — Curated catalog reflects my real stack (P1) 🎯 MVP

**Goal**: The catalog only contains the user's real technologies.

**Independent test**: `kn skills list` (or `ls ~/.kn/skills`) shows the 4 new skills, none of the 5 removed ones; every listed skill still installs.

- [ ] T008 [P] [US1] Remove `skills/aws-best-practices/`, `skills/jsonnet-best-practices/`, `skills/kubernetes-best-practices/`, `skills/terraform-best-practices/`, `skills/notion-reporting-standard/` from the repo → Rust Agent
- [ ] T009 [P] [US1] Author `skills/fastapi-best-practices/SKILL.md` (frontmatter + best practices), modeled on `skills/rust-best-practices/SKILL.md` → Docs Writer
- [ ] T010 [P] [US1] Author `skills/htmx-best-practices/SKILL.md` → Docs Writer
- [ ] T011 [P] [US1] Author `skills/go-best-practices/SKILL.md` → Docs Writer
- [ ] T012 [P] [US1] Author `skills/railway-best-practices/SKILL.md` → Docs Writer
- [ ] T013 [US1] Update `install.sh` skill-staging so removed skills are not staged and the 4 new ones are copied to `~/.kn/skills/`; prune removed skills from `~/.kn/skills/` only when unchanged → DevOps Agent
- [ ] T014 [US1] Update `kn update` (`cli/src/commands/update.rs`) to refresh `~/.kn/skills/` consistently with curation (add new, prune removed-if-unchanged) → Rust Agent
- [ ] T015 [US1] Make `sync`/`init` warn (not crash) when a project's `kn.toml` enables a skill absent from the catalog, leaving the file untouched → Rust Agent
- [ ] T016 [P] [US1] Integration test: after staging into a temp `KN_HOME`, assert removed skills absent and new skills present + installable in `cli/tests/` → QA Agent

**Checkpoint**: Catalog is curated and installable — US1 delivers value on its own.

---

## Phase 4: User Story 2 — Start a project from a stack-preset (P2)

**Goal**: `kn init --stack <name>` activates a whole bundle in one step.

**Independent test**: `kn init -y --stack web-astro` in a temp dir enables exactly that preset's skills in `kn.toml` and records the stack.

- [ ] T017 [P] [US2] Create shipped preset `stacks/web-astro.toml` (astro, htmx, supabase-postgres, railway, github-actions, docker) → Rust Agent
- [ ] T018 [P] [US2] Create shipped presets `stacks/api-fastapi.toml`, `stacks/cli-rust.toml`, `stacks/cli-go.toml`, `stacks/data-py.toml` per data-model.md → Rust Agent
- [ ] T019 [US2] Stage repo `stacks/` → `~/.kn/stacks/` in `install.sh` and in `kn update` (`cli/src/commands/update.rs`) → DevOps Agent
- [ ] T020 [US2] Add `--stack <name>` flag to `InitCommand` in `cli/src/commands/init.rs` → Rust Agent
- [ ] T021 [US2] In `init`, resolve the preset, enable exactly its skills merged/deduped with manual selections (reuse `KnConfig::add_skill`), and install/symlink them → Rust Agent
- [ ] T022 [US2] In `init`, record `stack = "<name>"` in `kn.toml` via `set_stack()` → Rust Agent
- [ ] T023 [US2] Unknown preset name → exit non-zero, write nothing, list valid presets (`init.rs`) → Rust Agent
- [ ] T024 [US2] Preset referencing a missing skill → exit non-zero before writing, naming missing skill + preset (`init.rs` using `resolve_skills`) → Rust Agent
- [ ] T025 [P] [US2] Integration tests (temp `KN_HOME`): happy path enables correct deduped set + records stack; unknown preset fails clean; missing skill fails clean in `cli/tests/` → QA Agent

**Checkpoint**: Presets can bootstrap a project end-to-end.

---

## Phase 5: User Story 3 — Discover and inspect presets (P3)

**Goal**: List/inspect presets any time, no initialized project needed.

**Independent test**: `kn stack list` in any dir prints each preset + its skills; empty case reports gracefully.

- [ ] T026 [US3] Create `cli/src/commands/stack.rs` with `kn stack list` (and optional `kn stack show <name>`) using `core::stack::list_presets`, flagging skills missing from the catalog → Rust Agent
- [ ] T027 [US3] Register the `Stack` subcommand in `cli/src/commands/mod.rs` and `cli/src/main.rs` → Rust Agent
- [ ] T028 [US3] Empty `~/.kn/stacks/` → `kn stack list` prints an informational "no presets available" message with exit 0 → Rust Agent
- [ ] T029 [P] [US3] Integration test for `kn stack list` populated vs empty in `cli/tests/` → QA Agent

**Checkpoint**: Presets are discoverable day-to-day.

---

## Phase 6: Polish & Cross-Cutting

- [ ] T030 [P] Update `README.md` / `README_ES.md`: document the curated catalog, `kn stack list`, and `kn init --stack` → Docs Writer
- [ ] T031 [P] Add an ADR under `docs/adr/` recording the stack-preset concept and the catalog-curation decision → Docs Writer
- [ ] T032 Run `cargo fmt`, `cargo clippy`, and full `cargo test` on `cli/`; fix findings → Rust Agent
- [ ] T033 Run through `quickstart.md` scenarios 1–6 manually against a temp `KN_HOME` and confirm expected outcomes → QA Agent

---

## Dependencies & Execution Order

- **Setup (T001–T002)** → **Foundational (T003–T007)** must finish before any user story.
- **US1 (T008–T016)** depends only on Foundational. It is the MVP.
- **US2 (T017–T025)** depends on Foundational + the new skills existing (T009–T012) so presets resolve; otherwise independent of US3.
- **US3 (T026–T029)** depends on Foundational (needs `core::stack` + `~/.kn/stacks/` staging from T019). Can run in parallel with US2 once T019 lands.
- **Polish (T030–T033)** last.

## Parallel Opportunities

- Foundational: T003, T004, T007 are `[P]` (different files).
- US1: T008–T012 and T016 are `[P]` — deletions + four SKILL.md authorings + test run in parallel (different files).
- US2: T017 and T018 (preset files) are `[P]`; T025 `[P]`.
- Polish: T030, T031 `[P]`.

## Implementation Strategy

- **MVP = US1 only**: a curated catalog that matches the user's stack. Shippable alone.
- **Increment 2 = US2**: preset-driven `kn init`.
- **Increment 3 = US3**: `kn stack list` discoverability.
- Each increment is independently testable via its Independent Test + the matching
  quickstart scenario.
