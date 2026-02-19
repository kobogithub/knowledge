# GitHub Pages Setup for kn.foxlabar.ar

This directory contains GitHub Pages configuration for short URL redirects.

## URLs

- `https://kn.foxlabar.ar/` → Project homepage (redirects to install.sh)
- `https://kn.foxlabar.ar/install` → Install script (redirects to raw install.sh)

## DNS Configuration Required

To make this work, you need to configure DNS records for `foxlabar.ar`:

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
3. Custom domain: `kn.foxlabar.ar`
4. Enforce HTTPS: ✓ (after DNS propagates)

## Usage

Once configured, users can install with:

```bash
curl -fsSL https://kn.foxlabar.ar/install | bash
```

## Testing

Before DNS is configured, you can test locally:

```bash
# Open install/index.html in browser
# Should redirect to raw install.sh
```
