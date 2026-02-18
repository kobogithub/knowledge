Name:           kn
Version:        0.1.0
Release:        1%{?dist}
Summary:        CLI tool for AI-assisted development workflows

License:        MIT
URL:            https://github.com/kobogithub/knowledge
Source0:        https://github.com/kobogithub/knowledge/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust >= 1.70
BuildRequires:  git
BuildRequires:  gcc
Requires:       git
Requires:       nodejs >= 18.0
Recommends:     bd
Recommends:     dolt

%description
Knowledge Framework (kn) is a command-line tool that solves the "cold start
problem" in AI-assisted development by automating project setup with agents,
skills, and standardized workflows.

Features:
- Instant project initialization with kn init
- Skills management (install from registry or local files)
- Beads templates for structured issue tracking
- MCP server configuration for AI documentation access
- Multi-agent workflow coordination
- Dependency verification with kn doctor

This package includes:
- kn CLI binary
- Pre-installed skills for common frameworks
- Agent templates for multi-agent workflows
- Complete documentation

%prep
%autosetup -n knowledge-%{version}

%build
cd cli
cargo build --release --locked

%install
# Install kn binary
install -D -m 755 cli/target/release/kn %{buildroot}%{_bindir}/kn

# Install skills
mkdir -p %{buildroot}%{_datadir}/kn/skills
cp -r skills/* %{buildroot}%{_datadir}/kn/skills/

# Install agent templates
mkdir -p %{buildroot}%{_datadir}/kn/agents
cp -r agents/* %{buildroot}%{_datadir}/kn/agents/

# Install documentation
mkdir -p %{buildroot}%{_docdir}/kn
cp README.md README_ES.md %{buildroot}%{_docdir}/kn/
cp install.sh install.ps1 %{buildroot}%{_docdir}/kn/
cp HOMEBREW.md DEBIAN.md %{buildroot}%{_docdir}/kn/

# Install docs directory if it exists
if [ -d docs ]; then
    cp -r docs %{buildroot}%{_docdir}/kn/
fi

# Install man page if it exists
if [ -f docs/kn.1 ]; then
    install -D -m 644 docs/kn.1 %{buildroot}%{_mandir}/man1/kn.1
fi

%check
cd cli
cargo test --release

%files
%license LICENSE
%doc README.md README_ES.md
%{_bindir}/kn
%{_datadir}/kn/skills
%{_datadir}/kn/agents
%{_docdir}/kn

%if 0%{?_with_man:1}
%{_mandir}/man1/kn.1*
%endif

%post
echo "Knowledge Framework CLI has been installed!"
echo ""
echo "To verify installation and check dependencies:"
echo "  kn doctor"
echo ""
echo "To initialize a new project:"
echo "  cd your-project/"
echo "  kn init"
echo ""
echo "Skills have been installed to:"
echo "  %{_datadir}/kn/skills"
echo ""
echo "For more information:"
echo "  https://github.com/kobogithub/knowledge"

%changelog
* Tue Feb 18 2026 Knowledge Framework Contributors <hello@knowledge.dev> - 0.1.0-1
- Initial release
- Core CLI functionality:
  * kn init: Project initialization with auto-detection
  * kn skills install/list: Skills management
  * kn beads template: Issue template generation
  * kn mcp add/list/remove: MCP server configuration
  * kn doctor: Dependency verification
- Installation automation:
  * install.sh for Linux/macOS
  * install.ps1 for Windows
- Multi-agent workflow support:
  * Planner, Frontend, Backend, Rust, DevOps, QA agents
  * Beads integration for issue tracking
- Skills included:
  * astro-best-practices
  * bash-best-practices
  * docker-best-practices
  * python-best-practices
  * rust-best-practices
  * supabase-postgres-best-practices
  * bd-best-practices
