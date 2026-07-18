class Kn < Formula
  desc "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  version "0.8.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.8.0/kn-macos-arm64.tar.gz"
      sha256 "df43908085da7020398f20896a164828def5d74f06805fade3f49bd5e39d7885"
    end
    on_intel do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.8.0/kn-macos-x86_64.tar.gz"
      sha256 "c4a55e5f6c72e307f1115f1e112834575fffa114738a9d55fce4371d25aa8263"
    end
  end

  on_linux do
    url "https://github.com/kobogithub/knowledge/releases/download/v0.8.0/kn-linux-x86_64.tar.gz"
    sha256 "31052652ac1227f1d62a247859ddd2406cdc0e58e65242a88e15d917d5f2905c"
  end

  # Skills, agent templates and docs aren't in the binary release tarballs,
  # so pull them from the tagged source archive instead of compiling anything.
  resource "assets" do
    url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.8.0.tar.gz"
    sha256 "df181e0b72341a881e9bb5b9a22ac3ee65c3027cd50309da7934760acfe1e204"
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
