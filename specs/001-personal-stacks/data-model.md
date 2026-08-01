# Data Model: Personal Stacks & Curated Catalog

## Entities

### StackPreset (new)

A named bundle of catalog skills. Loaded from `~/.kn/stacks/<name>.toml`.

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | yes | Unique preset identifier; should match filename stem |
| `description` | String | no | One-line human description for `kn stack list` |
| `skills` | Vec<String> | yes | Ordered list of catalog skill names |

**Validation rules**:
- `name` non-empty; recommended to equal the file stem (warn if mismatch).
- `skills` non-empty.
- Each entry in `skills` should resolve to a directory in `~/.kn/skills/<skill>` with a
  `SKILL.md`. Unresolved entries are reported (see FR-009 / R6), not silently dropped.

**Rust shape** (in `core/stack.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackPreset {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub skills: Vec<String>,
}
```

### Skill (existing — catalog membership changes only)

Unit of reusable knowledge; directory `<name>/SKILL.md` under repo `skills/` and staged
to `~/.kn/skills/`. No schema change.

- **Removed from catalog**: `aws-best-practices`, `jsonnet-best-practices`,
  `kubernetes-best-practices`, `terraform-best-practices`, `notion-reporting-standard`.
- **Added to catalog**: `fastapi-best-practices`, `htmx-best-practices`,
  `go-best-practices`, `railway-best-practices`.

### ProjectSection in `kn.toml` (existing — one field added)

```rust
pub struct ProjectSection {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub workspace_standard: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,   // NEW — chosen preset name, if any
}
```

- Backward compatible: absent in pre-existing files, deserializes to `None`.

## Relationships

```
StackPreset.skills[*]  ──references──▶  Skill.name (catalog)
kn.toml.project.stack  ──references──▶  StackPreset.name
kn.toml.skills.enabled ──contains──▶    resolved skills (preset ∪ manual, deduped)
```

## Preset skill sets (shipped defaults)

| Preset | Skills |
|--------|--------|
| `web-astro` | astro, htmx, supabase-postgres, railway, github-actions, docker |
| `api-fastapi` | fastapi, python, supabase-postgres, railway, docker, github-actions |
| `cli-rust` | rust, docker, github-actions, bash |
| `cli-go` | go, docker, github-actions, bash |
| `data-py` | python, supabase-postgres, docker |

> Skill names above are shorthand; the actual files use the `-best-practices` suffix
> (e.g. `astro-best-practices`), except `supabase-postgres-best-practices`.

## State / lifecycle

Presets are static config (no runtime state transitions). The only mutable state is the
project's `kn.toml`, updated once during `init` when a preset is selected.
