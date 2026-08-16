# Quickstart: validation scenarios

**Feature**: `004-macos-arm64-homebrew` | **Date**: 2026-08-16

Ten scenarios that prove the feature works. Each names the contract clauses and success
criteria it covers. Record the outcome of every one in `tasks.md` at the polish phase,
including any recorded as unverified **with the reason** — the precedent from initiative
`002` is that an unverified check is reported honestly, not quietly checked off.

**Prerequisites**: an Apple Silicon Mac with Homebrew; `gh` authenticated as
`kobogithub`; Docker for V9.

---

## V1 — The current defect reproduces

Establishes the "before". Run this **first**; if it passes, the headline bug is already
gone and the plan needs revisiting.

```bash
gh api repos/kobogithub/homebrew-knowledge/contents/Formula/kn.rb -q .content \
  | base64 -d | grep -m1 'version'
gh release list --repo kobogithub/knowledge --limit 1
```

**Expected before the feature**: formula reports `0.9.0`, newest release is `v0.10.0`.
**Fails if** they already match — invariant I1 was fixed by something else.

---

## V2 — Formula lints clean (C1.1, C1.2, C1.3, C1.4)

```bash
brew style Formula/kn.rb
brew audit --strict --formula Formula/kn.rb
```

**Expected**: no offenses. **Fails if** audit reports a missing license (C1.4 — means
`002` has not merged) or an unversioned/checksum-less URL.

---

## V3 — Clean install yields the current version (SC-001, I4)

On a machine with no `kn`:

```bash
brew uninstall kn 2>/dev/null; brew untap kobogithub/knowledge 2>/dev/null
brew install kobogithub/knowledge/kn
kn --version
gh release list --repo kobogithub/knowledge --limit 1
```

**Expected**: install succeeds; `kn --version` matches the newest release tag exactly.
**Fails if** they differ — invariant I1 still violated.

---

## V4 — Installed kn is functional, not just present (C1.5, SC-001)

```bash
kn doctor
ls "$(brew --prefix)/share/kn/skills" | wc -l
ls "$(brew --prefix)/share/kn/agents" | wc -l
```

**Expected**: `kn doctor` reports healthy; both directories are non-empty.
**Fails if** either is empty — the assets resource did not install and the binary is a
shell without its skills (C1.5).

---

## V5 — A release produces exactly one artifact (C2.1, C2.2, C2.8, SC-004)

After publishing a version:

```bash
gh release view v<X.Y.Z> --repo kobogithub/knowledge --json assets \
  -q '.assets[].name'
```

**Expected**: exactly `kn-macos-arm64.tar.gz` and `checksums.txt`. **Fails if** any
Linux or Intel artifact appears, or if the binary was renamed (C2.2 — would break
in-tool `kn update` for every installed copy).

---

## V6 — A failed build leaves the tap untouched (C2.3, C2.4, edge case 2)

Simulate by pushing a tag on a branch with a deliberate compile error, or re-run the
release workflow with the build job forced to fail.

```bash
# record the tap formula version before
gh api repos/kobogithub/homebrew-knowledge/contents/Formula/kn.rb -q .content | base64 -d | grep -m1 version
# ... trigger the failing release ...
# record it after
```

**Expected**: identical before and after. **Fails if** the formula was updated to point at
a release whose artifact does not exist.

---

## V7 — Published checksums match published bytes (C2.5, I3, FR-008)

```bash
ver=$(gh release list --repo kobogithub/knowledge --limit 1 --json tagName -q '.[0].tagName')
curl -sSL -o /tmp/kn.tgz "https://github.com/kobogithub/knowledge/releases/download/$ver/kn-macos-arm64.tar.gz"
shasum -a 256 /tmp/kn.tgz
gh api repos/kobogithub/homebrew-knowledge/contents/Formula/kn.rb -q .content | base64 -d | grep -A1 'kn-macos-arm64'
```

**Expected**: the two checksums are identical. **Fails if** they differ — the formula was
rendered from something other than the published bytes.

---

## V8 — The drift check catches drift (C2.9, C2.10, C2.11, SC-008)

Point the check at a deliberately stale formula (a branch of the tap pinned to an older
version) and confirm it fails; then run it against the real tap and confirm it passes.

**Expected**: fails loudly on the stale input, passes on the current one.
**Fails if** it passes on the stale input — the check does not actually compare anything,
which would leave I1 unobservable exactly as it is today.

---

## V9 — The install script refuses elsewhere (C3.1, C3.2, SC-005)

```bash
docker run --rm -v "$PWD:/src:ro" debian:bookworm \
  bash -c 'apt-get update -qq >/dev/null && apt-get install -y -qq curl >/dev/null && bash /src/install.sh; echo "exit=$?"'
```

**Expected**: a message naming Apple Silicon macOS as the supported platform, non-zero
exit, and **no download attempted**. **Fails if** it downloads anything, or exits zero, or
the message does not say what platform would work.

---

## V10 — The install script still works under Rosetta (C3.3, edge case 6)

The scenario most likely to be broken by a careless narrowing. Both must succeed:

```bash
# native
bash install.sh --no-confirm && kn --version
# translated
arch -x86_64 bash install.sh --no-confirm && kn --version
```

**Expected**: both install the arm64 artifact and report the same version.
**Fails if** the translated run is refused as "Intel" — the Rosetta probe was deleted
during cleanup, and the maintainer's own machine now refuses the installer from a
translated terminal.

---

## Coverage

| Scenario | Covers |
|---|---|
| V1 | Baseline — I1 violated |
| V2 | C1.1-C1.4 |
| V3 | SC-001, SC-002, I4 |
| V4 | C1.5, FR-007 |
| V5 | C2.1, C2.2, C2.8, SC-004 |
| V6 | C2.3, C2.4, FR-011 |
| V7 | C2.5, I3, FR-008 |
| V8 | C2.9-C2.11, FR-013, SC-008 |
| V9 | C3.1, C3.2, FR-003, SC-005 |
| V10 | C3.3, FR-004 |

**Not covered by any scenario, and deliberately so**: SC-006 and SC-007 (no surviving
claim of unsupported platforms, no surviving retired packaging) are verified by repository
search rather than execution — they are grep assertions in `tasks.md`, not runnable
scenarios.
