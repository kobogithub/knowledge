---
name: docker-best-practices
description: Docker containerization best practices for development and production
version: 1.0.0
author: Knowledge Framework
tags: [docker, containers, devops, deployment]
---

# Docker Best Practices

Expert guidelines for building secure, efficient, and maintainable Docker containers.

## Core Principles

### 1. Dockerfile Optimization (Critical)

**Multi-stage Builds**
```dockerfile
# Build stage
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

# Production stage
FROM node:20-alpine
WORKDIR /app
COPY --from=builder /app/node_modules ./node_modules
COPY . .
EXPOSE 3000
CMD ["node", "server.js"]
```

**Layer Caching**
```dockerfile
# ✅ GOOD: Dependencies first (cached)
COPY package.json package-lock.json ./
RUN npm ci
COPY . .

# ❌ BAD: Code changes invalidate dependency cache
COPY . .
RUN npm ci
```

### 2. Base Images (Critical)

**Choose Minimal Images**
- `alpine` for smallest size (5MB vs 100MB+)
- `slim` for compatibility when alpine breaks
- Avoid `latest` tag - pin versions

```dockerfile
# ✅ Best: Specific alpine version
FROM node:20.11-alpine3.19

# ⚠️ OK: Slim variant
FROM python:3.12-slim

# ❌ Bad: Large base image
FROM ubuntu:latest
```

### 3. Security (Critical)

**Non-root User**
```dockerfile
FROM node:20-alpine
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nodejs -u 1001
USER nodejs
WORKDIR /app
COPY --chown=nodejs:nodejs . .
```

**Scan for Vulnerabilities**
```bash
docker scan myimage:latest
trivy image myimage:latest
```

**Secrets Management**
```bash
# ❌ Never bake secrets into images
ENV API_KEY=secret123  # NO!

# ✅ Use runtime environment variables
docker run -e API_KEY=$API_KEY myimage

# ✅ Use Docker secrets (Swarm/K8s)
docker secret create api_key ./key.txt
```

### 4. .dockerignore (High)

```
# .dockerignore
node_modules
npm-debug.log
.git
.gitignore
README.md
.env
.env.*
dist
*.md
.vscode
.idea
coverage
.DS_Store
```

### 5. Efficient Layering (High)

**Order matters**
```dockerfile
# Dependencies (changes rarely)
COPY package*.json ./
RUN npm ci

# Source code (changes frequently)
COPY src ./src
COPY public ./public

# Build
RUN npm run build
```

**Combine RUN commands**
```dockerfile
# ✅ GOOD: Single layer
RUN apt-get update && \
    apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*

# ❌ BAD: Multiple layers
RUN apt-get update
RUN apt-get install -y curl
RUN rm -rf /var/lib/apt/lists/*
```

### 6. Health Checks (High)

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1
```

**Application health endpoint**
```javascript
app.get('/health', (req, res) => {
  res.status(200).json({ status: 'healthy' });
});
```

### 7. Docker Compose (High)

**Development Setup**
```yaml
# docker-compose.yml
version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=development
      - DATABASE_URL=postgresql://user:pass@db:5432/mydb
    volumes:
      - .:/app
      - /app/node_modules
    depends_on:
      - db
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: mydb
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  postgres_data:
```

### 8. Production Optimization (Critical)

**Remove Development Dependencies**
```dockerfile
# Node.js
RUN npm ci --only=production

# Python
RUN pip install --no-cache-dir -r requirements.txt

# Go (already produces static binary)
FROM scratch
COPY --from=builder /app/main /main
ENTRYPOINT ["/main"]
```

**Minimize Image Size**
```bash
# Before optimization
docker images myapp
# myapp    latest    850MB

# After multi-stage + alpine
# myapp    latest    45MB
```

### 9. Networking (Medium)

**Bridge Network (Default)**
```bash
docker network create my-network
docker run --network my-network --name app myapp
docker run --network my-network --name db postgres
```

**Host Network (Performance)**
```bash
# Use sparingly - reduces isolation
docker run --network host myapp
```

### 10. Volumes & Persistence (High)

**Named Volumes (Recommended)**
```bash
docker volume create app-data
docker run -v app-data:/data myapp
```

**Bind Mounts (Development)**
```bash
docker run -v $(pwd):/app myapp
```

**Tmpfs (Temporary Data)**
```bash
docker run --tmpfs /tmp myapp
```

## Common Patterns

### Python Application
```dockerfile
FROM python:3.12-slim
WORKDIR /app
RUN pip install --no-cache-dir --upgrade pip
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```

### Rust Application
```dockerfile
FROM rust:1.75-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.19
COPY --from=builder /app/target/release/myapp /usr/local/bin/myapp
EXPOSE 8080
CMD ["myapp"]
```

### Static Website (Nginx)
```dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

## Common Pitfalls

❌ **Using `latest` tag**: Unpredictable builds  
✅ Pin specific versions: `node:20.11-alpine3.19`

❌ **Running as root**: Security risk  
✅ Create and use non-root user

❌ **Large image sizes**: Slow builds, deploys  
✅ Use multi-stage builds and alpine images

❌ **No health checks**: Can't detect failures  
✅ Implement `/health` endpoint and HEALTHCHECK

❌ **Secrets in images**: Exposed in layer history  
✅ Use environment variables or Docker secrets

❌ **No .dockerignore**: Huge build contexts  
✅ Exclude unnecessary files

## Performance Checklist

- [ ] Multi-stage builds for production
- [ ] Alpine or slim base images
- [ ] Non-root user configured
- [ ] Health checks implemented
- [ ] .dockerignore file present
- [ ] Dependencies cached efficiently
- [ ] Secrets passed at runtime
- [ ] Image scanned for vulnerabilities
- [ ] Image size optimized (< 100MB ideally)
- [ ] Docker Compose for local development

## Useful Commands

```bash
# Build with cache
docker build -t myapp:latest .

# Build without cache
docker build --no-cache -t myapp:latest .

# Check image size
docker images myapp

# Inspect layers
docker history myapp:latest

# Remove unused images
docker image prune -a

# View container logs
docker logs -f container_name

# Execute shell in running container
docker exec -it container_name sh

# Copy files from container
docker cp container_name:/path/to/file ./local/path
```

## Resources

- [Dockerfile Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Docker Security](https://docs.docker.com/engine/security/)
- [Multi-stage Builds](https://docs.docker.com/build/building/multi-stage/)
- [Docker Compose](https://docs.docker.com/compose/)
