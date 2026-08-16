# Contract C1 — The published formula

**Feature**: `004-macos-arm64-homebrew`

Defines what `Formula/kn.rb` must contain, in both the authored copy (this repo) and the
published copy (the tap). Violations are install-time failures for the user.

## Required shape

```ruby
class Kn < Formula
  desc     "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  version  "<X.Y.Z>"                                   # rendered per release
  license  "MIT"                                       # C1.4

  url      ".../releases/download/v<X.Y.Z>/kn-macos-arm64.tar.gz"
  sha256   "<binary_sha256>"                           # C1.3

  depends_on :macos                                    # C1.2
  depends_on arch: :arm64                              # C1.2

  resource "assets" do                                 # C1.5
    url    ".../archive/refs/tags/v<X.Y.Z>.tar.gz"
    sha256 "<source_sha256>"                           # C1.3
  end

  def install
    bin.install "kn"
    resource("assets").stage do
      (share/"kn/skills").install Dir["skills/*"]
      (share/"kn/agents").install Dir["agents/*"]
      doc.install "README.md", "README_ES.md"
    end
  end

  test do
    assert_match "kn", shell_output("#{bin}/kn --version")
  end
end
```

## Clauses

| ID | Clause | Requirement | Fails if |
|---|---|---|---|
| **C1.1** | No `on_linux`, `on_intel`, or `on_macos`/`on_arm` nesting anywhere | FR-006 | A branch references an artifact that is no longer built |
| **C1.2** | Both `depends_on :macos` and `depends_on arch: :arm64` present | FR-006, edge case 1 | An Intel Mac downloads before failing, instead of being refused upfront |
| **C1.3** | Every `url` has a matching `sha256`. **There are two** — the binary and the assets resource | FR-008 | An unverified artifact is installed |
| **C1.4** | `license` declared | FR-009 | Blocked until `002` merges (research R9) |
| **C1.5** | `resource "assets"` present and installed | FR-007 | `kn` installs with no skills or agents and is not functional |
| **C1.6** | `version`, both `url`s and both `sha256`s are the *only* per-release variables | FR-012 | Hand-authored values reappear and drift returns |
| **C1.7** | The published copy is byte-identical to the rendered authored copy | I2 | The two sources disagree, which is the current bug |

## Verification

- `brew style kobogithub/knowledge/kn` — clauses C1.1, C1.2 structurally
- `brew audit --strict kn` — C1.3, C1.4
- Quickstart V2, V3 — C1.5, C1.7
- Quickstart V4 — C1.2 refusal behaviour, where an Intel Mac is available
