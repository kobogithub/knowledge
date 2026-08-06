# Quickstart / Validation Guide: Personal Stacks & Curated Catalog

How to prove the feature works end-to-end. Assumes a debug build of `kn` and an isolated
`KN_HOME` so the real `~/.kn` is untouched.

## Setup

```bash
cd "$(git rev-parse --show-toplevel)"   # run from the repository root
cargo build --manifest-path cli/Cargo.toml
export KN_HOME="$(mktemp -d)/.kn"
# Stage catalog + presets into the isolated home:
./install.sh --skip-deps --no-confirm   # or the update path once implemented
kn update                                # refreshes ~/.kn/skills and ~/.kn/stacks
```

## Scenario 1 — Catalog is curated (US1)

```bash
kn skills list        # or: ls "$KN_HOME/skills"
```
**Expected**: No `aws-best-practices`, `jsonnet-best-practices`,
`kubernetes-best-practices`, `terraform-best-practices`, `notion-reporting-standard`.
Present: `fastapi-best-practices`, `htmx-best-practices`, `go-best-practices`,
`railway-best-practices`, plus all retained skills.

## Scenario 2 — List presets (US3)

```bash
kn stack list
```
**Expected**: Lists `web-astro`, `api-fastapi`, `cli-rust`, `cli-go`, `data-py`, each with
its bundled skills. Runs with exit 0 even in an empty/uninitialized directory.

## Scenario 3 — Init from a preset (US2)

```bash
mkdir /tmp/demo-web && cd /tmp/demo-web
kn init -y --stack web-astro
```
**Expected**:
- `kn.toml` contains `stack = "web-astro"` under `[project]`.
- `[skills].enabled` lists exactly the `web-astro` skills (no duplicates).
- Those skills are installed/symlinked into the workspace.

## Scenario 4 — Unknown preset fails cleanly

```bash
kn init -y --stack does-not-exist
```
**Expected**: Exit non-zero, message lists valid presets, **no** `kn.toml` written.

## Scenario 5 — Preset with a missing skill fails cleanly

```bash
# Temporarily remove a skill the preset needs:
rm -rf "$KN_HOME/skills/htmx-best-practices"
cd /tmp && mkdir demo-missing && cd demo-missing
kn init -y --stack web-astro
```
**Expected**: Exit non-zero naming `htmx-best-practices` and `web-astro`; nothing written.

## Scenario 6 — Existing project referencing a removed skill doesn't break (SC-005)

```bash
# Craft a kn.toml enabling a removed skill, then:
kn sync
```
**Expected**: Warning that the skill is no longer in the catalog; command completes
without error; `kn.toml` unchanged.

## Automated coverage

- Unit: preset parse/validate, resolution, idempotent merge, missing-skill detection.
- Integration (`tempfile` + `KN_HOME`): `init --stack` happy path, unknown preset,
  missing skill, `stack list` empty vs populated.

Run: `cargo test --manifest-path cli/Cargo.toml`
