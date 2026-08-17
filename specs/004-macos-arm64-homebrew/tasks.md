---

description: "Task list for 004-macos-arm64-homebrew"
---

# Tasks: kn targets macOS Apple Silicon only, distributed via Homebrew

**Input**: Design documents from `specs/004-macos-arm64-homebrew/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: No automated test tasks. The feature changes packaging, CI and documentation —
not application code — so verification is the quickstart scenarios V1–V11 plus repository
grep assertions, not new `cargo test` cases. Existing tests stay green and unchanged.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1–US5)
- Exact file paths are included in every task

## Path Conventions

Repository root is `/Users/kobo/Github/personal/knowledge`. All paths below are
repo-relative. The tap is a **separate repository**, `kobogithub/homebrew-knowledge`,
referred to as `$TAP`.

## Assignment by role

| Phase | Role |
|---|---|
| Phase 1 Setup | **Planner** + **maintainer personally** (T003 cannot be automated) |
| Phase 2 Foundational | **DevOps** |
| Phase 3 US1 | **DevOps** |
| Phase 4 US2 | **DevOps** |
| Phase 5 US3 | **DevOps** |
| Phase 6 US4 | **Rust** (install.sh) + **Docs Writer** (READMEs, RELEASE.md) |
| Phase 7 US5 | **DevOps** |
| Phase 8 Polish | **QA** |

---

## Phase 1: Setup (Prerequisites)

**Purpose**: Establish the preconditions without which nothing downstream can be verified.

- [x] T001 **EXTERNAL DEPENDENCY — not this feature's work.** ✅ **Satisfied 2026-08-17.** PR #5 merged to `dev`, then `dev` → `prod` via PR #11. `gh api repos/kobogithub/knowledge` now reports `license=MIT` (it read `null` until the merge reached `prod`, because GitHub takes the licence from the **default branch**, not from wherever the file first lands). 002's T022/T041/T042/T046/T047 were closed as superseded or carried forward, each recording what was and was not actually verified. Original text: Confirm PR #5 (`epic/002-public-ready-repo` → `dev`) has merged, so a `LICENSE` file exists on the default branch. Homebrew formulas must declare a license (FR-009, contract C1.4) and `gh repo view kobogithub/knowledge --json licenseInfo` currently returns null. This task owns the *check*, not the merge — 002 is a separate initiative with its own tasks. Note for that initiative: its open T022/T041/T042 verify RPM/DEB packaging that US5 below deletes, so they are superseded rather than pending. **Blocking for US1**
- [x] T002 ✅ **Done 2026-08-17 — as a merge, not a rebase.** `dev` was merged into `epic/004-macos-arm64-homebrew` (commit 3562da5) rather than rebasing: the branch is already pushed and shared, and `CLAUDE.md` forbids force-pushing shared branches, which a rebase would require. One trivial conflict in `.specify/feature.json` (the active-spec pointer), resolved to `004`. `LICENSE` is now present on this branch. Original text: Rebase `epic/004-macos-arm64-homebrew` onto `dev` once T001 confirms. `002` rewrites the README installation section wholesale; editing the pre-`002` structure and merging later produces avoidable conflicts in exactly the section both features rewrite (research R10). **Blocking for US4**
- [x] T003 **MAINTAINER ONLY** — ✅ **Done 2026-08-17.** `TAP_GITHUB_TOKEN` exists in `kobogithub/knowledge`, created by the maintainer. Its scope has not been independently verified from here (secret values are write-only via the API); T006 proves it functionally. Original text: create a **fine-grained** Personal Access Token scoped to *only* `kobogithub/homebrew-knowledge`, permission `Contents: read and write`. Store it in `kobogithub/knowledge` as the Actions secret `TAP_GITHUB_TOKEN`. Do **not** use a classic PAT with `repo` scope — that grants write access to every repository the maintainer owns (research R3). The repo currently has zero secrets configured. **Blocking for US2**
- [x] T004 [P] ✅ **Run 2026-08-17 — defect confirmed.** tap formula `0.9.0`, newest release `v0.10.0`, and the machine's installed `kn --version` reports `0.9.0`. Invariant I1 is violated in production. Original: Record the baseline by running quickstart **V1**: confirm the tap formula reports `0.9.0` while the newest release is `v0.10.0`. Already observed during planning — `kn --version` on the maintainer's machine returns `0.9.0` from `Cellar/kn/0.9.0`, so invariant I1 is violated in production, not merely on paper. Re-confirm before changing anything; if V1 *passes*, stop and revisit the plan
- [x] T005 [P] ✅ **Run 2026-08-17 — all green.** `cargo fmt --check` clean, `cargo clippy -D warnings` clean, `cargo test` 55 passed / 0 failed across 4 targets. No pre-existing failures to report. Original: Confirm the pre-existing quality gates are green before any change: `cd cli && cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`. Any failure here is pre-existing — report it, do not absorb it into this feature

**Checkpoint**: License confirmed on default branch, branch rebased, tap credential in place, baseline recorded.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Prove the new credential actually works before building automation on top of it.

**⚠️ CRITICAL**: T006 must pass before any US2 work begins. Discovering a bad token while
debugging a release workflow is far more expensive than discovering it here.

- [ ] T006 Verify `TAP_GITHUB_TOKEN` can read and write `$TAP` by making a trivial round-trip from a scratch workflow run or a local `curl` against the GitHub API using the token: read `Formula/kn.rb`, write an unrelated file, delete it. Confirms permission scope and that the token is not expired (contract C2.6)

**Checkpoint**: Cross-repo write path proven.

---

## Phase 3: User Story 1 — Installing kn on the Mac Mini gives the current version (Priority: P1) 🎯 MVP

**Goal**: `brew install kobogithub/knowledge/kn` on the M4 yields the newest published
release instead of the 0.9.0 it serves today.

**Independent test**: quickstart V3 — clean install, compare `kn --version` against the
newest release tag. Delivers the maintainer's stated goal with nothing else completed.

- [x] T007 [US1] ✅ Flattened. `on_macos`/`on_arm`/`on_intel` and `on_linux` removed; flat `url`+`sha256` on `kn-macos-arm64.tar.gz`. Original: Rewrite `Formula/kn.rb` to a single-platform formula per contract C1: remove the `on_macos`/`on_arm`/`on_intel` nesting and the top-level `on_linux` block; replace with flat `url` and `sha256` pointing at `kn-macos-arm64.tar.gz` (FR-006, C1.1)
- [x] T008 [US1] ✅ Added `depends_on arch: :arm64` and `depends_on :macos`. Original: Add `depends_on :macos` and `depends_on arch: :arm64` to `Formula/kn.rb`. These make Homebrew refuse an Intel Mac upfront with its own message, rather than downloading a nonexistent artifact and failing at checksum verification (C1.2, spec edge case 1)
- [x] T009 [US1] ✅ `license "MIT"` present; `gh api` now reports `license=MIT` on `prod`. Original: Confirm `license "MIT"` is present in `Formula/kn.rb` and that `LICENSE` exists on the default branch (C1.4, FR-009). Depends on T001
- [x] T010 [US1] ✅ Survived. Verified by install: 21 skills and 11 agents landed in `pkgshare`. Also now asserted by the formula's own `test` block. Original: Verify the `resource "assets"` block survives the rewrite intact and still installs skills, agents and docs. The binary tarball contains **only** the binary — without this block `kn` installs as a shell with no skills and is not functional (C1.5, FR-007)
- [x] T011 [US1] ✅ Both recomputed from the real bytes, not trusted from the file: binary `3272dc67…`, source archive `4715b356…`. Both matched what the formula already carried. Original: Compute both checksums for the current release: `sha256` of `kn-macos-arm64.tar.gz` from the `v0.10.0` release, **and** `sha256` of the `v0.10.0` source archive for the assets resource. There are two, not one (C1.3, research R4)
- [x] T012 [US1] ✅ Extended the existing `caveats` (which already covered `~/.kn` population) with conflict detection over `~/.local/bin/kn` and `/usr/local/bin/kn`. Verified by V11: warning appears with a staged second copy, and does **not** appear without one. Original: Add a `caveats` block to `Formula/kn.rb` that detects a `kn` on the path outside the Homebrew prefix and reports both paths, which one the shell will run given PATH order, and how to remove the other (FR-020, SC-009, C1.8). Without this, a user with a leftover install-script copy debugs a stale version with no signal that two copies exist
- [x] T013 [US1] ✅ **V2 passes.** `brew style` no offenses; `brew audit --strict` clean. Took 5 iterations — audit rejected `version` as redundant with the URL, required `pkgshare` over `share/"kn"`, and required `bin/"kn"` over `"#{bin}/kn"` in `system`. Audited via a scratch tap (`brew audit` refuses bare paths). Original: Run quickstart **V2**: `brew style Formula/kn.rb` and `brew audit --strict --formula Formula/kn.rb` must report no offenses
- [x] T014 [US1] ✅ **Published 2026-08-17** — tap commit `7ecad69`, generated from this repo's `Formula/kn.rb` at `99b4d63`. The tap had been stranded on 0.9.0 since v0.10.0 shipped. Commit message records that the file is generated and must not be hand-edited from the next release on. **Verified end to end**: untapped and uninstalled, then `brew install kobogithub/knowledge/kn` from the public tap → `kn 0.10.0`, newest release `v0.10.0`, tap URL `v0.10.0`. All three agree — **invariant I1 satisfied**. Original: Publish the corrected formula to `$TAP` at `Formula/kn.rb`. This is the last time it is done by hand — US2 automates it. Record the result so the automation in T019 can be diffed against a known-good formula (C1.7)
- [x] T015 [US1] ✅ **V3 passes.** Installed from a scratch tap: `kn --version` → `kn 0.10.0`, newest release `v0.10.0`. Invariant I1 satisfied locally; the machine had been on 0.9.0. Original: Run quickstart **V3** on the Mac Mini M4: uninstall and untap, then `brew install kobogithub/knowledge/kn`, and confirm `kn --version` matches the newest release tag exactly (SC-001, invariants I1 and I4)
- [x] T016 [US1] ⚠️ **V4 partially passes.** Assets confirmed: 21 skills, 11 agents. But `kn doctor` **exits 1** and reports a critical missing dependency — see the finding below. The C1.5/FR-007 half (assets installed) is verified; "doctor reports healthy" is not, for a pre-existing reason outside this feature. Original: Run quickstart **V4**: `kn doctor` reports healthy, and `$(brew --prefix)/share/kn/skills` and `.../agents` are both non-empty (C1.5, FR-007)
- [x] T017 [US1] ✅ **V11 passes.** Staged a second `kn` at `~/.local/bin/kn`; caveats named both paths, explained PATH precedence, and gave the removal command. Removed it; the warning correctly disappeared. Original: Run quickstart **V11**: stage a second `kn` at `~/.local/bin/kn`, reinstall, and confirm the caveats name both copies, which one wins, and how to remove the other. Clean up afterwards (FR-020, SC-009)

**Checkpoint**: Invariants I1 and I2 satisfied. `brew install` gives the current version. The MVP is delivered and independently valuable even if nothing below ships.

### Discovered during implementation (US1)

**`kn doctor` is stale and its exit code broke the formula's own test.** Found while running V4:

- It treats **`bd` (beads) as a required, critical dependency** and exits `1` when absent. [ADR-006](../../docs/adr/006-adopt-speckit-remove-beads.md) removed beads from this project's workflow — the check was never updated to match.
- Its remediation text points at **`https://github.com/your-org/bd`**, an unreplaced placeholder.
- Consequently the formula's `test do` block, which called `system bin/"kn", "doctor"`, failed. **This was pre-existing** — the original formula carried the same call, so `brew test kn` has been broken for anyone without `bd` installed.

Fixed *in the formula* (in scope): the test now asserts `--version` and the presence of `pkgshare/skills` and `pkgshare/agents` — what this formula actually installs — instead of depending on tools the machine happens to have. Homebrew's sandboxed test environment lacks even Rust and Cargo, so asserting on `doctor` output was fragile regardless.

**Not fixed** (out of scope — this feature does not change what `kn` does): `kn doctor` itself. It should stop treating `bd` as required and the placeholder URL should go. Worth its own task under a future initiative, or folded into `003-absorb-framework-ia`.

---

## Phase 4: User Story 2 — Publishing a release updates the published formula automatically (Priority: P2)

**Goal**: The next release updates the tap with no human step, and drift is visible if the
automation ever breaks.

**Independent test**: T027 — cut a real release and observe the tap update itself, with no
manual action. V6/V7/V8 verify the parts; T027 verifies the whole, and it lives in this
phase rather than in polish so US2 can actually be signed off within its own phase.

**Depends on**: T003 (credential), T006 (credential proven), T014 (a known-good formula to reproduce)

- [ ] T018 [US2] Add a `publish-formula` job to `.github/workflows/release.yml`, with `needs: [create-release]` so it runs only after the release is successfully published (C2.3, FR-011)
- [ ] T019 [US2] In `publish-formula`, render `Formula/kn.rb` from the repo by substituting `version`, both `url`s and both `sha256`s, then commit and push the result to `$TAP` at `Formula/kn.rb`. The authored formula stays the reviewable source of truth; the tap becomes a generated output (research R2, C1.6, C1.7)
- [ ] T020 [US2] Compute both checksums inside the job **from the artifacts actually uploaded to the release**, never from a value stored in the repo. A hand-maintained checksum is the drift mechanism this feature exists to remove (C2.5, FR-012)
- [ ] T021 [US2] Authenticate the push with `secrets.TAP_GITHUB_TOKEN`. `GITHUB_TOKEN` is scoped to the running repository and **cannot** push to `$TAP` (C2.6, research R3)
- [ ] T022 [US2] Ensure the job is a no-op-safe re-run: publishing the same tag twice must leave `$TAP` in a consistent state rather than a half-applied edit (C2.7, spec edge case 5)
- [ ] T023 [US2] Verify the failure path: if `build-binaries` or `create-release` fails, `publish-formula` must not run and `$TAP` must be untouched. A formula pointing at a nonexistent download is worse than a stale one (C2.4, FR-011, spec edge case 2)
- [ ] T024 [P] [US2] Create `.github/workflows/tap-drift-check.yml`: on a schedule, read the `version` declared in `$TAP`'s `Formula/kn.rb`, read the newest release tag from this repo, and fail when they disagree (C2.9, C2.10, FR-013)
- [ ] T025 [US2] Decide and implement how a drift-check failure actually reaches the maintainer, and record the decision in the workflow file. GitHub emails the repository owner on scheduled-workflow failure by default, which may well be sufficient for a solo maintainer — but confirm that default is active for this account rather than assuming it. A check that fails silently in the Actions tab leaves I1 as unobservable as it is today (FR-013)
- [ ] T026 [US2] Confirm the drift check runs **independently of the release workflow**, on its own schedule. An assertion inside the release job cannot detect a push that later reverted or a credential that expired months after the last release — exactly the class of failure that produced the current 0.9.0/0.10.0 gap (C2.11, research R6)
- [ ] T027 [US2] Cut a release and confirm the tap updates end-to-end with zero manual steps (SC-003). Then run quickstart **V6** (failed build leaves tap untouched), **V7** (published checksums match published bytes) and **V8** (drift check fails on a deliberately stale formula, passes on the current one). This is the first exercise of the full pipeline — every earlier verification reused an existing release

**Checkpoint**: US1 can no longer silently regress. Invariant I1 is continuously observable, and the observation reaches a human.

---

## Phase 5: User Story 3 — Releases build exactly one artifact (Priority: P3)

**Goal**: One binary per release, for Apple Silicon macOS.

**Independent test**: quickstart V5 — a published release carries exactly one binary artifact.

- [ ] T028 [US3] Reduce the `build-binaries` matrix in `.github/workflows/release.yml` (lines ~27-45) to the single existing `aarch64-apple-darwin` / `macos-latest` entry. Remove the `x86_64-unknown-linux-gnu` and `x86_64-apple-darwin` entries (FR-002, C2.1)
- [ ] T029 [US3] **Do not rename the artifact.** It must stay `kn-macos-arm64.tar.gz`. `install.sh` selects on this name and in-tool `kn update` parses it — `RELEASE.md:273` records that the `kn-{os}-{arch}.tar.gz` shape is required for update to work. Renaming it now that there is only one target would break self-update for every already-installed copy (C2.2, research R5). This task is a deliberate no-op — check it off to confirm the decision was applied, not forgotten
- [ ] T030 [US3] Update the release-notes template in `.github/workflows/release.yml` (lines ~109-111) to list only the Apple Silicon download. It currently advertises Linux x86_64 and macOS x86_64 downloads that will no longer exist (C2.8, FR-016)
- [ ] T031 [US3] Run quickstart **V5** against a real release: `gh release view <tag> --json assets` returns exactly `kn-macos-arm64.tar.gz` and `checksums.txt` (SC-004, C2.1)

**Checkpoint**: One artifact per release; the name that other tooling depends on is intact.

---

## Phase 6: User Story 4 — The project stops promising platforms it does not support (Priority: P4)

**Goal**: No document or script claims support for Linux, Windows or Intel Macs.

**Independent test**: quickstart V9 and V10, plus the grep assertion in T039.

**Depends on**: T002 (rebase) — `002` rewrites the same README section

- [ ] T032 [US4] Narrow platform detection in `install.sh` (lines ~119-170) to Apple Silicon macOS, refusing everything else before any network request, with a message naming the supported platform and a non-zero exit (FR-003, C3.1, C3.2, SC-005)
- [ ] T033 [US4] **Retain the Rosetta probe** at `install.sh:143-149`. It looks like dead code under an arm64-only policy and is not: it detects `x86_64` on macOS, tests whether the process is translated, and rewrites `ARCH` to `arm64`. Deleting it makes the installer refuse the maintainer's own M4 whenever it is invoked from a translated terminal — a regression created by the cleanup itself. Narrow the *genuine Intel* outcome to a refusal; keep the architecture check that distinguishes genuine Intel from translated Apple Silicon (C3.3, research R7, FR-004, spec edge case 6)
- [ ] T034 [US4] Reduce artifact selection in `install.sh` (lines ~384-390) to the single `kn-macos-arm64.tar.gz` name, removing the Linux and Intel branches (C3.4)
- [ ] T035 [US4] Leave the install destination and PATH handling in `install.sh` unchanged. Only platform detection and artifact selection are in scope (C3.5)
- [ ] T036 [P] [US4] In `README.md`: reduce "Supported Platforms" (lines ~150-153) to Apple Silicon macOS only; delete the Windows installation section (~205-214); delete the "Package Managers (Coming Soon)" section advertising APT and DNF (~218-226); make Homebrew the primary install, and assert the command string is exactly `brew install kobogithub/knowledge/kn` (FR-005, FR-014, FR-015, FR-016)
- [ ] T037 [P] [US4] Apply the same edits to `README_ES.md`: platforms (~126-129), Windows section (~181-190), "Gestores de Paquetes (Próximamente)" (~194-202), Homebrew as primary install with the identical command string (FR-005, FR-014, FR-015, FR-016)
- [ ] T038 [US4] Rewrite the release procedure in `RELEASE.md`: delete the manual formula-edit steps (~175-193), delete the manual tap `cp` under "Post-Release Tasks" (~283-296), delete the APT repository steps, and delete the `Formula/kn.rb` update items (~76, ~107). Replace with a short statement that the tap is updated automatically by `publish-formula`. This prose *is* the process that failed — it is being deleted, not documented better (research R1, R8)
- [ ] T039 [US4] Run the SC-006 grep assertion: search the repository for claims of Linux, Windows or Intel Mac support. Zero results expected outside `CHANGELOG.md` and `specs/`. Changelog entries describing past releases are historical record and stay untouched — the same precedent 002 set in its T020 (research R8)
- [ ] T040 [US4] Run quickstart **V9** (install script refuses on Linux via container, downloads nothing, non-zero exit) and **V10** (install script succeeds on Apple Silicon both natively and under `arch -x86_64`, both reporting the same version)

**Checkpoint**: Documentation and install script tell the truth. V10 is the one most likely to catch a careless narrowing.

---

## Phase 7: User Story 5 — Retired packaging is gone from the repository (Priority: P5)

**Goal**: The recipes for platforms no longer supported are removed, with no dangling references.

**Independent test**: T044's grep assertion — no retired artifact remains and nothing links to one.

- [ ] T041 [P] [US5] Delete `kn.spec`. Verified stale during planning: it declares `Version: 0.1.0` while the project ships `0.10.0`, so it has not produced a current package in nine minor versions. Deletion is strictly a cleanup with no loss (FR-018, research R8)
- [ ] T042 [P] [US5] Delete the `debian/` packaging directory (FR-018)
- [ ] T043 [P] [US5] Delete `install.ps1`, and `docs/packaging/` if present after the T002 rebase — that directory is created by `002` (FR-018)
- [ ] T044 [US5] Repoint or delete every inbound reference catalogued in research R8: the packaging doc links at `README.md:230-235`, and any surviving mention in `RELEASE.md`. Then run the SC-007 grep assertion — zero results for the deleted files, and no surviving file linking to them (FR-017)
- [ ] T045 [US5] Confirm the deleted content remains retrievable from git history and that no history was rewritten: `git log --diff-filter=D --name-only -- kn.spec debian install.ps1` returns the deletion commit (FR-019)

**Checkpoint**: Repository carries only what it supports.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [ ] T046 Add a `CHANGELOG.md` entry describing the platform narrowing as a **breaking change** for anyone on Linux, Windows or an Intel Mac, and naming Homebrew as the primary install. Per the project's commit conventions this warrants a `!` breaking-change marker and a MAJOR bump consideration
- [ ] T047 Run the full quickstart **V1–V11** and record the outcome of each in this file, including any check recorded as unverified **with its reason**. The precedent from `002` is that an unverified check is reported honestly rather than quietly checked off
- [ ] T048 [P] Confirm `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test` are still green. This feature should not have touched `cli/` at all — a change here means scope leaked
- [ ] T049 [P] **Non-blocking follow-up, not this feature's scope**: `.specify/memory/constitution.md` is still an unfilled template — every principle is a `[PRINCIPLE_N_NAME]` placeholder. This is the second consecutive initiative whose Constitution Check passed by vacuity rather than compliance. Consider running `/speckit-constitution` before an initiative where it would actually bind

---

## Dependencies

```
T001 (002 merged: license) ──┬──→ T002 (rebase) ──→ Phase 6 (US4)
                             └──→ T009 ──→ Phase 3 (US1)

T003 (PAT) ──→ T006 (prove credential) ──→ Phase 4 (US2)

Phase 3 (US1) ──→ T014 (known-good formula) ──→ T019 (automation diffed against it)

Phase 4 (US2) ──→ T027 needs the automation in place; cuts a real release
Phase 5 (US3) ──→ T031 needs a real release

Phase 7 (US5) — independent of everything except T002, for docs/packaging/
```

**Story independence**: US1 delivers alone and is the MVP. US2 depends on US1 only for a
known-good formula to reproduce. US3, US4 and US5 are independent of each other and of
US1/US2, except where the rebase gate (T002) applies.

**Note on T027 and T031**: both need a real release. If US3 lands before US2's release is
cut, one release satisfies both — worth sequencing US3 before T027 to avoid cutting two.

---

## Parallel execution examples

**Phase 1**: T004 and T005 in parallel (different concerns, read-only).

**Phase 6**: T036 and T037 in parallel — the two READMEs are separate files with the same
edits. T032–T035 are sequential; they all touch `install.sh`.

**Phase 7**: T041, T042 and T043 in parallel — three independent deletions. T044 must
follow all three, since it asserts nothing references them.

**Phase 8**: T048 and T049 in parallel.

---

## Implementation strategy

**MVP is Phase 1 → Phase 3 (US1).** That is 17 tasks and closes the headline defect:
`brew install` on the M4 gives the current version instead of the 0.9.0 it serves today.
Everything after it is either durability (US2) or honesty (US3–US5).

**Recommended order**: Ship US1, verify V3, V4 and V11 on the actual Mac Mini, then do US2
before anything else. US2 is what prevents the manual fix in T014 from decaying back into
the state this feature was created to repair — shipping US1 alone leaves the process
exactly as fragile as it was.

**Two gates block everything and neither is this feature's own work**: T001 (002 must merge,
for the license) and T003 (the maintainer must create the fine-grained PAT).

---

## Task summary

| Phase | Story | Tasks | Count |
|---|---|---|---|
| 1 | Setup | T001–T005 | 5 |
| 2 | Foundational | T006 | 1 |
| 3 | US1 (P1) 🎯 | T007–T017 | 11 |
| 4 | US2 (P2) | T018–T027 | 10 |
| 5 | US3 (P3) | T028–T031 | 4 |
| 6 | US4 (P4) | T032–T040 | 9 |
| 7 | US5 (P5) | T041–T045 | 5 |
| 8 | Polish | T046–T049 | 4 |
| | **Total** | | **49** |

**Parallel opportunities**: 10 tasks marked `[P]` — T004, T005, T024, T036, T037, T041, T042, T043, T048, T049.

**Deliberate no-ops** (check off to confirm the decision was applied, not forgotten): T029
(do not rename the artifact), T035 (leave install paths alone).

---

## Analyze findings applied

`/speckit-analyze` was run against spec/plan/tasks before this revision. Zero critical
issues; five findings applied here:

| ID | Finding | Applied as |
|---|---|---|
| **C1** (HIGH) | The dual-install edge case was declared in the spec but had no requirement, no task and no scenario | New **FR-020** and **SC-009** in `spec.md`, clause **C1.8** in `contracts/formula.md`, scenario **V11** in `quickstart.md`, tasks **T012** and **T017** |
| **I1** (MED) | "Cut a release" sat in polish while claiming to be US2's first real test, so US2 could not be signed off in its own phase | Moved into Phase 4 as **T027** |
| **U1** (MED) | The drift check had no defined route to a human | New **T025** — decide and confirm the notification path rather than assuming the GitHub default is active |
| **G1** (MED) | T001 owned a checkbox for merging another initiative's PR | Reframed as an external dependency the task *checks*, not performs |
| **A1** (LOW) | FR-005's "single documented command" was never asserted against the README text | Folded into **T036** and **T037** |
