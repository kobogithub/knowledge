# Implementation Plan: kn targets macOS Apple Silicon only, distributed via Homebrew

**Branch**: `epic/004-macos-arm64-homebrew` | **Date**: 2026-08-16 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/004-macos-arm64-homebrew/spec.md`

## Summary

Narrow `kn` from three build targets to one — Apple Silicon macOS — and make the
maintainer's existing public tap the primary way it is installed.

The substantive engineering is not the narrowing, which is subtraction from an existing
working build. It is closing the gap that made the published formula stale: there is no
Homebrew automation in CI at all, and the formula exists in two hand-synced copies that
currently disagree. The approach keeps the authored formula in this repository, has the
release workflow render version and checksums into it and push the result to the tap, and
adds an independent scheduled check so the same drift is visible if the automation ever
breaks.

## Technical Context

**Language/Version**: Rust 2021 edition, crate `kn` at 0.10.0 (`cli/Cargo.toml`)

**Primary Dependencies**: None added. The feature touches packaging, CI and docs, not
application code. `clap`, `serde` etc. are untouched.

**Storage**: N/A

**Testing**: `cargo test` (existing integration tests `cli/tests/mcp_integration.rs`,
`cli/tests/stack_integration.rs`), `cargo clippy -D warnings`, `cargo fmt --check` — all
already enforced by `ci.yml` and unchanged. Feature-level verification is the quickstart
scenarios, run against real releases and a real Apple Silicon machine.

**Target Platform**: macOS on Apple Silicon (`aarch64-apple-darwin`), sole supported
target. Already built natively on the `macos-latest` runner today (research R5).

**Project Type**: CLI tool distributed as a pre-built binary through a Homebrew tap.

**Performance Goals**: N/A — no runtime behaviour changes. The incidental effect is a
release pipeline that builds one artifact instead of three.

**Constraints**:
- The artifact name `kn-macos-arm64.tar.gz` MUST NOT change — `install.sh` and the
  in-tool `kn update` both select on it (research R5).
- The Rosetta detection branch in `install.sh` MUST survive the narrowing (research R7).
- Cross-repository push requires a credential that does not exist yet (research R3).
- A license must be present on the default branch before the formula can declare one
  (research R9).

**Scale/Scope**: One maintainer, one machine, one tap, one artifact per release. Roughly
8 files changed and 4 deleted, plus one new workflow and one new repository secret.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

`.specify/memory/constitution.md` is an **unfilled template** — every principle is still a
`[PRINCIPLE_N_NAME]` placeholder with example comments. There are no ratified principles
to check against, so no gate can pass or fail on its content.

**Gate result**: PASS by vacuity, not by compliance.

**Recorded risk**: This is the second consecutive initiative planned without a
constitution. Nothing in this feature depends on one, so it is not a blocker here, but the
project is accumulating decisions with no written principles to check them against.
Running `/speckit-constitution` is worth doing before an initiative where it would
actually bind — it is noted in `tasks.md` as a non-blocking follow-up rather than smuggled
into this feature's scope.

**Post-Phase-1 re-check**: Unchanged. No design decision below conflicts with any written
principle, there being none.

## Project Structure

### Documentation (this feature)

```text
specs/004-macos-arm64-homebrew/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output — 10 findings, all verified against live state
├── data-model.md        # Phase 1 output — release/formula/tap entities and their invariants
├── quickstart.md        # Phase 1 output — validation scenarios V1-V10
├── contracts/
│   ├── formula.md       # What the published formula must contain
│   ├── release.md       # What a release must produce
│   └── install-script.md# Platform detection and refusal behaviour
├── checklists/
│   └── requirements.md  # Spec quality checklist (16/16)
└── tasks.md             # Phase 2 output (/speckit-tasks — NOT created here)
```

### Source Code (repository root)

```text
.github/workflows/
├── ci.yml                    # unchanged
├── release.yml               # MODIFY — matrix 3→1; new publish-formula job
└── tap-drift-check.yml       # NEW — scheduled tap-vs-release version comparison

Formula/
└── kn.rb                     # MODIFY — flatten to single platform, add guards

install.sh                    # MODIFY — narrow detection, keep Rosetta probe
README.md                     # MODIFY — platforms, primary install, remove Windows/APT
README_ES.md                  # MODIFY — same
RELEASE.md                    # MODIFY — replace manual tap steps with automated flow
GITHUB_PAGES.md               # untouched by this feature

kn.spec                       # DELETE — RPM recipe, stale at Version 0.1.0
debian/                       # DELETE — Debian packaging
install.ps1                   # DELETE — Windows installer
docs/packaging/               # DELETE — exists on the 002 branch; delete after rebase
```

**Structure Decision**: No source layout change. `cli/` is untouched — this feature does
not alter what `kn` does, only which machines it is built for and how it reaches them.
Changes concentrate in three places: the release pipeline (`.github/workflows/`), the
distribution recipe (`Formula/kn.rb` plus the tap), and the honesty of the documentation.

## Implementation Phases

Ordered by the spec's user-story priorities, with one ordering gate imposed by research
R9/R10 rather than by priority.

**Phase 1 — Setup and prerequisites (blocking)**
Merge `002` so a license exists on the default branch; rebase this branch on `dev`; create
the fine-grained PAT and store it as `TAP_GITHUB_TOKEN`. Nothing downstream can be
verified without these. Two of the three require the maintainer personally.

**Phase 2 — US1: a correct, current published formula (P1, MVP)**
Flatten `Formula/kn.rb` to the single Apple Silicon artifact with `depends_on` guards and
a declared license; publish the corrected formula to the tap so `brew install` yields
0.10.0 rather than 0.9.0. Delivers the maintainer's stated goal on its own.

**Phase 3 — US2: automated formula publication (P2)**
Add the `publish-formula` job to `release.yml`, gated on successful artifact builds,
computing both checksums from what was actually published. Add the scheduled drift check.
This is what stops Phase 2 from silently regressing.

**Phase 4 — US3: single-artifact releases (P3)**
Reduce the build matrix to `aarch64-apple-darwin`, keeping the artifact name intact.

**Phase 5 — US4: honest documentation (P4)**
Narrow `install.sh` detection preserving the Rosetta probe; correct both READMEs and the
release-notes template.

**Phase 6 — US5: delete retired packaging (P5)**
Remove `kn.spec`, `debian/`, `install.ps1`, `docs/packaging/`, and every inbound
reference catalogued in research R8.

**Phase 7 — Polish**
Run the full quickstart V1-V10 and record each outcome, including anything unverified and
why.

## Complexity Tracking

No constitution gate was violated, there being no constitution. One deliberate deviation
from the simplest possible approach is worth recording:

| Decision | Simpler alternative | Why the simpler one was rejected |
|---|---|---|
| Keep the authored formula in-repo and render into the tap | Generate the formula entirely inside the workflow YAML | Buries install logic in CI config where it cannot be reviewed in a normal PR alongside the code it installs, nor linted locally with `brew style` |
| Add an independent scheduled drift check | Assert correctness inside the release job only | An in-job assertion cannot detect a push that later reverted, or a credential that expired months after the last release — precisely the class of failure that produced the current 0.9.0/0.10.0 gap |
| Retain the `x86_64` branch in `install.sh` as a Rosetta probe | Delete all non-arm64 detection | Deleting it would make the script refuse to install on the maintainer's own M4 whenever invoked from a translated terminal — a regression created by the cleanup itself |
