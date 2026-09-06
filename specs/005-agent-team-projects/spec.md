# Feature Specification: Cadena de producto sobre spec-kit (EPIC-01)

**Feature Branch**: `epic/005-agent-team-projects`

**Created**: 2026-09-06

**Estado**: Borrador — pendiente de firma
**Firmado por**: —
**Fecha de firma**: —

**Épica**: EPIC-01 de [`docs/product/PRD.md`](../../docs/product/PRD.md)

**Input**: PRD aprobado + stories [`docs/product/stories/EPIC-01/`](../../docs/product/stories/EPIC-01/).
Las user stories de abajo se **derivan** de esos archivos: mismo ID, mismos escenarios.
Si divergen, manda la story (decisión 2 del PRD).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Referencias rotas saneadas (Priority: P1)

Fuente: [US-01](../../docs/product/stories/EPIC-01/US-01.md). Un agente que arranca en un
clon fresco o en una sesión remota encuentra un `CLAUDE.md` que importa un archivo que
solo existe tras `kn sync`, un Biz Agent que exige un skill borrado del catálogo y READMEs
que documentan un comando removido. La story deja el repo sin referencias a artefactos
inexistentes.

**Why this priority**: destraba a todos los agentes; sin esto las stories siguientes
arrancan en un entorno inconsistente.

**Independent Test**: clonar el repo sin `kn sync`, cargar `CLAUDE.md`, y correr las
búsquedas de texto de los escenarios. No requiere nada de las otras stories.

**Acceptance Scenarios**:

1. **Given** un clon fresco sin `kn sync`, **When** Claude Code carga `CLAUDE.md`, **Then** todo archivo importado con `@` existe en el repo y el Planner sigue siendo el rol por defecto.
2. **Given** el generador de `CLAUDE.md` en la CLI, **When** corren los tests de la CLI, **Then** el contenido generado no importa rutas que solo existen tras `kn sync` y los tests reflejan el nuevo contenido.
3. **Given** `agents/biz/AGENTS.md`, **When** se comparan sus `required_skills` con `skills/`, **Then** no hay skill requerido inexistente.
4. **Given** `README.md`, `README_ES.md` y `.agent/README.md`, **When** se busca "beads", "bd-best-practices" o "aws-best-practices", **Then** no hay ocurrencias fuera de `docs/adr/` y `CHANGELOG.md`.

---

### User Story 2 - Discovery con rol Analyst produce PROJECT.md (Priority: P1)

Fuente: [US-02](../../docs/product/stories/EPIC-01/US-02.md). El PM guarda la
transcripción de la reunión en `docs/product/discovery/`, corre `/product-discovery` como
Analyst y obtiene un `PROJECT.md` en borrador con las diez secciones de la guía, donde
todo dato ausente del input aparece como pregunta abierta.

**Why this priority**: es la primera etapa de la cadena y la que evita que el agente
invente requerimientos.

**Independent Test**: con un input de prueba sin presupuesto ni fechas, correr el comando
y verificar secciones, encabezado y que Restricciones no contiene cifras inventadas.

**Acceptance Scenarios**:

1. **Given** `agents/`, **When** se lista `agents/analyst/AGENTS.md`, **Then** tiene frontmatter con `name`, `id_prefix`, `model` y `description`, y `AGENTS.md` raíz y `kn.toml` lo declaran.
2. **Given** un input en `docs/product/discovery/<fecha>-<tema>.md`, **When** el Analyst corre `/product-discovery` sobre él, **Then** se crea `docs/product/PROJECT.md` con las secciones de la plantilla en orden, Estado "Borrador", firmante vacío y link al input, y todo dato ausente del input queda en Preguntas abiertas.
3. **Given** un input sin presupuesto ni fecha límite, **When** se genera `PROJECT.md`, **Then** Restricciones no inventa cifras ni fechas y Preguntas abiertas tiene una pregunta por dato faltante.
4. **Given** el repo, **When** se busca la plantilla de `PROJECT.md`, **Then** existe una con placeholders y las diez secciones de la guía.

---

### User Story 3 - PRD con épicas derivado de un PROJECT.md aprobado (Priority: P1)

Fuente: [US-03](../../docs/product/stories/EPIC-01/US-03.md). El Planner corre
`/product-prd` sobre un `PROJECT.md` aprobado y obtiene el contrato de alcance: épicas
con prioridad, sprint y spec asociado, roadmap y riesgos.

**Why this priority**: sin PRD firmado no hay contrato y no hay compuerta para las specs.

**Independent Test**: correr `/product-prd` con `PROJECT.md` en Borrador (debe detenerse)
y luego con Estado Aprobado (debe generar el PRD con las tablas exigidas).

**Acceptance Scenarios**:

1. **Given** `PROJECT.md` con Estado "Borrador", **When** el Planner corre `/product-prd`, **Then** el comando se detiene sin escribir `PRD.md` e informa que falta la firma.
2. **Given** `PROJECT.md` con Estado "Aprobado", **When** el Planner corre `/product-prd`, **Then** se crea `PRD.md` con tabla de épicas (ID, prioridad, sprint, spec, estado), valor y alcance por épica, roadmap por sprint, riesgos, Estado "Borrador" y link a `PROJECT.md`.
3. **Given** una épica que entra a un sprint, **When** el Planner corre `/speckit-specify` para ella, **Then** la carpeta `specs/NNN-*/` queda referenciada en la columna Spec y `spec.md` referencia la épica en su encabezado.
4. **Given** un PRD aprobado, **When** aparece un requerimiento fuera de toda épica, **Then** se registra como story nueva para un sprint futuro sin modificar el alcance del sprint en curso.

---

### User Story 4 - Stories con Gherkin derivadas de una épica aprobada (Priority: P1)

Fuente: [US-04](../../docs/product/stories/EPIC-01/US-04.md). El Planner corre
`/product-stories EPIC-xx` y obtiene un archivo por story con Gherkin, que es la fuente
única para el spec, los tests y la validación de QA.

**Why this priority**: el Gherkin es el puente entre la capa de producto y spec-kit.

**Independent Test**: correr el comando con PRD en Borrador (se detiene) y con PRD
Aprobado (genera los archivos), luego verificar que `spec.md` copia los escenarios.

**Acceptance Scenarios**:

1. **Given** `PRD.md` con Estado "Borrador", **When** el Planner corre `/product-stories EPIC-01`, **Then** el comando se detiene sin escribir stories e informa que falta la firma.
2. **Given** un PRD aprobado con EPIC-01, **When** el Planner corre `/product-stories EPIC-01`, **Then** existe un `US-NN.md` por story con encabezado (Épica, Estado, Prioridad, Rol ejecutor), la frase "Como / quiero / para", un bloque gherkin con al menos un Scenario y una sección Notas / dependencias.
3. **Given** una story con tres escenarios, **When** el Planner corre `/speckit-specify` para su épica, **Then** `spec.md` tiene la "User Story N" correspondiente con link a la story y los mismos tres escenarios, y `/speckit-analyze` reporta divergencia si difieren.
4. **Given** un PR que referencia una story, **When** el QA Agent lo revisa, **Then** su comentario lista cada Scenario con resultado pasa/falla.

---

### User Story 5 - Compuertas de firma verificadas por los agentes (Priority: P1)

Fuente: [US-05](../../docs/product/stories/EPIC-01/US-05.md). Cada artefacto lleva
Estado, Firmado por y Fecha de firma; los agentes verifican el estado del artefacto
anterior antes de derivar el siguiente, y esta iniciativa queda firmada de punta a punta.

**Why this priority**: es lo que convierte la cadena en una protección y no en una
convención de nombres.

**Independent Test**: dejar `spec.md` en Borrador y correr `/speckit-plan` (se detiene);
aprobar y volver a correr (avanza). No requiere código de aplicación.

**Acceptance Scenarios**:

1. **Given** `PROJECT.md`, `PRD.md`, cada `US-xx.md` y cada `spec.md`, **When** se lee el encabezado, **Then** tiene Estado, Firmado por y Fecha de firma, con Estado en {Borrador, En revisión, Aprobado, Reemplazado}.
2. **Given** un `spec.md` con Estado "Borrador", **When** el Planner corre `/speckit-plan`, **Then** el comando se detiene sin escribir `plan.md` e informa qué artefacto necesita firma.
3. **Given** un PRD aprobado sin la épica EPIC-09, **When** el Planner corre `/speckit-specify` para EPIC-09, **Then** el comando se detiene y sugiere agregar la épica al PRD como cambio de alcance.
4. **Given** un artefacto "En revisión", **When** el humano completa firmante, fecha y cambia a "Aprobado", **Then** el siguiente comando avanza sin ningún otro paso.
5. **Given** los artefactos de EPIC-01, **When** se lee la cadena de punta a punta, **Then** `PROJECT.md`, `PRD.md`, US-01 a US-05 y `spec.md` tienen Estado "Aprobado" con fecha, y la única excepción de orden (esta primera corrida) está documentada en `docs/STATUS.md`.

---

### Edge Cases

- **Input de discovery vacío o sin sección reconocible**: `/product-discovery` genera un
  `PROJECT.md` con todas las secciones como preguntas abiertas y lo dice explícitamente;
  no aborta.
- **Artefacto aprobado que se edita después de la firma**: el estado pasa a "En revisión"
  y hay que volver a firmar. La regla se documenta; no se automatiza en EPIC-01.
- **Épica sin stories** al correr `/speckit-specify`: el comando se detiene y pide correr
  `/product-stories` primero.
- **Story con escenarios editados después de generado el spec**: `/speckit-analyze`
  reporta la divergencia; corregir el spec es una tarea nueva, no se sobrescribe solo.
- **Proyecto sin `docs/product/`** (repos anteriores a esta iniciativa): los hooks de
  compuerta no bloquean; informan que la capa de producto no está inicializada y dejan
  avanzar. Así 001–004 siguen siendo válidas.
- **`kn sync` sí corrió**: el `CLAUDE.md` saneado tiene que seguir funcionando cuando el
  symlink `.claude/agents/planner.md` existe.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El repo NO DEBE contener referencias a archivos, skills o comandos que no
  existan en el repo o en el catálogo vigente (`skills/`), excepto en `docs/adr/` y
  `CHANGELOG.md` como registro histórico.
- **FR-002**: El `CLAUDE.md` del repo y el generado por la CLI DEBEN cargar sin depender de
  archivos creados por `kn sync`, y DEBEN seguir funcionando cuando esos archivos existen.
- **FR-003**: DEBE existir un rol `analyst` en `agents/analyst/AGENTS.md`, declarado en la
  tabla de roles de `AGENTS.md` y en `kn.toml`, con responsabilidad exclusiva de
  relevamiento (no escribe PRD, stories ni specs).
- **FR-004**: DEBEN existir plantillas versionadas para `PROJECT.md`, `PRD.md` y `US-xx.md`
  con las secciones definidas en el input de discovery.
- **FR-005**: DEBEN existir los comandos `/product-discovery`, `/product-prd` y
  `/product-stories`, distribuidos como skills de Claude Code en `.claude/skills/product-*/`
  y versionados en git.
- **FR-006**: Cada comando de la cadena DEBE verificar que el artefacto anterior tenga
  Estado "Aprobado" con firmante y fecha, y detenerse informando el faltante si no.
- **FR-007**: `/speckit-specify` y `/speckit-plan` DEBEN respetar la misma compuerta
  mediante hooks `before_specify` y `before_plan` en `.specify/extensions.yml`, sin
  modificar los skills `speckit-*` generados por `specify-cli`.
- **FR-008**: Todo artefacto de la cadena DEBE llevar en su encabezado los campos Estado,
  Firmado por y Fecha de firma, con Estado en {Borrador, En revisión, Aprobado,
  Reemplazado}.
- **FR-009**: Las secciones "User Story N" de un `spec.md` DEBEN referenciar el archivo de
  story correspondiente y reproducir sus escenarios Gherkin; `/speckit-analyze` DEBE
  reportar divergencias de ID o escenarios.
- **FR-010**: La decisión de capa de producto, el mapeo épica ↔ spec y la convención de
  firma DEBEN quedar registrados en un ADR.
- **FR-011**: `AGENTS.md` y `agents/planner/AGENTS.md` DEBEN describir la cadena completa
  y las compuertas, y `agents/qa/AGENTS.md` DEBE establecer la validación de PRs contra
  el Gherkin de la story.
- **FR-012**: Esta iniciativa DEBE recorrer su propia cadena: sus artefactos de producto,
  spec, plan y tasks existen en el repo con encabezado de firma.

### Key Entities

- **Artefacto de cadena**: documento versionado con encabezado de firma; instancias:
  PROJECT, PRD, Story, Spec. Cada uno declara de qué artefacto se deriva.
- **Compuerta**: transición entre dos artefactos; pasa solo si el origen está Aprobado.
- **Épica**: unidad de alcance del PRD; mapea 1:1 a una carpeta `specs/NNN-*/`.
- **Story**: unidad de valor dentro de una épica; ID `US-NN`; contiene Gherkin; es la
  fuente de la "User Story N" del spec.
- **Rol**: agente responsable de un artefacto. Analyst → PROJECT; Planner → PRD, Stories,
  Spec, Plan, Tasks; roles técnicos → PRs; QA → validación.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un clon fresco del repo carga `CLAUDE.md` sin ningún import roto (cero
  archivos faltantes) y las tres búsquedas de texto de US-01 devuelven cero resultados
  fuera de `docs/adr/` y `CHANGELOG.md`.
- **SC-002**: Un PM recorre Discovery → PRD → Stories con tres comandos y sin editar a
  mano nada más que los encabezados de firma.
- **SC-003**: Correr `/product-prd`, `/product-stories`, `/speckit-specify` o
  `/speckit-plan` sobre un artefacto previo sin firma se detiene el 100% de las veces
  sin escribir el artefacto derivado.
- **SC-004**: Los cinco artefactos de producto de EPIC-01 más su `spec.md` tienen Estado
  "Aprobado" con firmante y fecha antes de que empiece EPIC-02.
- **SC-005**: Las iniciativas 001–004 siguen pasando `/speckit-analyze` sin cambios: la
  capa de producto no rompe lo existente.

## Assumptions

- El firmante es siempre el mantenedor humano; no hay firmas delegadas a agentes.
- El idioma de los artefactos de esta iniciativa es español; para proyectos de cliente
  queda como pregunta abierta en `PROJECT.md`.
- Los hooks de `.specify/extensions.yml` se ejecutan como pre-hooks obligatorios cuando se
  declaran sin `optional: true` (comportamiento leído en los skills `speckit-*`); si en la
  práctica no bloquean, la compuerta se implementa como paso inicial de los skills
  `product-*` y como nota en el Planner, y se registra en `plan.md`.
- El rol Analyst usa el modelo `sonnet`: sintetiza texto largo, no escribe código.
- Esta primera corrida escribe PROJECT, PRD, stories, spec, plan y tasks en una sola
  sesión como borradores; a partir de la firma rige la compuerta. La excepción queda
  documentada en `docs/STATUS.md` (escenario 5 de US-05).
- No se crea un rol Architect en esta épica; el Planner sigue produciendo `plan.md`.
