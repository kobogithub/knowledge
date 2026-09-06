# Implementation Plan: Cadena de producto sobre spec-kit (EPIC-01)

**Branch**: `epic/005-agent-team-projects` | **Date**: 2026-09-06 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/005-agent-team-projects/spec.md`

> **Nota de compuerta**: este plan se escribió en la misma sesión que el spec, con el spec
> todavía en Borrador. Es la excepción documentada en `docs/STATUS.md` para la primera
> corrida (US-05, escenario 5). A partir de la firma del spec rige FR-006/FR-007: ningún
> `plan.md` futuro se escribe sobre un spec sin Estado "Aprobado".

## Summary

Agregar a `knowledge` la capa que va *antes* del spec: un rol Analyst, tres artefactos de
producto (`PROJECT.md`, `PRD.md`, `US-xx.md`) con plantillas, tres comandos por compuerta
(`/product-discovery`, `/product-prd`, `/product-stories`) y una convención de firma que
los agentes verifican antes de derivar el siguiente artefacto. Primero se sanean cuatro
referencias rotas que hoy impiden arrancar limpio en un clon fresco.

No se agrega código de aplicación salvo un cambio de una línea en el generador de
`CLAUDE.md` de la CLI. Todo lo demás es Markdown: definiciones de agente, skills,
plantillas, un ADR y un archivo de hooks de spec-kit.

## Technical Context

**Language/Version**: Markdown (agentes, skills, plantillas, ADR). Rust 2021 solo para
`cli/src/core/claude_md.rs` (crate `kn` 0.11.0).

**Primary Dependencies**: spec-kit ya instalado (`.specify/`, `.claude/skills/speckit-*`).
Se usa su mecanismo de extensiones (`.specify/extensions.yml`, hooks `before_specify` /
`before_plan`) sin tocar los skills generados. Ninguna dependencia nueva.

**Storage**: N/A (archivos en git).

**Testing**: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` para el cambio
en la CLI. Para el resto, los escenarios Gherkin de `docs/product/stories/EPIC-01/`
ejecutados a mano por QA más aserciones `grep` sobre el repo (ver quickstart abajo).

**Target Platform**: repositorio git leído por Claude Code / OpenCode. La CLI sigue siendo
macOS Apple Silicon (sin cambios).

**Project Type**: framework de proceso (documentos + skills) con una CLI adjunta.

**Performance Goals**: N/A.

**Constraints**:
- No modificar `.claude/skills/speckit-*/` (los regenera `specify-cli`; FR-007).
- `.gitignore` ignora `.claude/skills/*` salvo `speckit-*`; los skills nuevos necesitan
  su propia excepción o no se versionan.
- `.claude/agents/*.md` está ignorado y lo crea `kn sync`; el `CLAUDE.md` saneado no puede
  depender de él (FR-002).
- No romper 001–004 (SC-005): los hooks no bloquean en repos sin `docs/product/`.

**Scale/Scope**: un mantenedor, un repo, cinco stories, ~20 archivos tocados.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

`.specify/memory/constitution.md` es la plantilla sin ratificar (placeholders intactos),
así que no impone principios verificables. La compuerta pasa por vacuidad. Se agrega una
tarea de seguimiento para correr `/speckit-constitution` en EPIC-02: la convención de
firma y la regla "la story manda" son candidatas a principios.

## Project Structure

### Documentation (this feature)

```text
specs/005-agent-team-projects/
├── spec.md                  # derivado de docs/product/stories/EPIC-01/
├── plan.md                  # este archivo
├── tasks.md                 # /speckit-tasks
└── checklists/requirements.md
```

Sin `research.md`, `data-model.md` ni `contracts/`: la investigación cabe abajo, no hay
modelo de datos y no hay API. Los "contratos" son las plantillas mismas.

### Source Code (repository root)

```text
docs/product/                     # capa de producto (nueva)
├── README.md                     # cómo funciona la cadena en este repo
├── PROJECT.md                    # etapa 1 (esta iniciativa)
├── PRD.md                        # etapa 2
├── discovery/<fecha>-<tema>.md   # inputs de discovery, inmutables
├── stories/EPIC-xx/US-xx.md      # etapa 3
└── templates/                    # PROJECT, PRD, US (nuevas)
docs/STATUS.md                    # vista PM (inicial; regenerable en EPIC-02)
docs/adr/008-product-layer-over-speckit.md   # nuevo
agents/analyst/AGENTS.md          # rol nuevo
agents/planner/AGENTS.md          # cadena y compuertas
agents/qa/AGENTS.md               # validación contra Gherkin
agents/biz/AGENTS.md              # skill removido
AGENTS.md                         # tabla de roles + cadena
CLAUDE.md                         # import saneado
.claude/skills/product-discovery/SKILL.md
.claude/skills/product-prd/SKILL.md
.claude/skills/product-stories/SKILL.md
.claude/skills/product-gate/SKILL.md          # verificación de firma reutilizable
.specify/extensions.yml           # hooks before_specify / before_plan → product-gate
.gitignore                        # excepción !.claude/skills/product-*/
kn.toml                           # [agents.analyst]
cli/src/core/claude_md.rs         # generador sin import roto
README.md, README_ES.md, .agent/README.md   # referencias removidas
```

## Research (Phase 0, resuelto en línea)

**R1 — ¿Dónde vive la verificación de compuerta?** Opciones: (a) dentro de cada skill
`product-*`; (b) hooks en `.specify/extensions.yml`; (c) editar los skills `speckit-*`.
Decisión: (a) para los comandos propios y (b) para `/speckit-specify` y `/speckit-plan`.
Los skills `speckit-*` leen `hooks.before_<comando>` y ejecutan los no-opcionales antes
de continuar; la verificación se factoriza en un skill `product-gate` que los hooks
invocan. (c) queda descartada: `specify-cli` regenera esos archivos. Riesgo: si en la
práctica un hook no bloquea, el fallback es la nota en el Planner (Assumptions del spec) y
se registra acá.

**R2 — ¿Cómo sanear el import de `CLAUDE.md`?** Opciones: (a) versionar el symlink; (b)
versionar una copia de `planner.md`; (c) reemplazar el import por un texto que apunte a
`agents/planner/AGENTS.md` con `@agents/planner/AGENTS.md`. Decisión: (c). El archivo
fuente ya está en el repo; el import apunta a él y sigue válido con o sin `kn sync`.
El generador de la CLI cambia igual, y su test se ajusta.

**R3 — ¿Qué reemplaza a `notion-reporting-standard` en el Biz Agent?** Decisión: quitar el
skill de `required_skills`, y dejar el formato de tres secciones y el semáforo que ya
describe `docs/reports/README.md` como referencia inline en el agente. No se crea un skill
nuevo (ADR-007 curó el catálogo a propósito).

**R4 — ¿Plantillas en `.specify/templates/` o en `docs/product/templates/`?** Decisión:
`docs/product/templates/`. Las de `.specify/` las administra `specify-cli` y son de la
capa spec; las de producto son de `knowledge`. EPIC-03 las embebe en `kn init`.

**R5 — Modelo del Analyst.** `sonnet`: síntesis de texto largo, sin código, costo medio.
Consistente con `docs/AGENT_MODEL_STRATEGY.md` (opus solo para el Planner).

## Design (Phase 1)

### Encabezado de firma (contrato común)

```markdown
**Estado**: Borrador | En revisión | Aprobado | Reemplazado
**Firmado por**: <nombre>
**Fecha de firma**: YYYY-MM-DD
```

`product-gate` lo parsea con una expresión fija: la línea `**Estado**:` del archivo que se
le pasa debe contener `Aprobado`, y `Firmado por` y `Fecha de firma` no pueden ser `—`.
Si el archivo no existe y el repo no tiene `docs/product/`, informa y deja avanzar
(edge case "proyecto sin capa de producto").

### Cadena de comandos y compuertas

| Comando | Verifica (Aprobado) | Produce | Rol |
|---|---|---|---|
| `/product-discovery <input>` | — | `docs/product/PROJECT.md` | Analyst |
| `/product-prd` | `PROJECT.md` | `docs/product/PRD.md` | Planner |
| `/product-stories EPIC-xx` | `PRD.md` y que la épica exista | `docs/product/stories/EPIC-xx/US-*.md` | Planner |
| `/speckit-specify` (hook) | `PRD.md`, épica en el PRD, stories de la épica | `specs/NNN/spec.md` | Planner |
| `/speckit-plan` (hook) | `specs/NNN/spec.md` | `plan.md` | Planner |

### Skill `product-gate`

Argumentos: ruta del artefacto y, opcionalmente, ID de épica. Salida: "PASA" o "SE
DETIENE: <archivo> tiene Estado <x>; completar Firmado por y Fecha de firma". Es el único
lugar donde vive la regla, para que cambiarla sea un solo archivo.

### `.specify/extensions.yml`

```yaml
hooks:
  before_specify:
    - extension: product-layer
      command: product-gate
      description: Exige PRD aprobado y stories de la épica antes de especificar
  before_plan:
    - extension: product-layer
      command: product-gate
      description: Exige spec.md aprobado antes de planificar
```

(Forma exacta a validar contra cómo `speckit-*` construye el slash command: `.` → `-`.)

## Quickstart (verificación)

| ID | Escenario | Cómo se verifica |
|---|---|---|
| V1 | Clon fresco carga `CLAUDE.md` | `git clone` a un dir temporal; cada `@ruta` de `CLAUDE.md` existe |
| V2 | CLI no genera import roto | `cd cli && cargo test claude_md` |
| V3 | Ningún agente con skill inexistente | script: cada `required_skills` y `recommended_skills` de `agents/*/AGENTS.md` existe en `skills/` |
| V4 | READMEs limpios | `grep -rn "beads\|bd-best-practices\|aws-best-practices" README*.md .agent/README.md` → solo links a `docs/adr/` |
| V5 | Analyst instalable | `agents/analyst/AGENTS.md` con frontmatter; fila en `AGENTS.md`; `[agents.analyst]` en `kn.toml` |
| V6 | Compuerta detiene | poner `PROJECT.md` en Borrador y correr `/product-prd` → no escribe |
| V7 | Compuerta avanza | aprobar y repetir → escribe |
| V8 | Hook en specify/plan | `spec.md` en Borrador + `/speckit-plan` → se detiene |
| V9 | 001–004 intactas | `/speckit-analyze` sobre 004 sin cambios |
| V10 | Cadena propia firmada | los seis artefactos de EPIC-01 en Aprobado con fecha |

## Complexity Tracking

Sin violaciones de constitución (no ratificada). La única pieza de código es un cambio de
cadena en `claude_md.rs`; no justifica abstracción nueva.
