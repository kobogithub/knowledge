---
description: "Task list for 002-public-ready-repo"
---

# Tasks: Public-Ready Repository

**Input**: Design documents from `specs/002-public-ready-repo/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: No automated test tasks. The spec requested none, and this feature adds no executable code. Verification is done through the numbered checks in [quickstart.md](./quickstart.md) (referenced below as V1–V14), which serve as the acceptance gate.

**Organization**: Tasks are grouped by user story so each can be implemented, verified and merged independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on an incomplete task)
- **[Story]**: Which user story the task belongs to (US1–US4)
- Every task names its exact file path

## Path Conventions

Documentation reorganization at the repository root. No `src/` or `tests/` involvement — `cli/`, `skills/`, `stacks/` and `agents/` are out of scope and must not be touched (FR-021, FR-022).

## Assignment by role

| Phase | Story | Owner |
|---|---|---|
| 1 | Setup | QA Agent (`knowledge-pu1`) |
| 3 | US1 — Licence | Docs Writer (`knowledge-doc`) |
| 4 | US2 — Root reorganization | Docs Writer + **DevOps** (`knowledge-w5p`) for the build recipes |
| 5 | US3 — README | Docs Writer |
| 6 | US4 — Contributing | Docs Writer |
| 7 | Polish | DevOps (CI) + Docs Writer + QA (final sweep) |

---

## Phase 1: Setup

**Purpose**: Establish the baseline so any later failure is attributable to this feature.

- [ ] T001 [P] Record the current root inventory to compare against later: `ls *.md` in the repository root should list 12 files (`AGENTS.md`, `CHANGELOG.md`, `CLAUDE.md`, `DEBIAN.md`, `GITHUB_PAGES.md`, `HOMEBREW.md`, `README.md`, `README_ES.md`, `RELEASE.md`, `RELEASE_v0.1.0_CHECKLIST.md`, `RPM.md`, `SECURITY_AUDIT_REPORT.md`)
- [ ] T002 [P] Confirm the pre-existing quality gates are green before any change, per quickstart V13: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --release` (all with `--manifest-path cli/Cargo.toml`) and `shellcheck install.sh`. Any failure here is pre-existing — report it, do not absorb it into this feature
- [ ] T003 Create the three documentation groups: `docs/packaging/`, `docs/release/`, `docs/security/`

---

## Phase 2: Foundational

**None required.** No shared prerequisite blocks the user stories: US1 creates a new file, US2 moves existing ones, US3 edits two files, US4 creates new ones. They touch disjoint paths except both US1 and US3 edit the two READMEs — sequenced in the Dependencies section rather than blocked here.

---

## Phase 3: User Story 1 — Licensed for reuse (Priority: P1) 🎯 MVP

**Goal**: A visitor can determine reuse terms without asking anyone.

**Independent test**: GitHub's repository sidebar displays "MIT license" and the file is reachable from the root (quickstart V2).

- [x] T004 [US1] Create `LICENSE` at the repository root containing unmodified MIT licence text with the copyright line `Copyright (c) 2026 Kevin Barroso`. The filename must be exactly `LICENSE` with no extension, and the body must not be editorialized — GitHub matches against known licence texts and any modification drops detection back to `licenseInfo: null` (research R7, contract C2)
- [x] T005 [P] [US1] Add a License section at the end of `README.md` stating MIT terms and linking to `./LICENSE` — **no edit needed**: the section already existed and already linked to `LICENSE`; the link was dangling because the target file did not exist. T004 resolved it
- [x] T006 [P] [US1] Add the equivalent License section to `README_ES.md`, matching T005's structure (FR-003, FR-012) — **no edit needed**, same as T005
- [x] T007 [US1] Verify quickstart V2. Note that `gh repo view --json licenseInfo` reflects the pushed default branch, so before merge verify the file content locally instead

**Checkpoint**: US1 is independently shippable — the legal blocker is cleared even if nothing else lands.

---

## Phase 4: User Story 2 — A root that explains itself (Priority: P2) ⚠️ BUILD-BREAKING

**Goal**: The root lists only reader-facing and agent-facing entry points; every relocated document stays reachable.

**Independent test**: quickstart V1, V3, V5, V7 pass and both package builds still succeed (V6).

> **Task order in this phase is mandatory.** T008–T011 move files that two package build recipes copy by name from the repository root. Until T012 and T013 land, `rpmbuild` and `dpkg-buildpackage` both fail on a missing path. Do not leave the repository between T011 and T013.

- [ ] T008 [US2] `git mv HOMEBREW.md DEBIAN.md RPM.md docs/packaging/` — preserve filenames so git records renames and history follows each file (research R1)
- [ ] T009 [US2] `git mv RELEASE.md GITHUB_PAGES.md docs/release/`
- [ ] T010 [US2] `git mv SECURITY_AUDIT_REPORT.md docs/security/`
- [ ] T011 [US2] Delete `RELEASE_v0.1.0_CHECKLIST.md` — stale by nine minor versions, zero inbound references, and the recurring process it derives from is retained in `docs/release/RELEASE.md`. Git history preserves it (research R3, FR-006)
- [ ] T012 [US2] **DevOps**: Delete line 61 of `kn.spec` (`cp HOMEBREW.md DEBIAN.md %{buildroot}%{_docdir}/kn/`). Do **not** repoint it — the recipe's existing `cp -r docs` below already ships these files, and repointing would ship each document twice (research R2, contract C4)
- [ ] T013 [US2] **DevOps**: Delete line 34 of `debian/rules` (`cp HOMEBREW.md $(CURDIR)/debian/kn/usr/share/doc/kn/`), same reasoning as T012
- [ ] T014 [US2] **DevOps**: Confirm the recursive copy survived in both recipes — `grep -n 'cp -r docs' kn.spec debian/rules` must return exactly one hit in each. Without it the documents stop shipping entirely (quickstart V5)
- [ ] T015 [P] [US2] Correct the installed-path reference at `docs/packaging/DEBIAN.md:121`: `/usr/share/doc/kn/HOMEBREW.md` → `/usr/share/doc/kn/docs/packaging/HOMEBREW.md`. This describes a path inside an installed package, so a repo-relative link sweep will not catch it
- [ ] T016 [P] [US2] Correct the two installed-path references at `docs/packaging/RPM.md:196` and `:197` the same way (`HOMEBREW.md` and `DEBIAN.md` → `docs/packaging/…`)
- [ ] T017 [P] [US2] Repoint the three packaging links at `README.md:233-235` from `./HOMEBREW.md`, `./DEBIAN.md`, `./RPM.md` to `./docs/packaging/…`
- [ ] T018 [P] [US2] Repoint `docs/ENVIRONMENT_VARIABLES.md:222` from `../SECURITY_AUDIT_REPORT.md` to `./security/SECURITY_AUDIT_REPORT.md`
- [ ] T019 [US2] Create `docs/README.md` indexing every document under `docs/` — the six existing top-level files, the three new groups, plus `adr/` and `reports/` (FR-008, contract C3)
- [ ] T020 [US2] Leave `CHANGELOG.md` lines 490, 496, 502, 536-539 **unchanged**. They mention the relocated filenames inside released-version sections; a changelog is a historical record and they are prose mentions, not links (research R4). This task is a deliberate no-op — check it off to confirm the decision was applied, not forgotten
- [ ] T021 [US2] Verify quickstart V1, V3, V5, V7
- [ ] T022 [US2] **DevOps**: Verify quickstart V6 — build both packages and confirm each relocated document ships exactly once (`rpm -qlp … | grep -c 'HOMEBREW.md'` returns 1). Requires Linux; if unavailable, record explicitly as unverified with the reason rather than checking this off

**Checkpoint**: root Markdown reduced from 12 to 5 at this point — `AGENTS.md`, `CHANGELOG.md`, `CLAUDE.md`, `README.md`, `README_ES.md` — plus `LICENSE` from US1. `CONTRIBUTING.md` arrives with US4, bringing the final total to 7 documents. Packages still build and still ship the docs.

---

## Phase 5: User Story 3 — Understanding kn in under a minute (Priority: P3)

**Goal**: A new reader learns what `kn` is, installs it, and runs a first command without scrolling past the feature catalog.

**Independent test**: quickstart V9 and V10 pass.

- [ ] T023 [US3] Reorder `README.md` to: title + one-line description → what it is and who it is for → installation → first commands → feature catalog → documentation index → licence. Installation currently begins at line 115, after five feature subsections; move it above the catalog. The catalog keeps its content and simply moves down (research R8, FR-009..FR-011)
- [ ] T024 [US3] Apply the identical section order to `README_ES.md`. This is a reordering exercise in both files, not a retranslation — the two must present equivalent structure and equivalent claims (FR-012)
- [ ] T025 [P] [US3] Add one sentence near the `kn beads` section of both READMEs stating that this project's own workflow uses spec-kit per [ADR-006](../../docs/adr/006-adopt-speckit-remove-beads.md), while `kn beads` remains available for users who use beads independently. The command genuinely ships (`cli/src/commands/beads.rs`), so removing it from the docs would violate FR-013 — the fix is stating status, not deleting (research R9, FR-014)
- [ ] T026 [US3] Verify quickstart V10 — each of the nine shipped commands (`agents`, `beads`, `doctor`, `init`, `mcp`, `skills`, `stack`, `sync`, `update`) appears in both READMEs, and nothing undocumented is claimed (FR-013)
- [ ] T027 [US3] Verify quickstart V9 — installation precedes the feature catalog, and both READMEs have the same top-level section count

**Checkpoint**: a newcomer reaches the install command within the first screen.

---

## Phase 6: User Story 4 — Knowing how to contribute (Priority: P4)

**Goal**: A contributor can determine the target branch and commit format without asking, and gets a structured form when filing.

**Independent test**: quickstart V11 passes and the new-issue chooser offers both forms with no blank option.

- [ ] T028 [US4] Create `CONTRIBUTING.md` at the repository root covering the branch hierarchy (`prod` ← `dev` ← `epic/<feature-id>` ← `<feature-id>/<role>`), the pull request target for each branch type, and the conventional commit format with its allowed types and SemVer impact. Derive from the rules already in `AGENTS.md` so the two cannot drift, and link to the `standard-commits` skill rather than restating it (research R11, FR-015)
- [ ] T029 [US4] In `CONTRIBUTING.md`, describe how an initiative is specified before implementation (`/speckit-specify` → `/speckit-plan` → `/speckit-tasks`), linking to `specs/` for worked examples rather than duplicating the workflow (FR-016)
- [ ] T030 [P] [US4] Create `.github/ISSUE_TEMPLATE/bug_report.yml` as a GitHub issue form with required fields for `kn` version, operating system, reproduction steps, expected and actual behaviour (research R5, FR-017)
- [ ] T031 [P] [US4] Create `.github/ISSUE_TEMPLATE/feature_request.yml` as an issue form with required fields for the problem being solved and the proposed behaviour
- [ ] T032 [P] [US4] Create `.github/ISSUE_TEMPLATE/config.yml` with `blank_issues_enabled: false` so reporters are forced through a template (contract C5)
- [ ] T033 [P] [US4] Create `.github/pull_request_template.md` prompting for a summary, the related initiative under `specs/`, and the verification performed (FR-018)
- [ ] T034 [US4] Verify quickstart V11, including the manual confirmation at `https://github.com/kobogithub/knowledge/issues/new/choose` after push

**Checkpoint**: contribution intake is structured.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [ ] T035 [P] Replace the absolute path at `specs/001-personal-stacks/quickstart.md:9` (`cd /Users/kobo/Github/personal/knowledge`) with a repository-relative instruction. Keep the step — it exists to tell the reader to be at the repository root, so the replacement must say that rather than delete the line (research R10, FR-020)
- [ ] T036 [P] **DevOps**: Add a `lychee` link-check job to `.github/workflows/ci.yml`, scoped to tracked Markdown and **internal links only**. External URL checking stays off so a third-party outage cannot fail CI — a live risk, since the maintainer just hit exactly that with a paused badge service. Do not touch `.github/workflows/release.yml` (research R6, FR-023)
- [ ] T037 Add a `CHANGELOG.md` entry for this reorganization under a new version heading, recording the documentation move, the licence addition and the deleted stale checklist. Add only — do not edit existing entries (research R4)
- [ ] T038 Verify quickstart V4 (no broken internal links) and V8 (no `/Users/kobo` in any tracked file)
- [ ] T039 Verify quickstart V12 — `git diff --stat dev...HEAD -- cli/ skills/ stacks/ agents/ .github/workflows/release.yml` returns no output, confirming FR-021 through FR-023 held
- [ ] T040 Verify quickstart V13 — the existing quality gates still pass, unchanged from the T002 baseline
- [ ] T041 Verify quickstart V14 — follow `README.md` alone in a clean container and confirm `kn` installs and reports its version. If any step needs a document other than the README, record which one; that content belongs in the README (SC-009). This is distinct from CI's `test-install-script` job, which proves `install.sh` works rather than proving the README leads someone to it
- [ ] T042 Final sweep: run the full quickstart V1–V14 and record the outcome of each, including any check recorded as unverified with its reason

---

## Dependencies

```text
Phase 1 (Setup: T001–T003)
   │
   ├─▶ Phase 3 US1 (T004–T007) ──────────┐  independent, ships alone
   │                                      │
   ├─▶ Phase 4 US2 (T008–T022) ──────────┤  T003 required (target dirs)
   │      T008–T011 ─▶ T012–T014 ─▶ T015–T018 ─▶ T019 ─▶ T021–T022
   │                   ⚠️ strict order    │
   ├─▶ Phase 6 US4 (T028–T034) ──────────┤  independent, ships alone
   │                                      │
   └─▶ Phase 5 US3 (T023–T027) ──────────┘  after US1 + US2 (both edit README)
                                          │
                              Phase 7 (T035–T042) ─▶ last
```

**Cross-story ordering constraints**:

- **US3 after US1 and US2** — all three edit `README.md`. US1 appends a licence section, US2 repoints three links at lines 233-235, US3 reorders the whole file. Running US3 first would invalidate the line numbers the other two depend on.
- **US2 internal order is strict** — T008–T011 break both package builds until T012–T013 land.
- **US1, US2 and US4 are mutually independent** and may proceed in parallel by different agents on separate branches.
- **Phase 7 last** — T038–T042 verify the combined end state.

---

## Parallel execution examples

**Within US1** — after T004 creates the licence:

```text
T005 (README.md licence section)  ┐ different files
T006 (README_ES.md licence section) ┘
```

**Within US2** — after the moves and recipe fixes (T008–T014):

```text
T015 (docs/packaging/DEBIAN.md)      ┐
T016 (docs/packaging/RPM.md)         │ four different files,
T017 (README.md links)               │ no interdependency
T018 (docs/ENVIRONMENT_VARIABLES.md) ┘
```

**Within US4** — all four template files are new and disjoint:

```text
T030 (bug_report.yml)  T031 (feature_request.yml)  T032 (config.yml)  T033 (pull_request_template.md)
```

**Across stories** — three agents on separate branches:

```text
Docs Writer  ─▶ US1 (licence)
Docs Writer  ─▶ US4 (contributing + templates)
DevOps       ─▶ T036 (CI link check)
```

---

## Implementation strategy

**MVP = User Story 1 alone.** Four tasks, one new file plus two README sections, and the repository stops being legally unusable. Everything else is quality of presentation.

**Recommended increments**:

1. **US1** → merge. The legal blocker is gone.
2. **US2** → merge. Highest risk; verify the package builds before opening the PR.
3. **US3** → merge. Depends on US1 and US2 having settled the README.
4. **US4** + Phase 7 → merge. Lowest risk, finishes the surface.

Each increment is a separate PR into `epic/002-public-ready-repo`, which then targets `dev`.

**The one thing that can break something**: T012 and T013. Two package recipes copy relocated files by name from the repository root; the moves in T008–T011 invalidate those paths. Both recipes already copy `docs/` recursively, so the fix is deletion rather than repointing — and repointing would be worse than doing nothing, shipping every affected document twice.

---

## Task summary

| Phase | Story | Tasks | Count |
|---|---|---|---|
| 1 | Setup | T001–T003 | 3 |
| 2 | Foundational | — | 0 |
| 3 | US1 — Licence (P1) | T004–T007 | 4 |
| 4 | US2 — Root reorganization (P2) | T008–T022 | 15 |
| 5 | US3 — README (P3) | T023–T027 | 5 |
| 6 | US4 — Contributing (P4) | T028–T034 | 7 |
| 7 | Polish | T035–T042 | 8 |
| | **Total** | | **42** |

Parallelizable tasks: 15 marked `[P]`.
