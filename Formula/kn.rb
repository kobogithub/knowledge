class Kn < Formula
  desc "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  version "0.8.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.8.1/kn-macos-arm64.tar.gz"
      sha256 "e6897fe2ee53e66c00007acc3e10b1e1943d725aa892f7cf27b185f75853cecf"
    end
    on_intel do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.8.1/kn-macos-x86_64.tar.gz"
      sha256 "6f9aa41024e5a618bbaf56e76b5763829b44aab1ade9528ec02916e651a1b050"
    end
  end

  on_linux do
    url "https://github.com/kobogithub/knowledge/releases/download/v0.8.1/kn-linux-x86_64.tar.gz"
    sha256 "3e7d3fcc9f0a2b2ac89f69524818c40ea0a399136aa7a29b9afb1ddb0529b8d6"
  end

  # Skills, agent templates and docs aren't in the binary release tarballs,
  # so pull them from the tagged source archive instead of compiling anything.
  resource "assets" do
    url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.8.1.tar.gz"
    sha256 "b0e588100eeb5e063d1ff14ad2e7d39f20bf23449ad1bf79e05a78e040ecb492"
  end

  def install
    bin.install "kn"

    resource("assets").stage do
      (share/"kn/skills").install Dir["skills/*"]
      (share/"kn/agents").install Dir["agents/*"]
      doc.install "README.md", "README_ES.md"
      doc.install Dir["docs/*"] if Dir.exist?("docs")
    end
  end

  def caveats
    <<~EOS
      The Knowledge Framework CLI has been installed!

      Homebrew sandboxes $HOME during install, so this formula can't write to
      ~/.kn/ automatically. Populate it once with:

        mkdir -p ~/.kn/{skills,agents}
        cp -R #{share}/kn/skills/. ~/.kn/skills/
        cp -R #{share}/kn/agents/. ~/.kn/agents/

      To verify installation and check dependencies:
        kn doctor

      To initialize a new project:
        cd your-project/
        kn init

      Documentation is available at:
        #{doc}

      Optional dependencies (checked by `kn doctor`, not required for the CLI itself):
      - Node.js: for skills that assume a JS/TS toolchain
      - bd (beads): legacy, only used by the `kn beads template` subcommand

      For more information:
        https://github.com/kobogithub/knowledge
    EOS
  end

  test do
    assert_match "kn", shell_output("#{bin}/kn --version")
    system "#{bin}/kn", "doctor"
  end
end
