---
name: github-actions-best-practices
description: GitHub Actions CI/CD best practices, workflow patterns, security and optimization
version: 1.0.0
tags:
  - best-practices
  - github-actions
  - ci-cd
  - devops
  - automation
---

# github-actions-best-practices

Mejores practicas para GitHub Actions: workflows CI/CD, caching, seguridad, matrices de testing y patrones de deployment.

## Overview

GitHub Actions se usa para:
- **CI**: Lint, test, type checking en cada push/PR
- **CD**: Build y deploy automatizado a staging/production
- **Automation**: Release tagging, changelog, dependency updates
- **Security**: Scanning de vulnerabilidades, SAST, secret detection
- **Scheduled tasks**: Backups, cleanup, health checks periodicos

## Estructura de Workflows

### Ubicacion y naming

```
.github/
├── workflows/
│   ├── ci.yml              # CI en push/PR
│   ├── deploy.yml          # Deploy a produccion
│   ├── release.yml         # Release automation
│   └── scheduled.yml       # Tareas programadas
├── actions/
│   └── setup-env/          # Composite actions reutilizables
│       └── action.yml
└── CODEOWNERS
```

## CI Workflow Completo

### Python + FastAPI

```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

# Cancelar runs anteriores del mismo PR
concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

permissions:
  contents: read

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: astral-sh/setup-uv@v4
        with:
          version: "latest"

      - name: Install dependencies
        run: uv sync --all-extras

      - name: Lint
        run: uv run ruff check .

      - name: Format check
        run: uv run ruff format --check .

      - name: Type check
        run: uv run mypy src/

  test:
    runs-on: ubuntu-latest
    needs: lint
    services:
      postgres:
        image: postgres:17-alpine
        env:
          POSTGRES_DB: test_db
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U postgres"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - uses: astral-sh/setup-uv@v4
        with:
          version: "latest"

      - name: Install dependencies
        run: uv sync --all-extras

      - name: Run tests
        run: uv run pytest --cov --cov-report=xml -v
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test_db

      - name: Upload coverage
        if: github.event_name == 'pull_request'
        uses: codecov/codecov-action@v4
        with:
          file: coverage.xml
          token: ${{ secrets.CODECOV_TOKEN }}

  build:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker image
        run: docker build -t myapp:${{ github.sha }} .

      - name: Run Trivy scan
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: myapp:${{ github.sha }}
          severity: HIGH,CRITICAL
          exit-code: 1
```

### Rust

```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

permissions:
  contents: read

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo fmt -- --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Tests
        run: cargo test --all-features

      - name: Build release
        run: cargo build --release
```

### Node.js (Astro)

```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: npm

      - name: Install dependencies
        run: npm ci

      - name: Lint
        run: npm run lint

      - name: Type check
        run: npm run check

      - name: Build
        run: npm run build

      - name: Tests
        run: npm test
```

## Matrix Testing

### Multiple versiones/OS

```yaml
jobs:
  test:
    strategy:
      fail-fast: false     # No cancelar si uno falla
      matrix:
        os: [ubuntu-latest, macos-latest]
        python-version: ["3.11", "3.12", "3.13"]
        exclude:
          - os: macos-latest
            python-version: "3.11"
        include:
          - os: ubuntu-latest
            python-version: "3.12"
            coverage: true

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}

      - name: Run tests
        run: pytest -v

      - name: Upload coverage
        if: matrix.coverage
        uses: codecov/codecov-action@v4
```

## Caching

### Cache de dependencias

```yaml
# Python con uv — setup-uv cachea automaticamente
- uses: astral-sh/setup-uv@v4
  with:
    version: "latest"
    # Cache is enabled by default

# Node.js — setup-node con cache
- uses: actions/setup-node@v4
  with:
    node-version: 22
    cache: npm

# Rust — rust-cache maneja todo
- uses: Swatinem/rust-cache@v2
```

### Cache manual

```yaml
# Cache custom para cualquier cosa
- uses: actions/cache@v4
  with:
    path: |
      ~/.cache/pre-commit
      .mypy_cache
    key: misc-${{ runner.os }}-${{ hashFiles('.pre-commit-config.yaml') }}
    restore-keys: |
      misc-${{ runner.os }}-

# Cache con restore y save separados (para cache parcial)
- uses: actions/cache/restore@v4
  id: cache-restore
  with:
    path: ./build-cache
    key: build-${{ hashFiles('src/**') }}
    restore-keys: build-

# ... build steps ...

- uses: actions/cache/save@v4
  if: steps.cache-restore.outputs.cache-hit != 'true'
  with:
    path: ./build-cache
    key: build-${{ hashFiles('src/**') }}
```

## Artifacts

```yaml
# Subir artifacts
- uses: actions/upload-artifact@v4
  with:
    name: build-output
    path: dist/
    retention-days: 7

# Descargar en otro job
- uses: actions/download-artifact@v4
  with:
    name: build-output
    path: dist/
```

## Composite Actions (Reutilizables)

### Crear una composite action

```yaml
# .github/actions/setup-env/action.yml
name: Setup Environment
description: Install dependencies and setup the development environment

inputs:
  python-version:
    description: Python version to use
    default: "3.12"
  install-dev:
    description: Install dev dependencies
    default: "true"

runs:
  using: composite
  steps:
    - uses: astral-sh/setup-uv@v4
      with:
        version: "latest"

    - name: Install dependencies
      shell: bash
      run: |
        if [ "${{ inputs.install-dev }}" = "true" ]; then
          uv sync --all-extras
        else
          uv sync --no-dev
        fi
```

### Usar la composite action

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./.github/actions/setup-env
        with:
          python-version: "3.12"
      - run: uv run pytest
```

## Reusable Workflows

### Definir workflow reutilizable

```yaml
# .github/workflows/reusable-deploy.yml
name: Deploy
on:
  workflow_call:
    inputs:
      environment:
        required: true
        type: string
      image-tag:
        required: true
        type: string
    secrets:
      deploy-key:
        required: true

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: ${{ inputs.environment }}
    steps:
      - uses: actions/checkout@v4

      - name: Deploy
        run: |
          echo "Deploying ${{ inputs.image-tag }} to ${{ inputs.environment }}"
        env:
          DEPLOY_KEY: ${{ secrets.deploy-key }}
```

### Llamar workflow reutilizable

```yaml
# .github/workflows/deploy-staging.yml
name: Deploy Staging
on:
  push:
    branches: [main]

jobs:
  ci:
    uses: ./.github/workflows/ci.yml

  deploy:
    needs: ci
    uses: ./.github/workflows/reusable-deploy.yml
    with:
      environment: staging
      image-tag: ${{ github.sha }}
    secrets:
      deploy-key: ${{ secrets.STAGING_DEPLOY_KEY }}
```

## Deployment Patterns

### Deploy con environments y approvals

```yaml
name: Deploy
on:
  push:
    branches: [main]
  workflow_dispatch:
    inputs:
      environment:
        type: choice
        options: [staging, production]
        default: staging

jobs:
  deploy-staging:
    runs-on: ubuntu-latest
    environment: staging
    steps:
      - uses: actions/checkout@v4
      - name: Deploy to staging
        run: ./scripts/deploy.sh staging

  deploy-production:
    runs-on: ubuntu-latest
    needs: deploy-staging
    environment: production  # Requiere approval manual en Settings
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - name: Deploy to production
        run: ./scripts/deploy.sh production
```

### Docker build + push

```yaml
jobs:
  build-push:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - uses: actions/checkout@v4

      - uses: docker/setup-buildx-action@v3

      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - uses: docker/build-push-action@v6
        with:
          context: .
          push: true
          tags: |
            ghcr.io/${{ github.repository }}:${{ github.sha }}
            ghcr.io/${{ github.repository }}:latest
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

## Release Automation

```yaml
name: Release
on:
  push:
    tags:
      - "v*"

permissions:
  contents: write

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Build
        run: cargo build --release

      - name: Create release
        uses: softprops/action-gh-release@v2
        with:
          generate_release_notes: true
          files: |
            target/release/mycli
```

### Multi-platform binary release

```yaml
jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: mycli-linux-amd64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: mycli-macos-arm64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: mycli-macos-amd64

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - run: cargo build --release --target ${{ matrix.target }}

      - uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: target/${{ matrix.target }}/release/mycli
```

## Seguridad

### Permisos minimos

```yaml
# A nivel de workflow — SIEMPRE especificar
permissions:
  contents: read

# A nivel de job — override si necesario
jobs:
  deploy:
    permissions:
      contents: read
      packages: write
      id-token: write  # Para OIDC
```

### Pinear actions por SHA

```yaml
# Mal — tag mutable, puede ser comprometido
- uses: actions/checkout@v4

# Bien — SHA inmutable (verificar que corresponde a v4)
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4.1.1
```

### Secrets seguros

```yaml
# NUNCA hacer esto — expone el secret en logs
- run: echo ${{ secrets.API_KEY }}

# NUNCA pasar secrets en URLs
- run: curl https://api.example.com?key=${{ secrets.API_KEY }}

# Bien — como variable de entorno
- run: ./deploy.sh
  env:
    API_KEY: ${{ secrets.API_KEY }}

# Cuidado con pull_request de forks — no tienen acceso a secrets
# Usar pull_request_target con restricciones para forks
```

### OIDC para cloud providers (sin secrets)

```yaml
permissions:
  id-token: write
  contents: read

steps:
  - uses: aws-actions/configure-aws-credentials@v4
    with:
      role-to-assume: arn:aws:iam::123456789:role/github-actions
      aws-region: us-east-1

  # No necesita AWS_ACCESS_KEY_ID ni AWS_SECRET_ACCESS_KEY
```

## Scheduled Workflows

```yaml
name: Scheduled Tasks
on:
  schedule:
    # Cron syntax: minuto hora dia-mes mes dia-semana
    - cron: "0 6 * * 1"    # Lunes a las 6 AM UTC
    - cron: "0 */6 * * *"  # Cada 6 horas

  # Tambien permitir ejecucion manual
  workflow_dispatch:

jobs:
  dependency-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check for outdated dependencies
        run: uv pip list --outdated

  cleanup:
    runs-on: ubuntu-latest
    steps:
      - name: Delete old artifacts
        uses: actions/github-script@v7
        with:
          script: |
            const thirtyDaysAgo = new Date();
            thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
            // cleanup logic
```

## Conditional Execution

```yaml
jobs:
  test:
    # Solo en PRs
    if: github.event_name == 'pull_request'

  deploy:
    # Solo en push a main, no en PRs
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'

  release:
    # Solo en tags
    if: startsWith(github.ref, 'refs/tags/v')

steps:
  - name: Notify on failure
    if: failure()
    run: echo "Build failed!"

  - name: Only on success
    if: success()
    run: echo "All good!"

  # Path filters — solo ejecutar si cambiaron ciertos archivos
  # Alternativa: usar paths en on.push/on.pull_request
  - uses: dorny/paths-filter@v3
    id: changes
    with:
      filters: |
        backend:
          - 'src/**'
          - 'pyproject.toml'
        frontend:
          - 'frontend/**'

  - name: Backend tests
    if: steps.changes.outputs.backend == 'true'
    run: uv run pytest
```

## Debugging

```yaml
# Habilitar debug logging
# Agregar secret ACTIONS_RUNNER_DEBUG = true

steps:
  # Dump contexto completo
  - name: Debug context
    if: runner.debug == '1'
    run: |
      echo "Event: ${{ github.event_name }}"
      echo "Ref: ${{ github.ref }}"
      echo "SHA: ${{ github.sha }}"
      echo "Actor: ${{ github.actor }}"

  # SSH debug con tmate (solo para troubleshooting)
  - name: Debug SSH
    if: failure()
    uses: mxschmitt/action-tmate@v3
    with:
      limit-access-to-actor: true
```

## Mejores Practicas

### DO

- Usar `concurrency` para cancelar runs duplicados
- Fijar versiones de actions (`@v4`, o mejor por SHA)
- Especificar `permissions` minimos en cada workflow
- Usar composite actions para logica reutilizable
- Usar `needs` para dependencias entre jobs
- Cachear dependencias (uv, npm, cargo)
- Usar matrix para testing multi-version/OS
- Usar environments con approvals para deploys
- Usar OIDC en vez de secrets para cloud providers
- Separar CI y CD en workflows distintos
- Usar `fail-fast: false` en matrices cuando quieres ver todos los resultados

### DON'T

- Usar `permissions: write-all` — minimo privilegio siempre
- Hardcodear secrets en workflows — usar GitHub Secrets
- Usar `pull_request_target` sin restricciones — riesgo de seguridad
- Poner logica compleja inline — extraer a scripts o composite actions
- Ignorar costos de runners — optimizar con cache y concurrencia
- Usar `continue-on-error: true` sin justificacion
- Ejecutar en `self-hosted` runners sin hardening de seguridad
- Usar `@main` o `@master` para actions — usar tags o SHAs
- Confiar en `set-output` — usar `$GITHUB_OUTPUT` (nuevo formato)
- Hacer deploys sin health checks post-deploy
- Ignorar dependabot/renovate para actualizar actions

## Recursos

- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [Workflow Syntax Reference](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions)
- [Security Hardening](https://docs.github.com/en/actions/security-for-github-actions)
- [Actions Marketplace](https://github.com/marketplace?type=actions)
- [Reusable Workflows](https://docs.github.com/en/actions/using-workflows/reusing-workflows)
- [OIDC for Cloud](https://docs.github.com/en/actions/security-for-github-actions/security-hardening-your-deployments/about-security-hardening-with-openid-connect)
- [act - Local Testing](https://github.com/nektos/act)
