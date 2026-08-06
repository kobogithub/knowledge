# Phase 1 Data Model: Absorb framework_ia

**Feature**: 003-absorb-framework-ia
**Date**: 2026-08-06

No runtime data. The model is the skill catalog and the reference graph that connects it to agents and stack presets — the structure this feature changes and repairs.

---

## Entities

### Skill

A self-contained guidance document on one technology or practice.

| Attribute | Values |
|---|---|
| Directory | `skills/<name>/` — the CLI resolves skills by directory name |
| `name` (frontmatter) | Must equal the directory name |
| `description`, `version`, `tags` | Required frontmatter, identical format in both repositories |
| Family | `<topic>-best-practices` (11 today) \| `security-*` (4) \| `uiux-*` (4) \| unsuffixed process skill (2) |
| Origin | original \| imported \| merged |

**Validation**: after this feature, no reader may be able to tell origin from form (SC-001). Directory name and frontmatter `name` must match, or the CLI cannot resolve it.

### Agent

A role definition declaring which skills it requires and recommends.

| Attribute | Values |
|---|---|
| File | `agents/<role>/AGENTS.md` |
| `required_skills[]` | Skills the role cannot work without |
| `recommended_skills[]` | Skills it benefits from |
| `mcp_servers[]` | MCP servers available to the role — declares *availability*, not usage guidance |

**Validation**: every named skill must exist (FR-010). This is the invariant that is currently broken.

### Stack preset

A named bundle mapping a project shape to its skill set.

| Attribute | Values |
|---|---|
| File | `stacks/<name>.toml` |
| `skills[]` | Skill names, all of which must resolve |

**Validation**: `missing_skills_in()` in `cli/src/core/stack.rs` reports unresolved names; the count must be zero (FR-014).

### Vetting decision

The recorded accept-or-reject outcome for one candidate skill, with its reason. The artifact that keeps the catalog aligned with real use (FR-006).

| Attribute | Values |
|---|---|
| Candidate | Donor skill name |
| Outcome | `accept` \| `reject` \| `discard` |
| Basis | catalog evidence \| maintainer decision \| spec instruction |

---

## The catalog transition

### Imported — 8 accepted candidates

| Donor | Lines | Becomes | Basis |
|---|---:|---|---|
| `tailwind` | 1136 | `tailwind-best-practices` | catalog evidence |
| `testing` | 1026 | `testing-best-practices` | catalog evidence |
| `observability` | 1175 | `observability-best-practices` | catalog evidence |
| `redis` | 556 | `redis-best-practices` | maintainer decision |
| `object-storage` | 754 | `object-storage-best-practices` | maintainer decision |
| `bun` | 734 | `bun-best-practices` | maintainer decision |
| `engram` | 549 | `engram-memory` | catalog evidence |
| `context7` | 306 | `context7-docs` | catalog evidence |

### Not imported — 3

| Donor | Lines | Outcome | Basis |
|---|---:|---|---|
| `alembic` | 883 | reject | Zero catalog footprint; the maintainer's database path is Supabase CLI migrations |
| `framework-init` | 166 | discard | Superseded by `kn init` (FR-008) |
| `beads` | 347 | discard | Contradicts ADR-006 (FR-008) |

### Merged — 7 donor sources into 6 catalog skills

| Catalog skill | Now | Donor source | Donor lines |
|---|---:|---|---:|
| `fastapi-best-practices` | 152 | `fastapi` | 769 |
| `github-actions-best-practices` | 763 | `github-actions` | 1072 |
| `docker-best-practices` | 582 | `docker` | 948 |
| `astro-best-practices` | 467 | `astro` | 616 |
| `uiux-playwright` | 409 | `playwright` | 1017 |
| `supabase-postgres-best-practices` | 686 | `supabase` **+** `postgresql` | 988 + 747 |

`fastapi-best-practices` is the extreme case: 152 lines against a 769-line donor version. It is effectively a stub being replaced.

### Deliberate content drops — required by FR-005

| Dropped | From | Why |
|---|---|---|
| "Migrations (Alembic/SQLAlchemy)" section | donor `postgresql` | `alembic` was rejected; keeping the section would recommend a tool the catalog says is not used, and contradict the Supabase CLI migration guidance in the same file (research R5) |

### Overlaps resolved rather than concatenated

| Overlap | Resolution |
|---|---|
| `uiux-playwright` ∩ donor `playwright` ∩ donor `testing` | Donor `playwright` merges into `uiux-playwright`; `testing-best-practices` keeps a brief E2E section that **cross-references** it |
| `object-storage-best-practices` ∩ Supabase Storage section | Object-storage owns generic S3/MinIO; the Supabase skill owns Supabase-specific APIs; each links across |
| `engram-memory`, `context7-docs` ∩ agent `mcp_servers[]` | Skills document *how to use*; agent frontmatter continues to declare *availability* |

---

## The reference graph being repaired

### Current state, measured

| Agent | `required_skills` | `recommended_skills` |
|---|---|---|
| `backend` | python, supabase-postgres, docker | bash, **kubernetes ✗**, **terraform ✗** |
| `biz` | **notion-reporting-standard ✗** | bash |
| `devops` | docker, bash, **terraform ✗**, github-actions | supabase-postgres, **aws ✗**, **kubernetes ✗** |
| `docs-writer` | documentation-guide | bash, python, github-actions |
| `finanzas` | bash | python |
| `frontend` | astro, docker | github-actions |
| `planner` | *(none)* | *(none)* |
| `qa` | bash, python, rust | github-actions, docker |
| `rust` | rust, docker | github-actions, bash |
| `security` | trivy, semgrep, gitleaks, owasp-zap | docker, python, bash, github-actions |
| `uiux-tester` | pixelmatch, playwright, axe-core, viewport-testing | astro, github-actions, bash |

*(`-best-practices` suffixes elided for width; ✗ marks a skill that does not exist.)*

**Dangling**: 6 reference instances across 3 agents, pointing at 4 skills deleted by initiative 001 — `aws-best-practices`, `kubernetes-best-practices`, `terraform-best-practices`, `notion-reporting-standard`.

**Orphaned**: 5 skills referenced by no agent — `fastapi-best-practices`, `go-best-practices`, `htmx-best-practices`, `railway-best-practices`, `standard-commits`.

**Two structural observations**:

- **`planner` declares no skills at all.** This is why `standard-commits` — a process skill every role uses — is orphaned: the role that most obviously owns it has an empty list.
- **`biz` would be left with zero required skills** once its single dangling reference is removed. The repair must give it something valid or acknowledge that it legitimately requires none.

### Target state

```
every skill in skills/  ──referenced by──▶  ≥1 agent or stack preset     (FR-011)
every reference in agents/, stacks/  ──resolves to──▶  an existing skill  (FR-010, FR-014)
```

Both directions must hold simultaneously. The eight imported skills enter the graph as orphans unless wired in the same pass, which is why US3 runs after US2.

---

## Stack preset transition

| Preset | Skills now | Add |
|---|---:|---|
| `web-astro` | 6 | `tailwind`, `testing`, `bun` |
| `api-fastapi` | 6 | `testing`, `observability`, `redis`, `object-storage` |
| `data-py` | ? | `testing`, `observability` |
| `cli-rust` | ? | `testing` |
| `cli-go` | ? | `testing` |

`engram-memory` and `context7-docs` are agent tooling, not project technology, so they are wired to agents only — a preset describes what a *project* is built from.

---

## Donor retirement

| File | Change |
|---|---|
| `mcps/opencode.json:17` | Plaintext Context7 key → environment variable |
| `.opencode/opencode.json:17` | Same |
| `template/opencode.json.template` | Same |
| `README.md` | Retirement notice pointing at `knowledge`, listing what was left behind |

**Not changed**: git history (FR-024). The key remains in commits `3a2a39f` and `4593dd2` and **still requires rotation at Context7** — removing it from the working tree stops propagation, it does not invalidate the credential (FR-020).

---

## Invariants

Checkable at completion:

1. `skills/` contains 29 directories; every directory name equals its frontmatter `name`.
2. Zero agent or preset references name a skill that does not exist.
3. Zero skills are referenced by neither an agent nor a preset.
4. Every stack preset resolves with zero missing skills.
5. Every merged skill retains every donor section heading, or the drop is recorded.
6. `alembic`, `framework-init` and `beads` appear nowhere in `skills/`.
7. No file under `cli/` is modified.
8. The donor's working tree contains no plaintext credential.
