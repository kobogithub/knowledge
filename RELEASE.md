# Release Process

This document describes the release process for the Knowledge Framework CLI (`kn`).

## Overview

Our release process is automated via GitHub Actions and produces:
- ✅ Pre-compiled binaries for Linux, macOS, and Windows (x86_64 and ARM64)
- ✅ Debian packages (.deb) for Ubuntu/Debian
- ✅ RPM packages (.rpm) for Fedora, RHEL, Rocky, AlmaLinux
- ✅ Homebrew formula updates
- ✅ Checksums (SHA256) for all artifacts
- ✅ GitHub Release with release notes

---

## Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version (1.0.0): Incompatible API changes
- **MINOR** version (0.1.0): New functionality, backwards-compatible
- **PATCH** version (0.1.1): Backwards-compatible bug fixes

Version format: `v0.1.0`

---

## Pre-Release Checklist

Before creating a release, ensure:

### 1. Code Quality
- [ ] All tests pass: `cargo test`
- [ ] Linting passes: `cargo clippy`
- [ ] Formatting is correct: `cargo fmt --check`
- [ ] `kn doctor` works correctly
- [ ] All commands (`init`, `skills`, `beads`, `mcp`) are tested

### 2. Documentation
- [ ] README.md is up to date
- [ ] CHANGELOG.md has entry for new version
- [ ] CLI help text is accurate (`kn --help`)
- [ ] Platform-specific docs are updated (HOMEBREW.md, DEBIAN.md, RPM.md)

### 3. Version Bumps
- [ ] Update version in `cli/Cargo.toml`
- [ ] Update version in `kn.spec` (RPM)
- [ ] Update version in `debian/changelog`
- [ ] Update version in `Formula/kn.rb` (Homebrew)

### 4. Dependencies
- [ ] `Cargo.lock` is committed
- [ ] All dependencies are up to date (if desired)
- [ ] Security audit passes: `cargo audit` (install with `cargo install cargo-audit`)

---

## Release Steps

### Automated Release (Recommended)

#### 1. Update Version Numbers

```bash
# Update version in Cargo.toml
cd cli
vim Cargo.toml  # Change version = "0.1.0" to "0.2.0"

# Update Cargo.lock
cargo build

# Update Debian changelog
cd ..
vim debian/changelog  # Add new entry at top

# Update RPM spec
vim kn.spec  # Change Version: 0.1.0 to Version: 0.2.0

# Update Homebrew formula
vim Formula/kn.rb  # Change version (will be updated after release)
```

#### 2. Update CHANGELOG

```bash
vim CHANGELOG.md
```

Add entry:
```markdown
## [0.2.0] - 2026-02-20

### Added
- New feature X
- New command Y

### Changed
- Improved Z

### Fixed
- Bug fix A
```

#### 3. Commit Changes

```bash
git add cli/Cargo.toml cli/Cargo.lock debian/changelog kn.spec CHANGELOG.md
git commit -m "Bump version to 0.2.0"
git push origin prod
```

#### 4. Create Git Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push tag to trigger release workflow
git push origin v0.2.0
```

#### 5. Monitor GitHub Actions

- Go to https://github.com/kobogithub/knowledge/actions
- Watch the "Release" workflow
- It will:
  1. Build binaries for all platforms
  2. Build Debian and RPM packages
  3. Generate checksums
  4. Create GitHub Release with all artifacts

#### 6. Verify Release

- Check https://github.com/kobogithub/knowledge/releases
- Download and test a binary:
  ```bash
  wget https://github.com/kobogithub/knowledge/releases/download/v0.2.0/kn-linux-x86_64.tar.gz
  tar xzf kn-linux-x86_64.tar.gz
  ./kn --version
  ./kn doctor
  ```

#### 7. Update Homebrew Formula (Post-Release)

After the release is created, update the Homebrew formula:

```bash
# Download the source tarball
wget https://github.com/kobogithub/knowledge/archive/refs/tags/v0.2.0.tar.gz

# Calculate SHA256
sha256sum v0.2.0.tar.gz

# Update Formula/kn.rb
vim Formula/kn.rb
# Update:
#   url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.2.0.tar.gz"
#   sha256 "THE_SHA256_HASH"

# Commit and push
git add Formula/kn.rb
git commit -m "Update Homebrew formula to v0.2.0"
git push origin prod
```

#### 8. Announce Release

- Create announcement in GitHub Discussions
- Post to social media (if applicable)
- Notify users via mailing list (if applicable)

---

## Manual Release (Fallback)

If the automated process fails, you can build and upload manually:

### 1. Build Binaries

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu
tar czf kn-linux-x86_64.tar.gz -C cli/target/x86_64-unknown-linux-gnu/release kn

# macOS x86_64 (on macOS)
cargo build --release --target x86_64-apple-darwin
tar czf kn-macos-x86_64.tar.gz -C cli/target/x86_64-apple-darwin/release kn

# Windows (on Windows or with cross)
cargo build --release --target x86_64-pc-windows-msvc
zip kn-windows-x86_64.zip cli/target/x86_64-pc-windows-msvc/release/kn.exe
```

### 2. Build Packages

```bash
# Debian
dpkg-buildpackage -us -uc -b
# Creates: ../kn_0.2.0-1_amd64.deb

# RPM
rpmdev-setuptree
cp kn.spec ~/rpmbuild/SPECS/
tar czf ~/rpmbuild/SOURCES/v0.2.0.tar.gz .
rpmbuild -ba ~/rpmbuild/SPECS/kn.spec
# Creates: ~/rpmbuild/RPMS/x86_64/kn-0.2.0-1.*.rpm
```

### 3. Generate Checksums

```bash
sha256sum kn-*.tar.gz kn-*.zip ../kn*.deb ~/rpmbuild/RPMS/x86_64/kn*.rpm > checksums.txt
```

### 4. Create GitHub Release

```bash
# Using GitHub CLI
gh release create v0.2.0 \
  --title "Release v0.2.0" \
  --notes-file release-notes.md \
  kn-linux-x86_64.tar.gz \
  kn-macos-x86_64.tar.gz \
  kn-windows-x86_64.zip \
  ../kn_0.2.0-1_amd64.deb \
  ~/rpmbuild/RPMS/x86_64/kn-0.2.0-1.*.rpm \
  checksums.txt
```

---

## Post-Release Tasks

### Update Package Repositories (When Available)

#### Homebrew Tap

```bash
# If using homebrew-knowledge tap
cd /path/to/homebrew-knowledge
cp ~/knowledge/Formula/kn.rb Formula/
git add Formula/kn.rb
git commit -m "Update kn to v0.2.0"
git push
```

#### APT Repository

```bash
# Update APT repository (if configured)
aptly repo add knowledge ../kn_0.2.0-1_amd64.deb
aptly snapshot create knowledge-0.2.0 from repo knowledge
aptly publish switch stable knowledge-0.2.0
```

#### YUM/DNF Repository

```bash
# Update YUM repository (if configured)
cp ~/rpmbuild/RPMS/x86_64/kn-0.2.0-1.*.rpm /path/to/repo/rpm/
createrepo --update /path/to/repo/rpm/
```

#### COPR (Fedora/RHEL)

```bash
# Build on COPR (if configured)
copr-cli build knowledge \
  kn.spec \
  https://github.com/kobogithub/knowledge/archive/refs/tags/v0.2.0.tar.gz
```

---

## Rollback Procedure

If a release has critical issues:

### 1. Delete the Release

```bash
# Using GitHub CLI
gh release delete v0.2.0 --yes

# Or via GitHub web interface
```

### 2. Delete the Tag

```bash
# Delete local tag
git tag -d v0.2.0

# Delete remote tag
git push --delete origin v0.2.0
```

### 3. Fix the Issue

```bash
# Fix the bug
# Test thoroughly
# Commit fixes
```

### 4. Create New Release

```bash
# Create patch release
git tag -a v0.2.1 -m "Release v0.2.1 (fixes issue from 0.2.0)"
git push origin v0.2.1
```

---

## CI/CD Workflows

### `.github/workflows/ci.yml`

Runs on every push and pull request:
- ✅ Tests on Linux, macOS, Windows
- ✅ Tests with stable Rust and MSRV (1.70.0)
- ✅ Linting (clippy, rustfmt)
- ✅ Shellcheck for install.sh
- ✅ Build verification

### `.github/workflows/release.yml`

Runs on git tag push (`v*`):
- ✅ Builds binaries for 5 platforms
- ✅ Builds Debian package
- ✅ Builds RPM packages (Fedora, EL8, EL9)
- ✅ Generates checksums
- ✅ Creates GitHub Release with all artifacts

---

## Troubleshooting

### Release workflow fails

**Check the logs:**
```bash
gh run list --workflow=release.yml
gh run view <run-id> --log
```

**Common issues:**
- **Cargo.lock not committed**: Commit it and re-tag
- **Version mismatch**: Ensure all version numbers match
- **Test failures**: Fix tests before releasing
- **Build errors**: Check cross-compilation setup

### Binary doesn't work on target platform

- Ensure target architecture matches
- Check dynamic linking (use `ldd` on Linux, `otool -L` on macOS)
- Consider static linking: `RUSTFLAGS='-C target-feature=+crt-static'`

### Package installation fails

**Debian:**
```bash
# Check dependencies
dpkg-deb -I kn_0.2.0-1_amd64.deb
# Install with dependency resolution
sudo apt install ./kn_0.2.0-1_amd64.deb
```

**RPM:**
```bash
# Check dependencies
rpm -qpR kn-0.2.0-1.*.rpm
# Install with dependency resolution
sudo dnf install ./kn-0.2.0-1.*.rpm
```

---

## Version History

| Version | Date | Notes |
|---------|------|-------|
| v0.1.0 | 2026-02-18 | Initial release |
| v0.2.0 | TBD | (Next release) |

See [CHANGELOG.md](./CHANGELOG.md) for detailed changes.

---

## Resources

- [Semantic Versioning](https://semver.org/)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [GitHub Actions](https://docs.github.com/en/actions)
- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
