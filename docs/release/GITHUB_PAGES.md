# GitHub Pages Setup for kn.foxlabar.online

This directory contains GitHub Pages configuration for short URL redirects.

## URLs

- `https://kn.foxlabar.online/` → Project homepage (redirects to install.sh)
- `https://kn.foxlabar.online/install` → Install script (serves install.sh directly)

**Note:** The `/install` endpoint serves the actual install.sh script content directly (not an HTML redirect), 
so it works with curl: `curl -fsSL https://kn.foxlabar.online/install | bash`

## DNS Configuration Required

To make this work, you need to configure DNS records for `foxlabar.online`:

### Option 1: CNAME Record (Recommended)
```
Type: CNAME
Name: kn
Value: kobogithub.github.io.
```

### Option 2: A Records + ALIAS
```
Type: A
Name: kn
Value: 185.199.108.153
Value: 185.199.109.153
Value: 185.199.110.153
Value: 185.199.111.153
```

## GitHub Pages Configuration

1. Go to: https://github.com/kobogithub/knowledge/settings/pages
2. Source: Deploy from branch `prod`
3. Custom domain: `kn.foxlabar.online`
4. Enforce HTTPS: ✓ (after DNS propagates)

## Usage

Once configured, users can install with:

```bash
curl -fsSL https://kn.foxlabar.online/install | bash
```

## Implementation Details

The `/install` endpoint (`install/index.html`) contains a **copy of install.sh** (not an HTML redirect).
This is because:
- GitHub Pages doesn't support HTTP 301 redirects
- HTML meta redirects don't work with curl
- Serving the script content directly ensures `curl | bash` works

To update the install script:
1. Edit `install.sh` 
2. Copy to `install/index.html`: `cp install.sh install/index.html`
3. Commit and push both files

## Testing

```bash
# Test the short URL (after GitHub Pages deploys)
curl -fsSL https://kn.foxlabar.online/install | head -10

# Should show bash script, not HTML
```
