# Feature Specification: kn targets macOS Apple Silicon only, distributed via Homebrew

**Feature Branch**: `epic/004-macos-arm64-homebrew`

**Created**: 2026-08-16

**Status**: Draft

**Input**: User description: "quiero que kn solo ahora este adaptado para Mac Mini m4 y publicarlo en homebrew"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Installing kn on the Mac Mini gives the current version (Priority: P1) 🎯 MVP

The maintainer sits down at the Mac Mini M4, runs a single Homebrew command, and gets
the version of `kn` that the project most recently released. No manual download, no
version drift, no wondering whether what landed is what was published.

**This is observable right now, not hypothetical.** On the maintainer's machine today,
`kn --version` reports `0.9.0`, served from `Cellar/kn/0.9.0` — installed through Homebrew
from the stale tap, two releases behind what the project has published.

**Why this priority**: This is the entire point of the feature, and it is broken today.
The published tap serves 0.9.0 while the project has released 0.10.0, so the advertised
install command hands users a version that is two releases behind. Everything else in
this feature is either enabling work or cleanup; this is the outcome the maintainer asked
for, and it is independently valuable even if nothing else ships.

**Independent Test**: On a machine with Homebrew and no prior `kn`, run the documented
install command and compare the reported version against the newest published release.
Delivers a working, current install with no other part of this feature completed.

**Acceptance Scenarios**:

1. **Given** a Mac Mini M4 with Homebrew installed and no existing `kn`, **When** the
   maintainer runs the documented Homebrew install command, **Then** the command
   completes without error and the installed `kn` reports the newest published release
   version.
2. **Given** `kn` installed via Homebrew, **When** the maintainer runs the tool's own
   health check, **Then** it reports a healthy installation with its bundled skills and
   agents present.
3. **Given** an older `kn` installed via Homebrew, **When** the maintainer runs the
   Homebrew upgrade command, **Then** it moves to the newest published release.

---

### User Story 2 - Publishing a release updates the published formula automatically (Priority: P2)

When the maintainer publishes a new version, the Homebrew formula that users actually
install from is updated as part of that release — without anyone remembering to do it by
hand.

**Why this priority**: This is what stops User Story 1 from silently regressing. The
current drift exists precisely because updating the published formula is a manual step
that was skipped. Fixing the version once without fixing the process means the same gap
reopens at the next release.

**Independent Test**: Publish a version and observe the published formula without taking
any manual action; it should reference the new version and correct checksums.

**Acceptance Scenarios**:

1. **Given** a new version is published, **When** the release process completes, **Then**
   the published formula references that version and its artifact checksum without any
   manual edit.
2. **Given** a release whose artifacts failed to build, **When** the release process runs,
   **Then** the published formula is left untouched rather than updated to point at a
   version that has no downloadable artifact.
3. **Given** the formula was just updated by the release process, **When** a user installs
   from the tap, **Then** the checksum recorded in the formula matches the artifact that
   was actually published.

---

### User Story 3 - Releases build exactly one artifact, for Apple Silicon (Priority: P3)

The release process produces a single binary for Apple Silicon macOS, instead of three
binaries for platforms the project no longer supports.

**Why this priority**: It removes the ongoing cost of building and storing artifacts that
nobody installs, and it removes the risk that the formula or docs reference a platform
whose artifact silently stopped being produced. It depends on nothing, but delivers less
standalone value than the two stories above.

**Independent Test**: Publish a version and confirm the release carries exactly one
downloadable artifact, for Apple Silicon macOS.

**Acceptance Scenarios**:

1. **Given** a new version is published, **When** the release completes, **Then** the
   release carries exactly one binary artifact and it is for Apple Silicon macOS.
2. **Given** the release process runs, **When** it builds, **Then** it uses a single
   build environment rather than a matrix spanning several operating systems.

---

### User Story 4 - The project stops promising platforms it does not support (Priority: P4)

Anyone reading the project's documentation, or running its install script on the wrong
machine, learns immediately and accurately that `kn` runs on Apple Silicon macOS — rather
than being told it supports Linux, Windows and Intel Macs, or being handed a download
link for an artifact that no longer exists.

**Why this priority**: This is a correctness and trust problem rather than a capability
gap. It cannot mislead anyone until the platform narrowing of the earlier stories has
actually happened, so it follows them.

**Independent Test**: Read every install-related document and run the install script on a
non-Apple-Silicon machine; no false platform claim should survive, and the script should
refuse clearly.

**Acceptance Scenarios**:

1. **Given** the install script runs on a machine that is not Apple Silicon macOS,
   **When** platform detection completes, **Then** it stops with a clear message naming
   the only supported platform, and downloads nothing.
2. **Given** a reader opens either README, **When** they look for supported platforms,
   **Then** they find Apple Silicon macOS named as the only one, with no Windows section
   and no claim of forthcoming Linux package-manager support.
3. **Given** a reader looks for how to install, **When** they read the installation
   section, **Then** the Homebrew command is presented as the primary method.

---

### User Story 5 - Retired packaging is gone from the repository (Priority: P5)

The recipes and scripts for the platforms the project no longer supports are removed, so
nobody builds from a stale recipe or mistakes their presence for a support commitment.

**Why this priority**: Pure cleanup with no user-facing behaviour change. Valuable — one
of these recipes is provably stale — but nothing depends on it, so it goes last.

**Independent Test**: Search the repository for the retired packaging artifacts; none
should remain, and no surviving file should reference them.

**Acceptance Scenarios**:

1. **Given** the repository after this story, **When** someone searches for the retired
   packaging recipes and the Windows install script, **Then** none are present.
2. **Given** the retired files are gone, **When** someone searches the remaining files for
   references to them, **Then** no link or instruction points at a removed file.
3. **Given** the files are removed, **When** someone needs their former contents, **Then**
   they remain retrievable from version history.

---

### Edge Cases

- **A user on an unsupported machine follows the docs anyway.** The install script must
  refuse with a message that names the supported platform, rather than downloading a
  nonexistent artifact and failing with a confusing error.
- **A release is published while the formula update fails.** The published formula must
  keep pointing at the previous working version rather than at a version whose artifact
  is missing or whose checksum is wrong. A half-updated formula is worse than a stale one.
- **The formula is updated but the artifact checksum does not match.** Installation must
  fail loudly at verification rather than installing an unverified binary.
- **A user already has `kn` installed from the old install script**, outside Homebrew.
  Installing via Homebrew must not silently leave two copies shadowing each other on the
  path without the conflict being detectable. Covered by FR-020 and SC-009.
- **Someone re-runs a release for a version that already exists.** The formula update
  must not corrupt the published formula into an inconsistent state.
- **A user is on Apple Silicon but running under x86 translation.** Detection must resolve
  to the Apple Silicon artifact rather than concluding the machine is Intel.

## Requirements *(mandatory)*

### Functional Requirements

**Platform scope**

- **FR-001**: The project MUST support exactly one platform: Apple Silicon macOS.
- **FR-002**: The release process MUST produce exactly one binary artifact per release,
  built for Apple Silicon macOS.
- **FR-003**: The install script MUST detect the running platform and, when it is not
  Apple Silicon macOS, stop before downloading anything and report which platform is
  supported.
- **FR-004**: The install script MUST resolve a machine running under x86 translation on
  Apple Silicon to the Apple Silicon artifact.

**Homebrew distribution**

- **FR-005**: `kn` MUST be installable from the maintainer's existing public tap using a
  single documented command.
- **FR-006**: The published formula MUST reference only the Apple Silicon macOS artifact,
  with no branches for other operating systems or processor families.
- **FR-007**: The published formula MUST install the binary together with the skills and
  agent templates the tool needs to be functional after install.
- **FR-008**: The published formula MUST record a verifiable checksum for every artifact
  it downloads, and installation MUST fail if a downloaded artifact does not match.
- **FR-009**: The published formula MUST declare the project's license.
- **FR-020**: When `kn` is installed through Homebrew on a machine that already carries an
  installation placed by the standalone install script, the conflict MUST be surfaced to
  the user rather than left as two copies silently shadowing each other on the path. The
  user MUST be told which copy will win and how to remove the other.

**Keeping the published formula current**

- **FR-010**: Publishing a release MUST update the published formula to that version
  without any manual step.
- **FR-011**: The formula update MUST NOT run when the release's artifacts did not build
  successfully.
- **FR-012**: The version and checksum written into the published formula MUST be derived
  from the artifacts actually published, not from a value maintained by hand.
- **FR-013**: The project MUST have a way to detect that the published formula and the
  newest published release disagree, so the drift this feature fixes is visible if it
  recurs.

**Honest documentation**

- **FR-014**: Both READMEs MUST name Apple Silicon macOS as the only supported platform.
- **FR-015**: Both READMEs MUST present the Homebrew command as the primary installation
  method.
- **FR-016**: Both READMEs MUST NOT contain a Windows installation section, nor claim that
  Linux package-manager distribution is forthcoming.
- **FR-017**: No file in the repository may instruct a reader to use a removed packaging
  recipe or a removed install script.

**Retiring old packaging**

- **FR-018**: The RPM recipe, the Debian packaging directory, the Windows install script,
  and the packaging documents describing them MUST be removed from the repository.
- **FR-019**: The removed content MUST remain retrievable from version history; no
  history rewriting.

### Key Entities

- **Release**: A published version of `kn`, carrying exactly one downloadable binary
  artifact for Apple Silicon macOS, plus a source archive used for bundled assets.
- **Published formula**: The installation recipe in the maintainer's public tap that
  Homebrew users install from. Distinct from the copy kept in this repository, and the
  divergence between the two is the defect this feature fixes.
- **Tap**: The maintainer's public repository of formulas, already existing and public,
  through which `kn` is distributed.
- **Bundled assets**: The skills and agent templates that ship alongside the binary and
  must be present for the tool to be usable after installation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On a clean Mac Mini M4, a single documented command installs `kn`, and the
  installed version matches the newest published release exactly.
- **SC-002**: Installing from the tap requires no manual steps beyond that one command,
  and completes without the user editing any file.
- **SC-003**: After a version is published, the published formula reflects that version
  with zero manual intervention.
- **SC-004**: A release carries exactly one binary artifact.
- **SC-005**: Running the install script on a machine that is not Apple Silicon macOS
  produces a clear refusal naming the supported platform, and downloads nothing.
- **SC-006**: A search of the repository for claims of Linux, Windows or Intel Mac support
  returns zero results outside version history and changelog entries describing past
  releases.
- **SC-007**: A search of the repository for the retired packaging recipes and the Windows
  install script returns zero results, and no surviving file links to them.
- **SC-008**: The published formula and the newest published release report the same
  version at any point after a release completes.
- **SC-009**: On a machine that already has a `kn` installed outside Homebrew, installing
  through Homebrew tells the user that both exist, which one the shell will run, and how
  to remove the other. The user never has to discover the conflict by observing a stale
  version.

## Assumptions

- **"Mac Mini M4" means Apple Silicon macOS generally**, not that specific machine. The
  artifact targets the Apple Silicon processor family, so it runs on any Apple Silicon
  Mac; the M4 is simply the maintainer's machine.
- **Intel Mac support is being dropped, not merely deprioritised.** The maintainer chose
  Apple Silicon only over a universal macOS build when asked directly.
- **The tap is the distribution channel; the central Homebrew repository is out of scope.**
  The project has no stars or forks and therefore falls far below that repository's
  notability requirements. This is recorded as a deliberate exclusion, not an oversight,
  and may be revisited if the project's adoption changes.
- **Removed packaging is recovered from version history if ever needed**, so no copy is
  retained in the working tree. This matches the precedent already set in this project
  for deleting stale release documents.
- **The maintainer has write access to the tap repository** and can grant the release
  process permission to update it.
- **A minimum supported macOS version is inherited from what the build toolchain already
  targets**; establishing a new floor is out of scope.

## Dependencies

- **The project's license file must be present on the default branch.** Homebrew formulas
  declare a license (FR-009), and the platform currently reports the repository as having
  none. The license is added by initiative `002-public-ready-repo`, which is open and
  unmerged. This feature's Homebrew work is blocked on that merge landing.
- **The install script's install path is unchanged by this feature**; only its platform
  detection and artifact selection are in scope.

## Out of Scope

- Submitting the formula to the central Homebrew repository.
- Any change to what `kn` does once installed — commands, skills content, and agent
  behaviour are untouched.
- Establishing or raising a minimum supported macOS version.
- Restoring support for any removed platform.
- Signing or notarizing the binary.
