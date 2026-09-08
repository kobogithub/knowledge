#!/usr/bin/env bash
#
# uninstall.sh - Uninstallation script for kn CLI
#
# This script removes kn and optionally removes all its data and configuration.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/uninstall.sh | bash
#   or
#   ./uninstall.sh
#
# Options:
#   --remove-data      Remove ~/.kn/ directory (agents, skills, MCPs)
#   --remove-config    Remove all project configurations (kn.toml, .opencode/, .gemini/)
#   --yes              Skip confirmation prompts
#   --help             Show this help message
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
INSTALL_LOCATIONS=(
    "$HOME/.local/bin/kn"
    "/usr/local/bin/kn"
)

# Flags
REMOVE_DATA=false
REMOVE_CONFIG=false
YES=false

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
            --remove-data)
                REMOVE_DATA=true
                shift
                ;;
            --remove-config)
                REMOVE_CONFIG=true
                shift
                ;;
            --yes)
                YES=true
                shift
                ;;
            --help)
                cat << EOF
kn CLI Uninstallation Script

Usage: $0 [OPTIONS]

Options:
    --remove-data       Remove ~/.kn/ directory (agents, skills, MCPs)
    --remove-config     Remove project configurations (kn.toml, .opencode/, .gemini/)
    --yes               Skip all confirmation prompts
    --help              Show this help message

This script will:
1. Remove the kn binary from your system
2. Optionally remove ~/.kn/ directory (if --remove-data is specified)
3. Optionally remove project configurations (if --remove-config is specified)
4. Clean up shell configuration files (.bashrc, .zshrc, etc.)

Note: This does NOT uninstall dependencies like Git or Node.js.

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

# Check if command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Find kn installation
find_kn_installation() {
    for location in "${INSTALL_LOCATIONS[@]}"; do
        if [[ -f "$location" ]]; then
            echo "$location"
            return 0
        fi
    done
    return 1
}

# Remove kn binary
remove_kn_binary() {
    header "Removing kn binary"
    
    local kn_path
    if kn_path=$(find_kn_installation); then
        info "Found kn at: $kn_path"
        
        # Check if we need sudo
        if [[ "$kn_path" == "/usr/local/bin/kn" ]]; then
            info "Removing kn (requires sudo)..."
            if sudo rm -f "$kn_path"; then
                success "kn binary removed from $kn_path"
            else
                error "Failed to remove kn binary"
                return 1
            fi
        else
            info "Removing kn..."
            if rm -f "$kn_path"; then
                success "kn binary removed from $kn_path"
            else
                error "Failed to remove kn binary"
                return 1
            fi
        fi
    else
        warn "kn binary not found in standard locations"
        if command_exists kn; then
            local which_kn=$(which kn)
            warn "kn found at: $which_kn"
            
            if [[ "$YES" == false ]]; then
                read -p "Remove this installation? (y/N) " -n 1 -r
                echo
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    rm -f "$which_kn" && success "Removed $which_kn"
                fi
            fi
        else
            info "kn is not installed"
        fi
    fi
    
    return 0
}

# Remove ~/.kn/ directory
remove_kn_data() {
    header "Removing ~/.kn/ directory"
    
    local kn_home="$HOME/.kn"
    
    if [[ ! -d "$kn_home" ]]; then
        info "~/.kn/ directory does not exist"
        return 0
    fi
    
    info "This will remove all globally installed resources:"
    echo "  - Agents: ~/.kn/agents/"
    echo "  - Skills: ~/.kn/skills/"
    echo "  - MCPs:   ~/.kn/mcps/"
    echo ""
    
    if [[ "$YES" == false ]]; then
        warn "This action cannot be undone!"
        read -p "Are you sure you want to remove ~/.kn/? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Skipping ~/.kn/ removal"
            return 0
        fi
    fi
    
    if rm -rf "$kn_home"; then
        success "Removed ~/.kn/ directory"
    else
        error "Failed to remove ~/.kn/ directory"
        return 1
    fi
    
    return 0
}

# Remove project configurations
remove_project_configs() {
    header "Scanning for project configurations"
    
    info "Searching for kn projects in common locations..."
    
    # Search in home directory and common project locations
    local search_paths=(
        "$HOME/projects"
        "$HOME/code"
        "$HOME/workspace"
        "$HOME/dev"
        "$HOME/Documents"
    )
    
    local found_projects=()
    
    for search_path in "${search_paths[@]}"; do
        if [[ -d "$search_path" ]]; then
            while IFS= read -r -d '' config_file; do
                local project_dir=$(dirname "$config_file")
                found_projects+=("$project_dir")
            done < <(find "$search_path" -maxdepth 3 -name "kn.toml" -print0 2>/dev/null)
        fi
    done
    
    if [[ ${#found_projects[@]} -eq 0 ]]; then
        info "No kn projects found in common locations"
        return 0
    fi
    
    info "Found ${#found_projects[@]} kn project(s):"
    for project in "${found_projects[@]}"; do
        echo "  - $project"
    done
    echo ""
    
    if [[ "$YES" == false ]]; then
        warn "This will remove kn.toml, .opencode/, and .gemini/ from these projects"
        read -p "Continue? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Skipping project configuration removal"
            return 0
        fi
    fi
    
    for project in "${found_projects[@]}"; do
        info "Cleaning $project..."
        rm -f "$project/kn.toml" 2>/dev/null || true
        rm -rf "$project/.opencode" 2>/dev/null || true
        rm -rf "$project/.gemini" 2>/dev/null || true
        rm -f "$project/AGENTS.md" 2>/dev/null || true
        success "Cleaned $project"
    done
    
    return 0
}

# Clean shell configuration
clean_shell_config() {
    header "Cleaning shell configuration"
    
    local current_shell
    current_shell=$(basename "$SHELL")
    
    local config_files=()
    
    case $current_shell in
        fish)
            config_files+=("$HOME/.config/fish/config.fish")
            ;;
        zsh)
            config_files+=("${ZDOTDIR:-$HOME}/.zshrc" "${ZDOTDIR:-$HOME}/.zshenv")
            ;;
        bash)
            config_files+=("$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile")
            ;;
        ash|sh)
            config_files+=("$HOME/.profile")
            ;;
        *)
            config_files+=("$HOME/.bashrc" "$HOME/.profile")
            ;;
    esac
    
    local cleaned=false
    
    for config_file in "${config_files[@]}"; do
        if [[ -f "$config_file" ]] && [[ -w "$config_file" ]]; then
            # Check if file contains kn references
            if grep -q "kn - Knowledge Framework CLI" "$config_file" 2>/dev/null; then
                info "Cleaning $config_file..."
                
                # Create backup
                cp "$config_file" "${config_file}.backup.$(date +%Y%m%d_%H%M%S)"
                
                # Remove kn-related lines
                sed -i.tmp '/# kn - Knowledge Framework CLI/,+1d' "$config_file" 2>/dev/null || \
                sed -i '' '/# kn - Knowledge Framework CLI/,+1d' "$config_file" 2>/dev/null || true
                
                rm -f "${config_file}.tmp" 2>/dev/null || true
                
                success "Cleaned $config_file (backup created)"
                cleaned=true
            fi
        fi
    done
    
    if [[ "$cleaned" == false ]]; then
        info "No shell configuration cleanup needed"
    fi
    
    return 0
}

# Main uninstallation flow
main() {
    header "kn CLI Uninstallation Script"
    
    parse_args "$@"
    
    # Check if kn is installed
    if ! command_exists kn && [[ ! -d "$HOME/.kn" ]]; then
        info "kn does not appear to be installed"
        exit 0
    fi
    
    # Confirmation prompt
    if [[ "$YES" == false ]]; then
        echo "This will uninstall kn from your system."
        echo ""
        echo "What will be removed:"
        echo "  ✓ kn binary"
        echo "  ✓ Shell configuration entries"
        
        if [[ "$REMOVE_DATA" == true ]]; then
            echo "  ✓ ~/.kn/ directory (all agents, skills, MCPs)"
        else
            echo "  ✗ ~/.kn/ directory (use --remove-data to remove)"
        fi
        
        if [[ "$REMOVE_CONFIG" == true ]]; then
            echo "  ✓ Project configurations (kn.toml, .opencode/, .gemini/)"
        else
            echo "  ✗ Project configurations (use --remove-config to remove)"
        fi
        
        echo ""
        echo "Dependencies (Git, Node.js) will NOT be removed."
        echo ""
        
        read -p "Continue with uninstallation? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Uninstallation cancelled"
            exit 0
        fi
    fi
    
    # Remove kn binary
    remove_kn_binary
    
    # Clean shell configuration
    clean_shell_config
    
    # Remove ~/.kn/ if requested
    if [[ "$REMOVE_DATA" == true ]]; then
        remove_kn_data
    else
        info "Keeping ~/.kn/ directory (use --remove-data to remove)"
    fi
    
    # Remove project configs if requested
    if [[ "$REMOVE_CONFIG" == true ]]; then
        remove_project_configs
    else
        info "Keeping project configurations (use --remove-config to remove)"
    fi
    
    # Final message
    echo ""
    echo -e "${GREEN}✓ Uninstallation complete!${NC}"
    echo ""
    
    if [[ "$REMOVE_DATA" == false ]]; then
        info "To completely remove all kn data, run:"
        echo "  ./uninstall.sh --remove-data"
        echo ""
    fi
    
    if [[ "$REMOVE_CONFIG" == false ]]; then
        info "To remove project configurations, run:"
        echo "  ./uninstall.sh --remove-config"
        echo ""
    fi
    
    info "Thank you for using kn!"
    echo ""
}

# Run main
main "$@"
