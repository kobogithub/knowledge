# Phase 0 Research: Absorb framework_ia

**Feature**: 003-absorb-framework-ia
**Date**: 2026-08-06
**Donor**: `/Users/kobo/Github/personal/framework_ia` (private, 18 skills)

---

## R1 — Vetting outcome for the candidates with no counterpart

**Decision**: 8 accepted, 1 rejected. FR-006 requires a recorded decision with a reason for every candidate; this is that record.

Evidence was gathered by searching the existing catalog (`skills/`, `stacks/`, `agents/`) for each technology. A technology already referenced by the catalog is one the maintainer works with; a technology with no footprint anywhere is a candidate for rejection under FR-007.

| Skill | Lines | Decision | Reason |
|---|---:|---|---|
| `tailwind` | 1136 | **Accept** | `astro-best-practices` and the frontend agent already reference Tailwind. The donor skill is named `tailwind-astro` — it is specifically Tailwind *with* Astro, which is the maintainer's web stack. |
| `testing` | 1026 | **Accept** | Covers pytest (FastAPI), vitest (Astro) and database testing — all three of the maintainer's stacks. `pytest` already appears in six catalog files. The catalog has no general testing skill; `uiux-playwright` covers browser interaction only. |
| `observability` | 1175 | **Accept** | Logging, health checks and structlog. `python-best-practices` already references structlog, and `railway-best-practices` covers the deployment target where health checks matter. |
| `engram` | 549 | **Accept** | Engram is actively installed and used in the maintainer's sessions. A skill documenting its tools is immediately applicable. |
| `context7` | 306 | **Accept** | Context7 is already declared as an MCP server in agent frontmatter (`agents/backend/AGENTS.md` among others). The tool is configured but undocumented; this closes that gap. |
| `redis` | 556 | **Accept** *(maintainer confirmed)* | `railway-best-practices` already instructs "Agregá Postgres/Redis como servicio del proyecto", so Redis is part of the deployment vocabulary. |
| `object-storage` | 754 | **Accept** *(maintainer confirmed)* | MinIO/S3 uploads, streaming and presigned URLs. Weaker catalog footprint than the others, and partially overlapping Supabase Storage — see R4 for the deduplication requirement. |
| `bun` | 734 | **Accept** *(maintainer confirmed)* | Evidence was against it: all 12 apparent catalog hits for "bun" were substrings of "bundle"/"bundler". Accepted on the maintainer's explicit decision rather than on catalog evidence. |
| `alembic` | 883 | **Reject** | **Zero** references anywhere in the catalog. The maintainer's database path is Supabase, whose CLI provides its own migration system; Alembic applies only to a direct SQLAlchemy-on-Postgres setup that nothing in the catalog describes. Importing it would re-inflate the catalog that initiative 001 deliberately shrank. |

**Also discarded, per FR-008** (these are not "candidates" — the spec names them for removal):

| Skill | Lines | Reason |
|---|---:|---|
| `framework-init` | 166 | Documents the donor's `framework-init.sh` scaffolding, superseded by `kn init`. |
| `beads` | 347 | Contradicts [ADR-006](../../docs/adr/006-adopt-speckit-remove-beads.md), which removed Beads from this project's workflow. |

**Net catalog change**: 21 → 29 skills (+8).

**Alternatives rejected**:
- *Import all 9 and prune later* — 001's whole purpose was a curated catalog; deferring the decision defeats it.
- *Reject the three the maintainer confirmed* — the catalog evidence was ambiguous for `redis` and `object-storage` and negative for `bun`, but FR-007 asks what the maintainer actually uses, and only they can answer that. Their answer overrides the inference.

---

## R2 — Naming convention for imported skills

**Decision**: Rename every accepted skill to the catalog's `<topic>-best-practices` form, except where an established prefix family applies.

| Donor name | Donor frontmatter `name` | Catalog name |
|---|---|---|
| `tailwind` | `tailwind-astro` | `tailwind-best-practices` |
| `testing` | `testing-qa` | `testing-best-practices` |
| `observability` | `observability-logging` | `observability-best-practices` |
| `redis` | `redis-caching` | `redis-best-practices` |
| `object-storage` | `object-storage` | `object-storage-best-practices` |
| `bun` | `bun-runtime` | `bun-best-practices` |
| `engram` | `engram-memory` | `engram-memory` *(unchanged)* |
| `context7` | `context7-documentation` | `context7-docs` |

**Rationale**: FR-004 requires imported skills to follow existing conventions, and SC-001's "a reader cannot tell which skills were imported" is the test. Of the 21 current skills, 11 use `-best-practices`, 4 use the `security-` prefix and 4 use `uiux-`; only `documentation-guide` and `standard-commits` sit outside, and both are process skills rather than technology skills.

`engram-memory` and `context7-docs` are kept outside the `-best-practices` family deliberately: they document *how to drive a specific tool*, like `documentation-guide` and `standard-commits`, not best practices for a technology. Calling them `engram-best-practices` would misdescribe them.

The directory name and the frontmatter `name` field must match, since the CLI resolves skills by directory.

---

## R3 — The two donor datastore skills: do not split the existing one

**Decision**: Keep the single `supabase-postgres-best-practices` skill and enrich it from both donor sources. Do **not** split it into separate Supabase and PostgreSQL skills.

The donor splits the topic in two, and the split is defensible on content — they cover genuinely different layers:

| Donor skill | Lines | Covers |
|---|---:|---|
| `supabase` | 988 | Platform: CLI, local dev, migrations, RLS, Auth, Storage, Realtime, Edge Functions, TS type generation, webhooks |
| `postgresql` | 747 | Engine: naming, indexing, JSONB, arrays, triggers, transactions, performance, RLS, patterns |

The existing `supabase-postgres-best-practices` (686 lines) blends both, thinner on each.

**Why not split anyway**: renaming or replacing that skill has fallout well beyond the catalog. It is referenced by three stack presets (`web-astro`, `api-fastapi`, `data-py`), two agent definitions (`backend`, `devops`), `.agent/README.md`, and — decisively — **`cli/tests/stack_integration.rs:151`, which hardcodes the string**. FR-021 and FR-022 put `cli/` out of scope for this feature, so a rename cannot be completed without violating the spec's own scope protection.

Initiative 001 also created this skill as one unit on purpose. Reversing that needs its own justification, not a side effect of an import.

**Consequence**: the merged skill will be the catalog's largest at roughly 1,400 lines. That is accepted, and mitigated by organising it with clear top-level sections so the platform and engine material remain separately navigable.

**Alternative rejected**: *Split into `supabase-best-practices` + `postgresql-best-practices`* — better organised, but requires editing `cli/tests/` and renaming a skill three presets depend on. If the maintainer wants this later, it belongs in its own initiative.

---

## R4 — Overlaps the merge must resolve

Three overlaps exist that a naive per-file merge would turn into duplicated or contradictory guidance, which FR-002 and FR-003 forbid.

**1. Playwright, three ways.**

| Source | Lines | Scope |
|---|---:|---|
| `uiux-playwright` (catalog) | 409 | Driving a browser through the Playwright MCP: install, MCP config, MCP tools, interaction workflows, wait patterns, per-component checklist |
| `playwright` (donor) | 1017 | The above **plus** Playwright Test for CI, accessibility testing, debugging patterns, Bun test-runner integration, and an explicit "MCP for exploration, Playwright Test for CI" split |
| `testing` (donor) | 1026 | A general testing strategy in which Playwright is one E2E section among pytest, vitest and database testing |

**Decision**: the donor's `playwright` is a superset of `uiux-playwright` — merge it there, keeping the `uiux-` name. The donor's `testing` skill keeps its brief E2E section but **cross-references** `uiux-playwright` instead of restating setup, so the two do not drift.

**2. Object storage vs Supabase Storage.** The accepted `object-storage-best-practices` covers MinIO/S3, while `supabase-postgres-best-practices` has a Storage section. Supabase Storage is S3-compatible, so the two can contradict each other on the same task. **Decision**: the object-storage skill owns generic S3/MinIO patterns; the Supabase skill owns Supabase-specific Storage APIs and links across rather than duplicating.

**3. `context7` and `engram` vs existing agent metadata.** Both are already declared as MCP servers in agent frontmatter. **Decision**: the skills document *how to use the tool*; the agent frontmatter continues to declare *that the server is available*. No agent frontmatter is rewritten to embed usage guidance.

---

## R5 — Consequence of rejecting `alembic`

The donor's `postgresql` skill contains a "Migrations (Alembic/SQLAlchemy)" section, and that skill is being merged into `supabase-postgres-best-practices`.

**Decision**: drop that section during the merge rather than carry it in. Keeping Alembic guidance while rejecting the Alembic skill would leave the catalog recommending a migration tool that R1 concluded the maintainer does not use, and would contradict the Supabase CLI migration guidance in the same file — exactly the contradiction FR-003 forbids.

This is a deliberate content drop, so FR-005 requires it be recorded. This paragraph is that record.

---

## R6 — How the merge is verified

**Decision**: for each merged skill, compare section headings between the donor and the catalog version, and confirm every donor heading is either present in the result or explicitly recorded as dropped.

**Rationale**: FR-001 and FR-005 require that no substantive guidance is lost without a recorded decision, and SC-001 requires the result be no less complete than the better original. Line count alone cannot show this — a merge could be longer while having lost a section. Heading-level comparison is checkable and catches the realistic failure mode (a whole topic dropped), which is why [quickstart.md](./quickstart.md) verifies it mechanically.

The Assumptions section of the spec already states that length is a signal, not proof.

---

## R7 — Repairing the agent↔skill graph

**Decision**: fix the four dangling references by removing them, and resolve the five orphans by assigning each to the agent that owns its subject.

Initiative 001 curated the catalog without updating the agents that referenced it. Measured state:

*Dangling — agents requiring skills that no longer exist:*

| Agent | Missing skill |
|---|---|
| `backend` | `kubernetes-best-practices`, `terraform-best-practices` |
| `devops` | `aws-best-practices`, `kubernetes-best-practices`, `terraform-best-practices` |
| `biz` | `notion-reporting-standard` |

These were deleted by 001 as outside the maintainer's stacks; the correct repair is removal from the agent frontmatter, not recreation.

*Orphaned — skills no agent references:* `fastapi-best-practices`, `go-best-practices`, `htmx-best-practices`, `railway-best-practices`, `standard-commits`.

The first four are technology skills that map cleanly onto existing agent roles. `standard-commits` is a process skill every role uses; it belongs on the planner at minimum. The eight newly imported skills must also be wired in, or they arrive orphaned on day one — FR-011 makes that a defect.

**A repeatable check is required (FR-013)** so this class of defect is caught in future initiatives rather than discovered by inspection two initiatives later. It ships as a script, verified by [quickstart.md](./quickstart.md).

---

## R8 — Stack preset updates

**Decision**: extend the existing five presets with newly accepted skills that belong to each stack's technology set. Add no new preset.

| Preset | Add |
|---|---|
| `web-astro` | `tailwind-best-practices`, `testing-best-practices`, `bun-best-practices` |
| `api-fastapi` | `testing-best-practices`, `observability-best-practices`, `redis-best-practices`, `object-storage-best-practices` |
| `data-py` | `testing-best-practices`, `observability-best-practices` |
| `cli-rust`, `cli-go` | `testing-best-practices` |

`engram-memory` and `context7-docs` are agent-tooling skills rather than project-stack skills, so they are wired to agents (R7) and not added to presets. This keeps presets describing *a project's technology*, which is what they are for.

FR-014 requires every preset to resolve completely; the CLI's `missing_skills_in` already reports unresolved names, and quickstart verifies it.

---

## R9 — Retiring the donor

**Decision**: replace the plaintext Context7 key with an environment variable in all three files that carry it, add a retirement notice to the donor's README, and leave the git history untouched.

The key `ctx7sk-…` appears in `mcps/opencode.json:17`, `.opencode/opencode.json:17` and `template/opencode.json.template`. The maintainer has decided not to rewrite history (FR-024).

**The key still requires rotation at Context7.** Removing it from the working tree stops it propagating into new projects; it does not invalidate it, and it remains in two pushed commits (`3a2a39f`, `4593dd2`). FR-020 requires the retirement record to say so, so that a reader does not mistake this work for remediation.

The donor is private, which bounds exposure but does not eliminate it.

**Retirement means marked retired, not deleted** — the repository stays as the record of what was deliberately left behind (`alembic`, `framework-init`, `beads`, the agent definitions, `framework-init.sh`, the beads workflow).

---

## Open items left to implementation

- The exact merged prose for each of the six skills — content work, not a decision.
- Which agent receives each newly imported skill as `required` versus `recommended` — decided per agent when its frontmatter is edited, following R7's principle.
