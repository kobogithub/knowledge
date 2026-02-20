---
name: security-trivy
description: Container and dependency vulnerability scanning with Trivy
version: 1.0.0
tags:
  - security
  - containers
  - scanning
  - dependencies
  - vulnerabilities
---

# security-trivy

Escaneo de imagenes Docker y dependencias de aplicaciones con Trivy para deteccion de vulnerabilidades y misconfigurations.

## Overview

Trivy es un escaner de seguridad integral que detecta vulnerabilidades en:
- **Container images** (OS packages, language-specific packages)
- **Filesystem** (dependencias de Python, Node.js, Rust, Go)
- **IaC** (Dockerfile, Kubernetes manifests, Terraform)
- **Git repositories** (secrets embebidos)

## Instalacion

```bash
# macOS
brew install trivy

# Linux (Debian/Ubuntu)
sudo apt-get install -y wget apt-transport-https gnupg lsb-release
wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | gpg --dearmor | sudo tee /usr/share/keyrings/trivy.gpg > /dev/null
echo "deb [signed-by=/usr/share/keyrings/trivy.gpg] https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | sudo tee /etc/apt/sources.list.d/trivy.list
sudo apt-get update && sudo apt-get install trivy

# Docker (sin instalacion local)
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock aquasec/trivy:latest image <image-name>
```

## Escaneo de Imagenes Docker

### Scan basico

```bash
# Escanear imagen local
trivy image backend:latest

# Escanear imagen remota
trivy image ghcr.io/org/backend:v1.0

# Solo vulnerabilidades CRITICAL y HIGH
trivy image --severity CRITICAL,HIGH backend:latest

# Formato JSON para CI/CD
trivy image --format json --output trivy-results.json backend:latest

# Formato table (legible)
trivy image --format table backend:latest
```

### Scan con thresholds (para CI/CD gates)

```bash
# Fallar si hay CRITICAL vulnerabilities (exit code 1)
trivy image --exit-code 1 --severity CRITICAL backend:latest

# Fallar si hay CRITICAL o HIGH
trivy image --exit-code 1 --severity CRITICAL,HIGH backend:latest

# Ignorar vulnerabilidades sin fix disponible
trivy image --ignore-unfixed --severity CRITICAL,HIGH backend:latest
```

### Scan de imagenes multi-stage

```bash
# Escanear solo la imagen final (no intermediate stages)
docker build -t backend:latest .
trivy image backend:latest

# Escanear imagen base por separado
trivy image python:3.12-slim-bookworm
```

## Escaneo de Dependencias (Filesystem)

### Python (FastAPI)

```bash
# Escanear requirements.txt / pyproject.toml
trivy fs --scanners vuln .

# Solo Python dependencies
trivy fs --scanners vuln --skip-dirs node_modules .

# Escanear un archivo especifico
trivy fs requirements.txt
trivy fs pyproject.toml
trivy fs poetry.lock
```

### Node.js (Astro)

```bash
# Escanear package-lock.json / yarn.lock
trivy fs --scanners vuln .

# Solo Node dependencies
trivy fs package-lock.json
trivy fs yarn.lock
trivy fs pnpm-lock.yaml
```

### Escaneo completo del proyecto

```bash
# Escanear todo: vulnerabilities + secrets + misconfigurations
trivy fs --scanners vuln,secret,misconfig .

# Con formato detallado
trivy fs --scanners vuln,secret,misconfig --format json --output trivy-full.json .
```

## Escaneo de Misconfigurations

### Dockerfile

```bash
# Detectar malas practicas en Dockerfile
trivy config Dockerfile

# Ejemplo de findings:
# - Running as root
# - Using latest tag
# - No HEALTHCHECK instruction
# - Secrets in ENV variables
```

### Docker Compose

```bash
# Escanear docker-compose.yml
trivy config docker-compose.yml

# Findings tipicos:
# - Privileged containers
# - Host network mode
# - Writable root filesystem
# - Missing resource limits
```

### Kubernetes / Terraform

```bash
# Escanear manifests de k8s
trivy config k8s/

# Escanear Terraform
trivy config terraform/
```

## Integracion con CI/CD (GitHub Actions)

### Workflow basico

```yaml
name: Security Scan
on:
  push:
    branches: [main]
  pull_request:

jobs:
  trivy-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker image
        run: docker build -t app:${{ github.sha }} .

      - name: Trivy image scan
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: 'app:${{ github.sha }}'
          format: 'sarif'
          output: 'trivy-results.sarif'
          severity: 'CRITICAL,HIGH'
          exit-code: '1'

      - name: Upload Trivy scan results
        uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: 'trivy-results.sarif'

      - name: Trivy filesystem scan
        uses: aquasecurity/trivy-action@master
        with:
          scan-type: 'fs'
          scan-ref: '.'
          severity: 'CRITICAL,HIGH'
          exit-code: '1'
```

### Workflow con cache

```yaml
- name: Trivy cache
  uses: actions/cache@v4
  with:
    path: ~/.cache/trivy
    key: trivy-db-${{ github.run_id }}
    restore-keys: trivy-db-

- name: Trivy scan
  uses: aquasecurity/trivy-action@master
  with:
    image-ref: 'app:latest'
    cache-dir: ~/.cache/trivy
```

## Supabase-Specific Scanning

### Escanear Supabase Docker images

```bash
# Si usas Supabase self-hosted
trivy image supabase/postgres:15.1.1.41
trivy image supabase/gotrue:v2.132.3
trivy image supabase/storage-api:v0.46.4
trivy image supabase/realtime:v2.25.50

# Escanear tu docker-compose de Supabase
trivy config docker/supabase/docker-compose.yml
```

### Escanear dependencias de Supabase client

```bash
# Python - supabase-py
trivy fs requirements.txt  # Incluye supabase, postgrest, gotrue

# Node.js - @supabase/supabase-js
trivy fs package-lock.json  # Incluye @supabase/supabase-js
```

## Ignorar Vulnerabilidades (Allowlist)

### .trivyignore

```bash
# Crear archivo .trivyignore en la raiz del proyecto
cat > .trivyignore << 'EOF'
# Vulnerabilidades aceptadas con justificacion
CVE-2024-12345  # Solo afecta test dependencies, no production
CVE-2024-67890  # False positive - no usamos la funcion afectada

# Expirar allowlist (requiere revision periodica)
# Revisar cada 30 dias
EOF
```

### Trivy config file

```yaml
# trivy.yaml
severity:
  - CRITICAL
  - HIGH

vulnerability:
  type:
    - os
    - library

ignore-unfixed: true

exit-code: 1

format: table
```

## Reportes

### Generar reporte HTML

```bash
trivy image --format template --template "@contrib/html.tpl" -o trivy-report.html backend:latest
```

### Generar reporte para Beads

```bash
# Script para generar reporte compatible con bd comments
trivy image --format json backend:latest | python3 -c "
import json, sys
data = json.load(sys.stdin)
for result in data.get('Results', []):
    target = result['Target']
    vulns = result.get('Vulnerabilities', [])
    if vulns:
        print(f'Target: {target}')
        for v in vulns:
            print(f'  - {v[\"Severity\"]}: {v[\"VulnerabilityID\"]} - {v[\"PkgName\"]} {v.get(\"InstalledVersion\", \"\")}')
            if v.get('FixedVersion'):
                print(f'    Fix: upgrade to {v[\"FixedVersion\"]}')
"
```

## Severity Thresholds

### Politica recomendada

| Severity | CI/CD Gate | Action Required | SLA |
|----------|-----------|-----------------|-----|
| CRITICAL | Block deploy | Fix immediately | 24h |
| HIGH | Block deploy | Fix before release | 7 days |
| MEDIUM | Warning | Fix in next sprint | 30 days |
| LOW | Info | Backlog | Best effort |

### Excepciones

- **Test dependencies**: MEDIUM y LOW pueden ignorarse si solo afectan dev/test
- **No fix available**: Documentar y monitorear, no bloquear
- **False positives**: Agregar a .trivyignore con justificacion

## Mejores Practicas

### DO

- Escanear en cada PR y antes de cada deploy
- Mantener base images actualizadas (monthly minimum)
- Usar `--ignore-unfixed` para no bloquear por vulnerabilidades sin fix
- Cachear la base de datos de Trivy en CI para velocidad
- Generar reportes SARIF para integracion con GitHub Security tab
- Documentar excepciones en .trivyignore con justificacion
- Escanear tanto la imagen como el filesystem del proyecto

### DON'T

- Ignorar CRITICAL vulnerabilities sin justificacion
- Usar `--skip-db-update` en production scans (puede dar false negatives)
- Confiar solo en image scanning - tambien escanear filesystem
- Olvidar escanear imagenes base por separado
- Dejar .trivyignore sin revisar por mas de 30 dias

## Recursos

- [Trivy Documentation](https://aquasecurity.github.io/trivy/)
- [Trivy GitHub Action](https://github.com/aquasecurity/trivy-action)
- [Trivy Vulnerability Database](https://github.com/aquasecurity/trivy-db)
- [Trivy Policy Reference](https://aquasecurity.github.io/trivy/latest/docs/scanner/misconfiguration/policy/builtin/)
