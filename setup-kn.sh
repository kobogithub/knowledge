#!/usr/bin/env bash
# Setup script to populate ~/.kn/ with all agents, skills, and MCPs
# This should be run once after installing the Knowledge Framework CLI

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 Setting up Knowledge Framework global resources...${NC}\n"

# Determine the script directory (where this repo is)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create ~/.kn directory structure
KN_HOME="${HOME}/.kn"
echo -e "${BLUE}📁 Creating directory structure in ${KN_HOME}...${NC}"
mkdir -p "${KN_HOME}/agents"
mkdir -p "${KN_HOME}/skills"
mkdir -p "${KN_HOME}/mcps"
echo -e "${GREEN}  ✓ Directories created${NC}\n"

# Install agents
echo -e "${BLUE}🤖 Installing agents...${NC}"
AGENTS_INSTALLED=0
if [ -d "${SCRIPT_DIR}/agents" ]; then
    for agent_dir in "${SCRIPT_DIR}/agents"/*; do
        if [ -d "$agent_dir" ]; then
            agent_name=$(basename "$agent_dir")
            echo -e "  Installing ${agent_name}..."
            cp -r "$agent_dir" "${KN_HOME}/agents/"
            ((AGENTS_INSTALLED++))
        fi
    done
    echo -e "${GREEN}  ✓ Installed ${AGENTS_INSTALLED} agents${NC}\n"
else
    echo -e "${YELLOW}  ⚠ No agents directory found${NC}\n"
fi

# Install skills (if any exist in this repo)
echo -e "${BLUE}📚 Installing skills...${NC}"
SKILLS_INSTALLED=0
if [ -d "${SCRIPT_DIR}/skills" ]; then
    for skill_dir in "${SCRIPT_DIR}/skills"/*; do
        if [ -d "$skill_dir" ]; then
            skill_name=$(basename "$skill_dir")
            echo -e "  Installing ${skill_name}..."
            cp -r "$skill_dir" "${KN_HOME}/skills/"
            ((SKILLS_INSTALLED++))
        fi
    done
    echo -e "${GREEN}  ✓ Installed ${SKILLS_INSTALLED} skills${NC}\n"
else
    echo -e "${YELLOW}  ℹ No skills directory found (this is normal)${NC}\n"
fi

# Setup MCP presets (metadata only, actual MCPs installed via kn mcp install)
echo -e "${BLUE}🔌 Setting up MCP presets...${NC}"
echo -e "${YELLOW}  ℹ MCP servers are installed on-demand via 'kn mcp install <name>'${NC}"
echo -e "${YELLOW}  ℹ Available presets: filesystem, github, postgres, brave-search, puppeteer${NC}\n"

# Summary
echo -e "${GREEN}✓ Setup complete!${NC}\n"
echo -e "${CYAN}Summary:${NC}"
echo -e "  Agents:  ${AGENTS_INSTALLED} installed in ${KN_HOME}/agents/"
echo -e "  Skills:  ${SKILLS_INSTALLED} installed in ${KN_HOME}/skills/"
echo -e "  MCPs:    Ready (install on-demand)"

echo -e "\n${CYAN}Next steps:${NC}"
echo -e "  1. View installed agents: ${YELLOW}kn agents list${NC}"
echo -e "  2. View installed skills: ${YELLOW}kn skills list${NC}"
echo -e "  3. Install MCPs as needed: ${YELLOW}kn mcp install <name>${NC}"
echo -e "  4. Initialize a project: ${YELLOW}cd <project> && kn init${NC}"

echo -e "\n${GREEN}🎉 You're all set to use the Knowledge Framework!${NC}"
