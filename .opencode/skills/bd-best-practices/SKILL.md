---
name: bd-best-practices
description: Best practices para gestión de issues y tareas con bd (beads)
version: 1.0.0
author: Knowledge Framework
tags: [bd, beads, issue-tracking, workflow, collaboration, project-management]
---

# bd (beads) Best Practices

Guía completa de mejores prácticas para gestión de issues, tareas y coordinación entre agentes usando **bd** (beads) - un sistema de issue tracking descentralizado.

## Priority Levels

- **Critical**: Workflow esencial, sincronización, integridad de datos
- **High**: Coordinación entre agentes, reporting, autonomía
- **Medium**: Optimización de workflow, organización
- **Low**: Conveniencia y mejoras menores

---

## 1. Conceptos Fundamentales (Critical)

### ¿Qué es bd (beads)?

**bd** es un sistema de issue tracking descentralizado que almacena issues en `.beads/issues.jsonl` y se sincroniza con git.

**Características clave:**
- ✅ Issues almacenados localmente en JSONL
- ✅ Sincronización vía git (no requiere servidor externo)
- ✅ Soporte para multi-agente workflow
- ✅ Backend opcional con Dolt para versionado SQL
- ✅ CLI intuitivo y rápido

### Anatomía de un Issue

```
ID: knowledge-abc
Title: Implementar autenticación JWT
Type: feature
Status: in_progress
Priority: 0 (P0 = crítico, P3 = bajo)
Assignee: knowledge-vlf
Labels: [backend, api, security]
Parent: knowledge-xyz (si es sub-tarea)
Dependencies: [knowledge-def] (issues que bloquean este)
Comments: Lista de comentarios con timestamps
Created: 2026-02-18T10:00:00Z
Updated: 2026-02-18T15:30:00Z
```

---

## 2. Comandos Esenciales (Critical)

### Buscar Trabajo

```bash
# Ver trabajo disponible (sin blockers)
bd ready

# Filtrar por label
bd ready -l frontend
bd ready -l backend
bd ready -l qa

# Ver issues por prioridad
bd ready -l frontend --priority 0  # Solo P0

# Ver tus tareas asignadas
bd list --assignee knowledge-vlf

# Ver todas las tareas de un label
bd list -l backend

# Ver issues en un estado específico
bd list --status in_progress
bd list --status blocked
```

### Reclamar y Trabajar

```bash
# Reclamar tarea atómicamente (RECOMENDADO)
bd update knowledge-abc --claim
# Esto te asigna Y marca como in_progress en una operación atómica

# Actualizar estado del agente
bd agent state knowledge-vlf working

# Agregar comentario de inicio
bd comments add knowledge-abc "[Backend Agent] Iniciando implementación con FastAPI + JWT"
```

### Reportar Progreso

```bash
# Agregar comentarios durante el trabajo
bd comments add knowledge-abc "[Backend Agent] ✓ Endpoints /auth/login y /auth/refresh implementados"
bd comments add knowledge-abc "[Backend Agent] ✓ Tests de integración: 15/15 pasando"
bd comments add knowledge-abc "[Backend Agent] 🔍 Encontrado: necesito configurar JWT_SECRET en env vars"

# Ver comentarios de un issue
bd comments knowledge-abc

# Heartbeat (opcional) - indica que estás activo
bd agent heartbeat knowledge-vlf
```

### Completar Trabajo

```bash
# Reportar completado con detalles
bd comments add knowledge-abc "[Backend Agent] ✓ Completado:
- Endpoints: /auth/login, /auth/refresh
- JWT tokens: 15min access, 7d refresh
- Tests: 100% coverage
- Docs: Swagger actualizado"

# Cerrar el issue (TÚ cierras tus propias tareas)
bd close knowledge-abc

# Actualizar estado del agente
bd agent state knowledge-vlf done

# Si continúas con más trabajo
bd agent state knowledge-vlf idle
```

### Sincronizar con Git

```bash
# SIEMPRE después de crear/cerrar issues
bd sync

# Commit y push
git add .beads/issues.jsonl
git commit -m "Backend Agent: Completar autenticación JWT"
git push
```

---

## 3. Crear Issues (High)

### Sintaxis de Creación

```bash
# Básico
bd create "Título del issue"

# Completo con todas las opciones
bd create "Implementar búsqueda de productos" \
  -t feature \
  -p 1 \
  -l backend,api,search \
  --assignee knowledge-vlf \
  -d "Implementar endpoint GET /api/products/search con filtros por categoría, precio y nombre"

# Con issue padre (sub-tarea)
bd create "Tests unitarios para búsqueda" \
  -t task \
  -p 2 \
  -l backend,testing \
  --assignee knowledge-vlf \
  --parent knowledge-abc

# Con dependencias
bd create "Integrar búsqueda en frontend" \
  -t feature \
  -p 1 \
  -l frontend \
  --assignee knowledge-4yh \
  --deps knowledge-abc  # Bloqueado por backend task

# Issue silencioso (no imprime, retorna solo ID)
TASK=$(bd create "Setup CI/CD" -t chore -p 0 --silent)
echo "Created: $TASK"
```

### Tipos de Issues

| Type | Uso | Ejemplo |
|------|-----|---------|
| `feature` | Nueva funcionalidad | "Implementar carrito de compras" |
| `bug` | Corrección de errores | "Fix: Login falla con espacios en email" |
| `task` | Tarea genérica | "Refactorizar módulo de auth" |
| `chore` | Mantenimiento, configs | "Actualizar dependencias a versiones latest" |
| `epic` | Agrupador de tareas | "Sistema de E-commerce completo" |
| `decision` | Decisión arquitectónica | "Decidir: PostgreSQL vs MongoDB" |

### Niveles de Prioridad

| Priority | Significado | Ejemplo |
|----------|-------------|---------|
| `0` (P0) | Crítico/Urgente | Producción caída, vulnerabilidad de seguridad |
| `1` (P1) | Alto | Feature bloqueante para release |
| `2` (P2) | Normal | Mejora importante pero no urgente |
| `3` (P3) | Bajo | Nice-to-have, mejora menor |

### Labels Comunes

**Por Área:**
- `frontend`, `backend`, `devops`, `qa`, `rust`

**Por Tecnología:**
- `api`, `database`, `ci-cd`, `docker`, `terraform`

**Por Naturaleza:**
- `security`, `performance`, `testing`, `documentation`

**Por Estado:**
- `blocked`, `urgent`, `bug`, `enhancement`

---

## 4. Actualizar Issues (High)

```bash
# Cambiar estado
bd update knowledge-abc --status in_progress
bd update knowledge-abc --status blocked
bd update knowledge-abc --status done  # Mejor usar 'bd close'

# Cambiar prioridad
bd update knowledge-abc --priority 0  # Escalate a P0

# Cambiar assignee
bd update knowledge-abc --assignee knowledge-4yh

# Agregar labels
bd update knowledge-abc --add-label urgent
bd update knowledge-abc --add-label security

# Remover labels
bd update knowledge-abc --remove-label enhancement
```

---

## 5. Comentarios (High)

### Formato de Comentarios

**Template recomendado:**

```bash
bd comments add <task-id> "[Agent Name] <emoji> <mensaje>

<detalles opcionales>"
```

**Ejemplos:**

```bash
# Inicio
bd comments add knowledge-abc "[Backend Agent] 🚀 Iniciando implementación"

# Progreso
bd comments add knowledge-abc "[Backend Agent] ✓ Endpoints implementados
- POST /auth/login
- POST /auth/refresh
- DELETE /auth/logout"

# Problema encontrado
bd comments add knowledge-abc "[Backend Agent] ⚠️ Encontrado: JWT_SECRET no configurado en staging"

# Decisión técnica
bd comments add knowledge-abc "[Backend Agent] 📝 Decisión: Usando PyJWT en lugar de python-jose por mejor mantenimiento"

# Bloqueo
bd comments add knowledge-abc "[Backend Agent] 🔴 Bloqueado: Necesito acceso a base de datos de staging"

# Completado
bd comments add knowledge-abc "[Backend Agent] ✅ Completado - Ver detalles arriba"
```

### Emojis Útiles

| Emoji | Significado |
|-------|-------------|
| 🚀 | Iniciando |
| ✅ ✓ | Completado |
| ⚠️ | Warning/Atención |
| 🔴 | Bloqueado/Error |
| 🔍 | Investigando |
| 📝 | Nota/Decisión |
| 🐛 | Bug encontrado |
| ⚡ | Performance |
| 🔒 | Security |

### Ver Comentarios

```bash
# Ver todos los comentarios de un issue
bd comments knowledge-abc

# Ver comentarios con formato
bd show knowledge-abc
```

---

## 6. Dependencias y Bloqueos (Critical)

### Agregar Dependencias

```bash
# Este issue DEPENDE de otro (está bloqueado por)
bd update knowledge-abc --deps knowledge-xyz

# Agregar múltiples dependencias
bd update knowledge-abc --deps knowledge-xyz,knowledge-def

# Usando bd dep (alias)
bd dep add knowledge-abc depends-on knowledge-xyz
```

### Reportar Bloqueos

```bash
# Marcar como bloqueado
bd update knowledge-abc --status blocked

# Agregar comentario explicando el bloqueo
bd comments add knowledge-abc "[Backend Agent] ⚠️ Bloqueado: Esperando que DevOps configure PostgreSQL en staging (knowledge-xyz)"

# Notificar en el issue bloqueante
bd comments add knowledge-xyz "[Backend Agent] 🔴 Bloqueando knowledge-abc - necesito esto ASAP"

# Actualizar estado del agente
bd agent state knowledge-vlf stuck
```

### Resolver Bloqueos

```bash
# Cuando el issue bloqueante se cierra, el status 'blocked' se puede cambiar
bd update knowledge-abc --status in_progress

bd comments add knowledge-abc "[Backend Agent] ✅ Desbloqueado - PostgreSQL configurado, continuando"
```

---

## 7. Gestión de Agentes (High)

### Estados de Agente

```bash
# Ver estado actual
bd agent show knowledge-vlf

# Actualizar estado
bd agent state knowledge-vlf working   # Trabajando activamente
bd agent state knowledge-vlf idle      # Disponible para trabajo
bd agent state knowledge-vlf stuck     # Bloqueado
bd agent state knowledge-vlf done      # Sesión completada
bd agent state knowledge-vlf stopped   # Detenido

# Heartbeat (indicar que estás activo)
bd agent heartbeat knowledge-vlf
```

### Ver Agentes

```bash
# Listar todos los agentes
bd list -l "gt:agent"

# Ver estado específico de un agente
bd agent show knowledge-vlf

# Ver trabajo de un agente
bd list --assignee knowledge-vlf
```

---

## 8. Coordinación entre Agentes (Critical)

### Crear Tareas para Otros Agentes

```bash
# Frontend crea tarea para Backend
bd create "Endpoint GET /api/products necesita campo 'discount_percentage'" \
  -t task \
  -p 1 \
  -l backend,api \
  --assignee knowledge-vlf

# Backend crea tarea para DevOps
bd create "Configurar Redis en staging para caching" \
  -t chore \
  -p 0 \
  -l devops,infrastructure \
  --assignee knowledge-w5p

# QA crea bug para Frontend
bd create "Bug: Botón submit deshabilitado con formulario válido" \
  -t bug \
  -p 0 \
  -l bug,frontend \
  --assignee knowledge-4yh \
  -d "Steps to reproduce:
1. Llenar form con datos válidos
2. Cambiar dropdown a 'Otro'
3. Botón submit se deshabilita incorrectamente"
```

### Mencionar Agentes en Comentarios

```bash
# Pedir clarificación al Planner
bd comments add knowledge-abc "[Backend Agent] @knowledge-x6e Necesito aclaración: ¿El sistema debe soportar OAuth además de JWT?"

# Notificar al DevOps que algo está listo
bd comments add knowledge-xyz "[Backend Agent] @knowledge-w5p API lista para deploy. Endpoints documentados en /docs"

# Reportar a QA que algo está listo para testing
bd comments add knowledge-abc "[Backend Agent] @knowledge-pu1 Feature completa y lista para QA. Ver staging: https://staging.app.com/api/products"
```

### Reportar a Issue Padre (Epic)

```bash
# Reportar progreso en epic
bd comments add epic-id "[Backend Agent] Progreso en epic:
- ✅ Auth endpoints completados (knowledge-abc)
- ✅ User CRUD completado (knowledge-def)
- 🚧 En progreso: Products API (knowledge-xyz)
- ⏳ Pendiente: Orders API"
```

---

## 9. Búsqueda y Filtrado (Medium)

### Búsqueda de Texto

```bash
# Buscar por texto en título/descripción
bd search "autenticación"
bd search "JWT"

# Buscar en comentarios también
bd search "PostgreSQL" --include-comments
```

### Filtros Avanzados

```bash
# Combinar múltiples filtros
bd list -l backend --priority 0 --status in_progress

# Ver issues bloqueados
bd list --status blocked

# Ver bugs de alta prioridad
bd list -l bug --priority 0,1

# Ver work de un sprint/epic
bd children epic-id
```

---

## 10. Workflow Completo - Ejemplo (Critical)

### Ciclo de Vida de un Issue

```bash
# ============================================
# 1. BUSCAR TRABAJO
# ============================================
bd agent state knowledge-vlf working
bd ready -l backend

# Output:
# 📋 Ready work (2 issues):
# 1. [● P0] [bug] knowledge-xyz: API retorna 500 en endpoint /products
# 2. [● P1] [feature] knowledge-abc: Implementar autenticación JWT

# ============================================
# 2. RECLAMAR TAREA
# ============================================
bd update knowledge-abc --claim

# Actualizar estado
bd agent state knowledge-vlf working

# ============================================
# 3. REVISAR DETALLES
# ============================================
bd show knowledge-abc

# ============================================
# 4. REPORTAR INICIO
# ============================================
bd comments add knowledge-abc "[Backend Agent] 🚀 Iniciando implementación
Stack: FastAPI + PyJWT
Plan:
1. Endpoints /auth/login, /auth/refresh
2. JWT middleware
3. Tests de integración
4. Documentación Swagger"

# ============================================
# 5. TRABAJO + PROGRESO
# ============================================
# ... desarrollo ...

bd comments add knowledge-abc "[Backend Agent] ✓ Endpoints implementados
- POST /auth/login
- POST /auth/refresh  
- Retornan access_token y refresh_token"

bd comments add knowledge-abc "[Backend Agent] ✓ JWT middleware creado
- Valida tokens en rutas protegidas
- Maneja expiración correctamente"

bd comments add knowledge-abc "[Backend Agent] ✓ Tests pasando
- 15 tests unitarios
- 8 tests de integración
- Coverage: 98%"

# ============================================
# 6. SI HAY BLOQUEO (opcional)
# ============================================
bd update knowledge-abc --status blocked
bd agent state knowledge-vlf stuck

bd comments add knowledge-abc "[Backend Agent] 🔴 Bloqueado: Necesito JWT_SECRET configurado en staging"

# Crear issue para DevOps
bd create "Configurar JWT_SECRET en staging environment" \
  -t chore \
  -p 0 \
  -l devops \
  --assignee knowledge-w5p

# Buscar otra tarea mientras tanto
bd ready -l backend

# Cuando se resuelve:
bd update knowledge-abc --status in_progress
bd comments add knowledge-abc "[Backend Agent] ✅ Desbloqueado - continuando"

# ============================================
# 7. COMPLETAR
# ============================================
bd comments add knowledge-abc "[Backend Agent] ✅ COMPLETADO

Implementación:
- ✅ Endpoints: /auth/login, /auth/refresh, /auth/logout
- ✅ JWT: 15min access tokens, 7d refresh tokens
- ✅ Middleware: Protección de rutas
- ✅ Security: Passwords con bcrypt, httpOnly cookies
- ✅ Tests: 100% coverage (23 tests)
- ✅ Docs: Swagger UI actualizado en /docs

Código: src/api/auth.py:45
Deploy: Listo para staging"

bd close knowledge-abc

bd agent state knowledge-vlf done

# ============================================
# 8. SINCRONIZAR (CRÍTICO)
# ============================================
bd sync
git add .beads/issues.jsonl
git commit -m "Backend Agent: Completar autenticación JWT"
git push
git status  # Verificar que esté up to date

# ============================================
# 9. SIGUIENTE TAREA
# ============================================
bd agent state knowledge-vlf idle
bd ready -l backend
```

---

## 11. Landing the Plane - Fin de Sesión (Critical)

**Al terminar tu sesión de trabajo, DEBES seguir estos pasos:**

### 1. Cerrar Issues Completados

```bash
# Cerrar solo si está 100% completo
bd close knowledge-abc
bd close knowledge-def

# Si no está completo, dejar comentario de handoff
bd comments add knowledge-xyz "[Backend Agent] 📝 Handoff:
- 80% completo
- Falta: agregar rate limiting a endpoints
- Próximo paso: implementar Redis cache
- Notas: ver TODO en línea 145 de auth.py"
```

### 2. Actualizar Estado del Agente

```bash
bd agent state knowledge-vlf idle    # Si habrá otra sesión pronto
bd agent state knowledge-vlf stopped # Si termina el día
```

### 3. Sincronizar con Git (OBLIGATORIO)

```bash
bd sync
git add .beads/issues.jsonl
git commit -m "Backend Agent: [resumen de lo hecho]"
git push
git status  # MUST show "up to date with origin"
```

### 4. Verificar Estado

```bash
# Ver qué dejaste en progreso
bd list --assignee knowledge-vlf --status in_progress

# Ver issues bloqueados
bd list --assignee knowledge-vlf --status blocked
```

---

## 12. Best Practices Checklist (High)

### ✅ SIEMPRE hacer:

- [ ] Usar `bd update <id> --claim` para reclamar tareas atómicamente
- [ ] Agregar comentarios durante el trabajo (no solo al inicio/fin)
- [ ] Usar formato consistente: `[Agent Name] emoji mensaje`
- [ ] Cerrar tus propias tareas cuando estén completas
- [ ] Sincronizar con git después de cambios: `bd sync && git push`
- [ ] Reportar bloqueos inmediatamente
- [ ] Crear issues para otros agentes cuando sea necesario
- [ ] Verificar `git status` al final de sesión

### ❌ NUNCA hacer:

- [ ] Cerrar tareas de otros agentes (cada agente cierra las suyas)
- [ ] Olvidar sincronizar con git (`bd sync && git push`)
- [ ] Dejar sesión sin hacer `git push`
- [ ] Cerrar tareas que no están 100% completas
- [ ] Usar `--force` en git push sin razón válida
- [ ] Hardcodear información en comentarios (usar referencias a issues)
- [ ] Crear issues duplicados sin verificar existentes

### 🎯 Recomendaciones:

- Usar `--silent` flag al crear issues en scripts
- Usar variables para IDs: `TASK=$(bd create "..." --silent)`
- Agregar labels específicos y descriptivos
- Documentar decisiones técnicas en comentarios
- Reportar métricas en épicos (coverage, performance, etc.)
- Usar emojis para rápida visualización de estado
- Hacer commits frecuentes de `.beads/issues.jsonl`

---

## 13. Comandos de Referencia Rápida

### Workflow Básico

```bash
# Buscar → Reclamar → Trabajar → Completar → Sync
bd ready -l <label>
bd update <id> --claim
bd agent state <agent-id> working
bd comments add <id> "[Agent] mensaje"
bd close <id>
bd sync && git push
```

### Issue Management

```bash
# Crear
bd create "Título" -t <type> -p <priority> -l <labels> --assignee <agent-id>

# Listar
bd list -l <label> --priority <0-3> --status <status>
bd ready -l <label>

# Ver
bd show <id>
bd comments <id>

# Actualizar
bd update <id> --status <status>
bd update <id> --priority <priority>
bd update <id> --claim

# Cerrar
bd close <id>
```

### Agent Management

```bash
# Estado
bd agent state <agent-id> <working|idle|stuck|done|stopped>
bd agent show <agent-id>
bd agent heartbeat <agent-id>

# Listar
bd list -l "gt:agent"
bd list --assignee <agent-id>
```

### Sincronización

```bash
# SIEMPRE después de cambios
bd sync
git add .beads/issues.jsonl
git commit -m "Agent: descripción"
git push
git status
```

---

## 14. Integración con Git (Critical)

### Flujo de Sincronización

```bash
# Antes de empezar sesión
git pull --rebase

# Durante el trabajo
bd update <id> --claim
bd comments add <id> "..."
bd close <id>

# Después de cambios
bd sync
git add .beads/issues.jsonl
git commit -m "Backend Agent: Completar autenticación JWT"
git push

# Verificar
git status  # Debe mostrar "up to date with origin"
```

### Mensajes de Commit

**Formato recomendado:**

```
[Agent Name]: [Acción] [Descripción breve]

Ejemplos:
✅ "Backend Agent: Completar autenticación JWT"
✅ "Frontend Agent: Cerrar dashboard UI, bloqueado products API"
✅ "DevOps Agent: Crear infraestructura staging con Terraform"
✅ "QA Agent: Reportar 3 bugs en checkout flow"

❌ "Update issues"
❌ "Work done"
❌ "Sync"
```

---

## 15. Troubleshooting (Medium)

### Issue no aparece en bd ready

```bash
# Verificar que no esté asignado
bd list -l <label> --status open

# Verificar dependencias
bd show <id>  # Ver si tiene blockers
```

### Conflictos en git

```bash
# Si hay conflicto en .beads/issues.jsonl
git pull --rebase
# Resolver manualmente el conflicto en .beads/issues.jsonl
bd sync  # Re-sincronizar
git add .beads/issues.jsonl
git rebase --continue
git push
```

### Issue desaparece después de sync

```bash
# Ver historial (requiere Dolt backend)
bd history <id>

# Buscar en git log
git log --all --grep="<id>"
```

---

## Recursos

- [bd (beads) Repository](https://github.com/beadtools/bd)
- [bd Documentation](https://beadtools.github.io/bd/)
- [Issue Tracking Best Practices](https://beadtools.github.io/bd/best-practices)

---

**Última actualización**: 2026-02-18  
**Mantenido por**: Platform Team

**Nota**: Este skill es esencial para todos los agentes. Dominar bd es crítico para la coordinación efectiva del equipo.
