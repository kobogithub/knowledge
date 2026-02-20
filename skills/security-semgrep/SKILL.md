---
name: security-semgrep
description: Static Application Security Testing (SAST) with Semgrep for FastAPI and Astro
version: 1.0.0
tags:
  - security
  - sast
  - code-analysis
  - fastapi
  - astro
  - supabase
---

# security-semgrep

Analisis estatico de seguridad (SAST) con Semgrep para detectar patrones inseguros en FastAPI, Astro y Supabase.

## Overview

Semgrep es una herramienta de analisis estatico rapida y configurable que permite:
- Detectar vulnerabilidades de seguridad con reglas predefinidas y custom
- Ejecutar en CI/CD con resultados deterministas (sin false positives por diseno)
- Crear reglas personalizadas con sintaxis intuitiva (pattern matching)
- Soportar 30+ lenguajes (Python, JavaScript, TypeScript, etc.)

## Instalacion

```bash
# pip (recomendado)
pip install semgrep

# Homebrew
brew install semgrep

# Docker
docker run --rm -v "${PWD}:/src" semgrep/semgrep semgrep --config auto /src
```

## Escaneo Basico

### Scan con reglas predefinidas

```bash
# Auto-detect: usa las mejores reglas para tu proyecto
semgrep --config auto .

# Solo reglas de seguridad
semgrep --config p/security-audit .

# OWASP Top 10
semgrep --config p/owasp-top-ten .

# Reglas especificas de Python
semgrep --config p/python .

# Reglas de JavaScript/TypeScript
semgrep --config p/javascript .
semgrep --config p/typescript .
```

### Scan con severity filtering

```bash
# Solo HIGH y CRITICAL
semgrep --config auto --severity ERROR .

# Solo MEDIUM+
semgrep --config auto --severity WARNING .

# Formato JSON para CI/CD
semgrep --config auto --json --output semgrep-results.json .

# Formato SARIF para GitHub
semgrep --config auto --sarif --output semgrep-results.sarif .
```

## Reglas Custom para FastAPI

### SQL Injection en FastAPI

```yaml
# .semgrep/rules/fastapi-sql-injection.yml
rules:
  - id: fastapi-raw-sql-query
    patterns:
      - pattern: |
          $CURSOR.execute(f"...", ...)
      - pattern-not: |
          $CURSOR.execute("...", ($PARAMS, ...))
    message: >
      Raw SQL query detected. Use parameterized queries to prevent SQL injection.
      Fix: cursor.execute("SELECT * FROM table WHERE id = %s", (id,))
    languages: [python]
    severity: ERROR
    metadata:
      category: security
      owasp: "A03:2021 - Injection"
      cwe: "CWE-89: SQL Injection"

  - id: fastapi-string-format-sql
    patterns:
      - pattern: |
          $QUERY = f"...SELECT...{$VAR}..."
      - pattern-inside: |
          async def $FUNC(...):
              ...
    message: >
      String formatting in SQL query. Use parameterized queries or Supabase client.
    languages: [python]
    severity: ERROR
    metadata:
      category: security
      owasp: "A03:2021 - Injection"

  - id: fastapi-raw-sql-text
    pattern: |
      from sqlalchemy import text
      ...
      text(f"...${{$VAR}}...")
    message: >
      SQLAlchemy text() with f-string is vulnerable to SQL injection.
      Use text("... :param ...").bindparams(param=value) instead.
    languages: [python]
    severity: ERROR
```

### Authentication Bypass en FastAPI

```yaml
# .semgrep/rules/fastapi-auth-bypass.yml
rules:
  - id: fastapi-missing-auth-dependency
    patterns:
      - pattern: |
          @$APP.$METHOD("$PATH")
          async def $FUNC($PARAMS):
              ...
      - pattern-not: |
          @$APP.$METHOD("$PATH")
          async def $FUNC(..., current_user: ... = Depends($AUTH), ...):
              ...
      - metavariable-regex:
          metavariable: $PATH
          regex: ".*/admin/.*|.*/users/.*|.*/settings/.*"
    message: >
      Endpoint with sensitive path missing authentication dependency.
      Add: current_user: User = Depends(get_current_user)
    languages: [python]
    severity: ERROR
    metadata:
      category: security
      owasp: "A01:2021 - Broken Access Control"

  - id: fastapi-no-permission-check
    patterns:
      - pattern: |
          @$APP.$METHOD("/admin/...")
          async def $FUNC(...):
              ...
      - pattern-not: |
          @$APP.$METHOD("/admin/...")
          async def $FUNC(...):
              ...
              if not $USER.is_admin:
                  ...
    message: >
      Admin endpoint without permission check. Verify user has admin role.
    languages: [python]
    severity: WARNING
```

### SSRF en FastAPI

```yaml
# .semgrep/rules/fastapi-ssrf.yml
rules:
  - id: fastapi-ssrf-requests
    patterns:
      - pattern: |
          requests.get($URL, ...)
      - pattern-not: |
          requests.get("https://api.supabase.co/...", ...)
      - pattern-inside: |
          async def $FUNC(..., $REQ: Request, ...):
              ...
              $URL = $REQ.$FIELD
              ...
    message: >
      Potential SSRF: HTTP request using user-controlled URL.
      Validate URL against allowlist before making request.
    languages: [python]
    severity: ERROR
    metadata:
      category: security
      owasp: "A10:2021 - SSRF"
      cwe: "CWE-918: Server-Side Request Forgery"

  - id: fastapi-ssrf-httpx
    pattern: |
      httpx.$METHOD($USER_INPUT, ...)
    message: >
      Potential SSRF with httpx. Validate URL before making external request.
    languages: [python]
    severity: WARNING
```

### Secrets Hardcoded en FastAPI

```yaml
# .semgrep/rules/fastapi-hardcoded-secrets.yml
rules:
  - id: fastapi-hardcoded-jwt-secret
    pattern: |
      $SECRET = "..."
      ...
      jwt.encode(..., $SECRET, ...)
    message: >
      Hardcoded JWT secret. Use environment variable: os.environ["JWT_SECRET"]
    languages: [python]
    severity: ERROR
    metadata:
      category: security
      cwe: "CWE-798: Hardcoded Credentials"

  - id: fastapi-hardcoded-db-url
    patterns:
      - pattern: |
          $URL = "postgresql://..."
      - pattern-not: |
          $URL = os.environ[...]
      - pattern-not: |
          $URL = os.getenv(...)
    message: >
      Hardcoded database URL. Use environment variable.
    languages: [python]
    severity: ERROR

  - id: fastapi-debug-mode-production
    pattern: |
      app = FastAPI(..., debug=True, ...)
    message: >
      Debug mode enabled. Disable in production: debug=False or use env var.
    languages: [python]
    severity: WARNING
```

## Reglas Custom para Astro

### XSS en Astro Components

```yaml
# .semgrep/rules/astro-xss.yml
rules:
  - id: astro-set-html-xss
    pattern: |
      <$TAG set:html={$VAR} />
    message: >
      Potential XSS: set:html renders raw HTML without sanitization.
      Sanitize input with DOMPurify or use set:text instead.
    languages: [html]
    severity: WARNING
    metadata:
      category: security
      owasp: "A03:2021 - Injection"
      cwe: "CWE-79: XSS"

  - id: astro-innerHTML-xss
    pattern: |
      $EL.innerHTML = $VAR
    message: >
      Potential XSS: innerHTML assignment without sanitization.
      Use textContent or sanitize with DOMPurify.
    languages: [javascript, typescript]
    severity: ERROR

  - id: astro-document-write
    pattern: |
      document.write(...)
    message: >
      document.write() is dangerous and can lead to XSS. Use DOM APIs instead.
    languages: [javascript, typescript]
    severity: ERROR
```

### CSP y Security Headers en Astro

```yaml
# .semgrep/rules/astro-security-headers.yml
rules:
  - id: astro-inline-script
    pattern: |
      <script>
        ...
      </script>
    message: >
      Inline script detected. For strict CSP, use external script files
      or add nonce/hash to Content-Security-Policy.
    languages: [html]
    severity: INFO

  - id: astro-eval-usage
    pattern: |
      eval(...)
    message: >
      eval() is dangerous and blocked by strict CSP. Use safer alternatives.
    languages: [javascript, typescript]
    severity: ERROR
```

## Reglas Custom para Supabase Client

### Supabase Security Patterns

```yaml
# .semgrep/rules/supabase-security.yml
rules:
  - id: supabase-service-role-frontend
    patterns:
      - pattern: |
          createClient($URL, $KEY, ...)
      - metavariable-regex:
          metavariable: $KEY
          regex: ".*service_role.*|.*SERVICE_ROLE.*"
      - pattern-inside: |
          # In frontend/client-side code
          ...
    message: >
      CRITICAL: service_role key used in client code. This key bypasses RLS!
      Use the anon key for client-side: NEXT_PUBLIC_SUPABASE_ANON_KEY
    languages: [javascript, typescript, python]
    severity: ERROR
    metadata:
      category: security
      impact: critical

  - id: supabase-rpc-without-auth
    pattern: |
      supabase.rpc($FUNC, ...)
    message: >
      Supabase RPC call detected. Ensure the function has proper security definer
      settings and validates auth in PostgreSQL.
    languages: [javascript, typescript, python]
    severity: INFO

  - id: supabase-storage-public-upload
    pattern: |
      supabase.storage.from($BUCKET).upload($PATH, $FILE, ...)
    message: >
      Storage upload detected. Verify bucket has proper RLS policies
      and file type/size restrictions.
    languages: [javascript, typescript, python]
    severity: INFO

  - id: supabase-anon-key-hardcoded
    patterns:
      - pattern: |
          $KEY = "eyJ..."
      - metavariable-regex:
          metavariable: $KEY
          regex: "eyJ[A-Za-z0-9_-]+\\.[A-Za-z0-9_-]+\\.[A-Za-z0-9_-]+"
    message: >
      Hardcoded Supabase key detected. Use environment variables.
    languages: [javascript, typescript, python]
    severity: WARNING
```

## Integracion con CI/CD (GitHub Actions)

```yaml
name: SAST Scan
on:
  push:
    branches: [main]
  pull_request:

jobs:
  semgrep:
    runs-on: ubuntu-latest
    container:
      image: semgrep/semgrep
    steps:
      - uses: actions/checkout@v4

      - name: Run Semgrep (predefined rules)
        run: semgrep --config auto --sarif --output semgrep.sarif .

      - name: Run Semgrep (custom rules)
        run: semgrep --config .semgrep/rules/ --sarif --output semgrep-custom.sarif .

      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: semgrep.sarif

      - name: Fail on HIGH findings
        run: semgrep --config auto --severity ERROR --error .
```

## Configuracion del Proyecto

### .semgrepignore

```bash
# Ignorar archivos que no necesitan escaneo
node_modules/
.venv/
venv/
dist/
build/
*.min.js
*.bundle.js
tests/fixtures/
__pycache__/
.git/
```

### semgrep.yml (config global)

```yaml
# .semgrep.yml
rules:
  - id: custom-rule-example
    pattern: |
      print(...)
    paths:
      exclude:
        - tests/
        - scripts/
    message: "Use logging instead of print"
    languages: [python]
    severity: INFO
```

## Reporte de Findings

### Formato para Beads

```bash
# Generar reporte para bd comments
semgrep --config auto --json . | python3 -c "
import json, sys
data = json.load(sys.stdin)
results = data.get('results', [])
by_severity = {}
for r in results:
    sev = r['extra']['severity']
    by_severity.setdefault(sev, []).append(r)

print('[Security Agent] Semgrep SAST Report:')
print(f'Total findings: {len(results)}')
for sev in ['ERROR', 'WARNING', 'INFO']:
    findings = by_severity.get(sev, [])
    if findings:
        print(f'\n{sev} ({len(findings)}):')
        for f in findings:
            print(f'  - {f[\"check_id\"]}: {f[\"path\"]}:{f[\"start\"][\"line\"]}')
            print(f'    {f[\"extra\"][\"message\"][:100]}')
"
```

## Mejores Practicas

### DO

- Ejecutar en cada PR con `--error` para bloquear merges con HIGH findings
- Mantener reglas custom en `.semgrep/rules/` versionadas en git
- Usar rulesets oficiales como base: `p/python`, `p/owasp-top-ten`, `p/security-audit`
- Crear reglas especificas para patrones de tu proyecto (Supabase, FastAPI)
- Revisar findings regularmente y ajustar reglas para reducir false positives
- Documentar excepciones con `# nosemgrep: rule-id` y justificacion

### DON'T

- Ignorar findings de severity ERROR sin justificacion
- Usar `# nosemgrep` sin comentario explicando por que
- Crear reglas demasiado genericas que generen muchos false positives
- Olvidar escanear tanto backend (Python) como frontend (JS/TS)
- Depender solo de reglas predefinidas - los patrones custom son cruciales
- Saltear el escaneo en hotfixes - son los mas propensos a introducir vulnerabilidades

## Recursos

- [Semgrep Documentation](https://semgrep.dev/docs/)
- [Semgrep Rules Registry](https://semgrep.dev/r)
- [Semgrep Playground](https://semgrep.dev/playground)
- [Writing Custom Rules](https://semgrep.dev/docs/writing-rules/overview/)
- [OWASP Top 10 Ruleset](https://semgrep.dev/p/owasp-top-ten)
