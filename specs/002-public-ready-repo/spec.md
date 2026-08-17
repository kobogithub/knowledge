# Feature Specification: Public-Ready Repository

**Feature Branch**: `epic/002-public-ready-repo`

**Created**: 2026-08-04

**Status**: Draft

**Input**: User description: "Preparar el repositorio knowledge (kn) para consumo público: agregar licencia MIT en la raíz, reorganizar los 12 archivos markdown sueltos de la raíz moviendo los de empaquetado y proceso de release (DEBIAN.md, RPM.md, HOMEBREW.md, GITHUB_PAGES.md, RELEASE.md, RELEASE_v0.1.0_CHECKLIST.md, SECURITY_AUDIT_REPORT.md) a docs/ dejando solo README.md, README_ES.md, CHANGELOG.md, AGENTS.md y CLAUDE.md en la raíz, agregar CONTRIBUTING.md y plantillas de issue y pull request en .github/, eliminar rutas absolutas personales hardcodeadas como /Users/kobo en la documentación y specs, y ajustar el README para que un visitante nuevo entienda en menos de un minuto qué es kn, cómo instalarlo y cómo empezar. No cambiar el comportamiento del CLI ni de los stacks."

## Context

The repository is **already published** at `kobogithub/knowledge` (public visibility, GitHub Pages at `kn.foxlabar.online`, releases through v0.10.0, Homebrew formula). This feature does not make it public — it closes the gaps that make a public repository unusable or untrustworthy to an outside visitor.

Baseline observed at spec time:

- No `LICENSE` file exists; GitHub reports `licenseInfo: null`.
- 12 markdown files sit at the repository root, 7 of which are packaging/release process documents relevant only to maintainers.
- No `CONTRIBUTING.md`, no issue templates, no pull request template.
- `specs/001-personal-stacks/quickstart.md` contains the absolute path `/Users/kobo/Github/personal/knowledge`.
- No secrets or `.env` files are tracked (verified); this feature does not address a leak.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Licensed for reuse (Priority: P1)

A developer finds `kn`, wants to use it at work or fork it, and checks the license before investing time. Today the repository has no license, which under default copyright means nobody may legally use, copy, modify, or distribute it. They abandon the evaluation.

**Why this priority**: Without a license the repository is legally unusable no matter how good the code or documentation is. Every other improvement in this feature is wasted effort until this is fixed. It is also the smallest change, making it a complete standalone slice.

**Independent Test**: Ship only this story, and GitHub's repository sidebar displays "MIT license" and the license file is reachable from the repository root. A visitor can determine reuse terms without asking anyone.

**Acceptance Scenarios**:

1. **Given** a visitor on the repository landing page, **When** they look at the sidebar or the file list, **Then** an MIT license is identified and linked.
2. **Given** the license file, **When** it is read, **Then** it contains the standard MIT text with the correct copyright holder and year.
3. **Given** the README, **When** a reader reaches the end, **Then** a license section states the terms and links to the license file.

---

### User Story 2 - A root that explains itself (Priority: P2)

A visitor lands on the repository and sees 12 markdown files competing for attention, most of them maintainer-only process documents (Debian packaging, RPM packaging, Homebrew, GitHub Pages setup, release process, a stale v0.1.0 release checklist, a security audit report). They cannot tell which document is the entry point.

**Why this priority**: This is the first impression, and it is the difference between a repository that reads as a maintained product and one that reads as a scratch directory. It is independently valuable and testable, but it is worthless if the project cannot be legally used, so it ranks below the license.

**Independent Test**: Ship only this story, and the repository root lists at most 5 markdown files, every relocated document is still reachable through a working link, and no cross-reference between documents is broken.

**Acceptance Scenarios**:

1. **Given** the repository root, **When** its markdown files are listed, **Then** only the reader-facing and agent-facing entry points remain: `README.md`, `README_ES.md`, `CHANGELOG.md`, `AGENTS.md`, `CLAUDE.md`, plus the license and contributing guide.
2. **Given** any relocated maintainer document, **When** it is opened at its new location, **Then** its content is unchanged apart from corrected internal links.
3. **Given** any document that previously linked to a relocated file, **When** every link in it is resolved, **Then** none returns a missing target.
4. **Given** the stale `RELEASE_v0.1.0_CHECKLIST.md` describing a release from many versions ago, **When** the reorganization is applied, **Then** it is either archived under the documentation tree or removed, and it is not presented as current process.

---

### User Story 3 - Understanding kn in under a minute (Priority: P3)

A developer who has never seen `kn` opens the README. They need three answers fast: what is this, how do I install it, and what is the first command I run. Today the README opens with feature sections before installation, and the reader must scroll past several subsections to reach the quick start.

**Why this priority**: This converts an interested visitor into a user. It depends on nothing else and can ship independently, but a confusing README wastes fewer opportunities than a missing license or an unreadable root.

**Independent Test**: Ship only this story, and a reader unfamiliar with the project can state what `kn` does, run the install command, and run a first command, using only the top portion of the README, without scrolling past the first screen for the install step.

**Acceptance Scenarios**:

1. **Given** the README, **When** a new reader reads only the opening section, **Then** they can state in one sentence what `kn` is and who it is for.
2. **Given** the README, **When** a reader looks for installation, **Then** the primary install method appears before the detailed feature catalog.
3. **Given** the README, **When** a reader has installed `kn`, **Then** a first-run command sequence is immediately available.
4. **Given** both README language versions, **When** they are compared section by section, **Then** they present the same structure and the same claims.
5. **Given** the documented feature list, **When** each documented command is compared against the shipped CLI, **Then** every documented command exists and no shipped top-level command is silently undocumented.

---

### User Story 4 - Knowing how to contribute (Priority: P4)

Someone wants to report a bug or propose a change. They do not know the branching model, the commit format, or what the maintainer expects in a pull request — all of which this project enforces strictly (`prod` / `dev` / `epic/*` hierarchy, conventional commits, spec-kit initiatives).

**Why this priority**: It reduces friction and bad-quality contributions, but a project with no inbound contributors yet loses least by lacking it. It ships last.

**Independent Test**: Ship only this story, and a would-be contributor can open the contributing guide and correctly name the target branch and the commit message format for a bug fix, and opening a new issue or pull request presents a structured template.

**Acceptance Scenarios**:

1. **Given** a contributor reading the contributing guide, **When** they look for where to send a change, **Then** the branch hierarchy and the correct pull request target are stated.
2. **Given** a contributor writing a commit, **When** they consult the guide, **Then** the conventional commit format and its allowed types are stated with examples.
3. **Given** a user opening a new issue, **When** the issue form appears, **Then** they are offered distinct structured templates for a bug report and a feature request.
4. **Given** a contributor opening a pull request, **When** the description field appears, **Then** it is pre-filled with a template prompting for summary, linked spec, and verification performed.

---

### Edge Cases

- A relocated document is referenced from a file that is not markdown (for example, the RPM spec file references packaging docs) — such references must be updated too, not only markdown-to-markdown links.
- A relocated document is referenced from an already-published release note or an external site; internal links are fixed, but previously published external URLs to root-level documents will break. This is accepted for maintainer-only process documents.
- The Spanish README drifts from the English one during the rewrite, leaving two documents making different claims.
- Removing personal absolute paths from a completed initiative's documents (`specs/001-personal-stacks/`) edits a historical record; the replacement must preserve the instructional intent rather than deleting the step.
- The published documentation describes a `kn beads` command family while an architecture decision record states that beads was removed from the workflow. The command still exists in the shipped CLI, so the documentation is not wrong, but a visitor reading both reaches contradictory conclusions.
- The license year and copyright holder must match the actual author; a placeholder left in the license text is worse than no license.

## Requirements *(mandatory)*

### Functional Requirements

**Licensing**

- **FR-001**: The repository MUST contain an MIT license file at its root, carrying the correct copyright holder and year.
- **FR-002**: The hosting platform MUST detect and display the license on the repository landing page.
- **FR-003**: Both README language versions MUST state the license and link to the license file.

**Root organization**

- **FR-004**: The repository root MUST retain only these markdown documents: `README.md`, `README_ES.md`, `CHANGELOG.md`, `AGENTS.md`, `CLAUDE.md`, the license, and the contributing guide.
- **FR-005**: The packaging and release process documents (`DEBIAN.md`, `RPM.md`, `HOMEBREW.md`, `GITHUB_PAGES.md`, `RELEASE.md`, `SECURITY_AUDIT_REPORT.md`) MUST be relocated under the documentation tree, grouped so their purpose is evident from location.
- **FR-006**: The stale v0.1.0 release checklist MUST NOT remain at the root presented as current process; it MUST be archived or removed.
- **FR-007**: Every internal reference to a relocated document MUST resolve to its new location, including references from non-markdown files.
- **FR-008**: The documentation tree MUST offer an index that lets a reader find any relocated document without knowing its filename in advance.

**README quality**

- **FR-009**: The README MUST state what `kn` is, who it is for, and the problem it solves, within its opening section and before any feature catalog.
- **FR-010**: The README MUST present the primary installation method before the detailed feature catalog.
- **FR-011**: The README MUST present a first-run command sequence immediately reachable after installation.
- **FR-012**: Both README language versions MUST present equivalent structure and equivalent claims.
- **FR-013**: Every command shown in the README MUST exist in the shipped CLI, and every shipped top-level command MUST be documented.
- **FR-014**: Where documentation and an architecture decision record describe the same capability differently, the documentation MUST state the current status explicitly so a reader is not left with a contradiction.

**Contribution guidance**

- **FR-015**: The repository MUST provide a contributing guide covering the branch hierarchy, the pull request target for each branch type, and the conventional commit format with its allowed types.
- **FR-016**: The contributing guide MUST describe how an initiative is specified before implementation, so a contributor understands the spec-driven workflow this project uses.
- **FR-017**: The repository MUST provide distinct structured issue templates for bug reports and feature requests.
- **FR-018**: The repository MUST provide a pull request template prompting for a summary, the related initiative, and the verification performed.

**Portability**

- **FR-019**: No tracked file MUST contain a machine-specific absolute path belonging to the maintainer.
- **FR-020**: Instructions that previously relied on a machine-specific path MUST remain executable by a reader on their own machine, preserving the original instructional intent.

**Scope protection**

- **FR-021**: The behavior of the CLI MUST be unchanged; no command is added, removed, or altered.
- **FR-022**: The stack presets and the skill catalog MUST be unchanged.
- **FR-023**: The installation and release automation MUST continue to work unchanged after documents are relocated.

### Key Entities

- **Root entry document**: A document a first-time visitor or an agent is expected to read directly from the repository root. Limited to the two README translations, the changelog, the two agent instruction files, the license, and the contributing guide.
- **Maintainer process document**: A document describing packaging, release, publishing, or audit procedure, relevant only to whoever cuts a release. Belongs under the documentation tree, not the root.
- **Contribution template**: A structured form presented at the moment a contributor opens an issue or a pull request, capturing the information the maintainer needs to triage it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The repository landing page identifies the project's license without the visitor opening any file.
- **SC-002**: The repository root contains at most 7 markdown documents, down from 12.
- **SC-003**: Every internal documentation link resolves to an existing target; zero broken links across all tracked markdown.
- **SC-004**: A reader unfamiliar with the project can state what `kn` does and reach the install command within one minute of opening the README.
- **SC-005**: Zero tracked files contain a maintainer-specific absolute filesystem path.
- **SC-006**: A contributor can determine the correct target branch and commit message format for a change without reading source code or asking the maintainer.
- **SC-007**: Opening a new issue or pull request presents a structured template rather than an empty text box.
- **SC-008**: Every command documented in the README exists in the shipped CLI, and every shipped top-level command is documented.
- **SC-009**: Installing the tool from a clean machine using only the README succeeds without consulting any other document.
- **SC-010**: The existing automated checks and the release automation pass unchanged after the reorganization.

## Assumptions

- The repository is already public; this feature closes readiness gaps rather than performing a publication.
- MIT is the chosen license, confirmed by the maintainer, with the copyright holder being the repository author.
- The two agent instruction files (`AGENTS.md`, `CLAUDE.md`) stay at the root because agent tooling discovers them there by convention; they are not visitor-facing clutter.
- The changelog stays at the root because it is reader-facing and conventionally located there.
- Relocated maintainer documents keep their content; this feature moves and relinks them rather than rewriting their substance.
- The `kn beads` command family remains in the CLI unchanged, consistent with the decision to leave the CLI's bd integration intact; only the documentation's framing of its status is clarified.
- Previously published external links pointing at root-level maintainer process documents may break, which is acceptable for maintainer-only content.
- No translation beyond the existing English and Spanish READMEs is in scope.
- Documentation hosting at the existing custom domain continues to work; this feature does not change the hosting setup.
