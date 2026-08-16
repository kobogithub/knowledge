# Contract C2 — What a release must produce

**Feature**: `004-macos-arm64-homebrew`

Defines the release pipeline's obligations. The `publish-formula` job is new; everything
else is a narrowing of what already runs.

## Job graph

```
tag v* pushed
      │
      ├── build-binaries        (macos-latest, 1 target: aarch64-apple-darwin)
      │        │
      │        ├── success ──→ create-release ──→ publish-formula ──→ tap updated
      │        │
      │        └── failure ──→ STOP                (tap untouched — C2.4)
      │
      └── (scheduled, independent) tap-drift-check
```

## Clauses

| ID | Clause | Requirement | Fails if |
|---|---|---|---|
| **C2.1** | The build matrix contains exactly one entry: `aarch64-apple-darwin` on `macos-latest` | FR-002, SC-004 | More than one artifact is published |
| **C2.2** | The binary artifact is named `kn-macos-arm64.tar.gz` — unchanged | research R5 | `install.sh` and in-tool `kn update` break for every installed copy |
| **C2.3** | `publish-formula` runs only after `create-release` succeeds | FR-011 | The tap points at a release that does not exist |
| **C2.4** | If any upstream job fails, the tap is left untouched | FR-011, edge case 2 | A broken formula replaces a working one |
| **C2.5** | Both checksums are computed from the uploaded artifacts, never read from a file in the repo | FR-012 | The formula's checksum and the published bytes diverge |
| **C2.6** | The push to the tap authenticates with `TAP_GITHUB_TOKEN`, not `GITHUB_TOKEN` | research R3 | The push fails — `GITHUB_TOKEN` cannot write to another repo |
| **C2.7** | Re-running a release for an existing tag leaves the published formula in a consistent state | edge case 5 | The formula is corrupted into a half-updated state |
| **C2.8** | The release-notes template lists exactly one platform | FR-016 | Release notes advertise downloads that do not exist |

## Drift check (independent)

| ID | Clause | Requirement |
|---|---|---|
| **C2.9** | A scheduled job compares the tap formula's `version` against the newest release tag | FR-013 |
| **C2.10** | It fails loudly when they disagree | FR-013, SC-008 |
| **C2.11** | It runs on a schedule, independent of the release workflow | research R6 — an in-release assertion cannot detect later divergence |

## Verification

- Quickstart V5 — C2.1, C2.2, C2.8 on a real release
- Quickstart V6 — C2.3, C2.4 by simulating a failed build
- Quickstart V7 — C2.5 by comparing formula checksums against downloaded bytes
- Quickstart V8 — C2.9-C2.11 by pointing the check at a deliberately stale formula
