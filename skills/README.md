# Skills Instalados

Este directorio contiene skills para AI agents que proporcionan conocimiento especializado sobre tecnologías específicas.

## Skills Disponibles

### 1. Astro Best Practices (`astro-best-practices/`)
**Fuente**: Custom (Knowledge Framework)  
**Descripción**: Guía completa de best practices para el framework Astro

**Temas cubiertos**:
- Islands Architecture y client directives
- Optimización de performance e imágenes
- Content Collections y type safety
- Routing, API routes y redirects
- Integración con React/Vue/Svelte
- View Transitions y SEO
- Build strategies (static, SSR, hybrid)

**Cuándo usar**:
- Construyendo sitios web con Astro
- Optimizando performance de aplicaciones Astro
- Migrando proyectos a Astro
- Implementando Islands Architecture

---

### 2. Bash Best Practices (`bash-best-practices/`)
**Fuente**: Custom (Knowledge Framework)  
**Descripción**: Guía de scripting bash robusto y mantenible

**Temas cubiertos**:
- Script headers y error handling (`set -euo pipefail`)
- Variables, quoting y command substitution
- Functions, loops y conditionals
- Input validation y logging
- Retry logic y parallel execution
- ShellCheck integration

**Cuándo usar**:
- Escribiendo scripts de automation
- Creando deployment scripts
- Tareas de DevOps y CI/CD
- Procesamiento de archivos y datos

---

### 3. Docker Best Practices (`docker-best-practices/`)
**Fuente**: Custom (Knowledge Framework)  
**Descripción**: Containerización eficiente y segura con Docker

**Temas cubiertos**:
- Multi-stage builds y layer caching
- Base images (alpine, slim)
- Security y non-root users
- .dockerignore y efficient layering
- Health checks y Docker Compose
- Production optimization

**Cuándo usar**:
- Creando Dockerfiles
- Optimizando imágenes Docker
- Configurando Docker Compose para desarrollo
- Implementando security best practices

---

### 4. Python Best Practices (`python-best-practices/`)
**Fuente**: Custom (Knowledge Framework)  
**Descripción**: Python moderno para aplicaciones backend y scripting

**Temas cubiertos**:
- Code style (PEP 8) y type hints
- Error handling y context managers
- List/dict comprehensions y generators
- Decorators y data classes (Pydantic)
- Async/await y testing (pytest)
- FastAPI patterns
- Virtual environments y dependency management

**Cuándo usar**:
- Desarrollo de APIs con FastAPI
- Scripts de automation
- Data processing
- Testing y type checking

---

### 5. Rust Best Practices (`rust-best-practices/`)
**Fuente**: Custom (Knowledge Framework)  
**Descripción**: Rust idiomático para systems programming y CLIs

**Temas cubiertos**:
- Ownership, borrowing y lifetimes
- Error handling (Result, anyhow, thiserror)
- Type design (newtype, builder patterns)
- Collections, iterators y performance
- Concurrency (channels, Arc, Mutex, tokio)
- CLI development con Clap
- Testing y benchmarking
- Cargo best practices

**Cuándo usar**:
- Desarrollando herramientas CLI
- Systems programming
- Performance-critical code
- Proyectos que requieren safety y concurrency

---

### 6. Supabase Postgres Best Practices (`supabase-postgres-best-practices/`)
**Fuente**: Supabase (https://github.com/supabase/agent-skills)  
**Descripción**: Optimización de performance de PostgreSQL desde Supabase

**Temas cubiertos**:
- Query performance (Critical)
- Connection management (Critical)
- Schema design (High)
- Concurrency & locking (Medium-High)
- Security & RLS (Row-Level Security)
- Data access patterns
- Monitoring & diagnostics
- Advanced features

**Cuándo usar**:
- Escribiendo queries SQL
- Diseñando schemas de bases de datos
- Implementando indexes y optimización
- Configurando connection pooling
- Trabajando con Row-Level Security (RLS)

---

## Instalación de Skills

### Usando el CLI `kn`

**Instalar desde URL**:
```bash
kn skills install https://raw.githubusercontent.com/supabase/agent-skills/main/skills/supabase-postgres-best-practices/SKILL.md
```

**Instalar desde path local**:
```bash
kn skills install ./custom-skill/SKILL.md
```

**Instalar por nombre** (desde agentskills.io):
```bash
kn skills install typescript
```

**Listar skills instalados**:
```bash
kn skills list
```

### Manualmente

1. Crear directorio en `skills/my-skill-name/`
2. Crear archivo `SKILL.md` con frontmatter YAML:
```yaml
---
name: my-skill-name
description: Short description
version: 1.0.0
author: Your Name
tags: [tag1, tag2]
---
```
3. Agregar contenido del skill en Markdown

---

## Formato de Skills

Los skills siguen el [Agent Skills Open Standard](https://agentskills.io/):

### Estructura Mínima
```
skills/
  my-skill/
    SKILL.md          # Requerido: Manifest con frontmatter + contenido
    AGENTS.md         # Opcional: Referencias compiladas
    references/       # Opcional: Archivos de referencia individuales
```

### Frontmatter YAML
```yaml
---
name: skill-name              # Requerido: Identificador único
description: Brief description # Requerido: Descripción corta
version: 1.0.0                # Requerido: Semantic versioning
author: Author Name           # Opcional
tags: [tag1, tag2]           # Opcional: Para categorización
---
```

---

## Uso con AI Agents

### OpenCode
Los skills son automáticamente disponibles una vez instalados. El agente los usará cuando detecte tareas relevantes.

**Ejemplos**:
- "Optimize this Postgres query" → Usa `supabase-postgres-best-practices`
- "Create a Dockerfile for this Node app" → Usa `docker-best-practices`
- "Write a deployment script" → Usa `bash-best-practices`
- "Build an API with FastAPI" → Usa `python-best-practices`

### Claude Code Plugin
```bash
/plugin marketplace add knowledge-framework
/plugin install astro-best-practices@knowledge-framework
```

### Cursor / GitHub Copilot
Los skills en el directorio `skills/` son automáticamente indexados y disponibles en el contexto.

---

## Contribuyendo

Para agregar nuevos skills al proyecto:

1. Crear directorio en `skills/nombre-skill/`
2. Seguir el formato Agent Skills Open Standard
3. Incluir ejemplos prácticos y code snippets
4. Priorizar contenido por impacto (Critical > High > Medium > Low)
5. Agregar checklist de validación
6. Incluir recursos y links a documentación oficial

---

## Recursos

- [Agent Skills Open Standard](https://agentskills.io/)
- [Skills Directory](https://skills.sh/)
- [Supabase Agent Skills](https://github.com/supabase/agent-skills)
- [Knowledge Framework](https://github.com/kobogithub/knowledge)
