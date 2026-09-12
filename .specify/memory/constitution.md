<!--
SYNC IMPACT REPORT
==================
Cambio de versión: plantilla sin ratificar → 1.0.0

Esta es la primera ratificación. El archivo era la plantilla de spec-kit con los
placeholders intactos (deuda T040 en docs/STATUS.md). No se inventó ningún principio:
los cinco salen de artefactos ya firmados el 2026-09-06 y de ADR aceptados.

Principios agregados (ninguno modificado ni removido — no había):
  I.   La story manda                      ← PRD decisión 2; AGENTS.md "La regla de derivación"
  II.  Una épica, una carpeta de spec       ← PRD decisión 1
  III. Todo artefacto vive en git           ← PROJECT.md (restricciones técnicas); PRD decisión 8
  IV.  El agente propone, el humano firma   ← docs/product/README.md (compuertas); PRD decisiones 4 y 9
  V.   spec-kit es la única capa de specs   ← ADR-006, ADR-008, ADR-009

Secciones agregadas:
  - Restricciones adicionales (ramas y releases, definición de agentes, commits)
  - Flujo de trabajo y compuertas
  - Governance

Plantillas y documentos dependientes:
  ✅ .specify/templates/plan-template.md — "Constitution Check" pasa de un placeholder
     genérico a las cinco compuertas verificables
  ✅ docs/STATUS.md — deuda T040 cerrada
  ✅ .specify/templates/spec-template.md — revisada, no requiere cambios: no impone ni
     contradice ninguna sección obligatoria de esta constitución
  ✅ .specify/templates/tasks-template.md — revisada, no requiere cambios: la
     categorización de tareas no depende de ninguno de los cinco principios
  ⚠  specs/005-agent-team-projects/plan.md — su "Constitution Check" dice que la compuerta
     "pasa por vacuidad". Queda como está a propósito: es el registro de lo que era cierto
     el 2026-09-06 y de dónde salió la tarea T040. Reescribirlo sería editar un artefacto
     cerrado.

Idioma: español, igual que los artefactos de los que se deriva (PROJECT.md, PRD.md,
docs/product/README.md, todos aprobados el 2026-09-06). Los encabezados fijos de la
plantilla de spec-kit se conservan en inglés para no romper lo que los busca por texto.

Ratificación: efectiva al mergear el PR que introduce este archivo. Ese merge es la firma
del mantenedor — por el Principio IV, ningún agente ratifica por su cuenta.

TODO diferidos: ninguno.
-->

# knowledge Constitution

Los principios que gobiernan este repositorio y todo proyecto que use `kn`. No son
aspiraciones: son las reglas que cada agente verifica antes de derivar el artefacto
siguiente. Lo que está acá prevalece sobre cualquier otra práctica del repo.

## Core Principles

### I. La story manda

Las secciones "User Story N" de un `spec.md` se derivan de
`docs/product/stories/EPIC-xx/US-NN.md`: mismo ID, mismos escenarios Gherkin.

- Todo `spec.md` MUST referenciar el ID de cada story de la que se deriva.
- Los escenarios Gherkin de un spec MUST ser copia de los de su story, no una reescritura.
- Cuando spec y story divergen, MUST corregirse el spec. La story nunca se ajusta al spec.
- `/speckit-analyze` MUST reportar la divergencia en vez de resolverla por su cuenta.

**Por qué**: la story es el artefacto que el humano leyó y firmó, y contra el que QA valida
cada PR. El spec es una vista técnica de eso, no una fuente paralela. Duplicar la capa es
el riesgo número uno registrado en `PROJECT.md`, y la única mitigación que funciona es que
una de las dos mande siempre.

### II. Una épica, una carpeta de spec

- Cada épica del PRD MUST corresponder a exactamente una carpeta `specs/NNN-nombre/`.
- Ninguna épica MUST tener dos specs, ni un spec cubrir dos épicas.
- MUST NOT crearse `specs/NNN-*/` para una épica que no figure en un PRD aprobado.

**Por qué**: spec-kit ya agrupa varias user stories dentro de un spec, así que la unidad
natural de spec es la épica y no la story. Fijar la correspondencia uno a uno hace que la
pregunta "¿en qué está esta épica?" se conteste mirando una sola carpeta.

### III. Todo artefacto vive en git

- Toda decisión que gobierne el trabajo MUST existir como archivo versionado en el repo.
  Nada que viva solo en un chat, en una herramienta externa o en la memoria de alguien
  cuenta como decidido.
- Notion, Drive y cualquier otra vista del cliente MUST ser copia derivada y de solo
  lectura. MUST NOT editarse del otro lado.
- Un cambio MUST empezar en el repo y recién después volver a exportarse.

**Por qué**: el historial de git es lo que prueba qué se aprobó exactamente el día que se
firmó. Editar la copia derivada crea una segunda fuente de verdad, y en cuanto hay dos, la
firma deja de significar algo.

### IV. El agente propone, el humano firma

- Todo artefacto de la cadena de producto MUST llevar `Estado`, `Firmado por` y
  `Fecha de firma` en su encabezado.
- Ningún comando MUST derivar el artefacto siguiente sin que el anterior esté `Aprobado`.
  La compuerta la verifica el agente; no depende de que el humano se acuerde.
- Un agente MUST NOT decidir que un artefacto está listo, ni firmar algo que el mantenedor
  no leyó. Transcribir una firma ya dada es admisible y MUST quedar registrado como
  transcripción.
- `PROJECT.md` y `PRD.md` MUST llevar además el bloque `Aprobado por cliente`. Ese bloque y
  el de firma MUST NOT completarse uno con el otro: dicen cosas distintas.

**Por qué**: la compuerta no vale por el campo, vale porque el registro es cierto. Un
agente que firma por conveniencia convierte todo el mecanismo en decoración. Y firmar es
barato —tres campos— justamente para que no se vuelva burocracia y se saltee.

### V. spec-kit es la única capa de specs

- La capa spec → plan → tasks MUST ser spec-kit.
- MUST NOT convivir una segunda herramienta de specs o de tracking en paralelo. Otras
  metodologías se toman como referencia de qué producir, nunca como herramienta a adoptar.
- Retirar una herramienta reemplazada MUST alcanzar al código, al instalador y a la
  documentación, no solo a la doc.

**Por qué**: ADR-006 eligió spec-kit y retiró Beads, pero el binario siguió ofreciendo
`kn beads` durante meses. Una decisión registrada que el artefacto no refleja es su propio
modo de falla, y es el que ADR-009 tuvo que venir a cerrar.

## Restricciones adicionales

Decisiones técnicas ya tomadas que los agentes MUST respetar sin volver a discutirlas:

- **Ramas y releases**: el flujo es siempre `<feature-id>/<rol>` → `epic/<feature-id>` →
  `dev` → `prod`. `dev` lleva los release candidate (`vX.Y.Z-rc.N`); `prod` solo releases
  estables (`vX.Y.Z`). Ningún cambio llega a `prod` sin haber pasado por un rc en `dev`,
  salvo `hotfix/*`. Cambiar el plan de releases es un cambio de contrato y exige volver a
  firmar el PRD.
- **Definición de agentes**: los roles se definen en `agents/<rol>/AGENTS.md` y se instalan
  con `kn`. Un rol que no tiene su archivo no existe.
- **Commits**: formato Conventional Commits (`<type>(<scope>): <message>`), obligatorio.
- **ADR**: toda decisión de arquitectura o de herramienta MUST quedar en `docs/adr/`. Un
  ADR en estado `Accepted` cuyo código no lo refleja es deuda registrable, no una opinión.

## Flujo de trabajo y compuertas

```text
Discovery → PROJECT.md → PRD.md (épicas) → stories/EPIC-xx/US-xx.md → specs/NNN-*/ → Issues → PRs
              ▲ firma       ▲ firma            ▲ firma                   ▲ firma
```

- Cada flecha con `▲ firma` es una compuerta: el artefacto de la izquierda MUST estar
  `Aprobado` antes de que exista el de la derecha.
- Los valores de `Estado` son `Borrador`, `En revisión`, `Aprobado` y `Reemplazado`.
- `/product-gate` es el verificador. Responde PASA o SE DETIENE, y los hooks de
  `.specify/extensions.yml` lo invocan antes de `/speckit-specify` y `/speckit-plan`.
- El `Constitution Check` de todo `plan.md` MUST nombrar cuál de los cinco principios
  aplica y por qué pasa. "Pasa por vacuidad" dejó de ser una respuesta válida con la
  ratificación de esta constitución.

## Governance

- **Precedencia**: esta constitución prevalece sobre cualquier otra práctica del repo. Si
  `AGENTS.md`, un skill o un agente la contradicen, gana la constitución y el otro
  documento se corrige en el mismo PR que detecta el conflicto.
- **Enmiendas**: se proponen en un PR propio que modifique este archivo, con el bump de
  versión aplicado y el Sync Impact Report actualizado. El merge del PR es la firma del
  mantenedor; ningún agente enmienda por su cuenta (Principio IV).
- **Versionado**: semántico sobre el contenido normativo.
  - MAJOR: se quita o redefine un principio de forma incompatible con lo anterior.
  - MINOR: se agrega un principio o una sección, o se amplía materialmente una guía.
  - PATCH: aclaraciones, redacción, correcciones que no cambian lo que exige.
- **Cumplimiento**: todo `plan.md` MUST pasar el `Constitution Check`. Una violación
  MUST justificarse en la sección "Complexity Tracking" del plan, o el plan no avanza.
  Una violación que no se puede justificar es un cambio de principio y va por enmienda.

**Version**: 1.0.0 | **Ratified**: 2026-09-12 | **Last Amended**: 2026-09-12
