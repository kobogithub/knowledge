---

description: "Task list for 005-agent-team-projects"
---

# Tasks: Cadena de producto sobre spec-kit (EPIC-01)

**Input**: Design documents from `specs/005-agent-team-projects/`

**Prerequisites**: `plan.md`, `spec.md`, `docs/product/PRD.md` (Aprobado), `docs/product/stories/EPIC-01/` (Aprobadas)

**Tests**: Sin tareas de test automatizado nuevas salvo el ajuste del test existente de
`claude_md.rs`. La verificación es el quickstart V1–V10 de `plan.md`, ejecutado por QA
contra los escenarios Gherkin de cada story.

> **Compuerta**: este `tasks.md` se generó en la primera corrida con `spec.md` en Borrador
> (excepción documentada en `docs/STATUS.md`). **No se ejecuta `/speckit-implement` hasta
> que `PROJECT.md`, `PRD.md`, US-01 a US-05 y `spec.md` tengan Estado "Aprobado".**

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede correr en paralelo (archivos distintos, sin dependencias)
- **[Story]**: User story a la que pertenece (US1–US5, mismos IDs que `docs/product/stories/EPIC-01/`)
- Cada tarea incluye la ruta exacta

## Path Conventions

Rutas relativas a la raíz del repo. Ramas de trabajo: `005-agent-team-projects/<rol>`
desde `epic/005-agent-team-projects`, PR a la rama epic.

## Assignment by role

| Phase | Role |
|---|---|
| Phase 1 Setup | **Planner** + **mantenedor** (firmas) |
| Phase 2 Foundational | **Planner** (gate skill), **Docs Writer** (plantillas) |
| Phase 3 US1 | **Docs Writer** (Markdown) + **Rust** (`claude_md.rs`) |
| Phase 4 US2 | **Planner** (rol Analyst, skill) + **Docs Writer** (plantilla) |
| Phase 5 US3 | **Planner** + **Docs Writer** (ADR-008) |
| Phase 6 US4 | **Planner** + **QA** (regla en su AGENTS.md) |
| Phase 7 US5 | **Planner** + **mantenedor** (firmas) |
| Phase 8 Polish | **QA** (quickstart V1–V10), **Docs Writer** (índices) |

---

## Phase 1: Setup (Prerequisites)

**Purpose**: cerrar las compuertas que esta primera corrida dejó abiertas.

- [ ] T001 **MANTENEDOR** — Revisar y firmar `docs/product/PROJECT.md` (Estado "Aprobado", Firmado por, Fecha de firma). Corregir antes de firmar lo que no refleje el input de discovery. **Bloquea todo lo demás**
- [ ] T002 **MANTENEDOR** — Revisar y firmar `docs/product/PRD.md`. Confirmar el orden de épicas y sprints. Responder o dejar explícitas las preguntas abiertas
- [ ] T003 **MANTENEDOR** — Revisar y firmar `docs/product/stories/EPIC-01/US-01.md` a `US-05.md`
- [ ] T004 **MANTENEDOR** — Revisar y firmar `specs/005-agent-team-projects/spec.md`. Recién acá pasa la compuerta de `/speckit-implement`
- [ ] T005 Planner — Crear `epic/005-agent-team-projects` desde `dev` y mover esta carpeta y `docs/product/` a esa rama vía PR (la primera corrida quedó en `claude/agent-team-projects-6r7rtg`)
- [ ] T006 [P] Planner — Confirmar que los quality gates de la CLI están verdes antes de tocarla: `cd cli && cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`. Cualquier fallo es preexistente: reportarlo, no absorberlo

**Checkpoint**: seis artefactos firmados, rama epic creada, baseline de la CLI verde.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: lo que todos los comandos comparten: la regla de compuerta y la excepción de `.gitignore` para versionarlos.

**⚠️ CRITICAL**: sin T007 y T008 ningún skill `product-*` se versiona ni puede verificar firmas.

- [ ] T007 Planner — Agregar a `.gitignore`, junto a la excepción `!.claude/skills/speckit-*/`, la línea `!.claude/skills/product-*/` con un comentario que explique por qué (FR-005)
- [ ] T008 Planner — Escribir `.claude/skills/product-gate/SKILL.md`: recibe ruta de artefacto (y opcionalmente ID de épica), parsea `**Estado**:`, `**Firmado por**:` y `**Fecha de firma**:`, responde "PASA" o "SE DETIENE: <archivo> tiene Estado <x>". Si el repo no tiene `docs/product/`, informa y deja avanzar (edge case del spec). Con ID de épica, además verifica que la épica esté en la tabla de `PRD.md` y que exista `docs/product/stories/<EPIC>/` con al menos una story (FR-006)
- [ ] T009 [P] Docs Writer — Crear `docs/product/templates/PROJECT.md` con encabezado de firma y las diez secciones del input de discovery, con placeholders `[...]` (FR-004)
- [ ] T010 [P] Docs Writer — Crear `docs/product/templates/PRD.md` (encabezado de firma, Resumen, Decisiones de estructura, tabla de épicas con columnas ID/Épica/Prioridad/Sprint/Spec/Estado, sección por épica, Roadmap, Fuera de alcance, Riesgos, Preguntas abiertas) (FR-004)
- [ ] T011 [P] Docs Writer — Crear `docs/product/templates/US.md` (encabezado con Épica/Estado/Firma/Prioridad/Rol ejecutor, frase Como/quiero/para, bloque ```gherkin, Notas / dependencias) (FR-004)

**Checkpoint**: `product-gate` existe y las tres plantillas están versionadas.

---

## Phase 3: User Story 1 — Referencias rotas saneadas (Priority: P1) 🎯 MVP

**Goal**: ningún agente falla al inicializarse en un clon fresco.

**Independent Test**: quickstart V1–V4.

- [ ] T012 [US1] Planner — En `CLAUDE.md` reemplazar `@.claude/agents/planner.md` por `@agents/planner/AGENTS.md` y agregar una línea que aclare que `kn sync` crea además el symlink en `.claude/agents/` (research R2, FR-002)
- [ ] T013 [US1] Rust — En `cli/src/core/claude_md.rs` cambiar el import generado a `@agents/planner/AGENTS.md`, sumar `analyst` a la lista de roles del texto generado, y ajustar el test `assert!(content.contains(...))` de la línea 66. Correr `cargo fmt`, `clippy -D warnings`, `cargo test` (FR-002)
- [ ] T014 [P] [US1] Docs Writer — En `agents/biz/AGENTS.md` quitar `notion-reporting-standard` de `required_skills` y de la sección "Skills Asignados"; reemplazar las referencias al skill por una referencia inline al estándar de tres secciones y semáforo de `docs/reports/README.md` (research R3, FR-001)
- [ ] T015 [P] [US1] Docs Writer — En `README.md` y `README_ES.md` eliminar la sección de `kn beads template` (líneas ~261–264 del README en inglés y su equivalente en español); verificar que no queden otras menciones a beads fuera de `docs/adr/` y `CHANGELOG.md` (FR-001)
- [ ] T016 [P] [US1] Docs Writer — Reescribir `.agent/README.md`: lista de skills tomada de `skills/` (sin `bd-best-practices` ni `aws-best-practices`), conteo correcto (FR-001)
- [ ] T017 [US1] QA — Ejecutar V1–V4 y dejar el resultado como comentario en el PR de la story, un renglón por Scenario de `US-01.md` con pasa/falla

**Checkpoint**: V1–V4 en verde; US-01 marcable como cerrada.

---

## Phase 4: User Story 2 — Discovery con rol Analyst produce PROJECT.md (Priority: P1)

**Goal**: `/product-discovery` genera `PROJECT.md` sin inventar datos.

**Independent Test**: quickstart V5 + escenarios 2–4 de `US-02.md` con un input de prueba.

- [ ] T018 [US2] Planner — Crear `agents/analyst/AGENTS.md` con frontmatter (`name: analyst`, `id_prefix: an1`, `model: sonnet`, `description`, `required_skills: [documentation-guide]`, `mcp_servers: github`) y cuerpo en español: responsabilidad exclusiva de relevamiento, prohibición explícita de escribir PRD/stories/specs, workflow `/product-discovery`, regla "lo que no está en el input va a Preguntas abiertas", Landing the Plane (FR-003)
- [ ] T019 [US2] Planner — Agregar la fila **Analyst Agent** (`knowledge-an1`) a la tabla de roles de `AGENTS.md` y el bloque `[agents.analyst]` en `kn.toml` (FR-003)
- [ ] T020 [US2] Planner — Escribir `.claude/skills/product-discovery/SKILL.md`: argumento = ruta del input en `docs/product/discovery/`; carga `docs/product/templates/PROJECT.md`; genera `docs/product/PROJECT.md` con Estado "Borrador", link al input, y cada dato faltante como pregunta abierta; si `PROJECT.md` ya existe con Estado "Aprobado", se detiene y pide crear uno nuevo marcando el anterior como "Reemplazado" (FR-005, FR-008)
- [ ] T021 [P] [US2] Docs Writer — Crear `docs/product/README.md` (si no existe ya) describiendo la cadena, las compuertas, los comandos y dónde va cada archivo; enlazar desde `docs/README.md`
- [ ] T022 [US2] QA — Con un input de prueba sin presupuesto ni fechas, correr `/product-discovery` en una rama descartable y validar los escenarios 2–4 de `US-02.md`; comentar el resultado en el PR

**Checkpoint**: rol Analyst instalable, comando funcionando, plantilla en uso.

---

## Phase 5: User Story 3 — PRD con épicas derivado de un PROJECT.md aprobado (Priority: P1)

**Goal**: `/product-prd` produce el contrato y se niega sin firma.

**Independent Test**: quickstart V6–V7.

- [ ] T023 [US3] Planner — Escribir `.claude/skills/product-prd/SKILL.md`: primer paso invoca `product-gate docs/product/PROJECT.md`; carga `docs/product/templates/PRD.md`; genera `docs/product/PRD.md` en Borrador con tabla de épicas, sección por épica, roadmap y riesgos; hereda Preguntas abiertas del PROJECT (FR-005, FR-006)
- [ ] T024 [US3] Docs Writer — Escribir `docs/adr/008-product-layer-over-speckit.md` (plantilla `000-template.md`): contexto (guía de discovery, brechas), decisión (capa `docs/product/`, épica ↔ `specs/NNN`, la story manda, encabezado de firma, comandos como skills, hooks de spec-kit), consecuencias, alternativas descartadas (BMAD/OpenSpec como herramienta, spec por story). Agregar la fila en `docs/adr/README.md` (FR-010)
- [ ] T025 [US3] Planner — Documentar en `agents/planner/AGENTS.md` la cadena completa antes de la "REGLA FUNDAMENTAL": `/product-prd` → `/product-stories` → `/speckit-specify` por épica, con la tabla de compuertas del `plan.md`; actualizar el ejemplo práctico para que arranque desde una épica del PRD y no desde una frase del usuario (FR-011)
- [ ] T026 [US3] QA — Ejecutar V6 y V7 sobre una rama descartable; comentar en el PR

**Checkpoint**: PRD generable solo desde PROJECT aprobado; decisión registrada en ADR.

---

## Phase 6: User Story 4 — Stories con Gherkin derivadas de una épica aprobada (Priority: P1)

**Goal**: `/product-stories` produce un archivo por story y el spec las deriva.

**Independent Test**: escenarios 1–3 de `US-04.md` (el 4 se prueba completo en EPIC-02).

- [ ] T027 [US4] Planner — Escribir `.claude/skills/product-stories/SKILL.md`: argumento = `EPIC-xx`; invoca `product-gate docs/product/PRD.md EPIC-xx`; carga `docs/product/templates/US.md`; crea `docs/product/stories/EPIC-xx/US-NN.md` numeradas desde la última existente; cada una en Borrador (FR-005, FR-006)
- [ ] T028 [US4] Planner — Agregar a `AGENTS.md` (sección "Quick Reference") los tres comandos `product-*` y la regla "las User Story N del spec se derivan de `US-NN.md`: mismo ID, mismos escenarios; si divergen manda la story" (FR-009, FR-011)
- [ ] T029 [P] [US4] QA — En `agents/qa/AGENTS.md` agregar la sección "Validación contra Gherkin": para cada PR que referencie `US-xx`, listar cada Scenario con pasa/falla como comentario del PR antes de aprobar; un Scenario que falla bloquea el merge (FR-011)
- [ ] T030 [US4] QA — Validar escenarios 1–3 de `US-04.md` en una rama descartable; comentar en el PR

**Checkpoint**: stories generables solo desde PRD aprobado; regla de derivación documentada.

---

## Phase 7: User Story 5 — Compuertas de firma verificadas por los agentes (Priority: P1)

**Goal**: `/speckit-specify` y `/speckit-plan` respetan la compuerta; la iniciativa queda firmada.

**Independent Test**: quickstart V8 y V10.

- [ ] T031 [US5] Planner — Crear `.specify/extensions.yml` con `hooks.before_specify` y `hooks.before_plan` apuntando a `product-gate` (forma del `plan.md`, sección Design). Verificar contra `.claude/skills/speckit-plan/SKILL.md` cómo construye el slash command y si un hook sin `optional` bloquea (FR-007)
- [ ] T032 [US5] Planner — Si T031 demuestra que el hook **no** bloquea, registrar el hallazgo en `plan.md` (research R1) y agregar la verificación como paso obligatorio en `agents/planner/AGENTS.md` antes de `/speckit-specify` y `/speckit-plan` (Assumptions del spec)
- [ ] T033 [US5] Planner — Verificar V8: con `specs/005-agent-team-projects/spec.md` temporalmente en Borrador, `/speckit-plan` se detiene sin escribir; restaurar el estado
- [ ] T034 [US5] Planner — Verificar V9: `/speckit-analyze` sobre `specs/004-macos-arm64-homebrew/` sigue pasando sin cambios (la compuerta no bloquea iniciativas sin capa de producto)
- [ ] T035 [US5] **MANTENEDOR** — Confirmar V10: los seis artefactos de EPIC-01 (PROJECT, PRD, US-01..05, spec) en Estado "Aprobado" con fecha; registrar en `docs/STATUS.md` que la excepción de la primera corrida quedó cerrada

**Checkpoint**: compuertas activas en spec-kit; cadena propia firmada de punta a punta.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [ ] T036 [P] QA — Ejecutar el quickstart completo V1–V10 desde un clon fresco y dejar el reporte en `docs/reports/<fecha>-qa-005.md`
- [ ] T037 [P] Docs Writer — Actualizar `docs/README.md` (sección "Initiatives": la cadena empieza en `docs/product/`), `docs/ARCHITECTURE.md` (sección 4.5 spec-kit: agregar la capa de producto y los hooks) y `CHANGELOG.md` `[Unreleased]` (Added: rol analyst, comandos product-*, ADR-008; Fixed: import de CLAUDE.md, skill del Biz Agent, READMEs)
- [ ] T038 Planner — Regenerar `docs/STATUS.md` a mano al cierre del sprint (EPIC-02 lo automatiza): stories cerradas, PRs sin review, preguntas abiertas
- [ ] T039 Planner — Correr `/speckit-converge` sobre esta carpeta y cerrar la iniciativa cuando no queden checkboxes abiertos; mergear `epic/005-agent-team-projects` → `dev` por PR
- [ ] T040 Planner — Seguimiento para EPIC-02: correr `/speckit-constitution` con la convención de firma y "la story manda" como principios candidatos (Constitution Check del `plan.md`)

---

## Dependencies & Execution Order

- **T001–T004** (firmas) bloquean toda implementación: sin ellas no se corre `/speckit-implement`.
- **Phase 2** bloquea las fases 4–7 (todas usan `product-gate` y las plantillas).
- **Phase 3 (US1)** es independiente de Phase 2 y puede arrancar apenas se firma; es el MVP.
- **US2 → US3 → US4 → US5** en ese orden: cada comando verifica el artefacto que produce el anterior.
- Dentro de cada fase, las tareas `[P]` corren en paralelo entre roles distintos.

## Implementation Strategy

1. Firmas (T001–T004) y rama epic (T005).
2. MVP: US1 en paralelo con Phase 2.
3. US2, US3, US4, US5 en orden, cada una con su verificación de QA como comentario de PR.
4. Polish y cierre con `/speckit-converge`.
