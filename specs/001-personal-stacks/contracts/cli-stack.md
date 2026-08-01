# CLI Contract: Stacks

The user-facing surface introduced/changed by this feature.

## `kn stack list`

List available stack-presets found in `~/.kn/stacks/`.

- **Args**: none.
- **Behavior**: Scans `~/.kn/stacks/*.toml`, parses each `StackPreset`, prints name,
  description, and bundled skills. Skills missing from `~/.kn/skills/` are flagged.
- **Exit 0** even when no presets exist — prints an informational "no presets available"
  message (edge case; FR-008).
- **Output (example)**:
  ```
  Available stack-presets:

  web-astro    Astro + HTMX web app on Supabase, deployed to Railway
    skills: astro-best-practices, htmx-best-practices, supabase-postgres-best-practices,
            railway-best-practices, github-actions-best-practices, docker-best-practices

  api-fastapi  FastAPI service on Supabase, deployed to Railway
    skills: fastapi-best-practices, python-best-practices, ...
  ```

## `kn stack show <name>` (optional, P3)

Show one preset's details.

- **Args**: `<name>` — preset name.
- **Exit non-zero** with a clear message + list of valid presets when `<name>` is unknown.

## `kn init --stack <name>`

Initialize a project applying a preset's skill bundle.

- **New flag**: `--stack <name>` (also usable with `-y` / non-interactive).
- **Behavior**:
  1. Resolve preset `<name>` from `~/.kn/stacks/`. Unknown name → **exit non-zero**,
     nothing written, message lists valid presets (edge case; FR-006).
  2. Verify every skill in the preset exists in `~/.kn/skills/`. Any missing → **exit
     non-zero**, nothing written, message names the missing skill(s) and the preset
     (FR-009, R6).
  3. Enable exactly the preset's skills (merged with any manual selections, deduped —
     FR-011), install/symlink them as `init` already does.
  4. Record `stack = "<name>"` under `[project]` in `kn.toml` (FR-007).
- **Interaction with existing prompts**: When `--stack` is given, skip the interactive
  recommended-skills multiselect for the preset's skills; agents flow is unchanged.

## Preset file contract (`~/.kn/stacks/<name>.toml`)

```toml
name = "web-astro"                 # required, non-empty
description = "..."                 # optional
skills = ["astro-best-practices"]  # required, non-empty, ordered
```

- Unknown/extra keys are ignored (serde default). Malformed TOML → the preset is skipped
  in `stack list` with a warning naming the file; `init --stack` on it exits non-zero.

## Backward-compatibility guarantees

- `kn init` without `--stack` behaves exactly as before.
- A `kn.toml` without a `[project].stack` key loads unchanged (`stack = None`).
- A project referencing a removed skill still loads; `sync`/`init` warn and continue
  (SC-005).
