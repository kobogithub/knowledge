#!/usr/bin/env bash
#
# install.sh - Automated installation script for kn CLI
#
# This script installs kn and all its dependencies on Linux and macOS.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash
#   or
#   ./install.sh
#
# Options:
#   --version VER  Install specific version (default: latest)
#   --skip-deps    Skip dependency installation (assumes all deps are present)
#   --no-confirm   Skip confirmation prompts
#   --help         Show this help message
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
ORANGE='\033[38;5;214m'
NC='\033[0m' # No Color

# Configuration
NODE_MIN_VERSION="18.0"
INSTALL_DIR="${HOME}/.local/bin"
GITHUB_REPO="kobogithub/knowledge"
GITHUB_API="https://api.github.com/repos/${GITHUB_REPO}"
TMP_DIR="${TMPDIR:-/tmp}/kn-install-$$"

# Flags
SKIP_DEPS=false
NO_CONFIRM=false
NO_MODIFY_PATH=false
VERSION="latest"

# Logging functions
info() {
    echo -e "${CYAN}ℹ${NC} $*"
}

success() {
    echo -e "${GREEN}✓${NC} $*"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $*"
}

error() {
    echo -e "${RED}✗${NC} $*" >&2
}

header() {
    echo -e "\n${BLUE}===${NC} $* ${BLUE}===${NC}\n"
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                VERSION="$2"
                shift 2
                ;;
            --skip-deps)
                SKIP_DEPS=true
                shift
                ;;
            --no-confirm)
                NO_CONFIRM=true
                shift
                ;;
            --no-modify-path)
                NO_MODIFY_PATH=true
                shift
                ;;
            --help)
                cat << EOF
kn CLI Installation Script

Usage: $0 [OPTIONS]

Options:
    --version VER       Install specific version (e.g., v0.2.0)
    --skip-deps         Skip automatic dependency installation
    --no-confirm        Skip all confirmation prompts
    --no-modify-path    Don't modify shell config files (.bashrc, .zshrc, etc.)
    --help              Show this help message

Dependencies installed (if not present):
    - Git
    - Node.js (via package manager or manual instructions)
    - Rust/Cargo (for bd installation)
    - bd (beads) - via cargo install

Note: This script downloads pre-compiled binaries from GitHub releases.
No Rust toolchain is required for kn itself.

EOF
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
}

# Detect OS and package manager
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        ARCH=$(uname -m)
        if [[ "$ARCH" != "x86_64" ]]; then
            error "Unsupported architecture: $ARCH"
            error "Only x86_64 is supported on Linux"
            exit 1
        fi
        
        if command -v apt-get &> /dev/null; then
            PKG_MANAGER="apt"
        elif command -v dnf &> /dev/null; then
            PKG_MANAGER="dnf"
        elif command -v yum &> /dev/null; then
            PKG_MANAGER="yum"
        elif command -v pacman &> /dev/null; then
            PKG_MANAGER="pacman"
        else
            PKG_MANAGER="unknown"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        ARCH=$(uname -m)
        
        # Detect Rosetta: x86_64 process running on ARM Mac
        if [[ "$ARCH" == "x86_64" ]]; then
            local rosetta_flag
            rosetta_flag=$(sysctl -n sysctl.proc_translated 2>/dev/null || echo 0)
            if [[ "$rosetta_flag" == "1" ]]; then
                info "Detected Rosetta translation (x86_64 on ARM)"
                ARCH="arm64"
                info "Using ARM64 binary for better performance"
            fi
        fi
        
        if [[ "$ARCH" != "x86_64" && "$ARCH" != "arm64" ]]; then
            error "Unsupported architecture: $ARCH"
            exit 1
        fi
        
        if command -v brew &> /dev/null; then
            PKG_MANAGER="brew"
        else
            PKG_MANAGER="unknown"
        fi
    else
        error "Unsupported OS: $OSTYPE"
        error "This script only supports Linux and macOS"
        exit 1
    fi
    
    info "Detected OS: $OS ($ARCH, package manager: $PKG_MANAGER)"
}

# Check if command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Extract version from command output
extract_version() {
    local output="$1"
    echo "$output" | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1
}

# Compare versions (returns 0 if $1 >= $2)
version_ge() {
    printf '%s\n%s\n' "$2" "$1" | sort -V -C
}

# Check dependency
check_dependency() {
    local name="$1"
    local command="$2"
    local min_version="${3:-}"
    
    if command_exists "$command"; then
        local version_output
        version_output=$($command --version 2>&1 || echo "")
        local version
        version=$(extract_version "$version_output")
        
        if [[ -n "$min_version" ]] && [[ -n "$version" ]]; then
            if version_ge "$version" "$min_version"; then
                success "$name $version (>= $min_version required)"
                return 0
            else
                warn "$name $version found, but >= $min_version required"
                return 1
            fi
        else
            success "$name installed"
            return 0
        fi
    else
        error "$name not found"
        return 1
    fi
}

# Get latest release version from GitHub
get_latest_version() {
    local version
    version=$(curl -fsSL "${GITHUB_API}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [[ -z "$version" ]]; then
        error "Failed to fetch latest version from GitHub"
        return 1
    fi
    
    echo "$version"
}

# Check if kn is already installed with the target version
check_installed_version() {
    local target_version="$1"
    
    if ! command_exists kn; then
        return 1  # Not installed
    fi
    
    local installed_version
    installed_version=$(kn --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    
    if [[ -z "$installed_version" ]]; then
        return 1  # Can't determine version
    fi
    
    # Strip 'v' prefix from target version for comparison
    local target_clean="${target_version#v}"
    
    if [[ "$installed_version" == "$target_clean" ]]; then
        info "kn version $installed_version is already installed"
        info "Run with --version <ver> to install a different version"
        return 0  # Same version installed
    else
        info "Current version: $installed_version"
        info "Will upgrade to: $target_clean"
        return 1  # Different version
    fi
}

# Unbuffered sed for real-time processing
unbuffered_sed() {
    if echo | sed -u -e "" >/dev/null 2>&1; then
        sed -nu "$@"
    elif echo | sed -l -e "" >/dev/null 2>&1; then
        sed -nl "$@"
    else
        # Fallback: add padding to force line buffering
        local pad
        pad="$(printf "\n%512s" "")"
        sed -ne "s/$/\\${pad}/" "$@"
    fi
}

# Print visual progress bar
print_progress() {
    local bytes="$1"
    local length="$2"
    [ "$length" -gt 0 ] || return 0

    local width=50
    local percent=$(( bytes * 100 / length ))
    [ "$percent" -gt 100 ] && percent=100
    local on=$(( percent * width / 100 ))
    local off=$(( width - on ))

    local filled
    filled=$(printf "%*s" "$on" "")
    filled=${filled// /■}
    local empty
    empty=$(printf "%*s" "$off" "")
    empty=${empty// /･}

    printf "\r${ORANGE}%s%s %3d%%${NC}" "$filled" "$empty" "$percent" >&4
}

# Download with visual progress bar
download_with_progress() {
    local url="$1"
    local output="$2"

    # Check if we're in a TTY environment
    if [ -t 2 ]; then
        exec 4>&2
    else
        # Not a TTY - use simple curl with basic progress
        curl -# -L -o "$output" "$url"
        return $?
    fi

    local tmp_dir=${TMPDIR:-/tmp}
    local basename="${tmp_dir}/kn_install_$$"
    local tracefile="${basename}.trace"

    rm -f "$tracefile"
    mkfifo "$tracefile"

    # Hide cursor
    printf "\033[?25l" >&4

    # Ensure cleanup on exit
    # shellcheck disable=SC2064
    trap "trap - RETURN; rm -f \"$tracefile\"; printf '\033[?25h' >&4; exec 4>&-" RETURN

    # Start download in background
    (
        curl --trace-ascii "$tracefile" -s -L -o "$output" "$url"
    ) &
    local curl_pid=$!

    # Parse trace and show progress
    unbuffered_sed \
        -e 'y/ACDEGHLNORTV/acdeghlnortv/' \
        -e '/^0000: content-length:/p' \
        -e '/^<= recv data/p' \
        "$tracefile" | \
    {
        local length=0
        local bytes=0

        while IFS=" " read -r -a line; do
            [ "${#line[@]}" -lt 2 ] && continue
            local tag="${line[0]} ${line[1]}"

            if [ "$tag" = "0000: content-length:" ]; then
                length="${line[2]}"
                length=$(echo "$length" | tr -d '\r')
                bytes=0
            elif [ "$tag" = "<= recv" ]; then
                local size="${line[3]}"
                bytes=$(( bytes + size ))
                if [ "$length" -gt 0 ]; then
                    print_progress "$bytes" "$length"
                fi
            fi
        done
    }

    wait $curl_pid
    local ret=$?
    echo "" >&4
    return $ret
}

# Download kn binary from GitHub releases
download_kn() {
    header "Downloading kn binary"
    
    local version="$VERSION"
    
    if [[ "$version" == "latest" ]]; then
        info "Fetching latest version..."
        version=$(get_latest_version)
        if [[ -z "$version" ]]; then
            error "Failed to determine latest version"
            return 1
        fi
    fi
    
    info "Version to install: $version"
    
    # Determine asset name based on OS and architecture
    local asset_name
    if [[ "$OS" == "linux" ]]; then
        asset_name="kn-linux-x86_64.tar.gz"
    elif [[ "$OS" == "macos" ]]; then
        if [[ "$ARCH" == "arm64" ]]; then
            asset_name="kn-macos-arm64.tar.gz"
        else
            asset_name="kn-macos-x86_64.tar.gz"
        fi
    else
        error "Unsupported OS for binary download: $OS"
        return 1
    fi
    
    local download_url="https://github.com/${GITHUB_REPO}/releases/download/${version}/${asset_name}"
    
    info "Downloading from: $download_url"
    
    mkdir -p "$TMP_DIR"
    local tarball="$TMP_DIR/$asset_name"
    
    if ! download_with_progress "$download_url" "$tarball"; then
        error "Failed to download kn binary"
        error "URL: $download_url"
        return 1
    fi
    
    info "Extracting binary..."
    if ! tar -xzf "$tarball" -C "$TMP_DIR"; then
        error "Failed to extract tarball"
        return 1
    fi
    
    if [[ ! -f "$TMP_DIR/kn" ]]; then
        error "Binary not found in tarball"
        return 1
    fi
    
    success "Binary downloaded and extracted successfully"
    return 0
}

# Install Rust via rustup (only needed for bd)
install_rust() {
    header "Installing Rust (required for bd)"
    
    if command_exists rustc && command_exists cargo; then
        success "Rust is already installed"
        return 0
    fi
    
    info "Installing Rust via rustup..."
    if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; then
        # Source cargo env
        # shellcheck disable=SC1090,SC1091
        source "$HOME/.cargo/env" 2>/dev/null || true
        export PATH="$HOME/.cargo/bin:$PATH"
        success "Rust installed successfully"
        return 0
    else
        error "Failed to install Rust"
        return 1
    fi
}

# Install Git
install_git() {
    header "Installing Git"
    
    if check_dependency "Git" "git" 2>/dev/null; then
        info "Git is already installed"
        return 0
    fi
    
    case "$PKG_MANAGER" in
        apt)
            info "Installing Git via apt..."
            sudo apt-get update && sudo apt-get install -y git
            ;;
        dnf|yum)
            info "Installing Git via $PKG_MANAGER..."
            sudo "$PKG_MANAGER" install -y git
            ;;
        pacman)
            info "Installing Git via pacman..."
            sudo pacman -S --noconfirm git
            ;;
        brew)
            info "Installing Git via Homebrew..."
            brew install git
            ;;
        *)
            error "Cannot auto-install Git with package manager: $PKG_MANAGER"
            error "Please install Git manually: https://git-scm.com/downloads"
            return 1
            ;;
    esac
    
    if check_dependency "Git" "git"; then
        success "Git installed successfully"
        return 0
    else
        error "Git installation failed"
        return 1
    fi
}

# Install Node.js
install_nodejs() {
    header "Installing Node.js"
    
    if check_dependency "Node.js" "node" "$NODE_MIN_VERSION" 2>/dev/null; then
        info "Node.js is already installed"
        return 0
    fi
    
    warn "Node.js is not installed or version is too old"
    info "We recommend installing Node.js via one of these methods:"
    echo ""
    echo "  1. nvm (Node Version Manager) - Recommended"
    echo "     curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash"
    echo "     nvm install --lts"
    echo ""
    echo "  2. Package manager:"
    
    case "$PKG_MANAGER" in
        apt)
            echo "     curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -"
            echo "     sudo apt-get install -y nodejs"
            ;;
        dnf|yum)
            echo "     sudo $PKG_MANAGER install -y nodejs npm"
            ;;
        brew)
            echo "     brew install node"
            ;;
    esac
    
    echo ""
    echo "  3. Official installer: https://nodejs.org/"
    echo ""
    
    if [[ "$NO_CONFIRM" == false ]]; then
        read -p "Would you like to attempt automatic installation via package manager? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            warn "Skipping Node.js installation - you'll need to install it manually"
            return 1
        fi
    else
        warn "Skipping Node.js installation in non-interactive mode"
        return 1
    fi
    
    case "$PKG_MANAGER" in
        brew)
            brew install node
            ;;
        apt)
            curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && \
            sudo apt-get install -y nodejs
            ;;
        dnf|yum)
            sudo "$PKG_MANAGER" install -y nodejs npm
            ;;
        *)
            warn "Auto-install not available for package manager: $PKG_MANAGER"
            return 1
            ;;
    esac
    
    if check_dependency "Node.js" "node" "$NODE_MIN_VERSION"; then
        success "Node.js installed successfully"
        return 0
    else
        error "Node.js installation failed or version still too old"
        return 1
    fi
}

# Install bd (beads)
install_bd() {
    header "Installing bd (beads)"
    
    if check_dependency "bd" "bd" 2>/dev/null; then
        info "bd is already installed"
        return 0
    fi
    
    # Ensure Rust is installed first
    if ! command_exists cargo; then
        if ! install_rust; then
            error "Cannot install bd without Rust/Cargo"
            return 1
        fi
    fi
    
    info "Installing bd via cargo..."
    if cargo install bd; then
        success "bd installed successfully"
        return 0
    else
        error "Failed to install bd"
        error "You may need to install it manually from: https://github.com/beadlabs/beads"
        return 1
    fi
}

# Install kn binary
install_kn() {
    header "Installing kn binary"
    
    local binary="$TMP_DIR/kn"
    
    if [[ ! -f "$binary" ]]; then
        error "Binary not found at $binary"
        return 1
    fi
    
    # Make binary executable
    chmod +x "$binary"
    
    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    # Try to install to /usr/local/bin first (requires sudo)
    if [[ "$NO_CONFIRM" == false ]]; then
        info "Install location options:"
        echo "  1. $INSTALL_DIR (user-local, no sudo required)"
        echo "  2. /usr/local/bin (system-wide, requires sudo)"
        echo ""
        read -p "Choose installation location [1/2] (default: 1): " -n 1 -r choice
        echo
        
        case "$choice" in
            2)
                info "Installing to /usr/local/bin (requires sudo)..."
                if sudo cp "$binary" /usr/local/bin/kn && sudo chmod +x /usr/local/bin/kn; then
                    success "kn installed to /usr/local/bin/kn"
                else
                    error "Failed to install to /usr/local/bin"
                    return 1
                fi
                ;;
            *)
                info "Installing to $INSTALL_DIR..."
                cp "$binary" "$INSTALL_DIR/kn"
                chmod +x "$INSTALL_DIR/kn"
                success "kn installed to $INSTALL_DIR/kn"
                ;;
        esac
    else
        # Non-interactive: install to user directory
        info "Installing to $INSTALL_DIR..."
        cp "$binary" "$INSTALL_DIR/kn"
        chmod +x "$INSTALL_DIR/kn"
        success "kn installed to $INSTALL_DIR/kn"
    fi
    
    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]] && [[ ! -f "/usr/local/bin/kn" ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        
        if [[ "$NO_MODIFY_PATH" == false ]]; then
            info "Will attempt to add to shell configuration..."
        else
            info "Add the following line to your ~/.bashrc or ~/.zshrc:"
            echo ""
            echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
            echo ""
        fi
    fi
    
    return 0
}

# Populate ~/.kn/ with agents, skills, and MCP directories
setup_kn_resources() {
    header "Setting up kn resources"
    
    local kn_home="$HOME/.kn"
    local script_dir
    script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    
    info "Creating ~/.kn directory structure..."
    mkdir -p "$kn_home"/{agents,skills,mcps}
    
    # Copy agents
    if [[ -d "$script_dir/agents" ]]; then
        info "Installing agents to ~/.kn/agents/..."
        
        # Copy each agent directory
        for agent_dir in "$script_dir/agents"/*; do
            if [[ -d "$agent_dir" ]]; then
                local agent_name
                agent_name=$(basename "$agent_dir")
                local dest="$kn_home/agents/$agent_name"
                
                # Create or update agent directory
                mkdir -p "$dest"
                cp -r "$agent_dir"/* "$dest/" 2>/dev/null || true
                
                if [[ -f "$dest/AGENTS.md" ]]; then
                    success "Installed agent: $agent_name"
                fi
            fi
        done
    else
        warn "agents/ directory not found - skipping agent installation"
        warn "Agents will need to be installed manually"
    fi
    
    # Copy skills
    if [[ -d "$script_dir/skills" ]]; then
        info "Installing skills to ~/.kn/skills/..."
        
        # Copy each skill directory
        for skill_dir in "$script_dir/skills"/*; do
            if [[ -d "$skill_dir" ]]; then
                local skill_name
                skill_name=$(basename "$skill_dir")
                local dest="$kn_home/skills/$skill_name"
                
                # Create or update skill directory
                mkdir -p "$dest"
                cp -r "$skill_dir"/* "$dest/" 2>/dev/null || true
                
                if [[ -f "$dest/SKILL.md" ]]; then
                    success "Installed skill: $skill_name"
                fi
            fi
        done
    else
        warn "skills/ directory not found - skipping skill installation"
        warn "Skills will need to be installed manually"
    fi
    
    # Create MCPs directory (actual MCPs installed on-demand)
    mkdir -p "$kn_home/mcps"
    info "Created ~/.kn/mcps/ (MCP servers will be installed on-demand)"
    
    success "Resource setup complete"
    echo ""
    info "Resources installed to ~/.kn/:"
    echo "  - Agents: ~/.kn/agents/"
    echo "  - Skills: ~/.kn/skills/"
    echo "  - MCPs:   ~/.kn/mcps/ (installed on-demand)"
    echo ""
    
    return 0
}

# Add kn to PATH in shell configuration
add_to_shell_config() {
    if [[ "$NO_MODIFY_PATH" == true ]]; then
        return 0
    fi
    
    # Skip if already in system PATH
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]] || [[ -f "/usr/local/bin/kn" ]]; then
        return 0
    fi
    
    local current_shell
    current_shell=$(basename "$SHELL")
    
    local config_files=""
    local export_cmd="export PATH=\"\$PATH:$INSTALL_DIR\""
    
    case $current_shell in
        fish)
            config_files="$HOME/.config/fish/config.fish"
            export_cmd="fish_add_path $INSTALL_DIR"
            ;;
        zsh)
            config_files="${ZDOTDIR:-$HOME}/.zshrc ${ZDOTDIR:-$HOME}/.zshenv"
            ;;
        bash)
            config_files="$HOME/.bashrc $HOME/.bash_profile $HOME/.profile"
            ;;
        ash|sh)
            config_files="$HOME/.profile /etc/profile"
            ;;
        *)
            config_files="$HOME/.bashrc $HOME/.profile"
            ;;
    esac
    
    local config_file=""
    for file in $config_files; do
        if [[ -f "$file" ]] && [[ -w "$file" ]]; then
            config_file="$file"
            break
        fi
    done
    
    if [[ -z "$config_file" ]]; then
        warn "No writable config file found for $current_shell"
        info "Manually add to your shell config:"
        echo "  $export_cmd"
        return 1
    fi
    
    # Check if already added
    if grep -Fxq "$export_cmd" "$config_file" 2>/dev/null; then
        info "PATH already configured in $config_file"
        return 0
    fi
    
    # Add to config
    {
        echo ""
        echo "# kn - Knowledge Framework CLI"
        echo "$export_cmd"
    } >> "$config_file"
    
    success "Added kn to PATH in $config_file"
    info "Restart your shell or run: source $config_file"
    return 0
}

# Verify installation
verify_installation() {
    header "Verifying installation"
    
    # Ensure kn is in PATH for verification
    export PATH="$INSTALL_DIR:/usr/local/bin:$PATH"
    
    if command_exists kn; then
        success "kn is available in PATH"
        
        info "Running kn doctor to verify dependencies..."
        echo ""
        if kn doctor; then
            echo ""
            success "Installation verified successfully!"
            return 0
        else
            echo ""
            warn "Some dependencies may be missing. Review output above."
            return 1
        fi
    else
        error "kn is not available in PATH"
        error "You may need to restart your shell or source your profile"
        return 1
    fi
}

# Cleanup
cleanup() {
    if [[ -d "$TMP_DIR" ]]; then
        info "Cleaning up temporary files..."
        rm -rf "$TMP_DIR"
    fi
}

# Main installation flow
main() {
    header "kn CLI Installation Script"
    
    parse_args "$@"
    
    # Detect OS and architecture
    detect_os
    
    # Determine version to install
    local target_version="$VERSION"
    if [[ "$target_version" == "latest" ]]; then
        info "Fetching latest version from GitHub..."
        target_version=$(get_latest_version)
        if [[ -z "$target_version" ]]; then
            error "Failed to determine latest version"
            exit 1
        fi
    fi
    
    # Check if already installed with same version
    if check_installed_version "$target_version"; then
        success "Nothing to do!"
        exit 0
    fi
    
    # Check/Install dependencies
    if [[ "$SKIP_DEPS" == false ]]; then
        header "Checking dependencies"
        
        local deps_ok=true
        
        # Git (required)
        if ! check_dependency "Git" "git" 2>/dev/null; then
            if ! install_git; then
                error "Git installation failed"
                deps_ok=false
            fi
        fi
        
        # Node.js (required for MCP servers)
        if ! check_dependency "Node.js" "node" "$NODE_MIN_VERSION" 2>/dev/null; then
            if ! install_nodejs; then
                warn "Node.js installation skipped or failed"
                warn "You'll need to install Node.js manually for MCP servers"
            fi
        fi
        
        # bd (required) - will auto-install Rust if needed
        if ! check_dependency "bd" "bd" 2>/dev/null; then
            if ! install_bd; then
                error "bd installation failed"
                deps_ok=false
            fi
        fi
        
        if [[ "$deps_ok" == false ]]; then
            error "Some critical dependencies failed to install"
            error "Please resolve the issues above and try again"
            exit 1
        fi
    else
        info "Skipping dependency installation (--skip-deps)"
    fi
    
    # Update VERSION global for download_kn
    VERSION="$target_version"
    
    # Download kn binary
    if ! download_kn; then
        cleanup
        exit 1
    fi
    
    # Install kn
    if ! install_kn; then
        cleanup
        exit 1
    fi
    
    # Setup kn resources (agents, skills, MCPs)
    if ! setup_kn_resources; then
        warn "Resource setup had issues, but kn is installed"
    fi
    
    # Add to shell PATH configuration
    add_to_shell_config
    
    # Cleanup temporary files
    cleanup
    
    # Verify installation
    verify_installation
    
    # Final message with ASCII banner
    echo ""
    echo -e "${CYAN}╦╔═╔╗╔"
    echo -e "╠╩╗║║║"
    echo -e "╩ ╩╝╚╝${NC}"
    echo ""
    echo -e "${GREEN}Knowledge Framework CLI${NC}"
    echo ""
    info "Get started with: ${GREEN}kn init${NC}"
    info "Run ${GREEN}kn --help${NC} for more information"
    info "Update to latest version anytime with: ${GREEN}kn update${NC}"
    echo ""
    info "Documentation: https://github.com/kobogithub/knowledge"
    echo ""
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Run main
main "$@"
