# PRD: [nombre del proyecto]

**Estado**: Borrador
**Firmado por**: —
**Fecha de firma**: —
**Aprobado por cliente**: —
**Fecha de aprobación**: —
**Evidencia de aprobación**: —
**Agente autor**: Planner (`<prefijo>-x6e`)
**Derivado de**: [`PROJECT.md`](./PROJECT.md) (etapa 1)

> Etapa 2 de la cadena. Este documento es el **contrato**: lo que está acá es el alcance;
> lo que no está es una story nueva en un sprint nuevo. Cada épica entra a un sprint solo
> cuando el PRD está firmado, y recién entonces se le crea su `specs/NNN-*`.

## Resumen

[Dos o tres frases: qué se construye y para qué, derivado del Objetivo del PROJECT.md.]

## Decisiones de estructura

[Decisiones que adaptan la metodología a este repo en particular: mapeos de roles,
convenciones de nombres, qué tracker se usa. Si no hay ninguna, borrar la sección. Las que
tengan consecuencias arquitectónicas se formalizan además en un ADR.]

1. **[Decisión]**: [qué se decide y por qué.]

## Épicas

| ID | Épica | Prioridad | Sprint | Spec | Estado |
|---|---|---|---|---|---|
| EPIC-01 | [Título de la épica] | P1 | 1 | se crea al entrar al sprint | Sin spec |
| EPIC-02 | [...] | P2 | 2 | se crea al entrar al sprint | Sin spec |

- **Prioridad**: P1 (bloquea el objetivo), P2 (lo mejora), P3 (deseable).
- **Spec**: link a `specs/NNN-nombre/` cuando la épica entra a un sprint. Una épica ↔ una
  carpeta de spec.
- **Estado**: `Sin spec` → `Spec en borrador` → `Spec aprobado` → `En curso` → `Cerrada`.

### EPIC-01 — [Título] (Sprint 1)

**Valor**: [qué gana el cliente cuando esto está hecho, en una frase.]

**Alcance previsto**: [qué entra, en bullets o prosa corta.]

**Stories** (detalle en `stories/EPIC-01/`, que va como link en el archivo real):

| ID | Story | Rol ejecutor |
|---|---|---|
| `US-01` | [Título] | [Rol] |

**Entregables visibles**: [qué puede ver o usar el cliente al cerrar la épica.]

### EPIC-02 — [Título] (Sprint 2)

[Misma estructura. Las épicas que todavía no entran a un sprint pueden ir solo con Valor y
Alcance previsto; las stories se generan con `/product-stories EPIC-xx` cuando entran.]

## Roadmap

| Sprint | Épicas | Objetivo del sprint |
|---|---|---|
| 1 | EPIC-01 | [Qué queda funcionando al cierre.] |
| 2 | EPIC-02, EPIC-03 | [...] |

## Fuera de alcance

[Heredado del PROJECT.md, más lo que se descartó al definir las épicas. Esta lista es la
que se cita cuando aparece un pedido nuevo a mitad de sprint.]

## Riesgos

| Riesgo | Impacto | Mitigación | Épica que lo cubre |
|---|---|---|---|
| [Riesgo] | [Impacto] | [Mitigación] | [EPIC-xx o "—"] |

## Preguntas abiertas

[Las que quedaron sin responder en el PROJECT.md, más las que aparecieron al armar las
épicas. Una pregunta sin responder no bloquea la firma del PRD, pero sí debería bloquear
la épica que depende de ella: anotalo en la épica.]

---

**Plantilla**: etapa 2 de la cadena de producto. La genera `/product-prd`, que exige
`PROJECT.md` en Estado "Aprobado". Ver [`../README.md`](../README.md).

> **Dos aprobaciones, no una.** *Firmado por* es el mantenedor diciendo "el artefacto es
> correcto"; *Aprobado por cliente* es el cliente diciendo "acepto este alcance". No se
> completan entre sí. La evidencia es el mail de respuesta del cliente sobre el PDF que le
> mandaste; el PDF queda en Drive. En un proyecto sin cliente externo, borrá el bloque.
