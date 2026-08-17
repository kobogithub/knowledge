class Kn < Formula
  desc "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  # The version is scanned from this URL — do not add an explicit `version` line,
  # `brew audit --strict` rejects it as redundant. The release automation and the
  # drift check both read the tag from here.
  url "https://github.com/kobogithub/knowledge/releases/download/v0.10.0/kn-macos-arm64.tar.gz"
  sha256 "3272dc672453eac8845cb7196f2ef741dbfb9c600b47795bc9ca76f41ffeef91"
  license "MIT"

  # kn targets Apple Silicon macOS only. These guards make Homebrew refuse an
  # unsupported machine up front, instead of downloading an artifact that is not
  # built and failing later at checksum verification.
  depends_on arch: :arm64
  depends_on :macos

  # Skills, agent templates and docs aren't in the binary release tarballs,
  # so pull them from the tagged source archive instead of compiling anything.
  resource "assets" do
    url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.10.0.tar.gz"
    sha256 "4715b356bf3556c0c835ae6a6d42fb8295256c1471a64e2f29a4cd8b5bf2db5e"
  end

  def install
    bin.install "kn"

    resource("assets").stage do
      (pkgshare/"skills").install Dir["skills/*"]
      (pkgshare/"agents").install Dir["agents/*"]
      doc.install "README.md", "README_ES.md"
      doc.install Dir["docs/*"] if Dir.exist?("docs")
    end
  end

  # Locations the standalone install.sh writes to. A copy left there shadows or is
  # shadowed by this one depending on PATH order, which otherwise shows up only as
  # a confusingly stale `kn --version`.
  OTHER_INSTALL_PATHS = [
    "#{Dir.home}/.local/bin/kn",
    "/usr/local/bin/kn",
  ].freeze

  def conflicting_installs
    OTHER_INSTALL_PATHS.select { |p| File.executable?(p) && !p.start_with?(HOMEBREW_PREFIX.to_s) }
  end

  def caveats
    message = <<~EOS
      The Knowledge Framework CLI has been installed!

      Homebrew sandboxes $HOME during install, so this formula can't write to
      ~/.kn/ automatically. Populate it once with:

        mkdir -p ~/.kn/{skills,agents}
        cp -R #{pkgshare}/skills/. ~/.kn/skills/
        cp -R #{pkgshare}/agents/. ~/.kn/agents/

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

    others = conflicting_installs
    unless others.empty?
      message += <<~EOS

        WARNING: another kn was found outside Homebrew:
          #{others.join("\n  ")}

        This copy is at #{bin}/kn. Whichever directory comes first in your PATH
        wins, so `kn --version` may still report the other one. Check with:

          which -a kn

        To remove the other copy:
          rm #{others.join(" ")}
      EOS
    end

    message
  end

  test do
    assert_match "kn", shell_output("#{bin}/kn --version")

    # The binary tarball ships only the binary; skills and agents come from the
    # `assets` resource. Asserting they landed catches the failure mode where kn
    # installs successfully but has no knowledge to work with.
    assert_path_exists pkgshare/"skills"
    assert_path_exists pkgshare/"agents"

    # Deliberately does NOT run `kn doctor`. It exits non-zero when optional
    # third-party tools are absent, and its output varies with the environment —
    # inside Homebrew's sandboxed test env even Rust and Cargo are missing. It also
    # still treats `bd` (beads) as a required dependency although ADR-006 removed
    # beads from the workflow. A formula test must assert on what this formula
    # installs, not on what the machine running it happens to have.
  end
end
