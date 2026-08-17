# Phase 0 Research: Public-Ready Repository

**Feature**: 002-public-ready-repo
**Date**: 2026-08-04

All unknowns from the Technical Context are resolved below. Each decision records what was chosen, why, and what was rejected.

---

## R1 — Target layout for the relocated maintainer documents

**Decision**: Move the six documents into purpose-named subdirectories under `docs/`, keeping their existing filenames.

| From (root) | To |
|---|---|
| `HOMEBREW.md` | `docs/packaging/HOMEBREW.md` |
| `DEBIAN.md` | `docs/packaging/DEBIAN.md` |
| `RPM.md` | `docs/packaging/RPM.md` |
| `RELEASE.md` | `docs/release/RELEASE.md` |
| `GITHUB_PAGES.md` | `docs/release/GITHUB_PAGES.md` |
| `SECURITY_AUDIT_REPORT.md` | `docs/security/SECURITY_AUDIT_REPORT.md` |

**Rationale**: `docs/` already holds six top-level files in SCREAMING_CASE (`ARCHITECTURE.md`, `GETTING_STARTED.md`, `ENVIRONMENT_VARIABLES.md`, `MCP_ARCHITECTURE.md`, `AGENT_MODEL_STRATEGY.md`, `GEMINI_MCP_CONFIG.md`) plus `adr/` and `reports/` subdirectories. Dropping six more files flat would make `docs/` as cluttered as the root we are fixing. Subdirectories satisfy FR-005's "grouped so their purpose is evident from location", and the existing `adr/`+`reports/` precedent means this is not a new convention.

Filenames are preserved rather than lowercased: it keeps the change a pure `git mv` (so history follows the file and reviewers see renames, not delete+add), and it matches the SCREAMING_CASE already used inside `docs/`.

**Alternatives rejected**:
- *Flat in `docs/`* — 12 files at one level, no grouping, fails FR-005's intent.
- *Rename to lowercase while moving* — inconsistent with the six files already in `docs/`, and turns a rename into a larger diff for no reader benefit.
- *A single `docs/maintainers/` bucket* — hides the packaging-vs-release-vs-security distinction that makes the location self-explanatory.

---

## R2 — Package builds break on the move (highest-risk item)

**Decision**: Delete the explicit per-file copy lines from both package recipes and rely on the recursive `docs/` copy each recipe already performs.

Two build recipes copy the relocated files by name from the repository root:

- `kn.spec:61` — `cp HOMEBREW.md DEBIAN.md %{buildroot}%{_docdir}/kn/`
- `debian/rules:34` — `cp HOMEBREW.md $(CURDIR)/debian/kn/usr/share/doc/kn/`

After the move these paths do not exist, `cp` exits non-zero, and both the RPM and Debian builds fail. This is the single change in this feature that can break something outside documentation, and FR-023 requires the packaging automation to keep working.

Both recipes already contain a conditional recursive copy immediately below those lines (`if [ -d docs ]; then cp -r docs ...`), which is recursive and therefore already picks up the new subdirectories. So the fix is a deletion, not a rewrite: remove the two now-dangling `cp` lines and the documents still ship inside the packages.

**Consequence to propagate**: the installed location changes from `/usr/share/doc/kn/HOMEBREW.md` to `/usr/share/doc/kn/docs/packaging/HOMEBREW.md`. Three documents state the old installed paths and must be corrected — `DEBIAN.md:121`, `RPM.md:196`, `RPM.md:197`. These describe runtime paths inside an installed package, not repository paths, which is why a naive find-and-replace over repo-relative links would miss them.

**Alternatives rejected**:
- *Keep copying by name from the new paths* — works, but duplicates files already covered by the recursive `docs/` copy, so each document would ship twice at two different paths inside the package.
- *Leave symlinks at the root* — defeats the purpose of decluttering the root and confuses `git` and archive builds.

**Verification**: build both packages after the change, or at minimum confirm no recipe references a path that no longer exists.

---

## R3 — Disposition of `RELEASE_v0.1.0_CHECKLIST.md`

**Decision**: Delete it.

**Rationale**: It is a one-time checklist for the v0.1.0 release; the project is on v0.10.0, nine minor versions later. Nothing in the repository links to it (verified — the only hits are this feature's own spec). The recurring process it was derived from already lives in `RELEASE.md`, which is being kept and relocated. Git history preserves the file for anyone who wants it, so deletion loses nothing recoverable.

FR-006 permits either archiving or deleting; deletion is chosen because archiving a stale one-off would add a file to `docs/` that no reader ever benefits from.

**Alternative rejected**: *Archive under `docs/release/archive/`* — preserves clutter under a different name and invites the question "is this current?" that FR-006 exists to eliminate.

---

## R4 — Whether to rewrite CHANGELOG references

**Decision**: Do not modify historical CHANGELOG entries. Record the reorganization as a new entry instead.

`CHANGELOG.md` mentions the relocated files at lines 490, 496, 502, 536, 537, 538 and 539 — all inside released-version sections describing what shipped at the time.

**Rationale**: A changelog is a historical record. Rewriting past entries to point at today's paths would make it claim that v0.4.0 shipped files at locations that did not exist then. These are prose mentions of filenames, not navigational links, so leaving them does not produce a broken link for a reader.

FR-007 is satisfied because it governs *references that resolve to a document* — the CHANGELOG's mentions are not links.

**Alternative rejected**: *Update every mention for consistency* — falsifies the historical record to fix a problem that does not exist.

---

## R5 — Issue and pull request template format

**Decision**: GitHub issue forms — YAML files under `.github/ISSUE_TEMPLATE/` (`bug_report.yml`, `feature_request.yml`) plus a `config.yml`, and a Markdown `.github/pull_request_template.md`.

**Rationale**: Issue forms render as structured fields with required-field validation, so a bug report cannot be submitted without a version and reproduction steps. That is exactly what FR-017 and SC-007 ask for. `config.yml` with `blank_issues_enabled: false` forces reporters through a template. Pull request templates have no forms equivalent — Markdown is the only mechanism, which settles FR-018 by default.

**Alternative rejected**: *Legacy Markdown issue templates* — a pre-filled text blob contributors delete wholesale; no validation, no structure.

---

## R6 — Verifying "zero broken links" (SC-003)

**Decision**: Add a `lychee` link-check job to `.github/workflows/ci.yml`, scoped to tracked Markdown, plus a one-time manual sweep during implementation.

**Rationale**: CI today runs `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, `shellcheck install.sh`, and an install-script test — nothing checks Markdown. SC-003 and SC-007 are therefore not verifiable today, and the reorganization in this feature is precisely the kind of change that silently breaks links. A CI job makes the criterion checkable on every future change rather than once.

This does **not** touch `release.yml`, so FR-023's protection of the release automation holds. `ci.yml` is a quality gate, not release automation.

**Scoping note**: the check runs against internal/relative links. External URL checking is left off to avoid CI failures from third-party outages unrelated to this repository — a real risk here, since the maintainer just hit exactly that failure mode with a paused third-party badge service.

**Alternatives rejected**:
- *Manual verification only* — satisfies SC-003 once, then rots on the next docs change.
- *`markdown-link-check`* — per-file configuration and noisier output than `lychee` for the same result.

---

## R7 — MIT license attribution

**Decision**: `Copyright (c) 2026 Kevin Barroso`.

**Rationale**: The first commit in the repository is dated 2026, so a single-year notice is correct and no range is needed. The name matches the configured git author (`Barroso Kevin <kobouharriet@gmail.com>`), rendered in natural order. FR-001 requires the correct holder and year; the checklist warns that a leftover placeholder is worse than no license, so this is stated concretely rather than left to implementation.

GitHub's license detection (FR-002, SC-001) requires the file be named `LICENSE` (no extension) at the repository root and contain unmodified MIT text — any editorializing breaks automatic detection.

---

## R8 — README restructure

**Decision**: Reorder to `title + one-line description` → `what it is / who it is for` → `install` → `first commands` → `features` → `documentation index` → `license`. Apply the identical structure to `README_ES.md`.

**Rationale**: The README today opens with the feature catalog: `What Problem Does This Solve?` at line 14, then `Features` at 27 with five subsections, and installation does not appear until line 115. FR-010 requires install before the catalog, and SC-004 gives a one-minute budget for a new reader to reach it. Moving the install section above the catalog is the whole fix; the feature catalog keeps its content and simply moves down.

FR-012 requires structural equivalence between the two languages, so both files get the same section order — this is a reordering exercise in both, not a retranslation.

---

## R9 — Command/documentation parity (FR-013)

**Decision**: Document all nine shipped commands; keep `kn beads` documented and add a status note.

The CLI ships nine commands: `agents`, `beads`, `doctor`, `init`, `mcp`, `skills`, `stack`, `sync`, `update` (`cli/src/commands/` holds ten files, but `mod.rs` is the module declaration, not a command).

The tension FR-014 exists for: `README.md` documents a `kn beads template` family while ADR-006 records that beads was removed from the project's workflow. Both are true — the command genuinely still exists in `cli/src/commands/beads.rs`, and the workflow genuinely moved to spec-kit. A visitor reading both concludes the documentation is stale.

**Resolution**: keep documenting the command (removing it from the README would violate FR-013, since it ships) and add one sentence stating that this project's own workflow uses spec-kit per ADR-006, while `kn beads` remains for users who use beads independently. FR-021 forbids changing CLI behavior, so removing the command is out of scope regardless.

**Verification for FR-013**: enumerate the command modules and confirm each appears in both READMEs.

---

## R10 — Removing the maintainer-specific absolute path

**Decision**: In `specs/001-personal-stacks/quickstart.md:9`, replace `cd /Users/kobo/Github/personal/knowledge` with a repository-relative instruction.

**Rationale**: FR-020 requires the instruction stay executable and keep its intent — the step exists to tell the reader to be at the repository root, so the replacement must say that rather than delete the line. This is the only occurrence in any tracked file (verified).

Editing a completed initiative's document is acceptable here: the change corrects a portability defect without altering what the initiative decided or delivered.

---

## R11 — Contributing guide content

**Decision**: Derive `CONTRIBUTING.md` from the branching and commit rules already written in `AGENTS.md`, and link to the spec-kit workflow rather than restating it.

**Rationale**: FR-015 and FR-016 ask for the branch hierarchy, PR targets, conventional commit format with allowed types, and the spec-driven workflow. All of this already exists in `AGENTS.md`, written for agents. `CONTRIBUTING.md` needs the same rules addressed to a human contributor. Deriving from one source keeps the two from drifting; where detail already exists (`docs/adr/`, the `standard-commits` skill), the guide links rather than duplicates.

---

## Open items deliberately left to implementation

- Exact prose of the README opening paragraph — content, not structure; no decision needed at plan time.
- Whether `docs/README.md` (the index required by FR-008) lists every document or only the top-level groupings — decided when the final file set is in place.
