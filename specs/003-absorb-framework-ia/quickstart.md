# Quickstart: Verifying 003-absorb-framework-ia

**Feature**: 003-absorb-framework-ia
**Date**: 2026-08-06

Runnable checks proving the import worked. Each maps to a success criterion in [spec.md](./spec.md) and a contract in [contracts/catalog-surface.md](./contracts/catalog-surface.md).

Run from the repository root. `DONOR` refers to the donor checkout:

```bash
export DONOR=/Users/kobo/Github/personal/framework_ia
```

---

## V1 — Catalog composition (SC-003, C2)

```bash
ls skills/ | wc -l          # expect 29
for s in alembic framework-init beads aws-best-practices \
         kubernetes-best-practices terraform-best-practices notion-reporting-standard; do
  [ -d "skills/$s" ] && echo "PRESENT (should not be): $s"
done
echo "absence check done"
```

**Expected**: `29`, then no `PRESENT` lines.

**Fails if** a rejected or discarded skill was imported anyway, or a skill deleted by initiative 001 was recreated.

---

## V2 — Skill resolution (C1)

Directory name and frontmatter `name` must agree, or the CLI loads a skill under a name its own document denies.

```bash
for d in skills/*/; do
  n=$(basename "$d")
  f=$(awk -F': *' '/^name:/{print $2; exit}' "$d/SKILL.md")
  [ "$n" = "$f" ] || echo "MISMATCH  dir=$n  frontmatter=$f"
done
echo "resolution check done"
```

**Expected**: no `MISMATCH` lines. This catches the most likely import slip — copying a donor skill without renaming its frontmatter (`tailwind-astro`, `testing-qa`, `redis-caching` … all differ from their new directory names).

---

## V3 — Reference integrity (SC-004, SC-005, SC-007, C3)

The check that must survive this feature, per FR-013:

```bash
./scripts/check-skill-graph.sh
```

**Expected**: exit code 0, reporting zero dangling references and zero orphaned skills.

Equivalent inline, if the script is not yet written:

```bash
# every referenced skill exists
for f in agents/*/AGENTS.md; do
  awk '/^(required_skills|recommended_skills):/{i=1;next} /^[a-z_]+:/{i=0} i&&/^  - /{print $2}' "$f" |
  while read -r s; do [ -d "skills/$s" ] || echo "DANGLING  $f -> $s"; done
done
# every skill is referenced
for d in skills/*/; do
  s=$(basename "$d")
  grep -qr -- "$s" agents/ stacks/ || echo "ORPHAN    $s"
done
```

**Expected**: no output.

**Baseline before this feature**: 6 `DANGLING` lines across `backend`, `biz` and `devops`, and 5 `ORPHAN` lines. Both must reach zero.

---

## V4 — Preset resolution (SC-006, C4)

```bash
cargo build --release --manifest-path cli/Cargo.toml
for p in web-astro api-fastapi data-py cli-rust cli-go; do
  ./cli/target/release/kn stack show "$p"
done
```

**Expected**: each preset lists its skills with no "missing" flag. Counts should be 9, 10, 5, 5, 5 respectively.

Then confirm the CLI's own test still passes **untouched** — it hardcodes `supabase-postgres-best-practices`, which is why research R3 rejected splitting that skill:

```bash
cargo test --release --manifest-path cli/Cargo.toml
git diff --stat -- cli/    # must be empty
```

---

## V5 — Content completeness (SC-001, SC-002, C5)

The failure mode line counts cannot detect: a merged file longer than both originals that has silently lost a topic. Compare at heading level.

```bash
compare() {  # $1 = donor skill, $2 = catalog skill
  echo "── $1 -> $2"
  comm -23 \
    <(grep -E '^#{2,3} ' "$DONOR/skills/$1/SKILL.md" | sed 's/^#* *//' | sort -u) \
    <(grep -E '^#{2,3} ' "skills/$2/SKILL.md"        | sed 's/^#* *//' | sort -u)
}
compare fastapi        fastapi-best-practices
compare github-actions github-actions-best-practices
compare docker         docker-best-practices
compare astro          astro-best-practices
compare playwright     uiux-playwright
compare supabase       supabase-postgres-best-practices
compare postgresql     supabase-postgres-best-practices
```

**Expected**: for each pair, either no output, or only headings on the recorded-drops list.

**The one recorded drop**: `Migrations (Alembic/SQLAlchemy)` from `postgresql` — dropped because `alembic` was rejected and the section would contradict the Supabase CLI migration guidance in the same file.

**Fails if** any other donor heading is missing. Every such heading needs either merging in or an added entry to the recorded-drops list — silently losing it violates FR-005.

> Headings may legitimately be *reworded* during a merge. This check surfaces candidates for review; it is not a pass/fail oracle on its own. Read each reported heading before accepting it as a drop.

---

## V6 — No duplicated or contradictory guidance (FR-002, FR-003)

The three overlaps research R4 identified, checked for concatenation rather than reconciliation:

```bash
# Playwright setup must live in one place, referenced from the other
grep -c 'npx playwright install\|playwright install' skills/uiux-playwright/SKILL.md
grep -c 'npx playwright install\|playwright install' skills/testing-best-practices/SKILL.md
```

**Expected**: setup instructions in `uiux-playwright`; `testing-best-practices` should reference it rather than repeat it (expect 0, or a link).

```bash
# Object storage vs Supabase Storage: each owns its layer
grep -n 'presigned\|MinIO' skills/supabase-postgres-best-practices/SKILL.md
```

**Expected**: no generic MinIO/presigned-URL material in the Supabase skill — that belongs to `object-storage-best-practices`, cross-linked.

**Manual judgement required**: automation can find duplication but not contradiction. Read the merged sections where both sources gave advice on the same task and confirm a single recommendation is stated (FR-003).

---

## V7 — Vetting record is complete (SC-003, FR-006, FR-009)

```bash
grep -cE '^\| `(tailwind|testing|observability|redis|object-storage|bun|engram|context7|alembic)`' \
  specs/003-absorb-framework-ia/research.md
```

**Expected**: `9` — every candidate with no counterpart has a recorded decision and reason, so the catalog's growth from 21 to 29 is fully accounted for.

---

## V8 — Nothing out of scope was touched (FR-021, FR-022, C6)

```bash
git diff --stat dev...HEAD -- cli/
ls skills/ | grep -E '^(beads|framework-init)$'
git diff --stat dev...HEAD -- .specify/ install.sh
```

**Expected**: no output from any of the three. The donor's workflow, agent schema and shell initializer must not have arrived.

---

## V9 — Donor is de-secreted and retired (SC-008, SC-009, C7)

```bash
grep -rn 'ctx7sk-' "$DONOR" --exclude-dir=.git || echo "no plaintext key in working tree"
grep -rn 'CONTEXT7_API_KEY' "$DONOR/mcps/opencode.json"
head -20 "$DONOR/README.md"
```

**Expected**: no `ctx7sk-` in the working tree; the config reads the key from an environment variable; the README states retirement within the first screen and points at `knowledge`.

**Explicitly still true, and the record must say so**: the key remains in commits `3a2a39f` and `4593dd2` by the maintainer's decision not to rewrite history, and **still requires rotation at Context7**. Removing it from the working tree stops propagation; it does not invalidate the credential.

```bash
git -C "$DONOR" log --oneline -S'ctx7sk-' --all   # expect the two commits, unchanged
```

---

## V10 — Existing quality gates still pass (SC-011)

```bash
cargo fmt --check --manifest-path cli/Cargo.toml
cargo clippy --manifest-path cli/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --release --manifest-path cli/Cargo.toml
shellcheck install.sh scripts/check-skill-graph.sh
```

**Expected**: all pass. This feature touches no Rust, so any Rust failure is pre-existing and should be reported rather than absorbed. The new script is included in the shellcheck sweep because it ships as repository tooling.

---

## Completion gate

Done when V1–V10 pass, with V5 and V6 additionally reviewed by hand — those two are the ones automation can only partially answer, and they guard the substance of the feature.
