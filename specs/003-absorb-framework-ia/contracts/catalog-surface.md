# Contract: Skill Catalog Surface

**Feature**: 003-absorb-framework-ia
**Date**: 2026-08-06

The interface here is the skill catalog and everything that resolves names against it: the `kn` CLI, the agent definitions, the stack presets, and the maintainer reading a skill. This states what each consumer may rely on after the import.

---

## C1 — Skill resolution contract

**Consumer**: the `kn` CLI (`kn skills`, `kn init`, `kn stack`), which resolves skills by directory name.

**Guarantee**:

- Every skill is a directory `skills/<name>/` containing `SKILL.md`.
- The frontmatter `name` field equals the directory name, exactly.
- Frontmatter carries `name`, `description`, `version` and `tags`.

**Breaking change**: a directory whose `name` field disagrees with it — the CLI resolves by directory, so the skill loads under a name its own document denies.

**Verify**: for each `skills/*/SKILL.md`, the `name` field matches its parent directory.

---

## C2 — Catalog composition contract

**Consumer**: the maintainer, and initiative 001's curation decision.

**Guarantee**: the catalog holds **29** skills — the 21 before this feature plus 8 imported. Specifically absent, by recorded decision:

| Absent | Why |
|---|---|
| `alembic` | Rejected — zero catalog footprint (research R1) |
| `framework-init` | Discarded — superseded by `kn init` (FR-008) |
| `beads` | Discarded — contradicts ADR-006 (FR-008) |
| `aws-`, `kubernetes-`, `terraform-`, `notion-reporting-*` | Deleted by initiative 001; **not** recreated by this feature |

**Breaking change**: adding a skill without a recorded vetting decision. The catalog is deliberately curated, not exhaustive; growth without a reason reverses initiative 001.

**Verify**: `ls skills/ | wc -l` returns 29, and the four names above resolve to nothing.

---

## C3 — Reference integrity contract

**Consumer**: agent definitions and stack presets, which name skills by string.

**Guarantee**, in both directions:

1. Every skill named in any `required_skills`, `recommended_skills` or preset `skills[]` **exists** in the catalog.
2. Every skill in the catalog is named by **at least one** agent or preset.

**Breaking change**: either direction failing. Direction 1 breaks dependency resolution at install time. Direction 2 means a skill nobody can reach — dead weight the maintainer pays for in review effort.

This contract is currently **violated on both counts** and this feature repairs it: 6 dangling reference instances across 3 agents, and 5 orphaned skills.

**Verify**: `scripts/check-skill-graph.sh` exits zero. This script is the durable form of the contract — FR-013 requires the check be repeatable so the violation cannot recur undetected, as it did between initiative 001 and this feature.

---

## C4 — Preset resolution contract

**Consumer**: `kn stack list`, `kn stack show`, `kn init --stack`.

**Guarantee**: all five presets resolve completely — `missing_skills_in()` returns an empty list for each.

| Preset | Skills before | After |
|---|---:|---:|
| `web-astro` | 6 | 9 |
| `api-fastapi` | 6 | 10 |
| `data-py` | 3 | 5 |
| `cli-rust` | 4 | 5 |
| `cli-go` | 4 | 5 |

**Explicitly NOT guaranteed**: preset membership is stable. Presets gain skills in this feature; a project pinned to a preset gets more than it did before.

**Breaking change**: a preset naming a rejected or discarded skill, or a preset that no longer resolves.

**Verify**: `kn stack show <name>` reports no missing skill for each of the five, and `cargo test --manifest-path cli/Cargo.toml` still passes with `cli/tests/stack_integration.rs` untouched.

---

## C5 — Content completeness contract

**Consumer**: anyone reading a merged skill, expecting it to be the best available guidance on its topic.

**Guarantee**: for each of the six merged skills, every section heading present in the donor version is either present in the result or listed as a recorded drop.

One drop is recorded: the "Migrations (Alembic/SQLAlchemy)" section of the donor `postgresql` skill, dropped because `alembic` was rejected and the section would contradict the Supabase CLI migration guidance in the same file.

**Breaking change**: a donor heading absent from the result with no recorded decision. This is the failure mode line counts cannot detect — a merged file can be longer than both originals while having lost a topic.

**Verify**: heading-level diff between each donor source and its merged result; every missing heading must appear in the recorded-drops list.

---

## C6 — Unchanged surface contract

**Consumer**: existing users of the CLI.

**Guarantee** — unchanged by this feature:

- Everything under `cli/` — no command added, removed or altered (FR-021). This constraint is load-bearing rather than decorative: it is precisely why the datastore skill is not split, since `cli/tests/stack_integration.rs:151` hardcodes its name.
- The donor's beads workflow, agent definitions and `framework-init.sh` are not adopted (FR-022).
- The spec-driven workflow remains this project's workflow (FR-023).
- The donor's git history is not rewritten (FR-024).

**Verify**: `git diff --stat` touches no path under `cli/`; `skills/` contains no `beads` or `framework-init`; `agents/` gains no file.

---

## C7 — Donor retirement contract

**Consumer**: anyone who opens `framework_ia` after this feature.

**Guarantee**:

- The working tree contains no plaintext credential; the Context7 key is read from an environment variable, and which variable is required is evident.
- The entry documentation states the repository is retired and points at `knowledge`.
- The record lists what was deliberately left behind.

**Explicitly NOT guaranteed**: that the credential is safe. It remains in commits `3a2a39f` and `4593dd2` by the maintainer's decision not to rewrite history, and **still requires rotation at Context7**. The retirement record must say so — a reader must not mistake this for remediation (FR-020).

**Verify**: no `ctx7sk-` string in the donor's working tree; the README's first screen states retirement; the record names `alembic`, `framework-init`, `beads`, the agent definitions and the beads workflow as left behind.

---

## Contract test summary

| Contract | Automated | Manual |
|---|---|---|
| C1 skill resolution | name/directory match sweep | — |
| C2 catalog composition | count + absence assertions | vetting record reviewed |
| C3 reference integrity | `scripts/check-skill-graph.sh` | — |
| C4 preset resolution | `kn stack show` ×5, `cargo test` | — |
| C5 content completeness | heading-level diff | judgement on contradictions |
| C6 unchanged surface | `git diff --stat` path assertions | — |
| C7 donor retirement | secret scan of the working tree | README reads as retired |
