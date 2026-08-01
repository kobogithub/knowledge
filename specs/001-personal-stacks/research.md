# Research: Personal Stacks & Curated Catalog

Phase 0 — resolve unknowns and lock down technical decisions.

## R1 — How presets are stored and discovered

- **Decision**: Presets are standalone TOML files in `~/.kn/stacks/<name>.toml`, staged
  from a new repo `stacks/` tree by `install.sh` and `kn update`. Discovery mirrors
  `list_installed_skills()`: scan `~/.kn/stacks/` for `*.toml`.
- **Rationale**: Identical to the existing agents/skills/formulas staging pattern, so it
  needs no new distribution mechanism and stays user-editable (FR-010). One file per
  preset keeps user overrides trivial (drop a file in).
- **Alternatives considered**: (a) A single `stacks.toml` with all presets — rejected,
  harder to override one preset without touching others. (b) Presets embedded in the
  binary — rejected, not user-editable, violates FR-010.

## R2 — Preset file schema

- **Decision**:
  ```toml
  # ~/.kn/stacks/web-astro.toml
  name = "web-astro"
  description = "Astro + HTMX web app on Supabase, deployed to Railway"
  skills = [
    "astro-best-practices",
    "htmx-best-practices",
    "supabase-postgres-best-practices",
    "railway-best-practices",
    "github-actions-best-practices",
    "docker-best-practices",
  ]
  ```
- **Rationale**: Minimal, serde-friendly, mirrors `kn.toml` conventions. `skills` is an
  ordered list of catalog skill names (not paths).
- **Alternatives considered**: Adding per-skill options/versions — rejected as
  over-engineering for a single-user tool (YAGNI).

## R3 — `kn.toml` change

- **Decision**: Add an optional `stack: Option<String>` to `ProjectSection`, serialized
  with `skip_serializing_if = "Option::is_none"` so existing files are unaffected.
- **Rationale**: Backward compatible; records provenance (SC/FR-007) without breaking
  projects created before this feature.

## R4 — Curation without breaking existing projects

- **Decision**: Remove the 5 skills from repo `skills/`. `kn update` prunes them from
  `~/.kn/skills/` only if unchanged (leave user-modified dirs). If a project's `kn.toml`
  still lists a removed skill, `sync`/`init` emit a warning naming the skill and continue
  (edge case in spec); the project file is not rewritten.
- **Rationale**: Satisfies SC-005 (no existing project breaks) and the "removed skill
  still referenced" edge case.
- **Alternatives considered**: Hard-fail on missing skill — rejected, punishes existing
  projects for a catalog change.

## R5 — Preset resolution + idempotency

- **Decision**: Resolving a preset returns its ordered skill list; enabling merges into
  the existing set via the already-idempotent `KnConfig::add_skill` (dedupes). `--stack`
  and manual skill flags combine without duplicates (FR-011).
- **Rationale**: `add_skill` already guards duplicates; reuse it.

## R6 — Missing-skill handling in a preset

- **Decision**: When a preset references a skill absent from `~/.kn/skills/`, `kn init
  --stack` aborts before writing `kn.toml`, printing which skill is missing and which
  preset references it (FR-009). `kn stack list` still shows the preset but flags the
  missing skill.
- **Rationale**: Prevents partial/duplicate writes (edge case + FR-009) while keeping
  discovery informative.

## R7 — New skill content authoring

- **Decision**: Each new skill is a `skills/<name>-best-practices/SKILL.md` following the
  frontmatter+body structure of existing skills (e.g. `rust-best-practices`). Content is
  concise best-practices for FastAPI, HTMX, Go, Railway.
- **Rationale**: Consistency with the existing catalog; SKILL.md is what
  `list_installed_skills()` keys on.
- **Open**: Depth of each SKILL.md is authoring effort, not a design risk — templates
  from an existing skill will be copied and adapted in the implementation tasks.

## Out of scope (noted, not addressed here)

- `init.rs` still stages beads formulas to `.beads/formulas` and its generated `AGENTS.md`
  still references `bd` commands — stale after [[decision_migrate_bd_to_speckit]]. This is
  a **pre-existing** issue unrelated to this feature; flagged for a separate cleanup spec.
