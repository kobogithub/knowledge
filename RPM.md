# RPM Package Guide (Fedora/RHEL/openSUSE)

## For Users

### Install from DNF/YUM Repository (when available)

#### Fedora / RHEL 8+ / Rocky Linux / AlmaLinux

```bash
# Add Knowledge Framework repository
sudo dnf config-manager --add-repo https://repo.knowledge.dev/rpm/knowledge.repo

# Install kn
sudo dnf install kn

# Verify installation
kn doctor
```

#### RHEL 7 / CentOS 7

```bash
# Add Knowledge Framework repository
sudo yum-config-manager --add-repo https://repo.knowledge.dev/rpm/knowledge.repo

# Install kn
sudo yum install kn

# Verify installation
kn doctor
```

#### openSUSE

```bash
# Add repository
sudo zypper addrepo https://repo.knowledge.dev/rpm/opensuse knowledge

# Install kn
sudo zypper install kn

# Verify installation
kn doctor
```

### Install from .rpm File

```bash
# Download the latest .rpm package
wget https://github.com/kobogithub/knowledge/releases/download/v0.1.0/kn-0.1.0-1.el9.x86_64.rpm

# Install with dnf (Fedora/RHEL 8+)
sudo dnf install ./kn-0.1.0-1.el9.x86_64.rpm

# Or with yum (RHEL 7)
sudo yum localinstall ./kn-0.1.0-1.el7.x86_64.rpm

# Or with zypper (openSUSE)
sudo zypper install ./kn-0.1.0-1.x86_64.rpm
```

---

## For Maintainers

### Building the RPM Package

#### Prerequisites

##### Fedora

```bash
sudo dnf install -y \
  rpm-build \
  rpmdevtools \
  rpmlint \
  cargo \
  rust \
  git \
  gcc \
  nodejs \
  npm
```

##### RHEL / Rocky / AlmaLinux

```bash
# Enable EPEL repository
sudo dnf install -y epel-release

# Install build tools
sudo dnf install -y \
  rpm-build \
  rpmdevtools \
  rpmlint \
  git \
  gcc \
  nodejs \
  npm

# Install Rust (not in default repos)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

##### openSUSE

```bash
sudo zypper install -y \
  rpm-build \
  rpmdevtools \
  cargo \
  rust \
  git \
  gcc \
  nodejs \
  npm
```

#### Build Process

```bash
# 1. Set up RPM build environment
rpmdev-setuptree

# This creates:
# ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# 2. Copy spec file
cp kn.spec ~/rpmbuild/SPECS/

# 3. Download source tarball
cd ~/rpmbuild/SOURCES
wget https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz

# 4. Build the RPM
cd ~/rpmbuild
rpmbuild -ba SPECS/kn.spec

# This creates:
# - RPMS/x86_64/kn-0.1.0-1.*.x86_64.rpm (binary package)
# - SRPMS/kn-0.1.0-1.*.src.rpm (source package)

# 5. Find the built package
ls -lh RPMS/x86_64/kn-*.rpm
```

#### Test the Package

```bash
# Install locally
sudo dnf install ~/rpmbuild/RPMS/x86_64/kn-0.1.0-1.*.x86_64.rpm

# Test kn
kn --version
kn doctor
kn init --help

# Check installed files
rpm -ql kn

# Check package info
rpm -qi kn

# Uninstall
sudo dnf remove kn
```

#### Verify Package Quality

```bash
# Check with rpmlint
rpmlint ~/rpmbuild/SPECS/kn.spec
rpmlint ~/rpmbuild/RPMS/x86_64/kn-*.rpm
rpmlint ~/rpmbuild/SRPMS/kn-*.src.rpm

# Fix any errors or warnings before releasing
```

---

## Package Structure

The RPM package includes:

### Binary
- `/usr/bin/kn` - Main CLI executable

### Resources
- `/usr/share/kn/skills/` - Pre-installed skills
- `/usr/share/kn/agents/` - Agent templates

### Documentation
- `/usr/share/doc/kn/README.md` - Main documentation
- `/usr/share/doc/kn/README_ES.md` - Spanish documentation
- `/usr/share/doc/kn/HOMEBREW.md` - Homebrew guide
- `/usr/share/doc/kn/DEBIAN.md` - Debian package guide
- `/usr/share/doc/kn/install.sh` - Installation script
- `/usr/share/doc/kn/install.ps1` - Windows installer

### Man Page (if available)
- `/usr/share/man/man1/kn.1` - Manual page

---

## Publishing to YUM/DNF Repository

### Option 1: GitHub Releases (Simple)

```bash
# 1. Build the package
rpmbuild -ba ~/rpmbuild/SPECS/kn.spec

# 2. Upload to GitHub Releases
gh release create v0.1.0 \
  --title "Release v0.1.0" \
  --notes "See CHANGELOG.md" \
  ~/rpmbuild/RPMS/x86_64/kn-0.1.0-1.*.x86_64.rpm \
  ~/rpmbuild/SRPMS/kn-0.1.0-1.*.src.rpm

# 3. Users can download and install manually
```

### Option 2: YUM/DNF Repository (Advanced)

Set up a proper YUM repository:

```bash
# 1. Create repository directory
mkdir -p repo/rpm/{el7,el8,el9,fedora,opensuse}

# 2. Copy packages to appropriate directories
cp ~/rpmbuild/RPMS/x86_64/kn-*.el9.x86_64.rpm repo/rpm/el9/
cp ~/rpmbuild/RPMS/x86_64/kn-*.fc*.x86_64.rpm repo/rpm/fedora/

# 3. Create repository metadata
createrepo repo/rpm/el9/
createrepo repo/rpm/fedora/

# 4. Sign packages (optional but recommended)
rpm --addsign repo/rpm/el9/*.rpm

# 5. Create .repo file
cat > knowledge.repo <<EOF
[knowledge]
name=Knowledge Framework Repository
baseurl=https://repo.knowledge.dev/rpm/el\$releasever
enabled=1
gpgcheck=0
EOF

# 6. Upload repository to web server or S3
# - Serve repo/ directory via nginx, Apache, or S3
# - Users add the .repo file to /etc/yum.repos.d/
```

---

## Multi-Distribution Support

### Building for Different Distributions

The spec file works for multiple distributions, but you may need to build separately:

```bash
# For RHEL 9 / Rocky 9 / AlmaLinux 9
rpmbuild -ba --define "dist .el9" SPECS/kn.spec

# For RHEL 8 / Rocky 8 / AlmaLinux 8
rpmbuild -ba --define "dist .el8" SPECS/kn.spec

# For RHEL 7 / CentOS 7
rpmbuild -ba --define "dist .el7" SPECS/kn.spec

# For Fedora 39
rpmbuild -ba --define "dist .fc39" SPECS/kn.spec

# For openSUSE
rpmbuild -ba SPECS/kn.spec
```

### Multi-Architecture Support

```bash
# For ARM64 (aarch64)
rpmbuild -ba --target aarch64 SPECS/kn.spec

# For ARM (armhfp)
rpmbuild -ba --target armhfp SPECS/kn.spec
```

---

## CI/CD Integration

### GitHub Actions Example

Create `.github/workflows/rpm-package.yml`:

```yaml
name: Build RPM Package

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-rpm:
    strategy:
      matrix:
        os: [fedora:latest, rockylinux:9, rockylinux:8]
    runs-on: ubuntu-latest
    container: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install dependencies
        run: |
          if [ -f /etc/fedora-release ]; then
            dnf install -y rpm-build rpmdevtools cargo rust git gcc nodejs npm
          else
            dnf install -y rpm-build rpmdevtools git gcc nodejs npm epel-release
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source $HOME/.cargo/env
          fi
      
      - name: Setup RPM build tree
        run: rpmdev-setuptree
      
      - name: Copy spec and prepare sources
        run: |
          cp kn.spec ~/rpmbuild/SPECS/
          tar czf ~/rpmbuild/SOURCES/v${{ github.ref_name }}.tar.gz .
      
      - name: Build RPM
        run: |
          source $HOME/.cargo/env || true
          rpmbuild -ba ~/rpmbuild/SPECS/kn.spec
      
      - name: Upload to release
        uses: softprops/action-gh-release@v1
        if: startsWith(github.ref, 'refs/tags/')
        with:
          files: |
            ~/rpmbuild/RPMS/x86_64/kn-*.rpm
            ~/rpmbuild/SRPMS/kn-*.src.rpm
```

---

## Updating the Package

When releasing a new version:

```bash
# 1. Update version in kn.spec
sed -i 's/Version:.*/Version:        0.2.0/' kn.spec

# 2. Update changelog in kn.spec
# Add new entry at the top of %changelog section

# 3. Update version in cli/Cargo.toml

# 4. Build new package
rpmdev-setuptree
cp kn.spec ~/rpmbuild/SPECS/
cd ~/rpmbuild/SOURCES
wget https://github.com/kobogithub/knowledge/archive/refs/tags/v0.2.0.tar.gz
cd ~/rpmbuild
rpmbuild -ba SPECS/kn.spec

# 5. Test
sudo dnf install RPMS/x86_64/kn-0.2.0-1.*.x86_64.rpm
kn --version

# 6. Upload to repository or GitHub releases
```

---

## Troubleshooting

### Build Fails

**Missing build dependencies**:
```bash
# Install dependencies listed in BuildRequires
sudo dnf builddep kn.spec
```

**Cargo not found (RHEL/CentOS)**:
```bash
# Rust not in default repos, install with rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Source tarball not found**:
```bash
# Download to ~/rpmbuild/SOURCES
cd ~/rpmbuild/SOURCES
wget https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz
```

### Installation Issues

**Dependency conflicts**:
```bash
# Install with dnf to resolve dependencies automatically
sudo dnf install ./kn-0.1.0-1.*.rpm
```

**kn doctor shows missing deps**:
```bash
# Install recommended packages if available
sudo dnf install bd dolt
```

### Repository Issues

**Metadata error**:
```bash
# Rebuild repository metadata
createrepo --update /path/to/repo
```

---

## COPR (Community Build Service)

For easier distribution on Fedora/RHEL, use COPR:

```bash
# 1. Create account at https://copr.fedorainfracloud.org/

# 2. Install copr-cli
sudo dnf install copr-cli

# 3. Configure authentication
# Get API token from COPR web interface
# Save to ~/.config/copr

# 4. Create COPR project
copr-cli create knowledge \
  --chroot fedora-39-x86_64 \
  --chroot fedora-40-x86_64 \
  --chroot epel-8-x86_64 \
  --chroot epel-9-x86_64 \
  --description "Knowledge Framework CLI" \
  --instructions "Run: kn doctor"

# 5. Build from spec and sources
copr-cli build knowledge \
  --nowait \
  kn.spec \
  https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz

# 6. Users can now install with:
sudo dnf copr enable yourusername/knowledge
sudo dnf install kn
```

---

## Resources

- [RPM Packaging Guide](https://rpm-packaging-guide.github.io/)
- [Fedora Packaging Guidelines](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
- [RHEL RPM Packaging](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/packaging_and_distributing_software/)
- [openSUSE Build Service](https://build.opensuse.org/)
- [COPR Documentation](https://docs.pagure.org/copr.copr/)
