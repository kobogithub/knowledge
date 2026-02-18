# Homebrew Installation Guide

## For Users

### Install from Homebrew (when available)

```bash
# Add the Knowledge Framework tap
brew tap kobogithub/knowledge

# Install kn
brew install kn

# Verify installation
kn doctor
```

### Optional Dependencies

```bash
# Install bd (beads) for issue tracking
brew install bd

# Install dolt for Beads database (optional)
brew install dolt
```

---

## For Maintainers

### Publishing to Homebrew

#### 1. Create a GitHub Release

```bash
# Tag the release
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# GitHub will automatically create a tarball at:
# https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz
```

#### 2. Calculate SHA256

```bash
# Download the tarball
curl -L https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz -o kn-0.1.0.tar.gz

# Calculate SHA256
sha256sum kn-0.1.0.tar.gz
# or on macOS:
shasum -a 256 kn-0.1.0.tar.gz
```

#### 3. Update Formula

Edit `Formula/kn.rb`:
- Update `url` with the correct version
- Update `sha256` with the calculated hash
- Update version in `Cargo.toml` if needed

#### 4. Test Locally

```bash
# Install from local formula
brew install --build-from-source Formula/kn.rb

# Test the installation
kn doctor
kn --version

# Run formula tests
brew test kn

# Audit the formula
brew audit --strict kn
```

#### 5. Create Homebrew Tap (First Time Only)

```bash
# Create a new repository: homebrew-knowledge
# https://github.com/kobogithub/homebrew-knowledge

# Add the formula
cp Formula/kn.rb /path/to/homebrew-knowledge/Formula/kn.rb
cd /path/to/homebrew-knowledge
git add Formula/kn.rb
git commit -m "Add kn formula v0.1.0"
git push
```

#### 6. Users Can Now Install

```bash
brew tap kobogithub/knowledge
brew install kn
```

---

## Homebrew Formula Structure

The formula in `Formula/kn.rb` includes:

- **Dependencies**: Rust (build-time), Node.js, Git
- **Installation**: Builds kn from source using Cargo
- **Resources**: Installs skills and agent templates to share directory
- **Tests**: Verifies kn runs and responds to commands
- **Caveats**: Shows post-install instructions to users

---

## Testing the Formula

### Local Testing

```bash
# Install from local formula
brew install --build-from-source ./Formula/kn.rb

# Test that it works
kn doctor
kn init --help
kn skills list

# Uninstall
brew uninstall kn
```

### CI Testing (GitHub Actions)

Create `.github/workflows/homebrew-test.yml`:

```yaml
name: Test Homebrew Formula

on:
  push:
    branches: [prod, main]
  pull_request:

jobs:
  test-formula:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install from formula
        run: brew install --build-from-source ./Formula/kn.rb
      
      - name: Test kn
        run: |
          kn --version
          kn doctor
          kn init --help
      
      - name: Audit formula
        run: brew audit --strict ./Formula/kn.rb
```

---

## Publishing to Official Homebrew

To publish to the official Homebrew repository (homebrew-core):

1. **Meet Requirements**:
   - Stable 1.0+ release
   - Significant user base
   - Good documentation
   - Automated tests

2. **Submit PR**:
   ```bash
   # Fork homebrew-core
   # Add Formula/kn.rb
   # Create PR to Homebrew/homebrew-core
   ```

3. **Homebrew Guidelines**:
   - https://docs.brew.sh/Formula-Cookbook
   - https://docs.brew.sh/Acceptable-Formulae

For now, we use a **tap** (kobogithub/knowledge) which is easier to manage.

---

## Updating the Formula

When releasing a new version:

```bash
# 1. Update version in Cargo.toml
# 2. Create new release tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# 3. Calculate new SHA256
curl -L https://github.com/kobogithub/knowledge/archive/refs/tags/v0.2.0.tar.gz | shasum -a 256

# 4. Update Formula/kn.rb:
#    - url with new version
#    - sha256 with new hash

# 5. Test and commit
brew install --build-from-source ./Formula/kn.rb
brew test kn
git add Formula/kn.rb
git commit -m "Update kn formula to v0.2.0"
git push
```

---

## Troubleshooting

### Common Issues

**Build fails with "cargo not found"**:
- Ensure `depends_on "rust" => :build` is in formula
- User needs to install Rust: `brew install rust`

**Skills not found after installation**:
- Check that skills are installed to `#{share}/kn/skills`
- Users can verify with: `ls $(brew --prefix)/share/kn/skills`

**kn doctor shows missing dependencies**:
- This is expected! Not all deps are installed via formula
- Users should run `kn doctor` and follow instructions
- Or run `./install.sh` for automated setup

---

## Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Taps Documentation](https://docs.brew.sh/Taps)
- [Homebrew Formula Reference](https://rubydoc.brew.sh/Formula)
