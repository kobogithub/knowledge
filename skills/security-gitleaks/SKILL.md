---
name: security-gitleaks
description: Secret detection and prevention with Gitleaks
version: 1.0.0
tags:
  - security
  - secrets
  - credentials
  - pre-commit
  - supabase
---

# security-gitleaks

Deteccion de secrets y credenciales en repositorios git con Gitleaks. Previene que llaves de Supabase, JWT secrets, API keys y database URLs se suban al repositorio.

## Overview

Gitleaks detecta secrets en:
- **Codigo fuente**: Archivos actuales en el working tree
- **Historial git**: Commits pasados que pueden contener secrets
- **Pre-commit**: Prevenir que secrets se commiteen en primer lugar
- **CI/CD**: Gate de seguridad en pipelines

## Instalacion

```bash
# macOS
brew install gitleaks

# Linux
# Descargar desde GitHub releases
wget https://github.com/gitleaks/gitleaks/releases/latest/download/gitleaks_8.18.0_linux_x64.tar.gz
tar -xzf gitleaks_8.18.0_linux_x64.tar.gz
sudo mv gitleaks /usr/local/bin/

# Docker
docker run --rm -v "${PWD}:/src" zricethezav/gitleaks:latest detect --source /src

# Go
go install github.com/gitleaks/gitleaks/v8@latest
```

## Escaneo Basico

### Detectar secrets en archivos actuales

```bash
# Escanear directorio actual
gitleaks detect --source . --no-git

# Escanear con reporte verbose
gitleaks detect --source . --no-git --verbose

# Generar reporte JSON
gitleaks detect --source . --no-git --report-format json --report-path gitleaks-report.json

# Generar reporte SARIF (para GitHub)
gitleaks detect --source . --no-git --report-format sarif --report-path gitleaks-report.sarif
```

### Escanear historial git

```bash
# Escanear todo el historial
gitleaks detect --source .

# Escanear ultimos N commits
gitleaks detect --source . --log-opts="-n 50"

# Escanear un rango de commits
gitleaks detect --source . --log-opts="HEAD~10..HEAD"

# Escanear solo commits de un PR/branch
gitleaks detect --source . --log-opts="main..feature-branch"
```

### Escanear cambios staged (pre-commit)

```bash
# Solo cambios staged para commit
gitleaks protect --staged --source .

# Con verbose output
gitleaks protect --staged --source . --verbose
```

## Configuracion Custom (.gitleaks.toml)

### Configuracion base para Supabase + FastAPI

```toml
# .gitleaks.toml
title = "Gitleaks Configuration for Supabase + FastAPI Project"

[extend]
# Usar reglas default de gitleaks como base
useDefault = true

# === REGLAS CUSTOM PARA SUPABASE ===

[[rules]]
id = "supabase-service-role-key"
description = "Supabase service_role key (bypasses RLS - CRITICAL)"
regex = '''eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\.[A-Za-z0-9_-]{50,}\.[A-Za-z0-9_-]{40,}'''
keywords = ["service_role", "SERVICE_ROLE"]
tags = ["supabase", "critical"]

[[rules]]
id = "supabase-anon-key"
description = "Supabase anon key (should be in env vars)"
regex = '''eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\.[A-Za-z0-9_-]{50,}\.[A-Za-z0-9_-]{40,}'''
keywords = ["anon", "ANON", "supabase"]
tags = ["supabase", "medium"]

[[rules]]
id = "supabase-db-url"
description = "Supabase database connection URL"
regex = '''postgres(ql)?:\/\/[^:]+:[^@]+@[a-z0-9-]+\.supabase\.(co|com):\d+\/\w+'''
tags = ["supabase", "database", "high"]

[[rules]]
id = "supabase-project-ref"
description = "Supabase project reference with password"
regex = '''[a-z]{20}\.supabase\.co'''
keywords = ["supabase"]
tags = ["supabase", "info"]

# === REGLAS PARA JWT ===

[[rules]]
id = "jwt-secret-hardcoded"
description = "Hardcoded JWT secret"
regex = '''(?i)(jwt[_-]?secret|jwt[_-]?key)\s*[:=]\s*["'][^"']{8,}["']'''
tags = ["jwt", "high"]

[[rules]]
id = "jwt-token-in-code"
description = "JWT token hardcoded in source code"
regex = '''eyJ[A-Za-z0-9_-]{10,}\.eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}'''
keywords = ["token", "jwt", "bearer"]
tags = ["jwt", "high"]
[rules.allowlist]
paths = ['''.*test.*''', '''.*spec.*''', '''.*mock.*''']

# === REGLAS PARA API KEYS ===

[[rules]]
id = "generic-api-key-assignment"
description = "Generic API key assignment"
regex = '''(?i)(api[_-]?key|apikey)\s*[:=]\s*["'][a-zA-Z0-9]{16,}["']'''
tags = ["api-key", "high"]

[[rules]]
id = "stripe-key"
description = "Stripe API key"
regex = '''(?:sk|pk)_(test|live)_[a-zA-Z0-9]{20,}'''
tags = ["stripe", "critical"]

[[rules]]
id = "openai-key"
description = "OpenAI API key"
regex = '''sk-[a-zA-Z0-9]{20,}'''
keywords = ["openai", "sk-"]
tags = ["openai", "high"]

# === REGLAS PARA DATABASE ===

[[rules]]
id = "database-url-with-password"
description = "Database URL with embedded password"
regex = '''(?i)(postgres|mysql|mongodb)(?:ql)?:\/\/[^:]+:[^@\s]+@[^\s]+'''
tags = ["database", "critical"]

# === ALLOWLIST GLOBAL ===

[allowlist]
description = "Global allowlist"

# Archivos que pueden contener tokens de ejemplo
paths = [
  '''\.env\.example''',
  '''\.env\.template''',
  '''\.env\.sample''',
  '''docs/.*\.md''',
  '''.*test.*fixture.*''',
  '''.*mock.*''',
  '''CHANGELOG\.md''',
]

# Patrones que son safe
regexes = [
  '''SUPABASE_URL=https://your-project\.supabase\.co''',
  '''SUPABASE_KEY=your-anon-key-here''',
  '''DATABASE_URL=postgresql://user:password@localhost''',
  '''sk-your-openai-key-here''',
  '''example\.com''',
  '''placeholder''',
  '''changeme''',
  '''xxx+''',
]
```

## Pre-commit Hook

### Instalacion con pre-commit framework

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.18.0
    hooks:
      - id: gitleaks
```

```bash
# Instalar pre-commit
pip install pre-commit

# Instalar hooks
pre-commit install

# Test manual
pre-commit run gitleaks --all-files
```

### Git hook manual (sin framework)

```bash
# .git/hooks/pre-commit
#!/bin/bash
# Gitleaks pre-commit hook

echo "Running Gitleaks secret scan..."

# Scan staged changes only
gitleaks protect --staged --source . --config .gitleaks.toml

if [ $? -ne 0 ]; then
    echo ""
    echo "ERROR: Gitleaks detected secrets in staged changes!"
    echo "Please remove secrets and use environment variables instead."
    echo ""
    echo "If this is a false positive, add to .gitleaks.toml allowlist"
    echo "or use: git commit --no-verify (NOT recommended)"
    exit 1
fi

echo "Gitleaks: No secrets detected. Proceeding with commit."
```

```bash
# Hacer ejecutable
chmod +x .git/hooks/pre-commit
```

## Integracion con CI/CD (GitHub Actions)

### Workflow basico

```yaml
name: Secret Detection
on:
  push:
    branches: [main]
  pull_request:

jobs:
  gitleaks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Necesario para escanear historial

      - name: Gitleaks scan
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GITLEAKS_CONFIG: .gitleaks.toml
```

### Workflow con reporte

```yaml
- name: Run Gitleaks
  uses: gitleaks/gitleaks-action@v2
  with:
    args: '--report-format sarif --report-path gitleaks-report.sarif'
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v3
  if: always()
  with:
    sarif_file: gitleaks-report.sarif
```

### Scan solo de PR changes

```yaml
- name: Gitleaks PR scan
  run: |
    gitleaks detect \
      --source . \
      --log-opts="${{ github.event.pull_request.base.sha }}..${{ github.sha }}" \
      --config .gitleaks.toml \
      --report-format json \
      --report-path gitleaks-pr.json
```

## Remediacion de Secrets Expuestos

### Si un secret fue commiteado

```bash
# 1. INMEDIATAMENTE: Revocar el secret
# - Supabase: Dashboard > Settings > API > Regenerate keys
# - JWT: Cambiar JWT_SECRET en todas las environments
# - API keys: Revocar en el provider

# 2. Remover del codigo actual
# Reemplazar con variable de entorno

# 3. (Opcional) Limpiar del historial git
# CUIDADO: Esto reescribe historia - coordinar con el equipo
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch path/to/file-with-secret" \
  --prune-empty --tag-name-filter cat -- --all

# O usar BFG Repo-Cleaner (mas rapido)
bfg --replace-text secrets.txt .

# 4. Force push (coordinar con equipo!)
git push origin --force --all

# 5. Documentar el incidente como comentario en el PR/rama
# o como entrada nueva en specs/NNN-feature/tasks.md:
# "SECURITY: Secret expuesto y remediado. Tipo: [tipo]. Commit: [hash]. Revocado y limpiado."
```

### Gestion correcta de secrets

```bash
# .env (NUNCA commitear - debe estar en .gitignore)
SUPABASE_URL=https://xxxxxxxxxxxx.supabase.co
SUPABASE_ANON_KEY=eyJhbGci...
SUPABASE_SERVICE_ROLE_KEY=eyJhbGci...
JWT_SECRET=super-secret-key-here
DATABASE_URL=postgresql://user:pass@host:5432/db

# .env.example (SI commitear - sin valores reales)
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-anon-key-here
SUPABASE_SERVICE_ROLE_KEY=your-service-role-key-here
JWT_SECRET=change-me-to-a-secure-random-string
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
```

### .gitignore esencial

```bash
# Secrets - NEVER commit these
.env
.env.local
.env.production
.env.staging
*.pem
*.key
*.p12
credentials.json
service-account.json
*-credentials.*
*-secret.*
```

## Reporte de Hallazgos

```bash
# Script para generar reporte como comentario de PR o entrada de tasks.md
gitleaks detect --source . --no-git --report-format json --report-path /tmp/gl.json 2>/dev/null

python3 -c "
import json
try:
    with open('/tmp/gl.json') as f:
        findings = json.load(f)
    if not findings:
        print('[Security Agent] Gitleaks: CLEAN - 0 secrets detected')
    else:
        print(f'[Security Agent] Gitleaks: {len(findings)} secrets detected!')
        for f in findings:
            print(f'  - {f[\"RuleID\"]}: {f[\"File\"]}:{f[\"StartLine\"]}')
            print(f'    Match: {f[\"Match\"][:50]}...')
except (FileNotFoundError, json.JSONDecodeError):
    print('[Security Agent] Gitleaks: CLEAN - 0 secrets detected')
"
```

## Mejores Practicas

### DO

- Configurar pre-commit hook desde el dia 1 del proyecto
- Usar `.gitleaks.toml` con reglas custom para Supabase keys
- Escanear historial git periodicamente (no solo archivos actuales)
- Revocar secrets inmediatamente si son detectados en commits
- Mantener `.env.example` actualizado con todos los vars necesarios (sin valores reales)
- Documentar en allowlist por que cada excepcion es segura
- Usar SARIF format en CI para integracion con GitHub Security tab

### DON'T

- Commitear `.env` files bajo ninguna circunstancia
- Usar `--no-verify` para saltear el pre-commit hook
- Ignorar findings sin investigar si son false positives
- Poner secrets en archivos de configuracion versionados
- Confiar en que `.gitignore` es suficiente (el historial puede tener secrets)
- Hardcodear tokens JWT de ejemplo que sean reales
- Olvidar regenerar keys despues de un leak

## Tipos de Secrets a Detectar

| Tipo | Ejemplo Pattern | Severidad |
|------|----------------|-----------|
| Supabase service_role | `eyJhbGci...` con `service_role` | CRITICAL |
| Supabase anon key | `eyJhbGci...` con `anon` | MEDIUM |
| Database URL | `postgresql://user:pass@host` | CRITICAL |
| JWT Secret | `JWT_SECRET=mysecret` | HIGH |
| Stripe Key | `sk_live_...` / `sk_test_...` | CRITICAL |
| OpenAI Key | `sk-...` | HIGH |
| AWS Keys | `AKIA...` | CRITICAL |
| GitHub Token | `ghp_...` / `gho_...` | HIGH |
| Private Key | `-----BEGIN RSA PRIVATE KEY-----` | CRITICAL |
| Generic API Key | `api_key=...` | MEDIUM |

## Recursos

- [Gitleaks Documentation](https://github.com/gitleaks/gitleaks)
- [Gitleaks GitHub Action](https://github.com/gitleaks/gitleaks-action)
- [Gitleaks Config Reference](https://github.com/gitleaks/gitleaks#configuration)
- [Pre-commit Framework](https://pre-commit.com/)
- [BFG Repo-Cleaner](https://rtyley.github.io/bfg-repo-cleaner/)
