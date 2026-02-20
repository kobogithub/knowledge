---
name: security-owasp-zap
description: API security testing with OWASP ZAP for FastAPI endpoints
version: 1.0.0
tags:
  - security
  - api-testing
  - owasp
  - fastapi
  - penetration-testing
---

# security-owasp-zap

Testing de seguridad de APIs con OWASP ZAP (Zed Attack Proxy) enfocado en endpoints FastAPI y Supabase.

## Overview

OWASP ZAP es un proxy de seguridad que permite:
- **Baseline scan**: Deteccion pasiva de vulnerabilidades
- **Active scan**: Ataques automatizados (inyeccion, fuzzing)
- **API scan**: Importar OpenAPI/Swagger y escanear todos los endpoints
- **Authentication testing**: Validar flujos de auth
- **Automation**: CLI y Docker para CI/CD

## Instalacion

```bash
# Docker (recomendado - no requiere instalacion)
docker pull ghcr.io/zaproxy/zaproxy:stable

# macOS
brew install --cask owasp-zap

# Linux (snap)
sudo snap install zaproxy --classic

# Linux (manual)
wget https://github.com/zaproxy/zaproxy/releases/latest/download/ZAP_2.14.0_Linux.tar.gz
tar -xzf ZAP_2.14.0_Linux.tar.gz
```

## Modos de Escaneo

### 1. Baseline Scan (Pasivo - Rapido y Seguro)

El baseline scan solo observa responses, no ataca. Seguro para cualquier entorno.

```bash
# Scan basico contra API local
docker run --rm --network host ghcr.io/zaproxy/zaproxy:stable \
  zap-baseline.py \
  -t http://localhost:8000 \
  -r zap-report.html

# Con OpenAPI spec (mejor cobertura)
docker run --rm --network host \
  -v "${PWD}:/zap/wrk/:rw" \
  ghcr.io/zaproxy/zaproxy:stable \
  zap-baseline.py \
  -t http://localhost:8000 \
  -r zap-report.html \
  -z "-openapiurl http://localhost:8000/openapi.json"

# Con reglas custom (fail/warn/ignore)
docker run --rm --network host \
  -v "${PWD}:/zap/wrk/:rw" \
  ghcr.io/zaproxy/zaproxy:stable \
  zap-baseline.py \
  -t http://localhost:8000 \
  -c zap-rules.conf \
  -r zap-report.html
```

### 2. API Scan (Importar OpenAPI/Swagger)

```bash
# Scan de API usando OpenAPI spec
docker run --rm --network host \
  -v "${PWD}:/zap/wrk/:rw" \
  ghcr.io/zaproxy/zaproxy:stable \
  zap-api-scan.py \
  -t http://localhost:8000/openapi.json \
  -f openapi \
  -r zap-api-report.html

# Con formato JSON
docker run --rm --network host \
  -v "${PWD}:/zap/wrk/:rw" \
  ghcr.io/zaproxy/zaproxy:stable \
  zap-api-scan.py \
  -t http://localhost:8000/openapi.json \
  -f openapi \
  -J zap-api-report.json
```

### 3. Full Scan (Activo - Solo en Entornos de Test)

**PRECAUCION**: El full scan realiza ataques reales. Solo usar en entornos de test/staging.

```bash
# Full scan (incluye ataques activos)
docker run --rm --network host \
  -v "${PWD}:/zap/wrk/:rw" \
  ghcr.io/zaproxy/zaproxy:stable \
  zap-full-scan.py \
  -t http://localhost:8000 \
  -r zap-full-report.html
```

## Configuracion de Reglas

### zap-rules.conf

```conf
# Reglas de ZAP para FastAPI
# Formato: <rule-id> <action> (IGNORE, INFO, WARN, FAIL)

# === Headers de Seguridad ===
# Content-Security-Policy
10038 FAIL
# X-Content-Type-Options
10021 FAIL
# X-Frame-Options
10020 WARN
# Strict-Transport-Security
10035 FAIL
# Referrer-Policy
10049 WARN

# === Cookies ===
# Cookie without Secure flag
10011 FAIL
# Cookie without HttpOnly flag
10010 WARN
# Cookie without SameSite attribute
10054 WARN

# === Information Disclosure ===
# Server header disclosure
10036 WARN
# X-Powered-By header
10037 WARN
# Application error disclosure
90022 FAIL

# === Injection ===
# SQL Injection
40018 FAIL
# Cross-Site Scripting (Reflected)
40012 FAIL
# Cross-Site Scripting (Stored)
40014 FAIL
# OS Command Injection
90020 FAIL
# Path Traversal
6 FAIL

# === Authentication ===
# Authentication request identified
10111 INFO
# Session fixation
40013 FAIL

# === Low priority (no bloquear CI) ===
# Modern Web App detection
10109 IGNORE
# User Agent Fuzzer
10104 IGNORE
# Information disclosure in URL
10024 INFO
```

## Testing de Endpoints FastAPI

### Testing de Inyeccion SQL

```bash
# Crear script de automation ZAP
cat > zap-fastapi-test.py << 'SCRIPT'
#!/usr/bin/env python3
"""ZAP automation script for FastAPI API testing."""
import subprocess
import json
import sys

API_URL = sys.argv[1] if len(sys.argv) > 1 else "http://localhost:8000"

# Endpoints a testear con payloads de inyeccion
INJECTION_TESTS = [
    {
        "endpoint": "/api/search",
        "method": "GET",
        "params": {"q": "' OR 1=1--"},
        "description": "SQL injection in search"
    },
    {
        "endpoint": "/api/search",
        "method": "GET",
        "params": {"q": "'; DROP TABLE users;--"},
        "description": "SQL injection DROP TABLE"
    },
    {
        "endpoint": "/api/users",
        "method": "GET",
        "params": {"filter": "${7*7}"},
        "description": "Template injection"
    },
    {
        "endpoint": "/api/search",
        "method": "GET",
        "params": {"q": "../../../etc/passwd"},
        "description": "Path traversal"
    },
]

print("FastAPI Injection Tests:")
for test in INJECTION_TESTS:
    import urllib.request, urllib.parse
    params = urllib.parse.urlencode(test["params"])
    url = f"{API_URL}{test['endpoint']}?{params}"
    try:
        req = urllib.request.Request(url)
        response = urllib.request.urlopen(req, timeout=5)
        status = response.getcode()
        body = response.read().decode()
        # Si retorna 200 con datos de inyeccion, puede ser vulnerable
        if "error" in body.lower() or "sql" in body.lower() or "syntax" in body.lower():
            print(f"  POTENTIAL VULN: {test['description']}")
            print(f"    URL: {url}")
            print(f"    Status: {status}")
            print(f"    Response contains SQL error indicator")
        else:
            print(f"  OK: {test['description']} - Status {status}")
    except urllib.error.HTTPError as e:
        if e.code in (400, 403, 422):
            print(f"  SAFE: {test['description']} - Rejected with {e.code}")
        elif e.code == 500:
            print(f"  POTENTIAL VULN: {test['description']} - Server error 500")
        else:
            print(f"  CHECK: {test['description']} - Status {e.code}")
    except Exception as e:
        print(f"  ERROR: {test['description']} - {str(e)}")
SCRIPT

python3 zap-fastapi-test.py http://localhost:8000
```

### Testing de Headers de Seguridad

```bash
# Verificar security headers en FastAPI
curl -sI http://localhost:8000/api/health | grep -iE "^(content-security|x-content|x-frame|strict-transport|referrer-policy|x-xss|server|x-powered)"

# Script de verificacion completa
cat > check-security-headers.sh << 'SCRIPT'
#!/bin/bash
URL="${1:-http://localhost:8000}"
echo "Security Headers Check: $URL"
echo "================================"

HEADERS=$(curl -sI "$URL/api/health" 2>/dev/null)

check_header() {
    local header="$1"
    local required="$2"
    if echo "$HEADERS" | grep -qi "^$header:"; then
        VALUE=$(echo "$HEADERS" | grep -i "^$header:" | head -1)
        echo "  PASS: $VALUE"
    elif [ "$required" = "required" ]; then
        echo "  FAIL: $header not found (REQUIRED)"
    else
        echo "  WARN: $header not found (recommended)"
    fi
}

echo ""
echo "Required Headers:"
check_header "Content-Security-Policy" "required"
check_header "X-Content-Type-Options" "required"
check_header "Strict-Transport-Security" "required"
check_header "X-Frame-Options" "required"

echo ""
echo "Recommended Headers:"
check_header "Referrer-Policy" "recommended"
check_header "Permissions-Policy" "recommended"
check_header "X-XSS-Protection" "recommended"

echo ""
echo "Headers to REMOVE (information disclosure):"
if echo "$HEADERS" | grep -qi "^server:"; then
    echo "  WARN: Server header should be removed or genericized"
fi
if echo "$HEADERS" | grep -qi "^x-powered-by:"; then
    echo "  FAIL: X-Powered-By header exposes technology stack"
fi

echo ""
echo "CORS Check:"
CORS=$(curl -sI -H "Origin: https://evil.com" "$URL/api/health" 2>/dev/null | grep -i "access-control")
if [ -z "$CORS" ]; then
    echo "  OK: No CORS headers for unknown origin"
else
    echo "  CHECK: $CORS"
fi
SCRIPT

chmod +x check-security-headers.sh
bash check-security-headers.sh http://localhost:8000
```

### Testing de Autenticacion

```bash
# Test de auth endpoints
cat > test-auth-security.sh << 'SCRIPT'
#!/bin/bash
URL="${1:-http://localhost:8000}"
echo "Authentication Security Tests: $URL"
echo "======================================"

# Test 1: Acceso sin token a ruta protegida
echo ""
echo "Test 1: Unauthenticated access to protected endpoint"
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$URL/api/users/me")
if [ "$STATUS" = "401" ] || [ "$STATUS" = "403" ]; then
    echo "  PASS: Returned $STATUS (access denied)"
else
    echo "  FAIL: Returned $STATUS (expected 401/403)"
fi

# Test 2: Token invalido
echo ""
echo "Test 2: Invalid JWT token"
STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer invalid-token-here" \
    "$URL/api/users/me")
if [ "$STATUS" = "401" ] || [ "$STATUS" = "403" ]; then
    echo "  PASS: Returned $STATUS (invalid token rejected)"
else
    echo "  FAIL: Returned $STATUS (expected 401/403)"
fi

# Test 3: Token expirado (token de ejemplo con exp en el pasado)
echo ""
echo "Test 3: Expired JWT token"
EXPIRED_TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZXhwIjoxMDAwMDAwMDAwfQ.invalid"
STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer $EXPIRED_TOKEN" \
    "$URL/api/users/me")
if [ "$STATUS" = "401" ] || [ "$STATUS" = "403" ]; then
    echo "  PASS: Returned $STATUS (expired token rejected)"
else
    echo "  FAIL: Returned $STATUS (expected 401/403)"
fi

# Test 4: Brute force protection (rate limiting)
echo ""
echo "Test 4: Rate limiting on login endpoint"
FAIL_COUNT=0
for i in $(seq 1 20); do
    STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
        -X POST "$URL/auth/login" \
        -H "Content-Type: application/json" \
        -d '{"email":"test@test.com","password":"wrong"}')
    if [ "$STATUS" = "429" ]; then
        echo "  PASS: Rate limited after $i attempts (status 429)"
        FAIL_COUNT=-1
        break
    fi
done
if [ "$FAIL_COUNT" -ne -1 ]; then
    echo "  WARN: No rate limiting detected after 20 failed attempts"
fi

# Test 5: Login response no revela info
echo ""
echo "Test 5: Login error message doesn't reveal user existence"
RESPONSE=$(curl -s -X POST "$URL/auth/login" \
    -H "Content-Type: application/json" \
    -d '{"email":"nonexistent@test.com","password":"wrong"}')
if echo "$RESPONSE" | grep -qi "user not found\|no such user\|email not registered"; then
    echo "  FAIL: Response reveals user non-existence (user enumeration)"
else
    echo "  PASS: Generic error message (no user enumeration)"
fi
SCRIPT

chmod +x test-auth-security.sh
bash test-auth-security.sh http://localhost:8000
```

## Integracion con CI/CD (GitHub Actions)

### Baseline Scan en CI

```yaml
name: API Security Scan
on:
  push:
    branches: [main]
  pull_request:

jobs:
  zap-scan:
    runs-on: ubuntu-latest
    services:
      # Start your API service
      api:
        image: backend:latest
        ports:
          - 8000:8000
        env:
          DATABASE_URL: postgresql://test:test@postgres:5432/test
    steps:
      - uses: actions/checkout@v4

      - name: Wait for API to be ready
        run: |
          for i in $(seq 1 30); do
            if curl -s http://localhost:8000/api/health > /dev/null 2>&1; then
              echo "API is ready"
              break
            fi
            echo "Waiting for API... ($i/30)"
            sleep 2
          done

      - name: ZAP Baseline Scan
        uses: zaproxy/action-baseline@v0.12.0
        with:
          target: 'http://localhost:8000'
          rules_file_name: 'zap-rules.conf'
          cmd_options: '-z "-openapiurl http://localhost:8000/openapi.json"'

      - name: ZAP API Scan
        uses: zaproxy/action-api-scan@v0.7.0
        with:
          target: 'http://localhost:8000/openapi.json'
          format: openapi
          rules_file_name: 'zap-rules.conf'
```

### Full Scan en staging (scheduled)

```yaml
name: Full Security Scan (Staging)
on:
  schedule:
    - cron: '0 3 * * 1'  # Weekly on Monday at 3am
  workflow_dispatch:

jobs:
  zap-full-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: ZAP Full Scan
        uses: zaproxy/action-full-scan@v0.10.0
        with:
          target: 'https://staging.api.example.com'
          rules_file_name: 'zap-rules.conf'
          cmd_options: '-z "-openapiurl https://staging.api.example.com/openapi.json"'

      - name: Upload Report
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: zap-full-report
          path: report_html.html
```

## FastAPI Security Middleware (Recomendaciones)

### Security Headers Middleware

```python
# middleware/security.py
from fastapi import FastAPI
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import Response

class SecurityHeadersMiddleware(BaseHTTPMiddleware):
    """Add security headers to all responses."""

    async def dispatch(self, request: Request, call_next) -> Response:
        response = await call_next(request)

        # Prevent MIME type sniffing
        response.headers["X-Content-Type-Options"] = "nosniff"

        # Prevent clickjacking
        response.headers["X-Frame-Options"] = "DENY"

        # CSP - adjust based on your needs
        response.headers["Content-Security-Policy"] = (
            "default-src 'self'; "
            "script-src 'self'; "
            "style-src 'self' 'unsafe-inline'; "
            "img-src 'self' data:; "
            "connect-src 'self' https://*.supabase.co"
        )

        # HSTS
        response.headers["Strict-Transport-Security"] = (
            "max-age=31536000; includeSubDomains"
        )

        # Referrer Policy
        response.headers["Referrer-Policy"] = "strict-origin-when-cross-origin"

        # Remove server header
        if "server" in response.headers:
            del response.headers["server"]

        return response

# Usage in FastAPI app
app = FastAPI()
app.add_middleware(SecurityHeadersMiddleware)
```

### Rate Limiting

```python
# middleware/rate_limit.py
from slowapi import Limiter
from slowapi.util import get_remote_address

limiter = Limiter(key_func=get_remote_address)

# In routes
@app.post("/auth/login")
@limiter.limit("5/minute")  # 5 login attempts per minute
async def login(request: Request, credentials: LoginSchema):
    ...

@app.get("/api/search")
@limiter.limit("30/minute")  # 30 searches per minute
async def search(request: Request, q: str):
    ...
```

## OWASP Top 10 Checklist para FastAPI

```bash
# Reporte de OWASP Top 10 compliance
bd comments add task-id "[Security Agent] OWASP Top 10 (2021) Checklist:

A01 - Broken Access Control:
  - [ ] Auth middleware en todas las rutas protegidas
  - [ ] RBAC implementado correctamente
  - [ ] CORS configurado restrictivamente
  - [ ] Directory listing deshabilitado

A02 - Cryptographic Failures:
  - [ ] HTTPS enforced (HSTS)
  - [ ] Passwords hasheados con bcrypt/argon2
  - [ ] JWT signed con secret fuerte (256+ bits)
  - [ ] Sensitive data no en logs

A03 - Injection:
  - [ ] Parameterized queries (no raw SQL)
  - [ ] Input validation con Pydantic
  - [ ] Output encoding
  - [ ] No eval() or exec()

A04 - Insecure Design:
  - [ ] Rate limiting en auth endpoints
  - [ ] Account lockout despues de N intentos
  - [ ] Password policy enforced
  - [ ] Security headers configurados

A05 - Security Misconfiguration:
  - [ ] Debug mode OFF en production
  - [ ] Default credentials cambiados
  - [ ] Stack trace no expuesto a usuarios
  - [ ] Unnecessary features deshabilitados

A06 - Vulnerable Components:
  - [ ] Dependencias actualizadas (Trivy scan)
  - [ ] No dependencias con CVEs criticos
  - [ ] Base Docker images actualizadas

A07 - Auth Failures:
  - [ ] MFA disponible
  - [ ] Session management seguro
  - [ ] Token rotation implementado
  - [ ] Logout invalida tokens

A08 - Data Integrity:
  - [ ] Input validation en todos los endpoints
  - [ ] CSRF protection
  - [ ] Subresource integrity (SRI) para CDN

A09 - Logging & Monitoring:
  - [ ] Login failures logueados
  - [ ] Access control failures logueados
  - [ ] Alertas configuradas
  - [ ] Logs no contienen sensitive data

A10 - SSRF:
  - [ ] URL validation para requests externos
  - [ ] Allowlist de dominios
  - [ ] No user-controlled URLs sin validacion"
```

## Mejores Practicas

### DO

- Ejecutar baseline scan en cada PR (rapido y seguro)
- Usar OpenAPI spec para mejor cobertura de endpoints
- Configurar `zap-rules.conf` para ajustar severidades a tu proyecto
- Ejecutar full scan semanalmente en staging (con schedule)
- Implementar security headers middleware en FastAPI
- Testear auth endpoints con scripts automatizados
- Documentar findings y crear issues de remediacion

### DON'T

- Ejecutar full scan (active) contra produccion
- Ignorar alerts de headers de seguridad (son faciles de fixear)
- Saltear el scan en hotfixes
- Confiar solo en ZAP - complementar con Semgrep y Trivy
- Deshabilitar rate limiting "porque es molesto en development"
- Exponer stack traces en responses de produccion
- Usar CORS permisivo (`Access-Control-Allow-Origin: *`) en produccion

## Recursos

- [OWASP ZAP Documentation](https://www.zaproxy.org/docs/)
- [ZAP GitHub Actions](https://www.zaproxy.org/docs/docker/github-actions/)
- [OWASP Top 10 2021](https://owasp.org/Top10/)
- [FastAPI Security Best Practices](https://fastapi.tiangolo.com/tutorial/security/)
- [ZAP Automation Framework](https://www.zaproxy.org/docs/automate/)
- [OWASP API Security Top 10](https://owasp.org/API-Security/)
