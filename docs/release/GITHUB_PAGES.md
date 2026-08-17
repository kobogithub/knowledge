# GitHub Pages Setup

This repository publishes a small GitHub Pages site that serves the install script
over a short URL.

## Current URLs

- `https://kobogithub.github.io/knowledge/` → Project homepage (redirects to `install.sh`)
- `https://kobogithub.github.io/knowledge/install/` → Install script (serves `install.sh` directly)

**Note:** The `/install/` endpoint serves the actual `install.sh` script content directly
(not an HTML redirect), so it works with curl.

## Retired custom domain: `kn.foxlabar.online`

This site was previously served at `kn.foxlabar.online`, and the READMEs recommended
`curl -fsSL https://kn.foxlabar.online/install | bash` as the primary install command.

**That domain is gone.** The `foxlabar.online` registration lapsed and the domain is now
available for anyone to register. Because the install command piped it straight into
`bash`, whoever registered it next would have been able to execute arbitrary code on the
machine of anyone following the README.

It has therefore been retired, not restored:

- `CNAME` was removed, so GitHub Pages no longer claims the hostname
- Both READMEs and ADR-004 now point at the `raw.githubusercontent.com` URL, which is
  tied to this repository and cannot be taken over by a third party

**Never re-add a `CNAME` for `foxlabar.online`.** Pointing Pages at a hostname you do not
own is what created the exposure.

## Planned replacement: `kn.kobouharriet.me`

The short URL is moving to a subdomain of `kobouharriet.me`, which is held in the
maintainer's own Hostinger account and already serves `kobogithub/web-personal` at its
apex via GitHub Pages.

**The order matters.** Adding the `CNAME` file before DNS resolves puts Pages into the
`bad_authz` certificate state that the old domain was stuck in. Do it in this order:

1. **Create the DNS record at Hostinger** — a `CNAME` on host `kn` pointing at
   `kobogithub.github.io.` (trailing dot). Leave the apex `A` records alone; they belong
   to the personal site and the subdomain is independent.
2. **Wait for propagation** — `dig +short kn.kobouharriet.me` must return
   `kobogithub.github.io.` and then the four Pages IPs.
3. **Add `CNAME`** to this repo containing `kn.kobouharriet.me`, on `prod`.
4. **Wait for the certificate** — `gh api repos/kobogithub/knowledge/pages -q
   '.https_certificate.state'` must report `approved`, then enable *Enforce HTTPS*.
5. **Only then** promote `https://kn.kobouharriet.me/install` to the recommended install
   command in both READMEs. Until the URL actually serves the script over HTTPS, the
   READMEs must keep pointing at `raw.githubusercontent.com`.

### Keeping it alive

`kobouharriet.me` is registered through Hostinger and now expires **2027-09-08**
(renewed 2026-08-16, verified against the registry — not the resolver, which is the
mistake that misdiagnosed the previous domain).

The previous short URL died because a registration lapsed unnoticed while the public
README still recommended piping it into `bash`, which meant whoever registered the name
next could have run arbitrary code on anyone following the instructions. That is the
failure this section exists to prevent, so:

- **Check the expiry before it matters**, with `whois kobouharriet.me | grep -i expiry`.
  A calendar reminder a month out costs nothing.
- **Verify the domain at the GitHub account level** (Settings → Pages → Verified
  domains). This is the real safeguard — it stops anyone else from claiming the hostname
  on GitHub Pages even if the registration ever does lapse, which a renewal date alone
  does not.

If the domain is ever allowed to expire, **remove the `CNAME` file and the short URL
from both READMEs in the same change**. Pointing users at a hostname you no longer own
is worse than having no short URL at all.

## GitHub Pages Configuration

1. Go to: https://github.com/kobogithub/knowledge/settings/pages
2. Source: Deploy from branch `prod`
3. Custom domain: `kn.kobouharriet.me`
4. Enforce HTTPS: ✓ (once the certificate is issued)

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

**CI enforces this.** The `lint-scripts` job fails if the two files differ. They had
already drifted by 202 lines — `install/index.html` still carried the pre-004 platform
detection, so the short URL would have served a script that still tried to install on
Linux. Two hand-synced copies is the same failure mode that left the Homebrew tap two
releases behind; both are now asserted rather than trusted.

## Testing

```bash
# Test the short URL (after GitHub Pages deploys)
curl -fsSL https://kobogithub.github.io/knowledge/install/ | head -10

# Should show bash script, not HTML
```
