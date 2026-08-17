# Implementation Plan: Public-Ready Repository

**Branch**: `epic/002-public-ready-repo` | **Date**: 2026-08-04 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/002-public-ready-repo/spec.md`

## Summary

Close the gaps that make the already-public `knowledge` repository unusable to an outside visitor: add an MIT license, move six maintainer-only process documents out of the root into grouped subdirectories under `docs/`, delete one stale checklist, restructure both READMEs so installation precedes the feature catalog, add a contributing guide with issue and pull request templates, and remove the one maintainer-specific absolute path.

The approach is a documentation reorganization with one genuine engineering risk: two package build recipes copy the relocated files by name from the repository root and will fail after the move. Both recipes already copy `docs/` recursively, so the fix is to delete the now-dangling per-file copy lines and correct the three documents that state the resulting installed paths. A `lychee` link-check job is added to CI so "zero broken links" becomes verifiable on every future change rather than once.

No Rust source is touched. No CLI behavior changes.

## Technical Context

**Language/Version**: Markdown (documentation), YAML (GitHub issue forms and CI), plus untouched Rust 2021 / `kn` v0.10.0 and the existing packaging recipes (RPM spec, Debian rules)

**Primary Dependencies**: GitHub issue forms schema; `lychee-action` for link checking (new CI dependency, quality gate only)

**Storage**: N/A — no runtime data

**Testing**: Existing `ci.yml` (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo build --release`, `cargo test --release`, `shellcheck install.sh`, install-script test) plus one new link-check job. Package builds (`rpmbuild`, `debian/rules`) verified manually as the regression gate for the relocation.

**Target Platform**: GitHub repository surface (landing page, issue/PR creation flows), GitHub Pages at `kn.foxlabar.online`, and the `.deb`/`.rpm`/Homebrew artifacts that embed the documentation

**Project Type**: CLI tool distributed as packages, with an accompanying documentation site

**Performance Goals**: N/A — SC-004's "under one minute to reach the install command" is a reading-effort target met by section ordering, not a runtime metric

**Constraints**:
- CLI behavior, stack presets and the skill catalog must be unchanged (FR-021, FR-022)
- `release.yml` must not be modified (FR-023)
- Package builds must still succeed and still ship the documentation (FR-023)
- Historical CHANGELOG entries must not be rewritten (research R4)

**Scale/Scope**: 12 root Markdown files reduced to 7; 6 documents relocated; 1 deleted; 4 new files (`LICENSE`, `CONTRIBUTING.md`, `docs/README.md`, plus the `.github` template set); ~10 inbound references corrected across Markdown, an RPM spec and a Debian rules file

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Status: PASS (vacuously) — no constraints to check.**

`.specify/memory/constitution.md` is the unmodified spec-kit template: every principle is still a `[PRINCIPLE_N_NAME]` / `[PRINCIPLE_N_DESCRIPTION]` placeholder, and the governance section is `[GOVERNANCE_RULES]`. The project has never run `/speckit-constitution`, so there are no ratified principles for this feature to violate.

This is recorded rather than silently skipped: a future initiative that fills the constitution should re-examine this plan's choices — particularly the decision to add a CI dependency (research R6) — against whatever principles get ratified.

**Post-Phase 1 re-check**: unchanged. The design introduces no project structure, no new runtime dependency, and no abstraction that a constitution would typically govern.

## Project Structure

### Documentation (this feature)

```text
specs/002-public-ready-repo/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output — 11 decisions
├── data-model.md        # Phase 1 output — document inventory and transitions
├── quickstart.md        # Phase 1 output — verification guide
├── contracts/
│   └── repository-surface.md   # Phase 1 output — public layout contract
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2 output (/speckit-tasks — NOT created here)
```

### Source Code (repository root)

The affected surface, showing the end state. Files marked `←` are new, `↷` moved, `✎` edited, `✗` deleted.

```text
knowledge/
├── LICENSE                          ← MIT, "Copyright (c) 2026 Kevin Barroso"
├── CONTRIBUTING.md                  ← derived from AGENTS.md rules
├── README.md                        ✎ reordered: what → install → first run → features
├── README_ES.md                     ✎ same structure as README.md
├── CHANGELOG.md                     ✎ new entry only; history untouched
├── AGENTS.md                        (unchanged — agent tooling discovers it here)
├── CLAUDE.md                        (unchanged — same reason)
├── HOMEBREW.md                      ↷ docs/packaging/HOMEBREW.md
├── DEBIAN.md                        ↷ docs/packaging/DEBIAN.md
├── RPM.md                           ↷ docs/packaging/RPM.md
├── RELEASE.md                       ↷ docs/release/RELEASE.md
├── GITHUB_PAGES.md                  ↷ docs/release/GITHUB_PAGES.md
├── SECURITY_AUDIT_REPORT.md         ↷ docs/security/SECURITY_AUDIT_REPORT.md
├── RELEASE_v0.1.0_CHECKLIST.md      ✗ deleted (stale, unreferenced)
│
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml           ← issue form, required fields
│   │   ├── feature_request.yml      ← issue form
│   │   └── config.yml               ← blank_issues_enabled: false
│   ├── pull_request_template.md     ← summary / spec / verification
│   └── workflows/
│       ├── ci.yml                   ✎ + link-check job
│       └── release.yml              (untouched — FR-023)
│
├── docs/
│   ├── README.md                    ← index required by FR-008
│   ├── packaging/                   ← new group
│   ├── release/                     ← new group
│   ├── security/                    ← new group
│   ├── ENVIRONMENT_VARIABLES.md     ✎ link to relocated audit report
│   └── (ARCHITECTURE.md, GETTING_STARTED.md, adr/, reports/ … unchanged)
│
├── kn.spec                          ✎ drop `cp HOMEBREW.md DEBIAN.md` (line 61)
├── debian/rules                     ✎ drop `cp HOMEBREW.md` (line 34)
│
├── specs/001-personal-stacks/
│   └── quickstart.md                ✎ remove /Users/kobo absolute path (line 9)
│
├── cli/                             (untouched — FR-021)
├── skills/  stacks/  agents/        (untouched — FR-022)
└── install.sh  install.ps1  index.html  CNAME   (untouched — verified no refs)
```

**Structure Decision**: Documents are grouped into `docs/packaging/`, `docs/release/` and `docs/security/` rather than dropped flat into `docs/`, following the precedent already set by `docs/adr/` and `docs/reports/`. Filenames are preserved so the change registers as `git mv` renames and history follows each file. Full rationale and the rejected alternatives are in [research.md](./research.md) R1.

## Implementation Phases

Ordered by the spec's user story priorities, each phase independently shippable.

| Phase | Story | Work | Risk |
|---|---|---|---|
| 1 | US1 (P1) | `LICENSE` + license sections in both READMEs | None |
| 2 | US2 (P2) | `git mv` the six documents, delete the stale checklist, fix the two build recipes, correct inbound references, create `docs/README.md` | **Build-breaking** — see below |
| 3 | US3 (P3) | Reorder both READMEs; verify command/doc parity; add the beads status note | Translation drift |
| 4 | US4 (P4) | `CONTRIBUTING.md`, issue forms, PR template | None |
| 5 | cross-cutting | Remove the absolute path; add the CI link-check job | None |

**Phase 2 is the only phase that can break a build.** Its ordering is mandatory:

1. `git mv` the six files into their new locations
2. **Immediately** delete `kn.spec:61`'s `cp HOMEBREW.md DEBIAN.md …` and `debian/rules:34`'s `cp HOMEBREW.md …` — the recursive `docs/` copy each recipe already performs keeps the documents shipping
3. Correct the installed-path references in `docs/packaging/DEBIAN.md:121`, `docs/packaging/RPM.md:196` and `:197` — these describe paths *inside an installed package*, which shift from `/usr/share/doc/kn/HOMEBREW.md` to `/usr/share/doc/kn/docs/packaging/HOMEBREW.md`, and a repo-relative link sweep will not catch them
4. Correct the repo-relative links: `README.md:233-235` (the three packaging docs), `docs/ENVIRONMENT_VARIABLES.md:222` (the audit report), `RELEASE.md:70` (moves with the file)
5. Verify both package builds

Leaving step 2 for later means every intermediate commit has a broken RPM and Debian build.

## Complexity Tracking

> Fill ONLY if Constitution Check has violations that must be justified

No violations — the constitution contains no ratified principles (see Constitution Check).

One addition is worth flagging even though no gate forbids it:

| Addition | Why needed | Simpler alternative rejected because |
|---|---|---|
| `lychee` link-check job in `ci.yml` | SC-003 requires zero broken links, and this feature moves six documents — exactly the change that breaks links silently. Nothing in CI checks Markdown today. | A one-time manual sweep satisfies SC-003 at merge and then rots on the next docs change. Scoped to internal links only, so third-party outages cannot fail CI. |
