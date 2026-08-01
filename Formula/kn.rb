class Kn < Formula
  desc "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  version "0.9.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.9.0/kn-macos-arm64.tar.gz"
      sha256 "233e96aeef17a24e1962f006809bc53baadf49d0bc2158a6239716b03d54a4cd"
    end
    on_intel do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.9.0/kn-macos-x86_64.tar.gz"
      sha256 "47d5924a50b5cab0726a9276893ff29f7f92be7ccf0a69076e54add43a2889d2"
    end
  end

  on_linux do
    url "https://github.com/kobogithub/knowledge/releases/download/v0.9.0/kn-linux-x86_64.tar.gz"
    sha256 "37e46b94a60def48df43cb64649ab36c32d9de8033664b20883850a5c165db2f"
  end

  # Skills, agent templates and docs aren't in the binary release tarballs,
  # so pull them from the tagged source archive instead of compiling anything.
  resource "assets" do
    url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.9.0.tar.gz"
    sha256 "587cfb1390511615bac5db56c8da7b2a87133594c152b1df7d1adecbf454cd15"
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
