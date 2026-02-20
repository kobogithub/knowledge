# Environment Variables Guide

This document explains how to securely manage credentials and environment variables in the Knowledge Framework project.

## Quick Start

1. **Copy the example file**:
   ```bash
   cp .env.example .env
   ```

2. **Add your credentials** to `.env`:
   ```bash
   GITHUB_PERSONAL_ACCESS_TOKEN=your_actual_github_token
   SUPABASE_ACCESS_TOKEN=your_actual_supabase_token
   ```

3. **Never commit `.env`** - it's already in `.gitignore`

## Required Environment Variables

### GitHub Personal Access Token

**Variable**: `GITHUB_PERSONAL_ACCESS_TOKEN`

**How to get it**:
1. Go to https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Select scopes: `repo`, `read:org`, `read:user`
4. Copy the token (starts with `github_pat_`)

**Used by**: GitHub MCP Server for repository access

### Supabase Access Token

**Variable**: `SUPABASE_ACCESS_TOKEN`

**How to get it**:
1. Go to https://supabase.com/dashboard/account/tokens
2. Click "Generate new token"
3. Copy the token (starts with `sbp_`)

**Used by**: Supabase MCP Server for database access

## How It Works

### 1. Environment File (`.env`)

Store your secrets here:

```bash
# .env (gitignored - never committed)
GITHUB_PERSONAL_ACCESS_TOKEN=github_pat_11ADMH53Q0gDDmbLJwj53a_xxxxx
SUPABASE_ACCESS_TOKEN=sbp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### 2. MCP Configuration (`.gemini/antigravity/mcp_config.json`)

References environment variables using `${VAR}` syntax:

```json
{
  "mcpServers": {
    "github-mcp-server": {
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_PERSONAL_ACCESS_TOKEN}"
      }
    },
    "supabase-mcp-server": {
      "args": [
        "--access-token",
        "${SUPABASE_ACCESS_TOKEN}"
      ]
    }
  }
}
```

### 3. Runtime Expansion

When the MCP servers start, the `${VAR}` placeholders are replaced with actual values from your environment.

## Loading Environment Variables

### Option 1: Shell Session (Temporary)

```bash
# Load .env into current shell
export $(grep -v '^#' .env | xargs)

# Run your command
kn sync
```

### Option 2: dotenv Tool (Recommended)

```bash
# Install dotenv-cli
npm install -g dotenv-cli

# Run commands with .env automatically loaded
dotenv kn sync
```

### Option 3: direnv (Auto-load)

```bash
# Install direnv
# See: https://direnv.net/

# Allow the directory
direnv allow .

# Now .env is automatically loaded when you cd into this directory
```

## Security Best Practices

### ✅ DO

- Store secrets in `.env` (gitignored)
- Use `${VAR}` syntax in configuration files
- Rotate tokens regularly
- Use `.env.example` as a template (with placeholder values)
- Review `.gitignore` before committing

### ❌ DON'T

- Commit `.env` to git
- Hardcode tokens in configuration files
- Share tokens in chat/email
- Use production tokens in development
- Copy-paste secrets from screen recordings

## Token Rotation

If you need to rotate your tokens:

1. **Revoke old tokens**:
   - GitHub: https://github.com/settings/tokens
   - Supabase: https://supabase.com/dashboard/account/tokens

2. **Generate new tokens** (same scopes)

3. **Update `.env`** with new tokens

4. **Restart services** that use those tokens

## Verification

### Check if .env is properly ignored

```bash
git status .env
# Should show: "Untracked" or no output
```

### Check if config uses variables (not hardcoded)

```bash
# Should contain ${VAR} syntax, not actual tokens
cat .gemini/antigravity/mcp_config.json
```

### Test token expansion

```bash
# Load .env
export $(grep -v '^#' .env | xargs)

# Check if variables are set
echo $GITHUB_PERSONAL_ACCESS_TOKEN | head -c 20
# Should print: github_pat_11ADMH53...
```

## Troubleshooting

### "Token not found" or empty value

**Cause**: Environment variable not loaded

**Solution**:
```bash
# Load .env before running commands
export $(grep -v '^#' .env | xargs)
```

### "Invalid token" error

**Cause**: Token expired or revoked

**Solution**:
1. Generate new token
2. Update `.env`
3. Restart the service

### Config shows `${VAR}` literally

**Cause**: System doesn't support variable expansion

**Solution**: Use a tool like `envsubst` to manually expand:
```bash
envsubst < .gemini/antigravity/mcp_config.json
```

## Integration with kn CLI

The `kn` CLI supports environment variable expansion in `kn.toml`:

```toml
[mcp.config.github]
environment = { GITHUB_PERSONAL_ACCESS_TOKEN = "${GITHUB_PERSONAL_ACCESS_TOKEN}" }

[mcp.config.supabase]
args = ["--access-token", "${SUPABASE_ACCESS_TOKEN}"]
```

When you run `kn sync`, it expands variables from your environment and generates the final configuration.

## See Also

- [Security Audit Report](../SECURITY_AUDIT_REPORT.md) - Security findings and remediation
- [Gemini MCP Config](./GEMINI_MCP_CONFIG.md) - MCP configuration details
- [.env.example](../.env.example) - Template for environment variables
