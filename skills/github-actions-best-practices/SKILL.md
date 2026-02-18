---
name: github-actions-best-practices
description: CI/CD con GitHub Actions siguiendo arquitectura modular y segura
version: 1.0.0
author: Knowledge Framework
tags: [github-actions, ci-cd, devops, automation, security]
---

# GitHub Actions Best Practices

Guía completa para crear workflows de GitHub Actions modulares, seguros y eficientes, con énfasis en jobs independientes, escaneo de seguridad y reutilización.

## Priority Levels

- **Critical**: Seguridad, disponibilidad del pipeline, bloqueo de deployments
- **High**: Performance, mantenibilidad, costo de runners
- **Medium**: Optimización y mejores prácticas
- **Low**: Conveniencia y DX (Developer Experience)

---

## 1. Metadata del Workflow (High)

### Header Completo Obligatorio

**Cada workflow DEBE comenzar con metadata completa:**

```yaml
# .github/workflows/ci-cd.yml

# ============================================================================
# Project: E-commerce Platform CI/CD Pipeline
# Author: DevOps Team <devops@company.com>
# Version: 2.1.0
# Created: 2026-01-15
# Last Updated: 2026-02-18
# Description: Complete CI/CD pipeline with security scanning, testing, and deployment
# 
# Triggers:
#   - Push to main/develop branches
#   - Pull requests to main
#   - Manual workflow dispatch
#
# Jobs:
#   1. lint - Code quality checks
#   2. security-scan - Vulnerability scanning
#   3. test - Unit and integration tests
#   4. build - Build artifacts
#   5. deploy-staging - Deploy to staging (auto)
#   6. deploy-production - Deploy to production (manual approval)
#
# Dependencies:
#   - AWS Account (123456789012)
#   - Docker Hub (company/repo)
#   - Secrets: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, DOCKER_TOKEN
#
# Maintainer: @devops-team
# Documentation: https://docs.company.com/ci-cd
# ============================================================================

name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
  workflow_dispatch:

env:
  # Global environment variables
  PROJECT_NAME: ecommerce-platform
  AWS_REGION: us-east-1
  NODE_VERSION: '20'
```

---

## 2. Jobs Independientes (Critical)

### Arquitectura de Jobs Paralelos

**Principio**: Cada job debe ser independiente y no depender de un solo job monolítico.

❌ **MAL: Single Job Monolítico**

```yaml
jobs:
  build-test-deploy:  # ❌ Todo en un solo job
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Lint
        run: npm run lint
      - name: Test
        run: npm test
      - name: Build
        run: npm run build
      - name: Deploy
        run: ./deploy.sh
```

✅ **BIEN: Jobs Independientes y Paralelos**

```yaml
jobs:
  # ============================================================================
  # Phase 1: Code Quality (Parallel)
  # ============================================================================
  
  lint:
    name: Code Linting
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run ESLint
        run: npm run lint
      
      - name: Run Prettier
        run: npm run format:check
  
  typecheck:
    name: TypeScript Type Checking
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Type check
        run: npm run typecheck
  
  # ============================================================================
  # Phase 2: Security Scanning (Parallel)
  # ============================================================================
  
  security-dependencies:
    name: Dependency Security Scan
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Run npm audit
        run: npm audit --audit-level=high
        continue-on-error: true
      
      - name: Run Snyk
        uses: snyk/actions/node@master
        env:
          SNYK_TOKEN: ${{ secrets.SNYK_TOKEN }}
        with:
          args: --severity-threshold=high
  
  security-code:
    name: Code Security Scan
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Run CodeQL Analysis
        uses: github/codeql-action/init@v2
        with:
          languages: javascript, typescript
      
      - name: Autobuild
        uses: github/codeql-action/autobuild@v2
      
      - name: Perform CodeQL Analysis
        uses: github/codeql-action/analyze@v2
  
  security-secrets:
    name: Secret Scanning
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: TruffleHog Scan
        uses: trufflesecurity/trufflehog@main
        with:
          path: ./
          base: main
          head: HEAD
  
  # ============================================================================
  # Phase 3: Testing (Parallel)
  # ============================================================================
  
  test-unit:
    name: Unit Tests
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run unit tests
        run: npm run test:unit -- --coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/coverage-final.json
          flags: unit
  
  test-integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run integration tests
        run: npm run test:integration
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test
  
  test-e2e:
    name: E2E Tests
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Install Playwright
        run: npx playwright install --with-deps
      
      - name: Run E2E tests
        run: npm run test:e2e
      
      - name: Upload test results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: playwright-report
          path: playwright-report/
  
  # ============================================================================
  # Phase 4: Build (Requires Phase 1-3 to pass)
  # ============================================================================
  
  build:
    name: Build Application
    runs-on: ubuntu-latest
    needs: [lint, typecheck, security-dependencies, security-code, test-unit, test-integration]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build application
        run: npm run build
        env:
          NODE_ENV: production
      
      - name: Upload build artifacts
        uses: actions/upload-artifact@v3
        with:
          name: build-artifacts
          path: dist/
          retention-days: 7
  
  # ============================================================================
  # Phase 5: Docker Build & Push (Parallel with build)
  # ============================================================================
  
  docker:
    name: Build & Push Docker Image
    runs-on: ubuntu-latest
    needs: [lint, typecheck, security-dependencies, test-unit]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      
      - name: Log in to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_TOKEN }}
      
      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: company/ecommerce-platform
          tags: |
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{version}}
            type=sha,prefix={{branch}}-
      
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=registry,ref=company/ecommerce-platform:buildcache
          cache-to: type=registry,ref=company/ecommerce-platform:buildcache,mode=max
      
      - name: Scan Docker image
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: company/ecommerce-platform:${{ github.sha }}
          format: 'sarif'
          output: 'trivy-results.sarif'
      
      - name: Upload Trivy results
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: 'trivy-results.sarif'
  
  # ============================================================================
  # Phase 6: Deploy Staging (Auto on main)
  # ============================================================================
  
  deploy-staging:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    needs: [build, docker]
    if: github.ref == 'refs/heads/main'
    environment:
      name: staging
      url: https://staging.company.com
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: ${{ env.AWS_REGION }}
      
      - name: Download build artifacts
        uses: actions/download-artifact@v3
        with:
          name: build-artifacts
          path: dist/
      
      - name: Deploy to staging
        run: |
          aws s3 sync dist/ s3://staging-company-ecommerce-frontend-aws
          aws cloudfront create-invalidation --distribution-id ${{ secrets.STAGING_CF_DISTRIBUTION_ID }} --paths "/*"
      
      - name: Run smoke tests
        run: npm run test:smoke
        env:
          BASE_URL: https://staging.company.com
  
  # ============================================================================
  # Phase 7: Deploy Production (Manual approval)
  # ============================================================================
  
  deploy-production:
    name: Deploy to Production
    runs-on: ubuntu-latest
    needs: [deploy-staging]
    if: github.ref == 'refs/heads/main'
    environment:
      name: production
      url: https://company.com
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: ${{ env.AWS_REGION }}
      
      - name: Download build artifacts
        uses: actions/download-artifact@v3
        with:
          name: build-artifacts
          path: dist/
      
      - name: Deploy to production
        run: |
          aws s3 sync dist/ s3://prod-company-ecommerce-frontend-aws
          aws cloudfront create-invalidation --distribution-id ${{ secrets.PROD_CF_DISTRIBUTION_ID }} --paths "/*"
      
      - name: Run smoke tests
        run: npm run test:smoke
        env:
          BASE_URL: https://company.com
      
      - name: Notify deployment
        uses: 8398a7/action-slack@v3
        with:
          status: ${{ job.status }}
          text: 'Production deployment completed'
          webhook_url: ${{ secrets.SLACK_WEBHOOK }}
        if: always()
```

---

## 3. Security Scanning Layers (Critical)

### Pipeline de Seguridad Multi-Capa

Implementa SIEMPRE múltiples capas de escaneo:

```yaml
# .github/workflows/security.yml

# ============================================================================
# Project: Security Scanning Pipeline
# Author: Security Team
# Version: 1.0.0
# Description: Multi-layer security scanning for all code and dependencies
# ============================================================================

name: Security Scanning

on:
  push:
    branches: [main, develop]
  pull_request:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM

jobs:
  # Layer 1: Dependency vulnerabilities
  dependency-scan:
    name: Dependency Vulnerability Scan
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Run npm audit
        run: npm audit --audit-level=moderate
        continue-on-error: true
      
      - name: OWASP Dependency Check
        uses: dependency-check/Dependency-Check_Action@main
        with:
          project: 'ecommerce-platform'
          path: '.'
          format: 'HTML'
          args: >
            --failOnCVSS 7
            --enableRetired
      
      - name: Upload dependency check results
        uses: actions/upload-artifact@v3
        with:
          name: dependency-check-report
          path: reports/
  
  # Layer 2: Secret scanning
  secret-scan:
    name: Secret & Credential Scan
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Gitleaks Scan
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      
      - name: TruffleHog Scan
        uses: trufflesecurity/trufflehog@main
        with:
          path: ./
          base: ${{ github.event.repository.default_branch }}
          head: HEAD
  
  # Layer 3: SAST (Static Application Security Testing)
  sast-scan:
    name: SAST Code Analysis
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Initialize CodeQL
        uses: github/codeql-action/init@v2
        with:
          languages: javascript, typescript
          queries: security-extended
      
      - name: Autobuild
        uses: github/codeql-action/autobuild@v2
      
      - name: Perform CodeQL Analysis
        uses: github/codeql-action/analyze@v2
      
      - name: Semgrep Scan
        uses: returntocorp/semgrep-action@v1
        with:
          config: p/security-audit
  
  # Layer 4: Container scanning
  container-scan:
    name: Container Security Scan
    runs-on: ubuntu-latest
    if: github.event_name == 'push'
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Docker image
        run: docker build -t test-image:${{ github.sha }} .
      
      - name: Trivy vulnerability scan
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: test-image:${{ github.sha }}
          format: 'sarif'
          output: 'trivy-results.sarif'
          severity: 'CRITICAL,HIGH'
      
      - name: Upload Trivy results
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: 'trivy-results.sarif'
      
      - name: Grype vulnerability scan
        uses: anchore/scan-action@v3
        with:
          image: test-image:${{ github.sha }}
          fail-build: true
          severity-cutoff: high
  
  # Layer 5: Infrastructure as Code scanning
  iac-scan:
    name: IaC Security Scan
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Checkov scan
        uses: bridgecrewio/checkov-action@master
        with:
          directory: terraform/
          framework: terraform
          output_format: sarif
          output_file_path: checkov-results.sarif
      
      - name: KICS scan
        uses: checkmarx/kics-github-action@v1.7
        with:
          path: terraform/
          output_path: kics-results/
          fail_on: high
      
      - name: Upload KICS results
        uses: actions/upload-artifact@v3
        with:
          name: kics-results
          path: kics-results/
  
  # Layer 6: License compliance
  license-scan:
    name: License Compliance Check
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
      
      - name: Install dependencies
        run: npm ci
      
      - name: License check
        run: npx license-checker --summary --onlyAllow "MIT;Apache-2.0;BSD-2-Clause;BSD-3-Clause;ISC"
```

---

## 4. Reusable Workflows (High)

### Crear Workflows Reutilizables para Repos Múltiples

**Cuando un workflow es usado por muchos repos**, crea un workflow reutilizable:

```yaml
# .github/workflows/reusable-ci.yml

# ============================================================================
# Project: Reusable CI Workflow
# Author: Platform Team
# Version: 3.0.0
# Description: Enterprise-grade reusable CI workflow for all Node.js projects
# 
# Usage:
#   See: https://docs.company.com/workflows/reusable-ci
#
# Inputs:
#   - node-version: Node.js version (default: 20)
#   - run-e2e: Run E2E tests (default: true)
#   - security-level: Security scan level (default: high)
#
# Secrets:
#   - SNYK_TOKEN: Snyk authentication token
#   - CODECOV_TOKEN: Codecov upload token
# ============================================================================

name: Reusable CI Workflow

on:
  workflow_call:
    inputs:
      node-version:
        description: 'Node.js version to use'
        required: false
        type: string
        default: '20'
      
      run-e2e:
        description: 'Run E2E tests'
        required: false
        type: boolean
        default: true
      
      security-level:
        description: 'Security scan severity level'
        required: false
        type: string
        default: 'high'
      
      working-directory:
        description: 'Working directory'
        required: false
        type: string
        default: '.'
    
    secrets:
      SNYK_TOKEN:
        required: true
      CODECOV_TOKEN:
        required: false
    
    outputs:
      build-version:
        description: 'Build version number'
        value: ${{ jobs.build.outputs.version }}

jobs:
  # Lint job
  lint:
    name: Lint Code
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ inputs.node-version }}
          cache: 'npm'
          cache-dependency-path: ${{ inputs.working-directory }}/package-lock.json
      
      - name: Install dependencies
        working-directory: ${{ inputs.working-directory }}
        run: npm ci
      
      - name: Run linters
        working-directory: ${{ inputs.working-directory }}
        run: |
          npm run lint
          npm run format:check
  
  # Security scan job
  security:
    name: Security Scan
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ inputs.node-version }}
          cache: 'npm'
      
      - name: Install dependencies
        working-directory: ${{ inputs.working-directory }}
        run: npm ci
      
      - name: Run Snyk
        uses: snyk/actions/node@master
        env:
          SNYK_TOKEN: ${{ secrets.SNYK_TOKEN }}
        with:
          args: --severity-threshold=${{ inputs.security-level }}
      
      - name: CodeQL Analysis
        uses: github/codeql-action/init@v2
        with:
          languages: javascript
      
      - name: Perform Analysis
        uses: github/codeql-action/analyze@v2
  
  # Test job
  test:
    name: Run Tests
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ inputs.node-version }}
          cache: 'npm'
      
      - name: Install dependencies
        working-directory: ${{ inputs.working-directory }}
        run: npm ci
      
      - name: Run unit tests
        working-directory: ${{ inputs.working-directory }}
        run: npm run test:unit -- --coverage
      
      - name: Upload coverage
        if: ${{ secrets.CODECOV_TOKEN != '' }}
        uses: codecov/codecov-action@v3
        with:
          token: ${{ secrets.CODECOV_TOKEN }}
          files: ${{ inputs.working-directory }}/coverage/coverage-final.json
  
  # E2E tests (conditional)
  e2e:
    name: E2E Tests
    runs-on: ubuntu-latest
    if: ${{ inputs.run-e2e }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ inputs.node-version }}
          cache: 'npm'
      
      - name: Install dependencies
        working-directory: ${{ inputs.working-directory }}
        run: npm ci
      
      - name: Install Playwright
        working-directory: ${{ inputs.working-directory }}
        run: npx playwright install --with-deps
      
      - name: Run E2E tests
        working-directory: ${{ inputs.working-directory }}
        run: npm run test:e2e
  
  # Build job
  build:
    name: Build Application
    runs-on: ubuntu-latest
    needs: [lint, security, test]
    outputs:
      version: ${{ steps.version.outputs.version }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ inputs.node-version }}
          cache: 'npm'
      
      - name: Install dependencies
        working-directory: ${{ inputs.working-directory }}
        run: npm ci
      
      - name: Build
        working-directory: ${{ inputs.working-directory }}
        run: npm run build
      
      - name: Extract version
        id: version
        working-directory: ${{ inputs.working-directory }}
        run: echo "version=$(node -p "require('./package.json').version")" >> $GITHUB_OUTPUT
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: build-artifacts
          path: ${{ inputs.working-directory }}/dist/
```

### Uso del Workflow Reutilizable

```yaml
# En otro repo: .github/workflows/ci.yml

name: CI

on: [push, pull_request]

jobs:
  ci:
    uses: company/workflows/.github/workflows/reusable-ci.yml@v3
    with:
      node-version: '20'
      run-e2e: true
      security-level: 'high'
    secrets:
      SNYK_TOKEN: ${{ secrets.SNYK_TOKEN }}
      CODECOV_TOKEN: ${{ secrets.CODECOV_TOKEN }}
```

---

## 5. Best Practices Checklist

### Antes de crear un workflow

- [ ] Header con metadata completa (Project, Author, Version, Description)
- [ ] Jobs independientes que pueden correr en paralelo
- [ ] Capas de security scanning implementadas
- [ ] Secrets nunca hardcodeados en el workflow
- [ ] Uso de `needs:` solo cuando hay dependencia real
- [ ] Artifacts subidos para jobs que los necesiten
- [ ] Timeouts configurados para evitar jobs colgados
- [ ] Matriz de testing para múltiples versiones/plataformas (si aplica)

### Security checklist

- [ ] Dependency scanning (npm audit, Snyk)
- [ ] Secret scanning (TruffleHog, Gitleaks)
- [ ] SAST (CodeQL, Semgrep)
- [ ] Container scanning (Trivy, Grype)
- [ ] IaC scanning (Checkov, KICS) si hay Terraform/CloudFormation
- [ ] License compliance check
- [ ] Permisos mínimos en `GITHUB_TOKEN`

### Performance checklist

- [ ] Cache habilitado para dependencias
- [ ] Jobs paralelos en lugar de secuenciales
- [ ] `continue-on-error` usado apropiadamente
- [ ] Artifacts con retention days configurado
- [ ] Conditional execution con `if:` para jobs costosos

---

## Recursos

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Reusable Workflows](https://docs.github.com/en/actions/using-workflows/reusing-workflows)
- [Security Hardening](https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions)
- [Workflow Syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions)

---

**Última actualización**: 2026-02-18  
**Mantenido por**: DevOps Team
