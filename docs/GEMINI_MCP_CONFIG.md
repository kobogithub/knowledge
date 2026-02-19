# Gemini Code Assist MCP Configuration

This document explains how the Knowledge Framework CLI generates MCP configuration for Gemini Code Assist.

## Overview

The Knowledge Framework now supports dual workspace standards:
- **OpenCode**: Uses `.opencode/opencode.json`
- **Gemini Code Assist (Antigravity)**: Uses `.gemini/antigravity/mcp_config.json`
- **Both**: Generates both configuration files

## Configuration File Format

### Gemini Format (.gemini/antigravity/mcp_config.json)

```json
{
  "mcpServers": {
    "server-name": {
      "command": "executable",
      "args": ["arg1", "arg2"],
      "env": {
        "KEY": "value"
      }
    }
  }
}
```

### Differences from OpenCode Format

| Feature | OpenCode | Gemini |
|---------|----------|--------|
| Schema | `$schema` required | No schema |
| Type field | `type: "local"` required | Not used |
| Command format | Single array `["cmd", "arg1"]` | Separate `command` + `args` |
| Environment | Optional `environment` object | Always present `env` object |
| Server naming | Uses MCP name as-is | Appends `-mcp-server` suffix |

## Workflow

### 1. Project Initialization

```bash
# Initialize with Antigravity workspace
kn init
# Select "Antigravity" or "Both" as workspace standard
```

This creates:
- `.gemini/antigravity/` directory
- `mcp_config.json` (empty or with configured MCPs)

### 2. Install MCPs Globally

```bash
# Install MCP to ~/.kn/mcps/
kn mcp install github
kn mcp install supabase
```

### 3. Add MCPs to Project

```bash
# Add to project's kn.toml
kn mcp add github
kn mcp add supabase
```

### 4. Configure in kn.toml

```toml
[mcp]
enabled = ["github", "supabase"]

[mcp.config.github]
enabled = true
environment = { GITHUB_PERSONAL_ACCESS_TOKEN = "${GITHUB_TOKEN}" }

[mcp.config.supabase]
enabled = true
args = ["--access-token", "${SUPABASE_TOKEN}"]
```

### 5. Generate Configuration

```bash
kn sync
```

This generates `.gemini/antigravity/mcp_config.json`:

```json
{
  "mcpServers": {
    "github-mcp-server": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "-e",
        "GITHUB_PERSONAL_ACCESS_TOKEN",
        "ghcr.io/github/github-mcp-server"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "github_pat_xxx"
      }
    },
    "supabase-mcp-server": {
      "command": "npx",
      "args": [
        "-y",
        "@supabase/mcp-server-supabase@latest",
        "--access-token",
        "sbp_xxx"
      ],
      "env": {}
    }
  }
}
```

## Security Considerations

### Secrets Management

⚠️ **IMPORTANT**: The `.gemini/antigravity/mcp_config.json` file contains secrets and is automatically added to `.gitignore`.

**DO NOT** commit this file to version control!

### Environment Variable Expansion

The system supports `${VAR}` syntax for environment variables:

```toml
[mcp.config.github]
environment = { GITHUB_TOKEN = "${GITHUB_PERSONAL_ACCESS_TOKEN}" }
```

When `kn sync` runs, it expands these variables using the current environment.

### Recommended Pattern

1. Store secrets in `.env` file (also gitignored)
2. Reference them in `kn.toml` using `${VAR}` syntax
3. Run `kn sync` to generate the final config with expanded values

Example `.env`:
```bash
GITHUB_PERSONAL_ACCESS_TOKEN=github_pat_xxx
SUPABASE_ACCESS_TOKEN=sbp_xxx
```

Example `kn.toml`:
```toml
[mcp.config.github]
environment = { GITHUB_PERSONAL_ACCESS_TOKEN = "${GITHUB_PERSONAL_ACCESS_TOKEN}" }

[mcp.config.supabase]
args = ["--access-token", "${SUPABASE_ACCESS_TOKEN}"]
```

## Manual Configuration

You can manually add MCP servers to `.gemini/antigravity/mcp_config.json`. The `kn sync` command will preserve manually added servers that aren't managed by `kn`.

Example:
```json
{
  "mcpServers": {
    "custom-server": {
      "command": "node",
      "args": ["./custom-mcp.js"],
      "env": {}
    }
  }
}
```

After running `kn sync`, `custom-server` will be preserved alongside any MCPs managed by `kn`.

## Preset MCPs

The following preset MCPs are available and work with both OpenCode and Gemini:

- `filesystem` - File system access
- `github` - GitHub API integration
- `postgres` - PostgreSQL database access
- `brave-search` - Brave Search API
- `puppeteer` - Browser automation

See available presets:
```bash
kn mcp presets
```

## Commands Reference

```bash
# View workspace configuration
cat kn.toml

# List installed MCPs
kn mcp list

# Add MCP to project
kn mcp add <name>

# Regenerate configs
kn sync

# View generated Gemini config
cat .gemini/antigravity/mcp_config.json
```

## Implementation Details

### Code Structure

- `cli/src/config/gemini.rs` - Gemini config model
- `cli/src/config/opencode.rs` - OpenCode config model
- `cli/src/commands/sync.rs` - Config generation logic
- `cli/src/models/mcp.rs` - MCP metadata and presets

### Command Translation

The system automatically translates between MCP metadata format and Gemini's expected format:

1. **Load MCP metadata** from `~/.kn/mcps/<name>/mcp.toml`
2. **Apply project overrides** from `kn.toml`
3. **Generate full command** by combining executable + args
4. **Split into Gemini format** (first element = command, rest = args)
5. **Merge environment variables** (global + project-specific)
6. **Add server name suffix** (-mcp-server if not present)

### Example Transformation

MCP Metadata:
```toml
[command]
type = "local"
executable = "npx"
args = ["-y", "@package/name"]

[config]
default_args = ["arg1"]
```

Project Override:
```toml
[mcp.config.myserver]
args = ["custom-arg"]
```

Generated Gemini Config:
```json
{
  "command": "npx",
  "args": ["-y", "@package/name", "custom-arg"]
}
```

## Troubleshooting

### Config not generated

Check:
1. Workspace standard is set to "antigravity" or "both" in `kn.toml`
2. MCPs are listed in `enabled` array
3. MCP is installed in `~/.kn/mcps/`

### Secrets not expanded

Ensure:
1. Environment variables are set before running `kn sync`
2. Syntax is `${VAR}` not `$VAR`
3. Variable names match exactly (case-sensitive)

### Server not found

Gemini expects specific server names. Check:
1. Server name ends with `-mcp-server`
2. Command is executable in your PATH
3. For Docker commands, Docker is running

## See Also

- [MCP Architecture](./MCP_ARCHITECTURE.md) - Overall MCP system design
- [OpenCode Configuration](https://opencode.ai/docs/config) - OpenCode format reference
- [Gemini Code Assist Docs](https://cloud.google.com/gemini/docs/code-assist) - Gemini documentation
