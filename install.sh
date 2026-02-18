#!/usr/bin/env bash
#
# install.sh - Automated installation script for kn CLI
#
# This script installs kn and all its dependencies on Linux and macOS.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/main/install.sh | bash
#   or
#   ./install.sh
#
# Options:
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
NC='\033[0m' # No Color

# Configuration
RUST_MIN_VERSION="1.70"
NODE_MIN_VERSION="18.0"
INSTALL_DIR="${HOME}/.local/bin"
KN_REPO="https://github.com/kobogithub/knowledge.git"
TMP_DIR="${TMPDIR:-/tmp}/kn-install-$$"

# Flags
SKIP_DEPS=false
NO_CONFIRM=false

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
            --skip-deps)
                SKIP_DEPS=true
                shift
                ;;
            --no-confirm)
                NO_CONFIRM=true
                shift
                ;;
            --help)
                cat << EOF
kn CLI Installation Script

Usage: $0 [OPTIONS]

Options:
    --skip-deps     Skip automatic dependency installation
    --no-confirm    Skip all confirmation prompts
    --help          Show this help message

Dependencies installed (if not present):
    - Rust/Cargo (via rustup)
    - Git
    - Node.js (via package manager or manual instructions)
    - bd (beads) - via cargo install

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
    
    info "Detected OS: $OS (package manager: $PKG_MANAGER)"
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

# Install Rust via rustup
install_rust() {
    header "Installing Rust"
    
    if check_dependency "Rust" "rustc" "$RUST_MIN_VERSION" 2>/dev/null; then
        info "Rust is already installed"
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

# Clone repository
clone_repo() {
    header "Cloning kn repository"
    
    if [[ -d "$TMP_DIR" ]]; then
        rm -rf "$TMP_DIR"
    fi
    
    info "Cloning to $TMP_DIR..."
    if git clone --depth 1 "$KN_REPO" "$TMP_DIR"; then
        success "Repository cloned"
        return 0
    else
        error "Failed to clone repository"
        return 1
    fi
}

# Build kn
build_kn() {
    header "Building kn CLI"
    
    cd "$TMP_DIR/cli" || {
        error "Failed to enter cli directory"
        return 1
    }
    
    info "Building kn (this may take a few minutes)..."
    if cargo build --release; then
        success "kn built successfully"
        return 0
    else
        error "Failed to build kn"
        return 1
    fi
}

# Install kn binary
install_kn() {
    header "Installing kn binary"
    
    local binary="$TMP_DIR/cli/target/release/kn"
    
    if [[ ! -f "$binary" ]]; then
        error "Binary not found at $binary"
        return 1
    fi
    
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
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        info "Add the following line to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
    fi
    
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
    
    # Detect OS
    detect_os
    
    # Check/Install dependencies
    if [[ "$SKIP_DEPS" == false ]]; then
        header "Checking dependencies"
        
        local deps_ok=true
        
        # Rust (required)
        if ! check_dependency "Rust" "rustc" "$RUST_MIN_VERSION" 2>/dev/null; then
            if ! install_rust; then
                error "Rust installation failed"
                deps_ok=false
            fi
        fi
        
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
        
        # bd (required)
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
    
    # Clone repository
    if ! clone_repo; then
        cleanup
        exit 1
    fi
    
    # Build kn
    if ! build_kn; then
        cleanup
        exit 1
    fi
    
    # Install kn
    if ! install_kn; then
        cleanup
        exit 1
    fi
    
    # Cleanup temporary files
    cleanup
    
    # Verify installation
    verify_installation
    
    # Final message
    echo ""
    header "Installation Complete!"
    info "Get started with: kn init"
    info "Run 'kn --help' for more information"
    echo ""
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Run main
main "$@"
