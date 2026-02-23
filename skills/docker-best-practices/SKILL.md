---
name: docker-best-practices
description: Docker containerization best practices, multi-stage builds, security and Compose patterns
version: 1.0.0
tags:
  - best-practices
  - docker
  - containers
  - devops
  - compose
---

# docker-best-practices

Mejores practicas para Docker: Dockerfiles optimizados, multi-stage builds, seguridad, Docker Compose y patrones de produccion.

## Overview

Docker se usa para:
- **Development**: Entornos reproducibles, onboarding rapido
- **CI/CD**: Builds consistentes, test environments aislados
- **Production**: Deploys inmutables, escalado horizontal
- **Microservices**: Aislamiento, networking, service mesh

## Dockerfile — Multi-stage Build

### Python (FastAPI)

```dockerfile
# === Stage 1: Builder ===
FROM python:3.12-slim AS builder

# Instalar uv para package management rapido
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

WORKDIR /app

# Copiar solo dependency files primero (cache de layers)
COPY pyproject.toml uv.lock ./

# Instalar dependencias en un venv dentro del container
RUN uv sync --frozen --no-dev --no-install-project

# Copiar el codigo fuente
COPY src/ src/

# Instalar el proyecto
RUN uv sync --frozen --no-dev

# === Stage 2: Runtime ===
FROM python:3.12-slim AS runtime

# Crear usuario no-root
RUN groupadd --gid 1000 app && \
    useradd --uid 1000 --gid app --shell /bin/bash --create-home app

WORKDIR /app

# Copiar solo el venv instalado del builder
COPY --from=builder /app/.venv /app/.venv

# Agregar venv al PATH
ENV PATH="/app/.venv/bin:$PATH"
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# Cambiar a usuario no-root
USER app

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1

CMD ["uvicorn", "mypackage.main:app", "--host", "0.0.0.0", "--port", "8000"]
```

### Rust

```dockerfile
# === Stage 1: Builder ===
FROM rust:1.83-slim AS builder

WORKDIR /app

# Cache de dependencias — copiar solo Cargo files
COPY Cargo.toml Cargo.lock ./

# Crear dummy src para compilar dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copiar codigo real y compilar
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# === Stage 2: Runtime ===
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1000 app && \
    useradd --uid 1000 --gid app --shell /bin/bash --create-home app

COPY --from=builder /app/target/release/mycli /usr/local/bin/mycli

USER app

ENTRYPOINT ["mycli"]
```

### Node.js (Astro)

```dockerfile
# === Stage 1: Builder ===
FROM node:22-slim AS builder

WORKDIR /app

# Instalar dependencias primero (cache)
COPY package.json package-lock.json ./
RUN npm ci

# Copiar y buildar
COPY . .
RUN npm run build

# === Stage 2: Runtime ===
FROM node:22-slim AS runtime

WORKDIR /app

# Solo copiar el build output
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

USER node

EXPOSE 4321

CMD ["node", "dist/server/entry.mjs"]
```

## Optimizacion de Layers

### Orden de COPY importa

```dockerfile
# Mal — cualquier cambio en codigo invalida cache de dependencias
COPY . .
RUN npm install

# Bien — dependencias cacheadas, solo el codigo se re-copia
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build
```

### Combinar RUN para reducir layers

```dockerfile
# Mal — 3 layers, apt cache queda en layer intermedia
RUN apt-get update
RUN apt-get install -y curl git
RUN rm -rf /var/lib/apt/lists/*

# Bien — 1 layer, apt cache eliminada
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl git && \
    rm -rf /var/lib/apt/lists/*
```

### .dockerignore

```gitignore
# .dockerignore
.git
.github
.env
.env.*
*.md
!README.md
node_modules
__pycache__
*.pyc
.pytest_cache
.mypy_cache
.ruff_cache
target/
dist/
coverage/
.vscode/
.idea/
docker-compose*.yml
Dockerfile*
.dockerignore
```

## Seguridad

### Usuario no-root

```dockerfile
# SIEMPRE crear y usar usuario no-root
RUN groupadd --gid 1000 app && \
    useradd --uid 1000 --gid app --shell /bin/bash --create-home app

# Copiar archivos con ownership correcto
COPY --chown=app:app . .

# Cambiar a usuario no-root ANTES del CMD
USER app
```

### Imagenes base minimas

```dockerfile
# Preferencia (de mas a menos segura):
# 1. distroless — sin shell, sin package manager
FROM gcr.io/distroless/python3-debian12

# 2. slim — sin paquetes extras
FROM python:3.12-slim

# 3. alpine — pequena pero puede tener problemas con musl
FROM python:3.12-alpine

# EVITAR — imagen completa con toneladas de paquetes
FROM python:3.12
FROM ubuntu:24.04
```

### No secrets en imagen

```dockerfile
# NUNCA hacer esto — el secret queda en una layer
COPY .env .
ENV API_KEY=sk-1234567890

# Bien — secrets en runtime via env vars o mounted secrets
# docker run -e API_KEY=sk-xxx myimage
# docker run --secret id=api_key,src=./api_key.txt myimage

# BuildKit secrets para build-time
RUN --mount=type=secret,id=npmrc,target=/root/.npmrc npm ci
```

### Scanning con Trivy

```bash
# Escanear imagen
trivy image myapp:latest

# Fail on HIGH/CRITICAL
trivy image --severity HIGH,CRITICAL --exit-code 1 myapp:latest

# Escanear Dockerfile
trivy config Dockerfile
```

## Health Checks

```dockerfile
# HTTP health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:8000/health || exit 1

# Sin curl (Python)
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1

# TCP check (cuando no hay HTTP endpoint)
HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
  CMD pg_isready -U postgres || exit 1
```

## Docker Compose

### Development stack

```yaml
# docker-compose.yml
services:
  api:
    build:
      context: .
      dockerfile: Dockerfile
      target: builder  # Usar stage de builder para dev
    ports:
      - "8000:8000"
    volumes:
      - ./src:/app/src:cached     # Mount codigo para hot-reload
      - /app/.venv                # NO montar el venv
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@db:5432/myapp
      - REDIS_URL=redis://redis:6379
      - DEBUG=true
    env_file:
      - .env
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
    command: uvicorn mypackage.main:app --host 0.0.0.0 --reload

  frontend:
    build:
      context: ./frontend
      target: builder
    ports:
      - "4321:4321"
    volumes:
      - ./frontend/src:/app/src:cached
      - /app/node_modules
    command: npm run dev

  db:
    image: postgres:17-alpine
    ports:
      - "5432:5432"
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

### Production override

```yaml
# docker-compose.prod.yml
services:
  api:
    build:
      target: runtime             # Usar stage final
    volumes: []                   # NO montar codigo
    environment:
      - DEBUG=false
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: "1.0"
          memory: 512M
        reservations:
          cpus: "0.25"
          memory: 128M
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3

  db:
    ports: []                     # No exponer en prod
```

```bash
# Dev
docker compose up

# Prod
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Compose patterns utiles

```yaml
# Servicio base con extension fields
x-common: &common
  restart: unless-stopped
  logging:
    driver: json-file
    options:
      max-size: "10m"
      max-file: "3"

services:
  api:
    <<: *common
    build: .

  worker:
    <<: *common
    build: .
    command: celery -A mypackage.worker worker

# Profiles para servicios opcionales
services:
  monitoring:
    image: grafana/grafana:latest
    profiles:
      - monitoring
    ports:
      - "3000:3000"

# docker compose --profile monitoring up
```

## Networking

### Redes separadas

```yaml
services:
  api:
    networks:
      - frontend
      - backend

  db:
    networks:
      - backend          # Solo accesible desde backend network

  nginx:
    networks:
      - frontend

networks:
  frontend:
  backend:
    internal: true       # Sin acceso a internet
```

### DNS resolution

```yaml
# Los servicios se resuelven por nombre dentro de la misma red
# api puede conectar a: postgresql://db:5432/myapp
# No usar localhost, usar el nombre del servicio
services:
  api:
    environment:
      - DATABASE_URL=postgresql://db:5432/myapp  # "db" se resuelve via DNS
  db:
    image: postgres:17-alpine
```

## Volumes y Persistencia

```yaml
volumes:
  # Named volume — persistente, gestionado por Docker
  postgres_data:
    driver: local

  # Con opciones de backup
  postgres_data:
    driver: local
    labels:
      com.myapp.backup: "daily"

services:
  db:
    volumes:
      # Named volume para datos persistentes
      - postgres_data:/var/lib/postgresql/data

      # Bind mount para archivos de config (read-only)
      - ./config/postgresql.conf:/etc/postgresql/postgresql.conf:ro

      # tmpfs para datos temporales (en memoria, no persiste)
    tmpfs:
      - /tmp:size=100M
```

## Comandos Utiles

```bash
# Build con cache limpio
docker compose build --no-cache

# Build con progreso detallado
docker compose build --progress=plain

# Logs de un servicio
docker compose logs -f api

# Ejecutar comando one-off
docker compose run --rm api python -m pytest
docker compose exec api bash

# Limpiar todo
docker compose down -v --remove-orphans
docker system prune -af --volumes

# Ver tamano de imagenes
docker images --format "{{.Repository}}:{{.Tag}} {{.Size}}" | sort -k2 -h

# Inspeccionar layers
docker history myapp:latest

# Copiar archivos desde container
docker cp container_id:/app/data/export.csv ./export.csv
```

## BuildKit Features

```dockerfile
# Habilitar BuildKit
# export DOCKER_BUILDKIT=1

# Cache mounts — cache de package managers entre builds
RUN --mount=type=cache,target=/root/.cache/pip \
    pip install -r requirements.txt

RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Bind mounts — acceder a archivos sin copiarlos al layer
RUN --mount=type=bind,source=requirements.txt,target=/tmp/requirements.txt \
    pip install -r /tmp/requirements.txt

# Secret mounts — secrets disponibles solo durante build
RUN --mount=type=secret,id=npmrc,target=/root/.npmrc \
    npm ci
```

## Mejores Practicas

### DO

- Usar multi-stage builds para imagenes pequenas
- Usar `.dockerignore` para excluir archivos innecesarios
- Copiar dependency files ANTES que el codigo (cache de layers)
- Crear y usar usuario no-root
- Agregar HEALTHCHECK en todos los servicios
- Usar imagenes `-slim` o distroless como base
- Fijar versiones de imagenes base (no usar `:latest`)
- Usar `--mount=type=cache` para caches de package managers
- Combinar RUN commands para reducir layers
- Usar `docker compose --profile` para servicios opcionales
- Escanear imagenes con Trivy antes de deploy

### DON'T

- Usar `:latest` como tag en produccion — fijar version
- Copiar `.env`, secrets o SSH keys a la imagen
- Ejecutar como root en produccion
- Instalar paquetes innecesarios (debuggers, editors)
- Usar `COPY . .` sin `.dockerignore`
- Montar volumenes de codigo en produccion
- Exponer puertos de base de datos en produccion
- Usar `docker compose up` sin `-d` en produccion
- Ignorar health checks — pueden enmascarar problemas
- Crear imagenes de mas de 500MB — optimizar con multi-stage
- Usar `ADD` cuando `COPY` es suficiente — `ADD` tiene side effects

## Recursos

- [Docker Documentation](https://docs.docker.com/)
- [Dockerfile Best Practices](https://docs.docker.com/build/building/best-practices/)
- [Docker Compose Docs](https://docs.docker.com/compose/)
- [Distroless Images](https://github.com/GoogleContainerTools/distroless)
- [BuildKit](https://docs.docker.com/build/buildkit/)
- [Docker Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Docker_Security_Cheat_Sheet.html)
- [Hadolint - Dockerfile Linter](https://github.com/hadolint/hadolint)
