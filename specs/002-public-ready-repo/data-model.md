# Phase 1 Data Model: Public-Ready Repository

**Feature**: 002-public-ready-repo
**Date**: 2026-08-04

This feature has no runtime data. The "model" is the repository's document inventory: the entities are documents, their attributes determine where they belong, and the state transitions are the moves this feature performs.

---

## Entities

### Root entry document

A document a first-time visitor or an agent tool is expected to find at the repository root.

| Attribute | Values |
|---|---|
| Audience | Outside visitor, or agent tooling that discovers files by convention |
| Location | Repository root — mandatory, not preference |
| Admission rule | Either conventionally expected at the root by an external consumer (GitHub, agent tooling, package ecosystems), or the primary reading entry point |

**Validation**: exactly seven documents may hold this classification when the feature completes. Any eighth is a defect against FR-004.

| Document | Why it qualifies |
|---|---|
| `README.md` | Primary entry point; rendered by GitHub on the landing page |
| `README_ES.md` | Spanish translation of the entry point |
| `CHANGELOG.md` | Reader-facing; conventionally located at the root |
| `LICENSE` | GitHub's license detection requires this exact root path (FR-002) |
| `CONTRIBUTING.md` | GitHub surfaces it when a contributor opens an issue or PR |
| `AGENTS.md` | Agent tooling discovers it at the root by convention |
| `CLAUDE.md` | Same — moving it breaks the project's own workflow |

### Maintainer process document

A document describing packaging, release, publishing or audit procedure, useful only to whoever cuts a release.

| Attribute | Values |
|---|---|
| Audience | Maintainer only |
| Location | Grouped subdirectory under `docs/` |
| Group | `packaging` \| `release` \| `security` |
| Staleness | Current, or stale (describes a superseded process) |

**Validation**: none may remain at the repository root. A stale one must not be presented as current process (FR-006).

### Contribution template

A structured form presented at the moment a contributor opens an issue or a pull request.

| Attribute | Values |
|---|---|
| Kind | `bug_report` \| `feature_request` \| `pull_request` |
| Format | Issue forms are YAML with validated fields; pull request templates are Markdown (no forms equivalent exists) |
| Location | `.github/ISSUE_TEMPLATE/` for issues; `.github/pull_request_template.md` for PRs |

**Validation**: opening a new issue must offer a choice between distinct bug and feature templates, and must not offer a blank issue (`config.yml` sets `blank_issues_enabled: false`).

### Inbound reference

A pointer from one file to a document this feature relocates. The entity that makes the reorganization risky.

| Attribute | Values |
|---|---|
| Kind | `repo_link` (relative Markdown link) \| `build_path` (a path a build recipe copies) \| `installed_path` (a path inside an installed package) \| `prose_mention` (a filename in text, not a link) |
| Resolution required | `repo_link`, `build_path`, `installed_path` → yes; `prose_mention` → no |

**Why the distinction matters**: a repo-relative link sweep finds only `repo_link`. The other three kinds are invisible to it — and two of them break the package builds.

---

## State transitions

### Document relocation

```
root/<DOC>.md  ──git mv──▶  docs/<group>/<DOC>.md
                              │
                              ├──▶ update every repo_link pointing at it
                              ├──▶ update every installed_path it documents
                              └──▶ remove every build_path that copied it by name
```

Filenames are preserved across the transition so `git` records a rename and history follows the file.

### Stale document retirement

```
root/RELEASE_v0.1.0_CHECKLIST.md  ──delete──▶  (git history only)
```

No inbound references exist (verified), so no reference updates follow.

---

## The concrete inventory

### Documents relocated

| Document | From | To | Group |
|---|---|---|---|
| `HOMEBREW.md` | root | `docs/packaging/` | packaging |
| `DEBIAN.md` | root | `docs/packaging/` | packaging |
| `RPM.md` | root | `docs/packaging/` | packaging |
| `RELEASE.md` | root | `docs/release/` | release |
| `GITHUB_PAGES.md` | root | `docs/release/` | release |
| `SECURITY_AUDIT_REPORT.md` | root | `docs/security/` | security |

### Documents deleted

| Document | Reason |
|---|---|
| `RELEASE_v0.1.0_CHECKLIST.md` | Stale by nine minor versions; zero inbound references; recurring process retained in `RELEASE.md` |

### Documents created

| Document | Satisfies |
|---|---|
| `LICENSE` | FR-001, FR-002 |
| `CONTRIBUTING.md` | FR-015, FR-016 |
| `docs/README.md` | FR-008 |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | FR-017 |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | FR-017 |
| `.github/ISSUE_TEMPLATE/config.yml` | FR-017 |
| `.github/pull_request_template.md` | FR-018 |

### Inbound references to resolve

| Location | Kind | Target | Action |
|---|---|---|---|
| `kn.spec:61` | `build_path` | `HOMEBREW.md`, `DEBIAN.md` | **Delete the line** — recursive `docs/` copy covers it |
| `debian/rules:34` | `build_path` | `HOMEBREW.md` | **Delete the line** — same |
| `DEBIAN.md:121` | `installed_path` | `/usr/share/doc/kn/HOMEBREW.md` | Repoint to `/usr/share/doc/kn/docs/packaging/HOMEBREW.md` |
| `RPM.md:196` | `installed_path` | `/usr/share/doc/kn/HOMEBREW.md` | Same |
| `RPM.md:197` | `installed_path` | `/usr/share/doc/kn/DEBIAN.md` | Repoint to `…/docs/packaging/DEBIAN.md` |
| `README.md:233` | `repo_link` | `./HOMEBREW.md` | Repoint to `./docs/packaging/HOMEBREW.md` |
| `README.md:234` | `repo_link` | `./DEBIAN.md` | Repoint |
| `README.md:235` | `repo_link` | `./RPM.md` | Repoint |
| `docs/ENVIRONMENT_VARIABLES.md:222` | `repo_link` | `../SECURITY_AUDIT_REPORT.md` | Repoint to `./security/SECURITY_AUDIT_REPORT.md` |
| `RELEASE.md:70` | `prose_mention` | the three packaging docs | No change — moves with the file, mentions filenames not links |
| `CHANGELOG.md` ×7 | `prose_mention` | relocated files | **No change** — historical record (research R4) |

### Files edited for other requirements

| Location | Requirement | Change |
|---|---|---|
| `README.md` | FR-009..FR-014 | Reorder sections; add license section; add beads status note |
| `README_ES.md` | FR-003, FR-012 | Same structure and claims as `README.md` |
| `CHANGELOG.md` | — | New entry for this reorganization; history untouched |
| `specs/001-personal-stacks/quickstart.md:9` | FR-019, FR-020 | Replace `/Users/kobo/Github/personal/knowledge` with a repo-relative instruction |
| `.github/workflows/ci.yml` | SC-003 | Add internal-link-check job |

---

## Invariants

Checkable at completion:

1. Exactly 7 Markdown-or-license documents at the repository root (FR-004, SC-002 allows "at most 7").
2. Zero documents classified `maintainer process` remain at the root (FR-005).
3. Every `repo_link`, `build_path` and `installed_path` reference resolves (FR-007, SC-003).
4. Zero tracked files contain `/Users/kobo` (FR-019, SC-005).
5. Both package builds succeed and still ship every relocated document (FR-023).
6. Every document under `docs/` is reachable from `docs/README.md` (FR-008).
7. No file under `cli/`, `skills/`, `stacks/` or `agents/` is modified (FR-021, FR-022).
8. `.github/workflows/release.yml` is byte-identical to its pre-feature state (FR-023).
