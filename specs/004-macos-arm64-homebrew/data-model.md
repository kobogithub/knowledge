# Data Model: kn targets macOS Apple Silicon only, distributed via Homebrew

**Feature**: `004-macos-arm64-homebrew` | **Date**: 2026-08-16

This feature has no database and no runtime data structures. The entities that matter are
the **published artifacts** and the invariants that must hold between them. Modelling them
explicitly is what makes the drift condition statable — the current bug is precisely an
invariant between two of these entities being violated with nothing to notice.

---

## Entities

### Release

A published version of `kn`.

| Field | Type | Source | Notes |
|---|---|---|---|
| `tag` | string | git tag, `v<semver>` | Pushing it triggers `release.yml` |
| `version` | semver | derived from `tag` minus `v` | Must equal `cli/Cargo.toml` `version` |
| `binary_artifact` | file | built by `build-binaries` | Exactly one after this feature |
| `binary_sha256` | hex(64) | computed from the artifact | Feeds the formula |
| `source_archive` | URL | GitHub auto-generates per tag | Carries skills and agent templates |
| `source_sha256` | hex(64) | computed from the archive | Second checksum the automation must handle |

**Validation rules**

- `binary_artifact` MUST be named `kn-macos-arm64.tar.gz`. The name is load-bearing:
  `install.sh` selects on it and in-tool `kn update` parses it (research R5).
- A release MUST carry exactly one binary artifact (FR-002, SC-004).
- `version` MUST match the crate version. A tag that disagrees with `Cargo.toml` produces
  a formula pointing at a binary that reports a different version.
- Both checksums MUST be computed from the uploaded bytes, never authored by hand
  (FR-012).

**State transitions**

```
tag pushed → binaries built → release published → formula rendered → formula pushed to tap
                    │                                                        │
                    └── build fails ──→ STOP, tap untouched (FR-011) ────────┘
```

The stop condition is the important one. A release whose artifacts failed must leave the
tap pointing at the previous working version — a formula referencing a nonexistent
download is worse than a stale one (spec edge case 2).

---

### Authored formula

`Formula/kn.rb` in this repository. The reviewable source of truth (research R2).

| Field | Type | Notes |
|---|---|---|
| `desc`, `homepage` | string | Static |
| `license` | string | `"MIT"`. Requires the LICENSE file to exist (FR-009, research R9) |
| `version` | semver | Rendered per release |
| `url` | URL | Points at the release's `binary_artifact` |
| `sha256` | hex(64) | The release's `binary_sha256` |
| `resource "assets"` | block | URL + checksum of the release's `source_archive` |
| `depends_on :macos` | guard | Refuses non-macOS |
| `depends_on arch: :arm64` | guard | Refuses Intel Macs before downloading (research R4) |
| `caveats` | block | Surfaces a `kn` on the path outside the Homebrew prefix (FR-020, C1.8) |

**Validation rules**

- MUST contain no `on_linux`, `on_intel`, or `on_macos`/`on_arm` nesting (FR-006).
- MUST declare a license (FR-009).
- MUST install binary *and* bundled assets — the binary tarball alone is not a functional
  install (FR-007).
- Every downloaded URL MUST have a recorded checksum (FR-008). There are two, not one.

---

### Published formula

`Formula/kn.rb` in `kobogithub/homebrew-knowledge`. A **generated output** after this
feature; never hand-edited.

Same fields as the authored formula. The distinction is provenance, and it is the whole
point: today both are hand-edited and they disagree.

---

### Tap

`kobogithub/homebrew-knowledge` — public, already exists, named per Homebrew's required
`homebrew-<name>` convention.

| Field | Value |
|---|---|
| `install_command` | `brew install kobogithub/knowledge/kn` |
| `write_credential` | `TAP_GITHUB_TOKEN` — fine-grained PAT, `Contents: write`, tap-scoped only |

---

## Cross-entity invariants

These are the assertions the feature exists to establish. Each maps to a requirement and a
quickstart scenario.

| # | Invariant | Requirement | Currently |
|---|---|---|---|
| **I1** | `published_formula.version == latest_release.version` | FR-010, SC-008 | ❌ **VIOLATED** — 0.9.0 vs 0.10.0 |
| **I2** | `published_formula == authored_formula` modulo nothing | FR-012 | ❌ **VIOLATED** — 0.9.0 vs 0.10.0 |
| **I3** | `published_formula.sha256 == sha256(published binary)` | FR-008 | Holds, by luck of hand-copying |
| **I4** | Installed `kn --version` == `published_formula.version` | SC-001 | Holds |
| **I5** | A release has exactly one binary artifact | FR-002, SC-004 | ❌ Three today |
| **I6** | No repository file claims a platform outside Apple Silicon macOS | FR-014-017, SC-006 | ❌ Multiple claims |
| **I7** | A `kn` on the path outside the Homebrew prefix is surfaced, never silently shadowed | FR-020, SC-009 | Not addressed |

**I1 is the headline defect.** I2 is its mechanism. The scheduled drift check (research R6)
exists to make I1 continuously observable rather than discovered by accident, as it was
this session.

---

## Non-entities

Explicitly modelled as *absent*, so their removal is not mistaken for an oversight:

- **RPM package** — recipe `kn.spec` deleted. Was pinned to `Version: 0.1.0` while the
  project shipped 0.10.0; verified stale by building it in a container (research R8).
- **Debian package** — `debian/` deleted.
- **Windows installer** — `install.ps1` deleted.
- **Intel macOS artifact** and **Linux x86_64 artifact** — no longer produced.
- **Homebrew core formula** — out of scope; the project is far below the notability bar
  (0 stars, 0 forks).
