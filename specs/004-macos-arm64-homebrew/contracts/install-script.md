# Contract C3 — Install script platform detection

**Feature**: `004-macos-arm64-homebrew`

Defines how `install.sh` must behave after narrowing. The subtle clause is C3.3: the
naive narrowing breaks the maintainer's own machine.

## Decision table

| `OSTYPE` | `uname -m` | Translated? | Outcome |
|---|---|---|---|
| `darwin*` | `arm64` | — | **Install** `kn-macos-arm64.tar.gz` |
| `darwin*` | `x86_64` | **yes** (Rosetta) | **Install** `kn-macos-arm64.tar.gz` — C3.3 |
| `darwin*` | `x86_64` | no (genuine Intel) | **Refuse**, name the supported platform — C3.2 |
| `linux-gnu*` | any | — | **Refuse** — C3.2 |
| anything else | any | — | **Refuse** — C3.2 |

## Clauses

| ID | Clause | Requirement | Fails if |
|---|---|---|---|
| **C3.1** | Detection resolves before any network request | FR-003 | An unsupported machine downloads a 404 and reports a confusing error |
| **C3.2** | Refusal names Apple Silicon macOS as the supported platform and exits non-zero | FR-003, SC-005 | The user cannot tell why it failed or what would work |
| **C3.3** | The existing Rosetta probe (`install.sh:143-149`) is **retained**; `x86_64` on macOS resolves to arm64 when translated | edge case 6, research R7 | The script refuses to install on the maintainer's own M4 from a translated terminal — a regression created by the cleanup |
| **C3.4** | Exactly one artifact name remains in the selection logic | FR-002 | Dead branches reference artifacts that are no longer built |
| **C3.5** | The install destination and PATH handling are unchanged | spec Dependencies | Scope creep into behaviour this feature does not own |

## Anti-requirement

**Do not delete the `x86_64` handling.** It looks like dead code under an
Apple-Silicon-only policy and is not: it is the Rosetta detection path. Narrowing means
changing what the *genuine Intel* outcome does, not removing the architecture check that
distinguishes genuine Intel from translated Apple Silicon.

## Verification

- Quickstart V9 — refusal on Linux, via container
- Quickstart V10 — install on Apple Silicon, native and from a translated shell
  (`arch -x86_64 zsh`), both must succeed
