# Feature Specification: Personal Stacks & Curated Catalog

**Feature Branch**: `epic/001-personal-stacks`

**Created**: 2026-08-01

**Status**: Draft

**Input**: User description: "Curar el catálogo de skills/agentes de kn hacia los stacks personales del usuario e introducir un concepto liviano de stack-preset. Mantener el modelo plano y personal de kn (sin white-label, sin registries privados, sin capas Core/Blueprint como taligent)."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Curated catalog reflects my real stack (Priority: P1)

As the sole user of `kn`, when I browse or install skills I only see technologies I
actually work with (Astro, FastAPI, Supabase, Railway, HTMX, Go, Rust, Python, Docker,
GitHub Actions, Bash, plus my security and UI/UX testing skills), not generic ones I
never use (AWS, Jsonnet, Kubernetes, Terraform, Notion reporting).

**Why this priority**: This is the core of the request — kn must carry *my* knowledge,
not a generic catalog. It delivers value on its own even without presets: a leaner,
personally relevant skill set. It is also the prerequisite for presets (P2), which
reference these skills.

**Independent Test**: Run the catalog listing after curation; confirm the removed skills
are gone, the four new skills (fastapi, htmx, go, railway) are present and installable,
and every retained skill still installs into a project correctly.

**Acceptance Scenarios**:

1. **Given** the curated catalog, **When** I list available skills, **Then** aws,
   jsonnet, kubernetes, terraform, and notion-reporting-standard are absent.
2. **Given** the curated catalog, **When** I list available skills, **Then**
   fastapi, htmx, go, and railway are present.
3. **Given** a fresh project, **When** I install any retained or newly added skill,
   **Then** it is written to the workspace and recorded as enabled in `kn.toml`.

---

### User Story 2 - Start a project from a stack-preset (Priority: P2)

As the sole user of `kn`, when I start a new project I can pick one named stack-preset
(e.g. `web-astro`, `api-fastapi`, `cli-rust`, `cli-go`, `data-py`) and have all of that
stack's skills activated in one step, instead of enabling each skill by hand.

**Why this priority**: This is the productivity win on top of a curated catalog. It
depends on P1 (presets reference curated skills) so it comes second, but it is the
feature that makes starting a new personal project fast.

**Independent Test**: Initialize a project selecting a preset; confirm exactly the
preset's skills are enabled in `kn.toml` and installed, and the chosen stack is recorded.

**Acceptance Scenarios**:

1. **Given** the available presets, **When** I list them, **Then** I see each preset's
   name and the skills it bundles.
2. **Given** a preset selection during init, **When** initialization completes, **Then**
   `kn.toml` records the chosen stack and enables exactly that preset's skills.
3. **Given** a preset that references a skill, **When** I init with it, **Then** the
   referenced skill is installed into the workspace.
4. **Given** a preset name that does not exist, **When** I init with it, **Then** kn
   fails with a clear message listing valid presets and changes nothing.

---

### User Story 3 - Discover and inspect presets outside of init (Priority: P3)

As the sole user of `kn`, I can list the available stack-presets and see what each one
contains at any time, not only during initialization, so I can decide which fits a new
project before creating it.

**Why this priority**: Convenience/discoverability. The core value (P1, P2) works
without it, but it makes the preset system usable day-to-day.

**Independent Test**: Run the preset listing command in any directory; confirm it prints
each preset name and its bundled skills without requiring an initialized project.

**Acceptance Scenarios**:

1. **Given** the installed presets, **When** I run the list command, **Then** each
   preset is shown with its bundled skills.
2. **Given** no presets are installed yet, **When** I run the list command, **Then** kn
   reports that none are available rather than erroring.

---

### Edge Cases

- **Removed skill still referenced**: A project's existing `kn.toml` lists a skill that
  curation removed from the catalog. kn must not crash — it warns that the skill is no
  longer in the catalog and leaves the project file untouched.
- **Preset references a missing skill**: A preset definition names a skill absent from
  the catalog. kn reports which skill is missing and which preset references it, and does
  not silently enable a partial set.
- **Duplicate skills across a preset and manual flags**: Selecting a preset and also
  passing individual skills must not enable a skill twice in `kn.toml`.
- **Both `--stack` and `--yes` in non-interactive init**: The preset must apply without
  prompting.
- **User-defined preset**: The user drops a new preset definition into the presets
  location; kn picks it up on the next listing without a code change.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The skill catalog MUST NOT include aws, jsonnet, kubernetes, terraform, or
  notion-reporting-standard after curation.
- **FR-002**: The skill catalog MUST include new skills for FastAPI, HTMX, Go, and
  Railway, each installable the same way as existing skills.
- **FR-003**: The skill catalog MUST retain the user's active skills: rust, python,
  docker, github-actions, bash, supabase-postgres, standard-commits, documentation-guide,
  astro, all security-* skills, and all uiux-* skills.
- **FR-004**: kn MUST support a named "stack-preset" — a bundle that maps one name to an
  ordered set of catalog skills.
- **FR-005**: kn MUST ship the following presets: `web-astro`, `api-fastapi`, `cli-rust`,
  `cli-go`, `data-py`, with skill sets as defined in Assumptions.
- **FR-006**: During project initialization, the user MUST be able to select one preset,
  which enables exactly that preset's skills and installs them.
- **FR-007**: kn MUST record the chosen stack in the project configuration (`kn.toml`).
- **FR-008**: kn MUST provide a way to list available presets and the skills each bundles
  without requiring an initialized project.
- **FR-009**: When a referenced skill is missing from the catalog, kn MUST fail (or warn,
  per context) with a message naming the missing skill and the preset, and MUST NOT write
  a partial/duplicate skill set.
- **FR-010**: Preset definitions MUST live in a user-editable location so the user can add
  or edit presets without changing kn's source code.
- **FR-011**: Selecting a preset MUST be idempotent with manual skill flags — no skill is
  enabled more than once in `kn.toml`.
- **FR-012**: All curation and preset behavior MUST preserve kn's flat, personal model —
  no white-label forks, no private registries, no layered Core/Blueprint/Stack/Plugin
  hierarchy.

### Key Entities *(include if feature involves data)*

- **Skill**: A unit of reusable knowledge/best-practices for one technology, installable
  into a project and recorded as enabled in `kn.toml`. (Existing entity; catalog membership
  changes.)
- **Stack-Preset**: A named bundle mapping a stack name to an ordered list of skill names.
  Lives in a user-editable location; referenced by the project config.
- **Project config (`kn.toml`)**: Records the project's chosen stack (new) and its enabled
  skills (existing).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After curation, 0 of the 5 removed technologies appear in the catalog, and
  all 4 new technologies appear.
- **SC-002**: Starting a new project from a preset enables its full skill set in a single
  selection (1 step instead of one action per skill).
- **SC-003**: The user can identify which preset fits a project by listing presets and
  their skills, without opening source files.
- **SC-004**: Adding a new personal preset requires editing only a preset definition file,
  with no change to kn's source code.
- **SC-005**: No existing project breaks: initializing or re-syncing a project after
  curation completes without error, even if it referenced a now-removed skill.

## Assumptions

- **Preset skill sets** (informed defaults, adjustable during planning):
  - `web-astro`: astro, htmx, supabase-postgres, railway, github-actions, docker
  - `api-fastapi`: fastapi, python, supabase-postgres, railway, docker, github-actions
  - `cli-rust`: rust, docker, github-actions, bash
  - `cli-go`: go, docker, github-actions, bash
  - `data-py`: python, supabase-postgres, docker
- "HTPX" in the user's input is interpreted as **HTMX**.
- New skills follow the existing `<tech>-best-practices` naming and SKILL.md structure
  already used across the catalog.
- Preset definitions are stored under the user's kn home (e.g. `~/.kn/stacks/<name>.toml`)
  and referenced by name from the project's `kn.toml`, consistent with how kn already
  stages resources in `~/.kn/`.
- Security-* and uiux-* skills stay in the catalog even if no default preset bundles them,
  because the user activates them per project as needed.
- This is a single-user tool; no multi-user, permissions, or sharing concerns apply.
