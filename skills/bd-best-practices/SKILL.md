---
name: bd-best-practices
description: Beads (bd) issue tracking best practices for multi-agent workflows
version: 1.0.0
tags:
  - best-practices
  - bd
  - beads
  - issue-tracking
  - multi-agent
  - workflow
---

# bd-best-practices

Mejores practicas para bd (beads): issue tracking ligero con dependencias first-class, coordinacion multi-agente, git sync y workflows automatizados.

## Overview

bd es un issue tracker ligero que:
- **Vive en tu repo**: Base de datos SQLite + JSONL exportado, versionado con git
- **Dependencias first-class**: Bloques, relaciones, ciclos detectados automaticamente
- **Multi-agente**: Agents con estado, claims atomicos, coordinacion via gates
- **Epics y jerarquia**: Parent/child, epics con status aggregado
- **Git sync**: Exporta a JSONL, merge driver integrado, conflict resolution

## Instalacion y Setup

### Inicializar bd en un repo

```bash
# Inicializar bd en el directorio actual
bd init

# Verificar salud de la instalacion
bd doctor

# Configurar git hooks para auto-sync
bd hooks install

# Ver info de la base de datos
bd info
```

### Estructura de archivos

```
project/
├── .beads/
│   ├── beads.db          # SQLite database (NO commitear)
│   └── issues.jsonl      # Exportado para git (SI commitear)
├── AGENTS.md             # Instrucciones para agentes
└── ...
```

## Crear Issues

### Comandos basicos

```bash
# Crear un task basico
bd create "Fix login timeout bug"

# Crear con tipo, prioridad y labels
bd create "Add dark mode toggle" \
  -t feature -p 1 -l frontend,ui

# Crear un bug con descripcion
bd create "API returns 500 on empty payload" \
  -t bug -p 0 \
  -d "El endpoint POST /api/users devuelve 500 cuando el body esta vacio"

# Crear con assignee
bd create "Setup CI pipeline" \
  -t chore -p 2 -l devops \
  --assignee knowledge-w5p

# Crear tarea hijo de un epic
bd create "Implement user model" \
  -t task -p 1 -l backend \
  --parent bd-abc123

# Quick capture — solo devuelve el ID
bd q "TODO: revisar permisos de admin"
```

### Tipos de issue

```bash
# Tipos disponibles:
# bug       — Defecto a corregir
# feature   — Funcionalidad nueva
# task      — Tarea generica
# epic      — Agrupador de tareas relacionadas
# chore     — Mantenimiento, refactoring, cleanup
# decision  — Registro de decision arquitectural (ADR)
# gate      — Punto de espera asincrono

bd create "Redesign auth system" -t epic -p 1
bd create "Migrate to JWT tokens" -t task --parent bd-epic-id
bd create "Use bcrypt for password hashing" -t decision
```

### Prioridades

```bash
# P0 = critico (produccion down)
# P1 = alta (bloquea release)
# P2 = media (default)
# P3 = baja
# P4 = nice-to-have

bd create "Production DB is down" -t bug -p 0
bd create "Add unit tests for auth" -t task -p 3
```

### Crear multiples issues desde Markdown

```bash
# Crear un archivo con formato markdown
bd create -f tasks.md
```

## Listar y Buscar Issues

### Listados

```bash
# Listar issues abiertos (default: 50)
bd list

# Listar con filtros
bd list -l frontend             # Por label (AND)
bd list -l backend,api          # Multiples labels (AND: debe tener AMBOS)
bd list --label-any frontend,backend  # OR: al menos uno
bd list -a knowledge-4yh        # Por assignee
bd list -t bug                  # Por tipo
bd list --all                   # Incluyendo cerrados
bd list -n 100                  # Limite custom

# Filtros avanzados
bd list --overdue               # Issues vencidos
bd list --no-assignee           # Sin asignar
bd list --no-labels             # Sin labels
bd list --created-after 2025-01-01
bd list --deferred              # Issues diferidos

# Output JSON
bd list --json
bd list --json | jq '.[] | {id, title, priority}'
```

### Work ready (sin blockers)

```bash
# Issues listos para trabajar (sin dependencias bloqueantes)
bd ready

# Ready filtrado por label
bd ready -l frontend
bd ready -l backend -n 20

# Ready para un agente especifico
bd ready -a knowledge-4yh

# Ready con formato tree
bd ready --pretty
```

### Buscar

```bash
# Busqueda por texto
bd search "login timeout"

# Buscar por contenido de descripcion
bd list --desc-contains "authentication"

# Mostrar detalles de un issue
bd show bd-abc123

# Ver detalles en formato largo
bd list --long
```

### Contar issues

```bash
# Contar issues abiertos
bd count

# Contar por label
bd count -l frontend

# Contar bugs abiertos
bd count -t bug
```

## Actualizar Issues

### Update basico

```bash
# Actualizar titulo
bd update bd-abc123 --title "New title"

# Cambiar prioridad
bd update bd-abc123 -p 0

# Cambiar status
bd update bd-abc123 -s in_progress

# Asignar
bd update bd-abc123 -a knowledge-4yh

# Agregar labels
bd update bd-abc123 --add-label urgent,reviewed

# Remover labels
bd update bd-abc123 --remove-label wip

# Reemplazar todos los labels
bd update bd-abc123 --set-labels frontend,reviewed

# Agregar notas
bd update bd-abc123 --append-notes "Probado en staging, funciona"

# Establecer due date
bd update bd-abc123 --due "+2d"           # En 2 dias
bd update bd-abc123 --due "next monday"
bd update bd-abc123 --due "2025-03-15"

# Diferir un issue
bd update bd-abc123 --defer "+1w"          # Diferir 1 semana
```

### Claim atomico

```bash
# Claim atomico — falla si ya fue tomado por otro
bd update bd-abc123 --claim

# Equivale a:
# bd update bd-abc123 -a $USER -s in_progress
# Pero es atomico: si alguien mas lo tomo primero, falla
```

### Cerrar y reabrir

```bash
# Cerrar un issue
bd close bd-abc123

# Cerrar multiples
bd close bd-abc123 bd-def456

# Reabrir
bd reopen bd-abc123
```

## Comentarios

### Agregar y ver comentarios

```bash
# Ver comentarios de un issue
bd comments bd-abc123

# Agregar comentario
bd comments add bd-abc123 "Investigando el bug — parece ser race condition"

# Agregar comentario largo desde archivo
bd comments add bd-abc123 -f analysis.txt

# Patron: progress updates como comentarios
bd comments add bd-abc123 "[Backend Agent] Endpoint implementado, falta testing"
bd comments add bd-abc123 "[Frontend Agent] UI lista, esperando API"
```

## Dependencias

### Crear dependencias

```bash
# bd-xyz bloquea a bd-abc (bd-abc depende de bd-xyz)
bd dep bd-xyz --blocks bd-abc

# Equivalente:
bd dep add bd-abc bd-xyz

# Crear relacion bidireccional (relates-to)
bd dep relate bd-abc bd-def

# Remover dependencia
bd dep remove bd-abc bd-xyz

# Remover relacion
bd dep unrelate bd-abc bd-def
```

### Visualizar dependencias

```bash
# Listar dependencias de un issue
bd dep list bd-abc123

# Arbol de dependencias
bd dep tree bd-abc123

# Grafo de dependencias (Graphviz)
bd graph
bd graph --format dot | dot -Tpng -o deps.png

# Detectar ciclos
bd dep cycles

# Ver issues bloqueados
bd blocked
```

## Epics

### Crear y gestionar epics

```bash
# Crear epic
bd create "Auth System Redesign" -t epic -p 1 -l backend

# Crear tareas hijas del epic
bd create "Design new auth flow" -t task --parent bd-epic-id -l backend
bd create "Implement JWT middleware" -t task --parent bd-epic-id -l backend
bd create "Write auth tests" -t task --parent bd-epic-id -l backend,qa

# Ver status del epic (% completado)
bd epic status bd-epic-id

# Ver hijos de un epic
bd children bd-epic-id

# Listar issues de un epic
bd list --parent bd-epic-id

# Cerrar epics elegibles (todos los hijos cerrados)
bd epic close-eligible
```

## Agentes

### Estado de agentes

```bash
# Estados de agente:
# idle      — Esperando trabajo
# spawning  — Iniciando
# running   — Ejecutando (general)
# working   — Trabajando activamente
# stuck     — Bloqueado, necesita ayuda
# done      — Termino su trabajo actual
# stopped   — Apagado limpio
# dead      — Muerto sin shutdown (detectado por timeout)

# Cambiar estado
bd agent state knowledge-4yh working
bd agent state knowledge-4yh done

# Heartbeat (actualiza last_activity)
bd agent heartbeat knowledge-4yh

# Ver detalles de un agente
bd agent show knowledge-4yh
```

### Workflow tipico de un agente

```bash
# 1. Buscar trabajo disponible
bd ready -l frontend

# 2. Tomar un issue (claim atomico)
bd update bd-abc123 --claim
bd agent state knowledge-4yh working

# 3. Reportar progreso
bd comments add bd-abc123 "[Frontend Agent] Implementando componente..."

# 4. Completar el trabajo
bd comments add bd-abc123 "[Frontend Agent] Completado. Tests pasan."
bd close bd-abc123
bd agent state knowledge-4yh done

# 5. Sincronizar con git
bd sync
git add .beads/issues.jsonl
git commit -m "Frontend: implement dark mode toggle"
git push
```

## Gates (Coordinacion Asincrona)

### Tipos de gates

```bash
# human   — Requiere cierre manual
# timer   — Expira despues de timeout
# gh:run  — Espera workflow de GitHub Actions
# gh:pr   — Espera merge de PR
# bead    — Espera que un bead en otro rig se cierre

# Listar gates abiertos
bd gate list

# Listar todos (incluyendo cerrados)
bd gate list --all

# Resolver un gate manualmente
bd gate resolve bd-gate-id

# Evaluar gates (cerrar los que se cumplieron)
bd gate check

# Evaluar solo bead gates
bd gate check --type=bead
```

## Git Sync

### Workflow de sincronizacion

```bash
# Exportar cambios a JSONL (prep para push)
bd sync

# Importar desde JSONL (despues de pull)
bd sync --import

# Ver estado de sincronizacion
bd sync --status

# Sync completo (pull + merge + export + commit + push)
bd sync --full

# Dry run — ver que cambiaria
bd sync --dry-run
```

### Resolver conflictos

```bash
# Resolver con estrategia configurada
bd sync --resolve

# Resolver — mantener version local
bd sync --resolve --ours

# Resolver — mantener version remota
bd sync --resolve --theirs

# Resolucion interactiva
bd sync --resolve --manual
```

### Workflow con git

```bash
# Patron estandar despues de trabajar
bd sync                           # Exportar a JSONL
git add .beads/issues.jsonl       # Stage changes
git commit -m "Agent: description of work"
git push

# Patron despues de pull
git pull --rebase
bd sync --import                  # Importar cambios remotos
```

## Queries Avanzadas

```bash
# Query language
bd query "status=open AND priority<2 AND label=backend"
bd query "type=bug AND assignee=knowledge-vlf"

# Issues stale (sin actualizar recientemente)
bd stale
bd stale --days 14                # Sin actividad por 14 dias

# Buscar duplicados
bd find-duplicates

# Lint — verificar issues incompletos
bd lint

# Status overview
bd status
```

## Patrones Multi-Agente

### Coordinacion entre agentes

```bash
# Agente crea tarea para otro agente
bd create "Setup staging database" \
  -t chore -p 1 -l devops \
  --assignee knowledge-w5p

# Referencia cruzada en comentarios
bd comments add bd-abc "[Frontend] @knowledge-vlf Necesito el endpoint GET /api/users"
bd comments add bd-def "[Backend] Blocked by bd-abc (esperando schema de DB)"

# Verificar estado de agentes
bd list -l "gt:agent"
bd agent show knowledge-4yh

# Crear dependencia entre tareas de distintos agentes
bd dep bd-backend-task --blocks bd-frontend-task
```

### Patron: Planner crea, agents ejecutan

```bash
# Planner crea el epic y tareas
bd create "User Dashboard" -t epic -p 1

bd create "Design dashboard API" \
  -t task -p 1 -l backend \
  --parent bd-epic-id \
  --assignee knowledge-vlf

bd create "Build dashboard UI" \
  -t task -p 1 -l frontend \
  --parent bd-epic-id \
  --assignee knowledge-4yh

# Crear dependencia: UI depende de API
bd dep bd-api-task --blocks bd-ui-task

# Cada agente trabaja su parte
# Backend:
bd update bd-api-task --claim
bd agent state knowledge-vlf working
# ... trabajo ...
bd close bd-api-task

# Frontend (ahora desbloqueado):
bd ready -l frontend    # bd-ui-task aparece
bd update bd-ui-task --claim
```

## Landing the Plane (Session Completion)

Cuando terminas una sesion de trabajo, SIEMPRE completar estos pasos:

```bash
# 1. Crear issues para trabajo pendiente
bd create "TODO: add error handling to parser" -t task -p 2 -l backend

# 2. Actualizar estado de issues
bd comments add bd-current "[Agent] Progress: 80% complete, falta testing"
bd close bd-completed-task

# 3. Actualizar estado del agente
bd agent state knowledge-vlf done

# 4. PUSH a remoto (MANDATORIO)
git pull --rebase
bd sync
git add .beads/issues.jsonl
git commit -m "Backend: implement user API endpoints"
git push

# 5. Verificar
git status  # Debe mostrar "up to date with origin"
```

## Configuracion

```bash
# Ver configuracion actual
bd config list

# Configurar estrategia de conflictos
bd config set conflict.strategy ours

# Configurar sync mode
bd sync mode --set-mode git-portable

# Modos de sync:
# git-portable         — JSONL export para repos git normales
# realtime             — Sync en tiempo real
# dolt-native          — Usa Dolt como backend
# belt-and-suspenders  — Ambos
```

## Mejores Practicas

### DO

- Usar `bd update --claim` para tomar issues (atomico, evita race conditions)
- Reportar progreso con `bd comments add` regularmente
- Cerrar tus propios issues cuando estan completos
- Usar `bd sync` + `git push` al final de cada sesion
- Crear issues para trabajo pendiente antes de terminar
- Usar dependencias (`bd dep`) para coordinar entre agentes
- Usar epics para agrupar tareas relacionadas
- Asignar prioridades correctas (P0 = produccion down)
- Usar labels consistentes por equipo (frontend, backend, devops, qa)
- Verificar `bd ready` antes de buscar trabajo nuevo
- Usar `--parent` para crear jerarquias claras

### DON'T

- Dejar issues en `in_progress` sin trabajar — cerrar o liberar
- Olvidar hacer `git push` al terminar — el trabajo se pierde
- Crear issues sin tipo ni prioridad — dificulta el triage
- Ignorar dependencias — llevan a trabajo duplicado o bloqueado
- Editar `.beads/issues.jsonl` manualmente — usar comandos `bd`
- Commitear `.beads/beads.db` — solo el JSONL va a git
- Usar `bd list` en vez de `bd ready` para buscar trabajo — `ready` filtra bloqueados
- Cerrar issues de otros agentes sin coordinacion
- Crear epics sin hijos — un epic vacio no tiene valor
- Ignorar `bd doctor` si algo falla — diagnostica problemas

## Referencia Rapida

```bash
# Crear
bd create "titulo" -t tipo -p prioridad -l label1,label2
bd q "quick capture"

# Listar/Buscar
bd list -l label -a assignee -t type
bd ready -l label
bd search "texto"
bd show bd-id

# Actualizar
bd update bd-id --claim
bd update bd-id -s status -p priority
bd close bd-id
bd reopen bd-id

# Comentarios
bd comments bd-id
bd comments add bd-id "mensaje"

# Dependencias
bd dep bd-blocker --blocks bd-blocked
bd dep tree bd-id
bd blocked

# Epics
bd create "titulo" -t epic
bd create "hijo" --parent bd-epic-id
bd epic status bd-epic-id

# Agentes
bd agent state agent-id working|done|idle
bd agent show agent-id

# Sync
bd sync
bd sync --import
bd sync --status
bd sync --resolve

# Admin
bd status
bd doctor
bd stale
bd lint
```

## Recursos

- `bd --help` y `bd <command> --help` para referencia completa
- `bd quickstart` para guia rapida interactiva
- `bd human` para comandos esenciales para humanos
- `bd prime` para contexto optimizado para AI
- `bd onboard` para snippet de AGENTS.md
