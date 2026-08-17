# Contract: Public Repository Surface

**Feature**: 002-public-ready-repo
**Date**: 2026-08-04

The interface this feature exposes is not an API — it is the repository's public surface: what an outside visitor, GitHub itself, and the package builds each see. This contract states what each consumer is entitled to rely on after the feature ships.

---

## C1 — Root layout contract

**Consumer**: a first-time visitor, and agent tooling that discovers files by convention.

**Guarantee**: the repository root contains exactly these seven documents and no other Markdown.

```
LICENSE
README.md
README_ES.md
CHANGELOG.md
CONTRIBUTING.md
AGENTS.md
CLAUDE.md
```

**Breaking change**: adding an eighth root document, or moving any of these out of the root.

`AGENTS.md` and `CLAUDE.md` are load-bearing at this exact path — agent tooling discovers them by convention, so relocating them breaks the project's own workflow. `LICENSE` must be exactly this name with no extension or GitHub's license detection fails.

**Verify**: `ls *.md LICENSE` at the root returns these seven and nothing else.

---

## C2 — License detection contract

**Consumer**: GitHub's licence detector, and any visitor evaluating reuse terms.

**Guarantee**:

- A file named `LICENSE` (no extension) exists at the repository root.
- It contains unmodified MIT licence text.
- The copyright line reads `Copyright (c) 2026 Kevin Barroso`.
- The repository's API metadata reports a non-null licence, and the landing page displays "MIT License".

**Breaking change**: renaming the file, or editorializing the licence body — GitHub's detector matches against known licence texts and any modification drops it to `licenseInfo: null`, which is the exact failure this feature exists to fix.

**Verify**: `gh repo view kobogithub/knowledge --json licenseInfo` returns a non-null `MIT` entry.

---

## C3 — Documentation location contract

**Consumer**: anyone following a link to a maintainer process document.

**Guarantee**: every relocated document is reachable at its new path, and `docs/README.md` indexes every document under `docs/`.

| Document | Guaranteed path |
|---|---|
| Homebrew guide | `docs/packaging/HOMEBREW.md` |
| Debian guide | `docs/packaging/DEBIAN.md` |
| RPM guide | `docs/packaging/RPM.md` |
| Release process | `docs/release/RELEASE.md` |
| GitHub Pages setup | `docs/release/GITHUB_PAGES.md` |
| Security audit report | `docs/security/SECURITY_AUDIT_REPORT.md` |

**Explicitly NOT guaranteed**: previously published external URLs pointing at the old root paths. Those break. The spec accepts this for maintainer-only content (Edge Cases, Assumptions).

**Verify**: every relative link in tracked Markdown resolves; `docs/README.md` reaches every file under `docs/`.

---

## C4 — Package build contract

**Consumer**: `rpmbuild` via `kn.spec`, and `dpkg-buildpackage` via `debian/rules`.

**Guarantee**:

- Both builds succeed after the relocation.
- Both packages still ship every relocated document.
- The documents ship exactly once each — no path is copied twice.

**Mechanism**: each recipe's existing conditional recursive copy (`if [ -d docs ]; then cp -r docs …`) already carries the new subdirectories. The per-file copy lines that named the old root paths are deleted, not repointed — repointing them would ship each document twice.

**Consequence, contractually acknowledged**: the installed path changes.

| Before | After |
|---|---|
| `/usr/share/doc/kn/HOMEBREW.md` | `/usr/share/doc/kn/docs/packaging/HOMEBREW.md` |
| `/usr/share/doc/kn/DEBIAN.md` | `/usr/share/doc/kn/docs/packaging/DEBIAN.md` |

Any document stating the old installed path is wrong until corrected — three such statements exist (`DEBIAN.md:121`, `RPM.md:196`, `RPM.md:197`).

**Breaking change**: any build recipe referencing a repository-root path for a relocated document.

**Verify**: build both packages; confirm no recipe names a path that does not exist; confirm each relocated document appears exactly once in the built package.

---

## C5 — Contribution intake contract

**Consumer**: anyone opening an issue or a pull request.

**Guarantee**:

- Opening a new issue offers a choice between a bug report and a feature request, both as structured forms with required fields.
- A blank issue cannot be opened (`blank_issues_enabled: false`).
- Opening a pull request pre-fills a description prompting for summary, related initiative, and verification performed.
- `CONTRIBUTING.md` states the branch hierarchy, the pull request target for each branch type, the conventional commit format with its allowed types, and how initiatives are specified before implementation.

**Breaking change**: removing a template, or allowing blank issues.

**Verify**: open the new-issue chooser and the new-PR form on the repository.

---

## C6 — Unchanged surface contract

**Consumer**: existing users of the CLI and the release pipeline.

**Guarantee** — these are byte-identical before and after this feature:

- Everything under `cli/` — no command added, removed or altered (FR-021)
- Everything under `skills/`, `stacks/` and `agents/` (FR-022)
- `.github/workflows/release.yml` (FR-023)
- `install.sh`, `install.ps1`, `uninstall.sh`, `index.html`, `CNAME` — verified to contain no reference to any relocated document
- Historical `CHANGELOG.md` entries — a changelog is a record of what shipped, not a live index (research R4)

**Verify**: `git diff --stat` against the merge base touches no path under `cli/`, `skills/`, `stacks/`, `agents/`, and does not touch `release.yml`.

---

## Contract test summary

| Contract | Automated | Manual |
|---|---|---|
| C1 root layout | root Markdown count | — |
| C2 licence detection | `gh repo view --json licenseInfo` | landing page shows "MIT License" |
| C3 documentation location | link check in CI (`lychee`, internal links) | — |
| C4 package build | — | `rpmbuild` + `debian/rules`, inspect shipped paths |
| C5 contribution intake | — | open the new-issue chooser and new-PR form |
| C6 unchanged surface | `git diff --stat` path assertions | — |
