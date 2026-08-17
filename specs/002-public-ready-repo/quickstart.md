# Quickstart: Verifying 002-public-ready-repo

**Feature**: 002-public-ready-repo
**Date**: 2026-08-04

Runnable checks that prove the feature works end to end. Each maps to a success criterion in [spec.md](./spec.md) and a contract in [contracts/repository-surface.md](./contracts/repository-surface.md).

Run every command from the repository root.

## Prerequisites

- The repository checked out on `epic/002-public-ready-repo`
- `gh` authenticated as the account with access to this repository (`gh auth switch --user kobogithub`)
- For the package-build check only: a Linux environment with `rpmbuild` and `dpkg-buildpackage`

---

## V1 — Root is clean (SC-002, C1)

```bash
ls *.md LICENSE 2>/dev/null
```

**Expected**: exactly these seven, nothing else.

```
AGENTS.md  CHANGELOG.md  CLAUDE.md  CONTRIBUTING.md  LICENSE  README.md  README_ES.md
```

**Fails if**: any packaging or release document still sits at the root, or the stale v0.1.0 checklist survives.

---

## V2 — Licence is detected (SC-001, C2)

```bash
gh repo view kobogithub/knowledge --json licenseInfo
```

**Expected**: a non-null object naming MIT — `{"licenseInfo":{"key":"mit","name":"MIT License"}}`.

**Fails if**: `null`. The usual cause is a modified licence body — GitHub matches against known texts, so any editorializing drops detection. Confirm the copyright line reads exactly `Copyright (c) 2026 Kevin Barroso` and the rest is unmodified MIT text.

> This check reflects the pushed state on the default branch, so it only turns green after the feature merges to `prod`. Before then, verify the file's content locally instead.

---

## V3 — Relocated documents exist where promised (C3)

```bash
for f in docs/packaging/HOMEBREW.md docs/packaging/DEBIAN.md docs/packaging/RPM.md \
         docs/release/RELEASE.md docs/release/GITHUB_PAGES.md \
         docs/security/SECURITY_AUDIT_REPORT.md docs/README.md; do
  [ -f "$f" ] && echo "ok   $f" || echo "MISS $f"
done
```

**Expected**: seven `ok` lines.

---

## V4 — No broken internal links (SC-003, C3)

Every relative Markdown link resolves:

```bash
git ls-files '*.md' | while read -r f; do
  grep -oE '\]\(\.{0,2}/[^)#]+' "$f" | sed 's/^](//' | while read -r link; do
    target="$(cd "$(dirname "$f")" && printf '%s' "$(realpath -m "$link" 2>/dev/null)")"
    [ -e "$target" ] || echo "BROKEN $f -> $link"
  done
done
```

**Expected**: no output.

The same check runs in CI via the `lychee` job added by this feature, so it stays enforced on future changes.

---

## V5 — Build recipes name no missing path (SC-010, C4)

The highest-risk regression. Confirm neither recipe still copies a relocated file from the root:

```bash
grep -nE 'cp .*(HOMEBREW|DEBIAN|RPM|RELEASE|GITHUB_PAGES|SECURITY_AUDIT)[A-Za-z_.]*\.md' kn.spec debian/rules
```

**Expected**: no output. Both recipes rely on their existing recursive `docs/` copy instead.

Then confirm that recursive copy is still present in both:

```bash
grep -n 'cp -r docs' kn.spec debian/rules
```

**Expected**: one hit in each file. Without it the documents stop shipping entirely.

---

## V6 — Package builds succeed and ship the docs (SC-010, C4)

Requires a Linux environment; skip elsewhere and note it as unverified.

```bash
rpmbuild -bb kn.spec --define "_topdir $PWD/build-rpm" 2>&1 | tail -20
dpkg-buildpackage -us -uc -b 2>&1 | tail -20
```

**Expected**: both complete without error. Then confirm each relocated document ships exactly once:

```bash
rpm -qlp build-rpm/RPMS/*/kn-*.rpm | grep -c 'HOMEBREW.md'   # expect 1
```

**Fails if** the count is 0 (the recursive copy was removed too) or 2 (a per-file copy line was repointed instead of deleted, shipping the document twice).

### V6a — Static fallback when no Linux host is available

V6 needs `rpmbuild`/`dpkg-buildpackage`. When neither is available, run this instead: it resolves every source path both recipes copy, which catches the exact failure mode the relocation risks (a `cp` naming a path that no longer exists).

```bash
python3 - <<'PY'
import re, glob, os
for path, skip in (('kn.spec', ('%{', '$')), ('debian/rules', ('$(', 'debian/kn'))):
    print(f"=== {path} ===")
    for n, line in enumerate(open(path), 1):
        m = re.match(r'^(cp|install)\s+(.*)$', line.strip().rstrip('\\').strip())
        if not m: continue
        args = [a for a in m.group(2).split()
                if not a.startswith('-') and not re.fullmatch(r'[0-7]{3,4}', a)]
        for src in args[:-1]:                       # last arg is the destination
            if any(k in src for k in skip): continue
            ok = glob.glob(src) or os.path.exists(src)
            print(f"  {'ok  ' if ok else 'MISS'} L{n} {src}")
PY
```

**Expected**: every path reports `ok`, except `docs/kn.1`, which is guarded by `if [ -f docs/kn.1 ]` in both recipes and is legitimately absent (no man page is generated).

**Fails if** any unconditional source path reports `MISS` — that path will break the build.

---

## V7 — Installed-path references corrected (C4)

These are paths inside an installed package, invisible to a repo-relative link check:

```bash
grep -rnE '/usr/share/doc/kn/(HOMEBREW|DEBIAN|RPM|RELEASE|GITHUB_PAGES|SECURITY_AUDIT)' docs/ || echo "clean"
```

**Expected**: `clean`. Any hit means a document still claims the pre-move installed path — check `docs/packaging/DEBIAN.md` and `docs/packaging/RPM.md`.

> The pattern names the relocated documents specifically. A broader `[A-Z]` match produces false positives: both recipes still run `cp README.md README_ES.md …`, so `/usr/share/doc/kn/README.md` remains a correct installed path and must not be "fixed".

---

## V8 — No maintainer-specific paths (SC-005)

```bash
git grep -In '/Users/kobo' -- . | grep -v '^specs/002-public-ready-repo/' || echo "clean"
```

**Expected**: `clean`. The feature's own spec quotes the path when describing the defect, which is why it is excluded.

---

## V9 — README reads for a newcomer (SC-004, FR-010)

```bash
grep -nE '^#{1,2} ' README.md | head -12
```

**Expected**: installation appears **before** the feature catalog. Confirm the same ordering in the Spanish version:

```bash
diff <(grep -cE '^#{1,2} ' README.md) <(grep -cE '^#{1,2} ' README_ES.md)
```

**Expected**: no difference in top-level section count (FR-012).

---

## V10 — Command and documentation parity (SC-008, FR-013)

Every shipped command module is documented, and nothing undocumented is claimed:

```bash
for c in $(ls cli/src/commands/*.rs | xargs -n1 basename | sed 's/\.rs$//' | grep -v '^mod$'); do
  grep -q "kn $c" README.md && echo "ok   $c" || echo "UNDOC $c"
done
```

**Expected**: an `ok` line for each of `agents beads doctor init mcp skills stack sync update`.

`beads` is expected to appear — the command ships, so FR-013 requires it documented. FR-014 additionally requires a note stating this project's own workflow uses spec-kit per ADR-006:

```bash
grep -niE 'spec-kit|ADR-006' README.md | head -3
```

**Expected**: at least one hit near the beads section.

---

## V11 — Contribution intake (SC-007, C5)

```bash
ls .github/ISSUE_TEMPLATE/ .github/pull_request_template.md
grep -n 'blank_issues_enabled' .github/ISSUE_TEMPLATE/config.yml
```

**Expected**: `bug_report.yml`, `feature_request.yml`, `config.yml`, the PR template, and `blank_issues_enabled: false`.

Manual confirmation, after push: open `https://github.com/kobogithub/knowledge/issues/new/choose` and verify both forms appear and no blank-issue option is offered.

---

## V12 — Nothing out of scope was touched (FR-021, FR-022, FR-023, C6)

```bash
git diff --stat dev...HEAD -- cli/ skills/ stacks/ agents/ .github/workflows/release.yml
```

**Expected**: no output. Any change here violates the scope protection the spec pins in FR-021 through FR-023.

---

## V13 — Existing quality gates still pass (SC-010)

```bash
cargo fmt --check --manifest-path cli/Cargo.toml
cargo clippy --manifest-path cli/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --release --manifest-path cli/Cargo.toml
shellcheck install.sh
```

**Expected**: all pass, unchanged from before the feature. This feature touches no Rust, so any failure here is pre-existing and should be reported rather than absorbed into this work.

---

## V14 — A clean machine installs from the README alone (SC-009)

The README must be sufficient on its own — a new user should never need `docs/` to get running.

Start a container with nothing but a shell, and follow **only** what `README.md` says:

```bash
docker run --rm -it debian:stable-slim bash -c '
  apt-get update -qq && apt-get install -y -qq curl ca-certificates >/dev/null
  # From here, run only commands copied verbatim from README.md
  curl -fsSL https://kn.foxlabar.online | sh
  kn --version
'
```

**Expected**: `kn` installs and reports its version.

**Fails if**: any step needs information that appears only in `docs/`, in `CONTRIBUTING.md`, or in a packaging guide — for example a prerequisite the README never mentions. Record which document was needed; that content belongs in the README.

> This is not the same as CI's `test-install-script` job. That job proves `install.sh` works. V14 proves the **README** is complete enough to lead someone to it unaided.

---

## Completion gate

The feature is done when V1–V5 and V7–V14 pass, and V6 either passes or is explicitly recorded as unverified with the reason (no Linux environment available).
