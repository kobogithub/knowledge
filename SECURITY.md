# Security Policy

`kn` is not just source code people read — it is software people install. It is
distributed as a Homebrew tap and as an `install.sh` invoked with `curl | bash`, so
anything that compromises a release runs on someone else's machine with their
permissions. Reports about that path are taken seriously.

## Supported versions

Only the latest stable release receives fixes. There are no maintained release branches.

| Version | Supported |
|---|---|
| Latest stable (`vX.Y.Z`) | ✅ |
| Release candidates (`vX.Y.Z-rc.N`) | ⚠️ For testing only — not published to the Homebrew tap |
| Anything older | ❌ |

Check what you are running with `kn --version`, and upgrade with
`brew upgrade kn` or by re-running `install.sh`.

## Reporting a vulnerability

**Do not open a public issue for a vulnerability.**

Use GitHub's private reporting:

1. Go to [Security → Advisories → Report a vulnerability](https://github.com/kobogithub/knowledge/security/advisories/new).
2. Describe what you found, how to reproduce it, and what an attacker gains.

If private advisories are unavailable to you, email **kobouharriet@gmail.com** with
`[kn security]` in the subject.

This is a personal project maintained by one person, not a funded programme. Expect a
first reply within about a week. There is no bug bounty.

## What is in scope

- The release pipeline and its artifacts: anything that lets a non-maintainer change
  what a release publishes, what the Homebrew tap serves, or what `install.sh` executes.
- `install.sh` and `install.ps1` — they run with the user's permissions.
- The `kn` binary: command injection, path traversal, or writing outside the directories
  it declares it manages.
- Credential handling: anything that leaks a token, key, or MCP server secret into logs,
  arguments, or a file with loose permissions.

## What is out of scope

- Vulnerabilities in the MCP servers `kn mcp install` can install. They are third-party
  software; report those upstream.
- Findings that require an attacker to already control the machine, or to already have
  the user's GitHub credentials.
- Version fingerprinting, missing security headers on the docs site, and similar
  informational scanner output.
- Anything on a release candidate that is already fixed on the latest stable release.

## Verifying what you installed

Every release publishes `checksums.txt` generated from the bytes it uploaded, and
`install.sh` checks the tarball against it before extracting. To verify by hand:

```bash
VERSION=vX.Y.Z
BASE="https://github.com/kobogithub/knowledge/releases/download/${VERSION}"

curl -fsSLO "${BASE}/kn-macos-arm64.tar.gz"
curl -fsSL  "${BASE}/checksums.txt"
shasum -a 256 kn-macos-arm64.tar.gz
```

Releases also carry a build provenance attestation, which proves the tarball was built
by this repository's release workflow and not uploaded by hand:

```bash
gh attestation verify kn-macos-arm64.tar.gz --repo kobogithub/knowledge
```

The Homebrew formula pins the same checksums, so `brew install` verifies them for you.
