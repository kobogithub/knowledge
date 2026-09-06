# PRD: Metodología de proyectos con equipo de agentes

**Estado**: Borrador — pendiente de firma
**Firmado por**: —
**Fecha de firma**: —
**Agente autor**: Planner (`knowledge-x6e`)
**Derivado de**: [`PROJECT.md`](./PROJECT.md) (etapa 1)

> Etapa 2 de la cadena. Este documento es el **contrato**: lo que está acá es el alcance;
> lo que no está es una story nueva en un sprint nuevo. Cada épica entra a un sprint solo
> cuando el PRD está firmado, y recién entonces se le crea su `specs/NNN-*`.

## Resumen

Extender `knowledge` con la capa de producto que le falta por encima de spec-kit, con
compuertas de firma, seguimiento PM y scaffolding, validando todo con esta misma
iniciativa antes del primer proyecto piloto.

## Decisiones de estructura

Estas decisiones adaptan la guía a lo que `knowledge` ya tiene. Quedan registradas acá y
se formalizan en un ADR dentro de EPIC-01.

1. **Una épica del PRD equivale a una carpeta `specs/NNN-nombre/`.** La guía propone un
   spec por user story; spec-kit ya agrupa varias user stories por spec, así que la
   unidad de spec es la épica.
2. **Las user stories viven en `docs/product/stories/EPIC-xx/US-xx.md` con Gherkin, y
   son la fuente.** Las secciones "User Story N" del `spec.md` se derivan de ellas:
   mismo ID, mismos escenarios. Si divergen, manda la story.
3. **Roles**: Analyst es el único rol nuevo. PM → Planner. Architect → Planner (por
   ahora, ver pregunta abierta). Dev → Frontend / Backend / Rust. QA → QA.
4. **Compuerta = encabezado.** Cada artefacto lleva `Estado`, `Firmado por` y
   `Fecha de firma`. Valores de estado: `Borrador`, `En revisión`, `Aprobado`,
   `Reemplazado`. Los agentes verifican el estado del artefacto anterior antes de derivar
   el siguiente.
5. **Los comandos por compuerta son skills de Claude Code** (`.claude/skills/product-*/`),
   no `.claude/commands/`, porque así ya se distribuyen los `speckit-*`.
6. **Tracker**: GitHub Issues vía `/speckit-taskstoissues`, publicado por el Planner
   después de aprobar `tasks.md`.

## Épicas

| ID | Épica | Prioridad | Sprint | Spec | Estado |
|---|---|---|---|---|---|
| EPIC-01 | Cadena de producto sobre spec-kit: saneamiento, rol Analyst, plantillas, comandos por compuerta, firmas, ADR | P1 | 1 | [`specs/005-agent-team-projects/`](../../specs/005-agent-team-projects/) | Spec en borrador |
| EPIC-02 | Seguimiento PM: `/status` → `docs/STATUS.md`, issues desde tasks, QA contra Gherkin en cada PR, plantilla de PR | P1 | 2 | se crea al entrar al sprint | Sin spec |
| EPIC-03 | Scaffolding en `kn init`: `docs/product/`, `docs/adr/`, `STATUS.md`, verificación de spec-kit en `kn doctor`, roles en `CLAUDE.md` generado | P2 | 2 | se crea al entrar al sprint | Sin spec |
| EPIC-04 | Automatización n8n: daily digest de issues/PRs y reporte semanal desde `STATUS.md` | P3 | 3 | se crea al entrar al sprint | Sin spec |
| EPIC-05 | Proyecto piloto: primer cliente real recorriendo la cadena completa en su propio repo | P2 | 3–4 | fuera de este repo | Sin spec |

### EPIC-01 — Cadena de producto sobre spec-kit (Sprint 1)

**Valor**: sin esto no hay cadena; el resto de las épicas construye encima.

**Stories** (detalle en [`stories/EPIC-01/`](./stories/EPIC-01/)):

| ID | Story | Rol ejecutor |
|---|---|---|
| [US-01](./stories/EPIC-01/US-01.md) | Referencias rotas saneadas | Docs Writer, Rust |
| [US-02](./stories/EPIC-01/US-02.md) | Discovery con rol Analyst produce `PROJECT.md` | Planner, Docs Writer |
| [US-03](./stories/EPIC-01/US-03.md) | PRD con épicas derivado de un `PROJECT.md` aprobado | Planner |
| [US-04](./stories/EPIC-01/US-04.md) | Stories con Gherkin derivadas de una épica aprobada | Planner |
| [US-05](./stories/EPIC-01/US-05.md) | Compuertas de firma verificadas por los agentes | Planner, Docs Writer, QA |

**Entregables visibles**: rol `analyst` instalable con `kn`, plantillas en `docs/product/`,
skills `/product-discovery`, `/product-prd`, `/product-stories`, ADR-008, encabezados de
firma en todos los artefactos, y esta iniciativa recorriendo su propia cadena.

### EPIC-02 — Seguimiento PM (Sprint 2)

**Valor**: el PM entiende el proyecto sin leer código; es el reporte semanal del contrato.

**Alcance previsto**: skill `/product-status` que regenera `docs/STATUS.md` con stories
del sprint (planificadas vs. cerradas), specs sin issue, issues sin PR, PRs sin review
> 24 h, riesgos y preguntas abiertas, deuda técnica. Planner publica issues con
`/speckit-taskstoissues` al aprobar `tasks.md`. QA valida cada PR contra los escenarios
Gherkin de la story y lo deja como comentario. La plantilla de PR gana la sección
"Story y escenarios validados".

### EPIC-03 — Scaffolding en `kn init` (Sprint 2)

**Valor**: cada proyecto de cliente arranca con la cadena lista, en segundos.

**Alcance previsto**: `kn init` crea `docs/product/{PROJECT,PRD}.md`, `docs/product/stories/`,
`docs/adr/000-template.md` y `docs/STATUS.md` desde plantillas embebidas; `kn doctor`
avisa si `.specify/` no existe; el `CLAUDE.md` generado nombra al rol `analyst` y no
importa archivos que solo existen tras `kn sync`.

### EPIC-04 — Automatización n8n (Sprint 3)

**Valor**: daily y reporte semanal sin intervención manual.

**Alcance previsto**: un flujo n8n que lee `STATUS.md` e issues/PRs y envía el digest
diario y el reporte semanal. Depende de EPIC-02.

### EPIC-05 — Proyecto piloto (Sprints 3–4)

**Valor**: valida el modelo comercial con un cliente real.

**Alcance previsto**: repo del cliente inicializado con `kn init`, cadena completa con
firmas, un sprint de prueba y retro que vuelve a `CLAUDE.md`, skills o ADRs de `knowledge`.

## Roadmap

| Sprint | Semana | Épicas | Hito |
|---|---|---|---|
| 1 | 1 | EPIC-01 | Cadena operable; esta iniciativa firmada de punta a punta |
| 2 | 2 | EPIC-02, EPIC-03 | `STATUS.md` regenerable; `kn init` scaffoldea la cadena |
| 3 | 3 | EPIC-04, arranque EPIC-05 | Daily automático; piloto inicializado |
| 4 | 4 | EPIC-05 | Primer sprint de cliente cerrado con demo y changelog |

## Fuera de alcance del PRD

Lo listado en "Fuera de alcance" de [`PROJECT.md`](./PROJECT.md): BMAD/OpenSpec como
herramientas, ClickUp, cambios al modelo comercial, plataformas fuera de Apple Silicon.

## Riesgos

| Riesgo | Prob. | Impacto | Respuesta |
|---|---|---|---|
| Stories y user stories del spec divergen | Media | Alto | Decisión 2: la story manda; `/speckit-analyze` chequea IDs |
| Las compuertas se saltean bajo presión de tiempo | Media | Alto | La verificación es del agente (US-05), no de la memoria del humano |
| El Planner no se sostiene como PM + Architect | Media | Medio | Pregunta abierta en PROJECT.md; se decide antes de EPIC-02 |
| EPIC-03 toca Rust y la CI de `kn` | Baja | Medio | Se aísla en su propia spec y rama; no bloquea EPIC-01/02 |
| El piloto no aparece a tiempo | Media | Bajo | EPIC-01 a 04 tienen valor en `knowledge` aunque no haya cliente |

## Preguntas abiertas (heredadas de PROJECT.md)

Idioma de los artefactos, rol Architect separado, proyecto piloto y fecha, dónde vive la
guía comercial. Ninguna bloquea EPIC-01.
