# Release v0.1.0 - Checklist

## Pre-Release Verification ✅

- [x] Version numbers updated:
  - [x] cli/Cargo.toml: `version = "0.1.0"` ✅
  - [x] debian/changelog: `kn (0.1.0-1)` ✅
  - [x] kn.spec: `Version: 0.1.0` ✅

- [x] Documentation complete:
  - [x] CHANGELOG.md has v0.1.0 entry ✅
  - [x] README.md up to date ✅
  - [x] All platform guides complete ✅

- [x] Code quality:
  - [x] All epics closed ✅
    - knowledge-818 (CLI Development) - CLOSED
    - knowledge-cqb (Installation System) - CLOSED
  - [x] CI/CD workflows ready ✅

- [x] Git status:
  - [x] All changes committed locally ✅
  - [ ] All changes pushed to remote (PENDING - auth issue)

## Git Authentication Issue 🔐

**Current issue**: Permission denied when pushing to remote

```
remote: Permission to kobogithub/knowledge.git denied to kevintali.
fatal: unable to access 'https://github.com/kobogithub/knowledge.git/': The requested URL returned error: 403
```

**To resolve**:

Option 1: Re-authenticate with GitHub CLI:
```bash
gh auth login
gh auth setup-git
```

Option 2: Use SSH instead of HTTPS:
```bash
git remote set-url origin git@github.com:kobogithub/knowledge.git
```

Option 3: Update credentials:
```bash
git config --global credential.helper store
git push  # Will prompt for credentials
```

## After Authentication is Fixed

### 1. Push Commits

```bash
# Verify we're on prod branch
git branch

# Push all commits
git push origin prod

# Verify push succeeded
git status
```

### 2. Create Release Tag

```bash
# Create annotated tag
git tag -a v0.1.0 -m "Release v0.1.0 - Knowledge Framework CLI Initial Release

This is the first official release of the Knowledge Framework CLI.

Features:
- Project initialization (kn init)
- Skills management (kn skills install/list)
- Issue templates (kn beads template)
- MCP server configuration (kn mcp add/list/remove)
- Dependency verification (kn doctor)
- Multi-platform installation (7 methods)
- CI/CD automation

See CHANGELOG.md for complete details."

# Verify tag was created
git tag -l -n9 v0.1.0

# Push tag to remote
git push origin v0.1.0
```

### 3. Monitor GitHub Actions

Once the tag is pushed, GitHub Actions will automatically:

1. **Run CI Tests** (`.github/workflows/ci.yml`)
   - Test on Linux, macOS, Windows
   - Run with stable and MSRV (1.70.0)
   - Check formatting and linting

2. **Build Release** (`.github/workflows/release.yml`)
   - Build binaries for 5 platforms
   - Build Debian package
   - Build RPM packages (3 distributions)
   - Generate checksums
   - Create GitHub Release

**Monitor at**: https://github.com/kobogithub/knowledge/actions

Expected duration: ~10-15 minutes

### 4. Verify Release

Once GitHub Actions completes:

1. **Check Release Page**:
   - Go to: https://github.com/kobogithub/knowledge/releases
   - Verify v0.1.0 release exists
   - Check all artifacts are uploaded:
     - [ ] kn-linux-x86_64.tar.gz
     - [ ] kn-linux-arm64.tar.gz
     - [ ] kn-macos-x86_64.tar.gz
     - [ ] kn-macos-arm64.tar.gz
     - [ ] kn-windows-x86_64.exe.zip
     - [ ] kn_0.1.0-1_amd64.deb
     - [ ] kn-0.1.0-1.*.rpm (3 files for Fedora/EL8/EL9)
     - [ ] checksums.txt
   - Verify release notes are present

2. **Test a Binary**:
   ```bash
   # Download Linux binary
   wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn-linux-x86_64.tar.gz
   
   # Extract
   tar xzf kn-linux-x86_64.tar.gz
   
   # Test
   ./kn --version
   ./kn doctor
   ./kn --help
   ```

3. **Test Automated Installer**:
   ```bash
   # This should work immediately after release
   curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash
   ```

### 5. Update Homebrew Formula (Post-Release)

After the release is created, update the Homebrew formula with the correct SHA256:

```bash
# Download source tarball
wget https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz

# Calculate SHA256
sha256sum v0.1.0.tar.gz
# On macOS: shasum -a 256 v0.1.0.tar.gz

# Update Formula/kn.rb with:
# - url: https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz
# - sha256: <calculated hash>

# Commit and push
git add Formula/kn.rb
git commit -m "Update Homebrew formula SHA256 for v0.1.0"
git push origin prod
```

### 6. Announce Release (Optional)

- [ ] Create announcement in GitHub Discussions
- [ ] Update README badges (if any)
- [ ] Social media posts (Twitter, LinkedIn, etc.)
- [ ] Blog post (if applicable)

---

## Expected Artifacts

After successful release, users will be able to install via:

### 1. Automated Script (Immediate)
```bash
curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash
```

### 2. Pre-compiled Binaries (Immediate)
Download from: https://github.com/kobogithub/knowledge/releases/download/v0.1.0/

### 3. Homebrew (After formula update)
```bash
brew tap kobogithub/knowledge
brew install kn
```

### 4. Debian/Ubuntu (Immediate)
```bash
wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn_0.1.0-1_amd64.deb
sudo apt install ./kn_0.1.0-1_amd64.deb
```

### 5. Fedora/RHEL (Immediate)
```bash
wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn-0.1.0-1.el9.x86_64.rpm
sudo dnf install ./kn-0.1.0-1.el9.x86_64.rpm
```

---

## Troubleshooting

### If GitHub Actions Fails

1. **Check logs**: https://github.com/kobogithub/knowledge/actions
2. **Common issues**:
   - Build failure: Fix and create v0.1.1
   - Missing dependencies: Update workflow
   - Permission issues: Check GITHUB_TOKEN permissions

3. **If critical failure**:
   ```bash
   # Delete bad release
   gh release delete v0.1.0 --yes
   
   # Delete tag
   git tag -d v0.1.0
   git push --delete origin v0.1.0
   
   # Fix issue, then retry
   ```

### If Binary Doesn't Work

- Check platform compatibility
- Verify SHA256 checksum
- Test in clean environment
- Check GitHub Actions build logs

---

## Success Criteria

Release is successful when:

- [x] Epic knowledge-818 is closed
- [ ] Git commits are pushed to remote
- [ ] Tag v0.1.0 is created and pushed
- [ ] GitHub Actions completes successfully
- [ ] All 9 artifacts are uploaded to GitHub Release
- [ ] At least one binary is tested and works
- [ ] install.sh works from GitHub
- [ ] Release notes are visible on GitHub

---

## Current Status

✅ **Local work complete**:
- All code finished
- All epics closed
- Version numbers correct
- Documentation complete
- Commits made locally

⏳ **Pending**:
- Fix git authentication
- Push commits to remote
- Create and push v0.1.0 tag
- Monitor GitHub Actions
- Verify release

---

**Next Action**: Fix git authentication, then execute steps 1-6 above.

Once authentication is resolved, the release process will take about 15-20 minutes total.
