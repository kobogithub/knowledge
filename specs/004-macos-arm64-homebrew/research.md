# Research: kn targets macOS Apple Silicon only, distributed via Homebrew

**Feature**: `004-macos-arm64-homebrew` | **Date**: 2026-08-16

All findings below were verified against the repository and the live GitHub/DNS state on
2026-08-16, not assumed from convention.

---

## R1 — Why the published formula drifted, and what actually has to change

**Finding**: The drift is not a bug in a script. There is no script. `.github/workflows/release.yml`
contains exactly two jobs, `build-binaries` and `create-release`. Nothing in CI has ever
touched Homebrew.

The real process lives in `RELEASE.md` as prose, under a heading literally titled
**"Post-Release Tasks → Update Package Repositories (When Available)"**:

```bash
cd /path/to/homebrew-knowledge
cp ~/knowledge/Formula/kn.rb Formula/
git add Formula/kn.rb && git commit -m "Update kn to v0.2.0" && git push
```

Preceded by a separate manual step (`RELEASE.md:175-193`) that downloads the source
tarball, runs `sha256sum` by hand, and `vim`s the result into `Formula/kn.rb`.

So publishing correctly requires two manual, undocumented-as-mandatory steps, filed under
a section marked "when available". At v0.10.0 the first step was done (the in-repo formula
says 0.10.0) and the second was not (the tap still says 0.9.0).

**Decision**: Automate the whole path from published artifact to published formula. Treat
the manual procedure in `RELEASE.md` as the thing being deleted, not the thing being
documented better.

**Rationale**: A checklist that was skipped once will be skipped again. The spec's FR-010
exists precisely because fixing the version without fixing the process reopens the gap at
the next release.

**Alternatives considered**:
- *Make the manual step more prominent in `RELEASE.md`* — rejected. It was already written
  down; prominence was not the failure mode.
- *A pre-release checklist gate in CI* — rejected as the primary fix. It catches the
  omission but still requires a human to act, and would have blocked the v0.10.0 release
  rather than completing it.

---

## R2 — Single source of truth for the formula

**Finding**: The formula exists in two places today — `Formula/kn.rb` in this repo and
`Formula/kn.rb` in `kobogithub/homebrew-knowledge`. They are kept in sync by a `cp`. This
duplication *is* the drift mechanism: the two copies currently disagree (0.10.0 vs 0.9.0),
and nothing detects that.

**Decision**: Keep the authored formula in this repository as the single source of truth,
and have the release workflow render the version and checksum into it and push the result
to the tap. The tap becomes a **published output**, never hand-edited.

**Rationale**: Keeping the formula in-repo preserves reviewability — changes to install
logic arrive through normal PR review alongside the code they install. Making the tap a
generated output removes the human `cp` that failed. The version and checksum are the only
fields that change per release, and both are derivable from the release itself, so nothing
about the release needs to be authored twice.

**Alternatives considered**:
- *Delete the in-repo copy; generate the formula wholly inside the workflow* — rejected.
  It buries install logic in YAML where it is harder to review and impossible to lint
  locally with `brew style`.
- *Delete the tap copy; install from the repo directly* — not possible. Homebrew taps must
  be repositories named `homebrew-<name>`; a formula cannot be served from an arbitrary
  repo path.
- *Git submodule linking the tap into this repo* — rejected as disproportionate, and
  submodules were already rejected in this project's ADR-004 for similar reasons.

---

## R3 — Authenticating a push to a second repository

**Finding**: `GITHUB_TOKEN`, the token GitHub Actions injects automatically, is scoped to
the repository running the workflow. It cannot push to `kobogithub/homebrew-knowledge`.
Confirmed the repo currently has **no Actions secrets configured at all**
(`gh api repos/kobogithub/knowledge/actions/secrets` returns an empty list), so whatever
is chosen has to be created from scratch.

**Decision**: A **fine-grained Personal Access Token**, scoped to only the
`kobogithub/homebrew-knowledge` repository with `Contents: read and write`, stored in this
repository as an Actions secret. Referenced here as `TAP_GITHUB_TOKEN`.

**Rationale**: It is the least-privilege option that works, and it is a single secret for a
single-maintainer project. Fine-grained (not classic) matters: a classic PAT with `repo`
scope would grant write access to every repository the maintainer owns, which is a far
worse blast radius than the tap.

**Alternatives considered**:
- *Classic PAT with `repo` scope* — rejected. Over-broad; grants write to all repos.
- *Deploy key on the tap repo* — viable and equally least-privilege, but requires SSH
  remote handling in the workflow for no practical gain over a fine-grained PAT.
- *GitHub App installation token* — the most robust option for a team, rejected as
  disproportionate setup for one maintainer and one tap.
- *Third-party bump actions* (`dawidd6/action-homebrew-bump-formula`,
  `mislav/bump-homebrew-formula-action`) — rejected. Both are oriented toward opening PRs
  against `homebrew-core` and still require a PAT, so they add a supply-chain dependency
  without removing the credential.

**Consequence**: This is a task the maintainer must perform by hand; it cannot be scripted
from here. It is a prerequisite for User Story 2 and is called out as such in `tasks.md`.

---

## R4 — Formula shape for a single-platform binary

**Finding**: The current formula (`Formula/kn.rb`) nests `on_macos { on_arm {...}
on_intel {...} }` plus a top-level `on_linux`. Two of those three branches will reference
artifacts that stop being built.

**Decision**: Collapse to a flat `url` / `sha256` / `version` with explicit platform
guards:

```ruby
depends_on :macos
depends_on arch: :arm64
```

**Rationale**: `depends_on arch: :arm64` makes Homebrew refuse the install on an Intel Mac
with its own clear message, rather than downloading an artifact that does not exist and
failing at checksum verification. This is the formula-level counterpart to the install
script's refusal in FR-003, and it satisfies the spec's edge case about users on
unsupported machines following the docs anyway.

**Alternatives considered**:
- *Keep the `on_macos`/`on_arm` nesting with a single branch* — rejected as noise; the
  nesting exists to express variation that no longer exists, and a reader would reasonably
  wonder what the missing branches do.
- *Omit the guards and rely on checksum failure* — rejected. It produces a confusing
  download-then-fail rather than an upfront refusal.

**Retained as-is**: the `resource "assets"` block that pulls the tagged source archive for
skills and agent templates. The binary tarballs contain only the binary, so this block is
what satisfies FR-007. Its URL and checksum must be templated per release exactly like the
main artifact's — a second sha256 the automation has to compute, not one.

---

## R5 — Build target and runner

**Finding**: `release.yml` already builds `aarch64-apple-darwin` on `macos-latest`.
GitHub's `macos-latest` image is Apple Silicon, so this is a **native** build, not a
cross-compile. The Apple Silicon target is not new work — it exists and ships today as
`kn-macos-arm64.tar.gz`.

**Decision**: Reduce the `build-binaries` matrix to the single existing
`aarch64-apple-darwin` entry. Do not change toolchain, flags, or artifact naming.

**Rationale**: The artifact name `kn-macos-arm64.tar.gz` is load-bearing beyond the
formula — `install.sh` selects on it, and `RELEASE.md:273` warns that the
`kn-{os}-{arch}.tar.gz` shape is "Required for `kn update` to work". Renaming it would
break self-update for every already-installed copy. This feature narrows what is built; it
must not rename what survives.

**Alternatives considered**:
- *Build a universal binary via `lipo`* — rejected; directly contradicts the maintainer's
  decision to drop Intel.
- *Rename the artifact now that there is only one* — rejected for the self-update breakage
  above.

---

## R6 — Detecting drift if it recurs (FR-013)

**Finding**: Nothing today compares the tap's formula against the newest release. The
0.9.0/0.10.0 gap went unnoticed for weeks and was found only by manual inspection during
this session.

**Decision**: A scheduled check that reads the version declared in the tap's formula, reads
the newest published release tag, and fails when they disagree.

**Rationale**: FR-010 automation can itself break — a rotated token, a failed push, a
partially-applied edit. The spec asks for drift to be *visible*, which means an
independent observer, not a stronger assertion inside the thing being observed.

**Alternatives considered**:
- *Assert inside the release workflow only* — rejected. It cannot detect a push that
  succeeded and was later reverted, nor an expired token discovered months later.
- *Rely on install-time failure* — rejected. That surfaces the problem to a user rather
  than the maintainer, which is the situation this feature exists to end.

---

## R7 — Narrowing the install script

**Finding**: `install.sh` detects three platform combinations (`install.sh:119-170`) and
maps them to three artifact names (`install.sh:384-390`). It also contains deliberate
**Rosetta detection** (`install.sh:143-149`): when it sees `x86_64` on macOS it probes
whether the process is translated, and if so rewrites `ARCH` to `arm64`.

**Decision**: Narrow detection to Apple Silicon macOS and fail clearly otherwise —
**keeping the Rosetta branch intact**.

**Rationale**: The Rosetta branch is exactly what satisfies the spec's final edge case
("a user is on Apple Silicon but running under x86 translation"). A naive narrowing that
deleted the `x86_64` handling entirely would make the script refuse to install on the
maintainer's own M4 whenever it was invoked from a translated terminal — a regression
introduced by the cleanup itself. The `x86_64` path must be retained *as the Rosetta
probe*, and only the genuine-Intel outcome turned into a refusal.

**Alternatives considered**:
- *Delete all `x86_64` handling* — rejected for the regression above.
- *Leave the script untouched and let downloads 404* — rejected; FR-003 requires refusing
  before downloading.

---

## R8 — What gets deleted, and what points at it

**Finding**: The retired artifacts are `kn.spec`, `debian/`, `install.ps1`, and (on the
unmerged `002` branch) `docs/packaging/`. Inbound references found:

| Reference | Location | Action |
|---|---|---|
| Windows install section | `README.md`, `README_ES.md` | Delete section (FR-016) |
| "APT / DNF coming soon" | `README.md:218-226`, `README_ES.md:194-202` | Delete section (FR-016) |
| "Supported Platforms" list | `README.md:150-153`, `README_ES.md:126-129` | Reduce to one entry (FR-014) |
| Release notes platform list | `release.yml:109-111` | Reduce to one entry |
| Manual tap + APT steps | `RELEASE.md:181-193`, `RELEASE.md:283-296` | Replace with the automated flow (R1) |
| `Formula/kn.rb` update steps | `RELEASE.md:76,107` | Delete; now automated |
| Packaging doc links | `README.md:230-235` | Delete with the docs |

Also found, and deliberately **not** touched: `CHANGELOG.md:486` mentions the formula in a
released-version section. This project already set the precedent (initiative `002`, T020)
that changelog entries are historical record, not live links.

**Decision**: Delete the files; repoint or delete every inbound reference above; leave
changelog history alone.

**Separately noted**: `kn.spec` declares `Version: 0.1.0` while the project ships `0.10.0`.
Verified by building it in a Fedora container during this session — the recipe is stale by
nine minor versions. This makes the deletion strictly a cleanup with no loss: the recipe
has not produced a current package in a long time. No action beyond deletion.

---

## R9 — The license dependency is real and currently unmet

**Finding**: `gh repo view kobogithub/knowledge --json licenseInfo` returns **null**. The
GitHub API sees no license on the default branch. A `LICENSE` file is added by initiative
`002-public-ready-repo`, which is open as PR #5 and unmerged.

**Decision**: Treat `002` merging as a hard prerequisite for the Homebrew stories, and
sequence this feature's work behind it.

**Rationale**: FR-009 requires the formula to declare a license. Beyond formula
correctness, distributing a binary through a package manager with no license on the
project is a genuine problem for anyone installing it.

**Practical note**: PR #5's CI is green as of this session, and this feature's pivot
dissolved its remaining blockers (its outstanding tasks verify RPM/DEB packaging that this
feature deletes). Merging it is unblocked.

---

## R10 — Overlap with in-flight work on the same files

**Finding**: This feature edits `README.md`, `README_ES.md` and `install.sh`. All three are
also edited by the unmerged `002` branch, and two were just edited by the merged
dead-domain hotfix.

**Decision**: Rebase this feature's branch on `dev` after `002` lands, before starting
Phase 3 implementation work on the READMEs.

**Rationale**: `002` restructures the README's installation section wholesale — it moves
installation above the feature catalog. Editing the old structure here and merging later
produces avoidable conflicts in exactly the section both features rewrite.

**Consequence**: The setup phase in `tasks.md` carries an explicit ordering gate rather
than assuming a clean tree.
