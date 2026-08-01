class Kn < Formula
  desc "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  version "0.10.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.10.0/kn-macos-arm64.tar.gz"
      sha256 "3272dc672453eac8845cb7196f2ef741dbfb9c600b47795bc9ca76f41ffeef91"
    end
    on_intel do
      url "https://github.com/kobogithub/knowledge/releases/download/v0.10.0/kn-macos-x86_64.tar.gz"
      sha256 "23e1ff65fb6a4e4e7952d57a0253568d67181e40d988f6f52ad0be188d473f95"
    end
  end

  on_linux do
    url "https://github.com/kobogithub/knowledge/releases/download/v0.10.0/kn-linux-x86_64.tar.gz"
    sha256 "5ec19e5821b2cd40b5bf2b26b2363d70557fe27db7542bbc569ea59ccc700e98"
  end

  # Skills, agent templates and docs aren't in the binary release tarballs,
  # so pull them from the tagged source archive instead of compiling anything.
  resource "assets" do
    url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.10.0.tar.gz"
    sha256 "4715b356bf3556c0c835ae6a6d42fb8295256c1471a64e2f29a4cd8b5bf2db5e"
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
