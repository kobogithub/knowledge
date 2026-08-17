# Homebrew Installation Guide

## For Users

### Install from Homebrew

`kn` is published via a custom tap at
[kobogithub/homebrew-knowledge](https://github.com/kobogithub/homebrew-knowledge).

```bash
brew tap kobogithub/knowledge
brew install kobogithub/knowledge/kn
```

> **Use the fully-qualified name.** `kn` already exists in `homebrew-core` — it's the
> [Knative client CLI](https://github.com/knative/client), an unrelated tool. Plain
> `brew install kn` will install *that* one instead of ours. Always use
> `kobogithub/knowledge/kn`.

### First-time setup

Skills and agent templates ship inside the formula, but Homebrew sandboxes `$HOME` during
install, so it can't write to `~/.kn/` automatically. `brew install` prints the exact
commands to run once after installing (also reproduced here):

```bash
mkdir -p ~/.kn/{skills,agents}
cp -R "$(brew --prefix)/opt/kn/share/kn/skills/." ~/.kn/skills/
cp -R "$(brew --prefix)/opt/kn/share/kn/agents/." ~/.kn/agents/
```

### Verify

```bash
kn --version
kn doctor
```

### Optional Dependencies

`kn doctor` checks for these, but they're not required to use the CLI itself:

```bash
# Node.js — only needed for skills that assume a JS/TS toolchain
brew install node

# bd (beads) — legacy, only used by the `kn beads template` subcommand.
# Not part of the current agent workflow (see docs/adr/006-adopt-speckit-remove-beads.md).
cargo install bd
```

---

## For Maintainers

### How the formula is built

`Formula/kn.rb` in this repo is the source of truth; a copy lives in the
[homebrew-knowledge](https://github.com/kobogithub/homebrew-knowledge) tap repo
(`Formula/kn.rb` there too — that's what Homebrew actually reads).

It does **not** compile from source. It downloads the precompiled per-arch binary tarball
already published by `release.yml` for the tag (`kn-macos-arm64.tar.gz`,
`kn-macos-x86_64.tar.gz`, `kn-linux-x86_64.tar.gz`), plus a `resource "assets"` block that
pulls the tagged source archive just for `skills/`, `agents/`, and `docs/` (no build step
needed for those — they're markdown/JSON).

### Releasing a new version

```bash
# 1. Cut the release as usual (creates the binaries via release.yml)
git tag -a v0.9.0 -m "Release v0.9.0"
git push origin v0.9.0

# 2. Once release.yml has published the binaries, get the real hashes:
curl -sL "https://github.com/kobogithub/knowledge/releases/download/v0.9.0/checksums.txt"

# 3. Compute the source-archive hash (for the `assets` resource):
curl -sL "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.9.0.tar.gz" | shasum -a 256

# 4. Update Formula/kn.rb in THIS repo:
#    - version "0.9.0"
#    - the three binary url/sha256 pairs (macOS arm64/x86_64, Linux x86_64)
#    - the `resource "assets"` url/sha256

# 5. Test locally before publishing (see below)

# 6. Copy the updated formula to the tap repo and push
cp Formula/kn.rb ../homebrew-knowledge/Formula/kn.rb
cd ../homebrew-knowledge
git add Formula/kn.rb
git commit -m "kn 0.9.0"
git push
```

### Testing Locally

Homebrew (6.x+) requires formulae to live in a tap — you can't `brew install --formula
./Formula/kn.rb` directly against a bare file anymore. Test against the real tap:

```bash
brew untap kobogithub/knowledge 2>/dev/null
brew tap kobogithub/knowledge
brew install kobogithub/knowledge/kn

kn --version
kn doctor

brew uninstall kn
```

To iterate on formula changes without pushing every time, edit the tap's local clone
directly (`brew tap` clones it under `$(brew --repo kobogithub/knowledge)`), then
`brew install`/`brew postinstall` against it — push to the tap repo only once it works.

### Common Issues

**`brew install kn` installs the wrong thing**: expected — see the name-collision note
above. Always use `kobogithub/knowledge/kn`.

**Skills/agents missing after install**: expected — Homebrew's install/`post_install`
sandbox blocks writes to `$HOME` (confirmed by testing: `Dir.home` resolves to a private
tmp path during `post_install`, not the real home). The formula relies on `caveats` to
tell the user to copy `share/kn/{skills,agents}` into `~/.kn/` themselves, matching what
`install.sh`'s `setup_kn_resources()` does outside Homebrew's sandbox.

**Build fails / "cargo not found"**: shouldn't happen — this formula doesn't build from
source. If you see this, someone reverted it to a `cargo install`-based formula; re-check
`Formula/kn.rb` uses `on_macos`/`on_linux` binary URLs, not `depends_on "rust" => :build`.

---

## Publishing to Official Homebrew (homebrew-core)

Not planned for now — a custom tap is simpler to manage for a project at this stage, and
`homebrew-core` already has an unrelated `kn` (Knative), which would need a different
formula name to even be considered. Revisit only if this tool grows a broad enough user
base to justify it.

## Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Taps Documentation](https://docs.brew.sh/Taps)
- [Homebrew Formula Reference](https://rubydoc.brew.sh/Formula)
