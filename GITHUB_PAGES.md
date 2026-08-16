# GitHub Pages Setup

This repository publishes a small GitHub Pages site that serves the install script
over a short URL.

## Current URLs

- `https://kobogithub.github.io/knowledge/` → Project homepage (redirects to `install.sh`)
- `https://kobogithub.github.io/knowledge/install/` → Install script (serves `install.sh` directly)

**Note:** The `/install/` endpoint serves the actual `install.sh` script content directly
(not an HTML redirect), so it works with curl.

## Retired custom domain

This site was previously served at `kn.foxlabar.online`, and the READMEs recommended
`curl -fsSL https://kn.foxlabar.online/install | bash` as the primary install command.

**That domain is gone.** The `foxlabar.online` registration lapsed and the domain is now
available for anyone to register. Because the install command piped it straight into
`bash`, whoever registered it next would have been able to execute arbitrary code on the
machine of anyone following the README.

The custom domain has therefore been retired rather than restored:

- `CNAME` was removed, so GitHub Pages no longer claims the hostname
- Both READMEs and ADR-004 now point at the `raw.githubusercontent.com` URL, which is
  tied to this repository and cannot be taken over by a third party

**Do not re-add a `CNAME` for `foxlabar.online` unless the domain is re-registered under
an account you control.** Pointing Pages at a hostname you do not own is what created the
exposure in the first place.

If a short URL is wanted again in the future, prefer either the `github.io` path above or
a domain held in the maintainer's own registrar account with auto-renew enabled.

## GitHub Pages Configuration

1. Go to: https://github.com/kobogithub/knowledge/settings/pages
2. Source: Deploy from branch `prod`
3. Custom domain: *(empty)*
4. Enforce HTTPS: ✓ (automatic on `github.io`)

## Implementation Details

The `/install/` endpoint (`install/index.html`) contains a **copy of `install.sh`** (not an
HTML redirect). This is because:

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
curl -fsSL https://kobogithub.github.io/knowledge/install/ | head -10

# Should show bash script, not HTML
```
