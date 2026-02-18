# Debian/Ubuntu Package Guide

## For Users

### Install from APT Repository (when available)

```bash
# Add Knowledge Framework repository
echo "deb [trusted=yes] https://repo.knowledge.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/knowledge.list

# Update and install
sudo apt update
sudo apt install kn

# Verify installation
kn doctor
```

### Install from .deb File

```bash
# Download the latest .deb package
wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn_0.1.0-1_amd64.deb

# Install with apt
sudo apt install ./kn_0.1.0-1_amd64.deb

# Or with dpkg
sudo dpkg -i kn_0.1.0-1_amd64.deb
sudo apt-get install -f  # Install dependencies
```

---

## For Maintainers

### Building the Debian Package

#### Prerequisites

```bash
# Install build dependencies
sudo apt install -y \
  build-essential \
  debhelper \
  devscripts \
  cargo \
  rustc \
  git \
  nodejs \
  npm

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Build Process

```bash
# 1. Ensure you're in the repository root
cd /path/to/knowledge

# 2. Build the package
dpkg-buildpackage -us -uc -b

# This will create:
# - kn_0.1.0-1_amd64.deb (the package)
# - kn_0.1.0-1_amd64.buildinfo
# - kn_0.1.0-1_amd64.changes

# 3. The .deb will be in the parent directory
ls -lh ../kn_*.deb
```

#### Test the Package

```bash
# Install locally
sudo dpkg -i ../kn_0.1.0-1_amd64.deb

# Test kn
kn --version
kn doctor
kn init --help

# Check installed files
dpkg -L kn

# Check package info
dpkg -s kn

# Uninstall
sudo apt remove kn
```

#### Verify Package Quality

```bash
# Check with lintian
lintian ../kn_0.1.0-1_amd64.deb

# Fix any errors or warnings before releasing
```

---

## Package Structure

The Debian package includes:

### Binary
- `/usr/bin/kn` - Main CLI executable

### Resources
- `/usr/share/kn/skills/` - Pre-installed skills
- `/usr/share/kn/agents/` - Agent templates

### Documentation
- `/usr/share/doc/kn/README.md` - Main documentation
- `/usr/share/doc/kn/README_ES.md` - Spanish documentation
- `/usr/share/doc/kn/HOMEBREW.md` - Homebrew guide
- `/usr/share/doc/kn/install.sh` - Installation script
- `/usr/share/doc/kn/install.ps1` - Windows installer

### Man Page (if available)
- `/usr/share/man/man1/kn.1` - Manual page

---

## debian/ Directory Structure

```
debian/
├── changelog          # Version history
├── compat            # Debhelper compatibility level (13)
├── control           # Package metadata and dependencies
├── copyright         # License information
├── rules             # Build instructions (Makefile)
├── kn.install        # Files to install
└── source/
    └── format        # Source package format (3.0 quilt)
```

---

## Publishing to APT Repository

### Option 1: GitHub Releases (Simple)

```bash
# 1. Build the package
dpkg-buildpackage -us -uc -b

# 2. Upload to GitHub Releases
gh release create v0.1.0 \
  --title "Release v0.1.0" \
  --notes "See CHANGELOG.md" \
  ../kn_0.1.0-1_amd64.deb

# 3. Users can download and install manually
```

### Option 2: APT Repository (Advanced)

Set up a proper APT repository using:

#### Using aptly

```bash
# Install aptly
sudo apt install aptly

# Create repository
aptly repo create -distribution=stable -component=main knowledge

# Add package
aptly repo add knowledge ../kn_0.1.0-1_amd64.deb

# Create snapshot
aptly snapshot create knowledge-0.1.0 from repo knowledge

# Publish snapshot
aptly publish snapshot -distribution=stable knowledge-0.1.0

# Serve with nginx or upload to S3/CloudFlare Pages
```

#### Using reprepro

```bash
# Install reprepro
sudo apt install reprepro

# Set up repository structure
mkdir -p repo/{conf,dists,pool}

# Configure reprepro
cat > repo/conf/distributions <<EOF
Origin: Knowledge Framework
Label: Knowledge
Codename: stable
Architectures: amd64 arm64
Components: main
Description: Knowledge Framework APT Repository
EOF

# Add package
reprepro -b repo includedeb stable ../kn_0.1.0-1_amd64.deb

# Publish repository to web server
```

---

## Multi-Architecture Support

### Building for Different Architectures

```bash
# For ARM64 (aarch64)
dpkg-buildpackage -aarm64 -us -uc -b

# For ARM (armhf)
dpkg-buildpackage -aarmhf -us -uc -b

# Cross-compilation requires:
sudo apt install crossbuild-essential-arm64
```

---

## CI/CD Integration

### GitHub Actions Example

Create `.github/workflows/debian-package.yml`:

```yaml
name: Build Debian Package

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-deb:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install dependencies
        run: |
          sudo apt update
          sudo apt install -y build-essential debhelper devscripts
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
          source $HOME/.cargo/env
      
      - name: Build package
        run: dpkg-buildpackage -us -uc -b
      
      - name: Upload to release
        uses: softprops/action-gh-release@v1
        if: startsWith(github.ref, 'refs/tags/')
        with:
          files: ../kn_*.deb
```

---

## Updating the Package

When releasing a new version:

```bash
# 1. Update version in cli/Cargo.toml
# 2. Update debian/changelog
dch -v 0.2.0-1 "New release"
dch -r ""

# 3. Build new package
dpkg-buildpackage -us -uc -b

# 4. Test
sudo dpkg -i ../kn_0.2.0-1_amd64.deb
kn --version

# 5. Upload to repository or GitHub releases
```

---

## Troubleshooting

### Build Fails

**Missing dependencies**:
```bash
# Install build dependencies from control file
sudo apt build-dep .
```

**Cargo not found**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Installation Issues

**Dependency conflicts**:
```bash
# Install with apt to resolve dependencies automatically
sudo apt install ./kn_0.1.0-1_amd64.deb
```

**kn doctor shows missing deps**:
```bash
# Install recommended packages
sudo apt install bd dolt  # If available
```

---

## Resources

- [Debian New Maintainers' Guide](https://www.debian.org/doc/manuals/maint-guide/)
- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [Ubuntu Packaging Guide](https://packaging.ubuntu.com/html/)
- [debhelper Documentation](https://man7.org/linux/man-pages/man7/debhelper.7.html)
