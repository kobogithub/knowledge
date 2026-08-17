# Release Process

This document describes the release process for the Knowledge Framework CLI (`kn`).

## Overview

Cutting a release is one command: push a `v*` tag. Everything below happens in
`.github/workflows/release.yml` without further intervention.

A release produces:

- ✅ One pre-compiled binary — **Apple Silicon macOS only** (`kn-macos-arm64.tar.gz`)
- ✅ A SHA256 checksum file
- ✅ A GitHub Release with generated notes
- ✅ **An updated Homebrew formula in the tap**, published automatically

> **Do not update the tap by hand.** `kobogithub/homebrew-knowledge` is a generated
> output. It used to be a manual `cp` documented here, which is exactly how the tap
> came to serve 0.9.0 for weeks while the project shipped 0.10.0. If the formula needs
> to change, change `Formula/kn.rb` in this repository; the release publishes it.

### Binary Naming Convention

**IMPORTANT**: the release asset must be named exactly:

```
kn-macos-arm64.tar.gz
```

Two things parse this name and break if it changes:

- `install.sh`, which selects the artifact to download
- the in-tool `kn update` command, for **every copy already installed**

Renaming it is a breaking change for existing users even though only one artifact is
built. Leave it alone.

---

## Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** — incompatible changes, including dropping a supported platform
- **MINOR** — new functionality, backwards-compatible
- **PATCH** — backwards-compatible fixes

---

## Cutting a release

### 1. Update the version

```bash
vim cli/Cargo.toml          # version = "X.Y.Z"
cd cli && cargo check        # refreshes Cargo.lock
```

`Formula/kn.rb` carries no version of its own — its URLs are rewritten by the release
job, and `brew audit --strict` rejects an explicit `version` line as redundant.

### 2. Update the changelog

Move the `[Unreleased]` section to `[X.Y.Z] - YYYY-MM-DD` in `CHANGELOG.md`.

### 3. Commit, tag and push

```bash
git add cli/Cargo.toml cli/Cargo.lock CHANGELOG.md
git commit -m "chore: bump version to X.Y.Z"
git push

git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

Pushing the tag is what starts the release.

### 4. Watch it

```bash
gh run watch
```

Three jobs run in order:

| Job | Does |
|---|---|
| `build-binaries` | Native `aarch64-apple-darwin` build |
| `create-release` | Checksums, release notes, GitHub Release |
| `publish-formula` | Renders `Formula/kn.rb` from the published artifacts and pushes it to the tap |

`publish-formula` declares `needs: [create-release]`, so a failed build leaves the tap
untouched. A formula pointing at a release that does not exist is worse than a stale
one.

### 5. Verify

```bash
brew update && brew install kobogithub/knowledge/kn
kn --version        # must match the tag
```

The release job checks this itself and fails if the tap did not update. Independently,
`.github/workflows/tap-drift-check.yml` compares the tap against the newest release
every Monday and fails loudly if they diverge.

---

## If the tap did not update

1. Check the `publish-formula` job in the release run — it prints the rendered URLs and
   checksums before pushing.
2. Confirm the `TAP_GITHUB_TOKEN` secret has not expired. It is a fine-grained PAT
   scoped to `kobogithub/homebrew-knowledge` with `Contents: read and write`, and
   nothing else.
3. Re-run the job. It is idempotent — if the tap already matches, it exits without an
   empty commit.

Resist fixing it with a manual `cp`. That reintroduces the exact failure this pipeline
was built to remove; fix the pipeline instead.
