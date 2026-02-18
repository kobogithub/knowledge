class Kn < Formula
  desc "CLI tool for AI-assisted development workflows"
  homepage "https://github.com/kobogithub/knowledge"
  url "https://github.com/kobogithub/knowledge/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "" # Will be filled when creating a release
  license "MIT"
  head "https://github.com/kobogithub/knowledge.git", branch: "prod"

  depends_on "rust" => :build
  depends_on "node"
  depends_on "git"

  def install
    # Build the kn CLI
    cd "cli" do
      system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    end

    # Install skills
    (share/"kn/skills").install Dir["skills/*"]
    
    # Install agent templates
    (share/"kn/agents").install Dir["agents/*"]
    
    # Install installation scripts as documentation
    doc.install "install.sh", "install.ps1"
    
    # Install README and docs
    doc.install "README.md", "README_ES.md"
    doc.install Dir["docs/*"] if Dir.exist?("docs")
  end

  def caveats
    <<~EOS
      The Knowledge Framework CLI has been installed!

      To verify installation and check dependencies:
        kn doctor

      To initialize a new project:
        cd your-project/
        kn init

      Skills have been installed to:
        #{share}/kn/skills

      Agent templates are available at:
        #{share}/kn/agents

      Documentation is available at:
        #{doc}

      Optional dependencies:
      - bd (beads): brew install bd  # For issue tracking
      - dolt: brew install dolt       # For Beads database (optional)

      For more information:
        https://github.com/kobogithub/knowledge
    EOS
  end

  test do
    # Test that kn is installed and runs
    assert_match "kn", shell_output("#{bin}/kn --version")
    
    # Test that kn doctor runs (may show missing deps, which is OK)
    system "#{bin}/kn", "doctor"
  end
end
