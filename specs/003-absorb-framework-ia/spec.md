# Feature Specification: Absorb framework_ia

**Feature Branch**: `epic/003-absorb-framework-ia`

**Created**: 2026-08-04

**Status**: Draft

**Input**: User description: "Unificar el criterio de knowledge (kn) absorbiendo el contenido del repositorio framework_ia mediante una importación unidireccional, y archivar framework_ia después. Portar al formato de skills de kn las skills que solo existen en framework_ia, evaluando cada una contra los stacks reales del usuario antes de aceptarla. Fusionar las skills que se solapan quedándose con la versión más profunda y completa. Reparar el grafo agente-skill que quedó roto tras la iniciativa 001. Actualizar los stack-presets para aprovechar las skills nuevas. Sacar del código de framework_ia la API key de Context7 que está en texto plano reemplazándola por una variable de entorno, sin purgar el historial. No adoptar el workflow de beads ni el esquema de agentes de framework_ia ni framework-init.sh. No cambiar el comportamiento del CLI."

## Context

Two personal AI-agent frameworks have been maintained in parallel:

- **`knowledge` (kn)** — public, a Rust CLI with packaged distribution (Homebrew, .deb, .rpm, install scripts), the spec-kit workflow, 11 agents, 21 skills, and 5 stack presets. Released through v0.10.0.
- **`framework_ia`** — private, a shell-script framework (`framework-init.sh`) built on the beads issue tracker and targeting OpenCode specifically. 8 agents, 18 skills.

The decision is a **one-directional content import**: `knowledge` survives and absorbs the material worth keeping; `framework_ia` is retired. A bidirectional merge was rejected for three reasons — the agent frontmatter schemas are structurally incompatible (kn's CLI reads `required_skills`/`mcp_servers` fields that framework_ia's agents do not have), the beads workflow was deliberately removed from `knowledge` by ADR-006 in favor of spec-kit, and the model namespaces differ.

**What the import is actually worth**: framework_ia's skills use the identical `SKILL.md` format, so they port directly, and they are consistently deeper on shared topics. Measured at spec time:

| framework_ia skill | lines | kn counterpart | lines |
|---|---:|---|---:|
| `github-actions` | 1072 | `github-actions-best-practices` | 763 |
| `playwright` | 1017 | `uiux-playwright` | 409 |
| `supabase` + `postgresql` | 988 + 747 | `supabase-postgres-best-practices` | 686 |
| `docker` | 948 | `docker-best-practices` | 582 |
| `fastapi` | 769 | `fastapi-best-practices` | **152** |
| `astro` | 616 | `astro-best-practices` | 467 |

Nine framework_ia skills have no counterpart at all: `observability` (1175), `tailwind` (1136), `testing` (1026), `alembic` (883), `object-storage` (754), `bun` (734), `redis` (556), `engram` (549), `context7` (306).

Two are explicitly discarded: `framework-init` (superseded by the `kn` CLI's own initialization) and `beads` (contradicts ADR-006).

**Pre-existing defect this feature also repairs**: initiative 001 curated the skill catalog but never updated the agents that referenced it. Three agents point at four skills that 001 deleted, and five skills that 001 added are referenced by no agent. The CLI's dependency resolution is broken today as a result.

**Security item**: framework_ia carries a live Context7 API key in plaintext in two tracked files, present in two pushed commits. The maintainer has decided to remove it from the working code without rewriting history, accepting that it remains in the repository's past.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - One catalog, the best version of each topic (Priority: P1)

The maintainer asks an agent for guidance on FastAPI and receives 152 lines of thin material, while a far more complete 769-line treatment of the same topic sits in the other repository. Today they must remember which repo holds the better version of any given topic.

**Why this priority**: This is the substance of the unification and the reason it is worth doing. Every other story either supports it or cleans up around it. It is independently valuable — even shipped alone, the catalog becomes the single best source.

**Independent Test**: Ship only this story, and for every topic covered by both repositories, `knowledge` holds a single skill that is at least as complete as the better of the two originals, with no content silently lost.

**Acceptance Scenarios**:

1. **Given** a topic covered in both repositories, **When** the merged skill is compared against both originals, **Then** no substantive guidance present in either original is missing from the result.
2. **Given** the two framework_ia skills that both concern the same datastore topic, **When** they are reconciled against the single existing counterpart, **Then** the outcome is a coherent catalog with no duplicated or contradictory guidance across skills.
3. **Given** any imported skill, **When** its metadata is inspected, **Then** it follows the same naming and metadata conventions as the skills already in the catalog.
4. **Given** any imported skill, **When** the catalog is read as a whole, **Then** naming is internally consistent — a reader cannot tell which skills were imported and which were original.

---

### User Story 2 - Only skills that match the maintainer's real work (Priority: P2)

Initiative 001 deliberately shrank the catalog to match the maintainer's actual stacks, removing cloud and orchestration skills they do not use. Importing 9 new skills wholesale risks re-inflating the catalog with material they will never load.

**Why this priority**: It protects the decision 001 already made. Without an explicit gate, this feature quietly undoes the previous initiative's purpose. It ranks below the merge because a slightly oversized catalog is a smaller problem than a catalog with a stub as its FastAPI guidance.

**Independent Test**: Ship only this story, and every candidate skill has a recorded accept-or-reject decision with a stated reason, and no skill enters the catalog without one.

**Acceptance Scenarios**:

1. **Given** the set of candidate skills with no existing counterpart, **When** each is assessed, **Then** an explicit accept or reject decision is recorded for every one, with the reason stated.
2. **Given** a candidate that does not correspond to any technology the maintainer actually uses, **When** it is assessed, **Then** it is rejected rather than imported "just in case".
3. **Given** the two skills marked for discard, **When** the import completes, **Then** neither appears in the catalog.
4. **Given** the completed import, **When** the catalog size is compared against the pre-import state, **Then** the growth is accounted for skill by skill by the recorded decisions.

---

### User Story 3 - Agents and skills that actually match (Priority: P3)

An agent declares it requires a skill that no longer exists, so dependency resolution fails or silently skips it. Meanwhile five skills sit in the catalog that no agent will ever load. The maintainer cannot trust that installing an agent gives them a working setup.

**Why this priority**: This is a live defect, not an enhancement — but it is a defect in a personal tool with a single user who knows the workarounds. It ranks below getting the content right, and it must happen after the import because the import changes which skills exist.

**Independent Test**: Ship only this story, and every skill referenced by any agent exists in the catalog, and every skill in the catalog is reachable through at least one agent or stack preset.

**Acceptance Scenarios**:

1. **Given** every agent definition, **When** each skill it references is looked up, **Then** all of them exist in the catalog.
2. **Given** every skill in the catalog, **When** the agents and stack presets are searched, **Then** each skill is referenced by at least one of them.
3. **Given** an agent whose references were repaired, **When** its declared skills are read, **Then** they reflect the technologies that agent is actually responsible for.
4. **Given** the repaired state, **When** the consistency check is run again, **Then** it reports zero dangling references and zero orphaned skills.

---

### User Story 4 - Stack presets that use the enriched catalog (Priority: P4)

The maintainer starts a new project from a stack preset and gets the skill set defined before the import, missing newly available material that is directly relevant to that stack.

**Why this priority**: Presets are the maintainer-facing payoff of a richer catalog, but they are a thin mapping layer — worthless until the catalog underneath is right, and quick to update once it is.

**Independent Test**: Ship only this story, and each stack preset resolves to a skill set that includes the newly accepted skills relevant to that stack, with every referenced skill existing.

**Acceptance Scenarios**:

1. **Given** a stack preset, **When** it is resolved, **Then** every skill it names exists in the catalog.
2. **Given** a newly accepted skill clearly belonging to a stack's technology set, **When** that stack's preset is resolved, **Then** the skill is included.
3. **Given** all presets, **When** they are resolved, **Then** none references a discarded or rejected skill.

---

### User Story 5 - framework_ia retired without leaving a live secret (Priority: P5)

Someone opens framework_ia months from now, does not realize it was superseded, and works from stale material — or worse, copies its configuration files, propagating the API key they contain into a new project.

**Why this priority**: The unification is not finished while the donor repository still looks active. It ranks last because it is cleanup rather than capability, but the secret-removal portion is not optional.

**Independent Test**: Ship only this story, and framework_ia's working tree contains no plaintext credential, and anyone opening it is told within the first screen that it is retired and where the material now lives.

**Acceptance Scenarios**:

1. **Given** framework_ia's current files, **When** they are scanned for credentials, **Then** no plaintext API key is present and the configuration reads its credential from the environment instead.
2. **Given** framework_ia's configuration, **When** someone tries to use it after the change, **Then** it is evident which environment variable must be supplied.
3. **Given** framework_ia's entry documentation, **When** a visitor opens it, **Then** it states the repository is retired and points to `knowledge`.
4. **Given** any material in framework_ia that was rejected or discarded rather than imported, **When** the retirement is recorded, **Then** the record states what was left behind, so the decision is not silently lost.

---

### Edge Cases

- A merged skill contains guidance from both sources that genuinely conflicts (different recommended patterns for the same task); the merge must resolve the contradiction rather than concatenate both.
- The two framework_ia datastore skills overlap each other as well as the kn counterpart, so a naive three-way concatenation would triple some guidance.
- An imported skill references a technology stack that the maintainer's agents do not cover, leaving it accepted but unreachable — which User Story 3 would then flag as an orphan.
- The `context7` and `engram` skills describe tooling that `knowledge` already references as MCP servers in agent metadata; importing them could duplicate guidance that already exists in agent definitions.
- Removing the plaintext key from framework_ia's configuration breaks that repository's own tooling for anyone who has not set the environment variable — acceptable for a repository being retired, but it must not appear to be a silent breakage.
- The key remains in framework_ia's history by explicit decision; the retirement record must not imply the credential was fully revoked by this work.
- An imported skill may carry example credentials in its text that a secret scanner flags as real (framework_ia's object-storage skill contains a well-known documentation placeholder); these must be distinguished from genuine secrets rather than blindly stripped.

## Requirements *(mandatory)*

### Functional Requirements

**Content merge**

- **FR-001**: For every topic covered by both repositories, the catalog MUST end with exactly one skill, retaining all substantive guidance from whichever source was more complete.
- **FR-002**: Where two donor skills cover a topic held by a single existing skill, the result MUST be a coherent set with no duplicated or contradictory guidance.
- **FR-003**: Where the two sources give conflicting guidance for the same task, the merged skill MUST state a single recommendation rather than presenting both.
- **FR-004**: Every imported or merged skill MUST follow the catalog's existing metadata and naming conventions.
- **FR-005**: No substantive guidance present in either source MUST be lost without an explicit recorded decision to drop it.

**Import vetting**

- **FR-006**: Every candidate skill without an existing counterpart MUST receive a recorded accept-or-reject decision with a stated reason before entering the catalog.
- **FR-007**: The vetting decision MUST be made against the technologies the maintainer actually works with, not against hypothetical future use.
- **FR-008**: The skills identified for discard — the donor's own scaffolding skill and its issue-tracker skill — MUST NOT enter the catalog.
- **FR-009**: The record of decisions MUST cover every candidate, so the resulting catalog size is fully accounted for.

**Consistency repair**

- **FR-010**: Every skill referenced by any agent MUST exist in the catalog.
- **FR-011**: Every skill in the catalog MUST be referenced by at least one agent or stack preset.
- **FR-012**: Repaired agent references MUST reflect that agent's actual area of responsibility.
- **FR-013**: A repeatable check MUST exist that reports dangling agent references and orphaned skills, so this class of defect is detectable in future initiatives rather than discovered by inspection.

**Stack presets**

- **FR-014**: Every skill named by a stack preset MUST exist in the catalog.
- **FR-015**: Newly accepted skills belonging to a stack's technology set MUST be included in that stack's preset.
- **FR-016**: No stack preset MUST reference a discarded or rejected skill.

**Donor retirement**

- **FR-017**: The donor repository's working files MUST contain no plaintext credential; the credential MUST be supplied from the environment instead.
- **FR-018**: The donor's configuration MUST make clear which environment variable is required.
- **FR-019**: The donor's entry documentation MUST state that it is retired and point to `knowledge`.
- **FR-020**: The retirement record MUST list what was deliberately left behind, and MUST state that the credential remains in the donor's history and therefore still requires rotation at its provider.

**Scope protection**

- **FR-021**: The CLI's behavior MUST be unchanged; no command is added, removed, or altered.
- **FR-022**: The donor's issue-tracker workflow, its agent definitions, and its shell initialization script MUST NOT be adopted.
- **FR-023**: The existing spec-driven workflow MUST remain the project's workflow.
- **FR-024**: The donor repository's history MUST NOT be rewritten.

### Key Entities

- **Skill**: A self-contained guidance document on one technology or practice, carrying metadata that lets agents and presets reference it by name. The unit being merged, imported, or rejected.
- **Agent**: A role definition that declares which skills it requires and recommends. Consumer of the catalog; the source of dangling references when the catalog changes underneath it.
- **Stack preset**: A named bundle mapping one of the maintainer's real project shapes to the skill set it needs. Second consumer of the catalog.
- **Vetting decision**: The recorded accept-or-reject outcome for one candidate skill, with its reason. The artifact that keeps the catalog aligned with real use.
- **Donor repository**: `framework_ia`, the source of imported material, retired once the import completes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For every topic present in both repositories, exactly one skill remains, and it is no less complete than the better of the two originals.
- **SC-002**: The catalog's guidance on every shared topic is at least as extensive as the deepest available version; the topic that was previously a stub is no longer one.
- **SC-003**: Every candidate skill has a recorded accept-or-reject decision with a reason; zero skills enter the catalog undocumented.
- **SC-004**: Zero agent references point at a skill that does not exist, down from six such references across three agents (naming four distinct missing skills).
- **SC-005**: Zero skills in the catalog are unreachable from every agent and every stack preset, down from five.
- **SC-006**: Every stack preset resolves completely, with zero missing skills.
- **SC-007**: A consistency check can be run on demand and reports the agent-to-skill graph as clean.
- **SC-008**: Zero plaintext credentials remain in the donor repository's working files.
- **SC-009**: A visitor opening the donor repository learns it is retired without reading past the first screen.
- **SC-010**: The maintainer can find the best available guidance on any covered topic by consulting one repository instead of two.
- **SC-011**: The existing automated checks pass unchanged, and the CLI's observable behavior is identical before and after.

## Assumptions

- `knowledge` is the surviving repository; `framework_ia` is the donor and is retired at the end of this work.
- The maintainer's real technology set is the one recorded by initiative 001 — command-line tools, web applications, APIs, data work and scripts, built with Astro, FastAPI, Supabase, Railway, and increasingly HTMX and Go. Vetting decisions are made against this set.
- Skills from both repositories share a compatible document format, so importing is a content exercise rather than a format conversion.
- Where the donor's version of a shared topic is substantially longer, it is assumed to be more complete, but each merge is verified rather than assumed — length is a signal, not proof.
- The donor's agent definitions are not imported; `knowledge` keeps its own agent roster and metadata schema.
- The credential found in the donor repository is treated as compromised and requiring rotation at its provider regardless of this work, because removing it from current files does not invalidate it and the maintainer has chosen not to rewrite history.
- The donor repository is private, which bounds the exposure of that credential but does not eliminate it.
- Initiative 002 (public-ready repository) is independent of this work and may proceed in either order; the two touch different areas.
- Retiring the donor means marking it retired and no longer developing it, not deleting it.
