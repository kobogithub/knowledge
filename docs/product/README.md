# Capa de producto

Lo que va **antes** del spec. spec-kit resuelve del spec para abajo
(`specs/NNN-*/spec.md` → `plan.md` → `tasks.md`); esta carpeta guarda el relevamiento y el
alcance de los que esos specs se derivan.

```
Discovery → PROJECT.md → PRD.md (épicas) → stories/EPIC-xx/US-xx.md → specs/NNN-*/ → Issues → PRs
              ▲ firma       ▲ firma            ▲ firma                   ▲ firma
```

## Archivos

| Archivo | Etapa | Lo escribe | Se deriva de |
|---|---|---|---|
| `discovery/<fecha>-<tema>.md` | 0 | El humano (transcripción, notas, brief) | — (input inmutable) |
| `PROJECT.md` | 1 | Analyst (`/product-discovery`) | el input de discovery |
| `PRD.md` | 2 | Planner (`/product-prd`) | `PROJECT.md` aprobado |
| `stories/EPIC-xx/US-xx.md` | 3 | Planner (`/product-stories EPIC-xx`) | `PRD.md` aprobado |
| `../../specs/NNN-*/spec.md` | 4 | Planner (`/speckit-specify`) | las stories de la épica |
| `templates/` | — | Docs Writer | plantillas de los tres artefactos |

Una épica del PRD equivale a una carpeta `specs/NNN-*/`. Las "User Story N" del spec
llevan el mismo ID y los mismos escenarios Gherkin que `US-NN.md`; si divergen, manda la
story. Decisión registrada en el ADR-008 (en curso, ver EPIC-01).

## Compuertas

Cada artefacto lleva este encabezado y ningún comando deriva el siguiente sin que el
anterior esté **Aprobado**:

```markdown
**Estado**: Borrador | En revisión | Aprobado | Reemplazado
**Firmado por**: <nombre>
**Fecha de firma**: YYYY-MM-DD
```

El agente propone; el humano firma. Firmar es completar esos tres campos, nada más.

### Dos aprobaciones sobre el brief y el PRD

`PROJECT.md` (el **brief**) y `PRD.md` (el **contrato de alcance**) llevan además un
segundo bloque, porque son los dos que el cliente ve y acepta:

```markdown
**Aprobado por cliente**: <nombre>
**Fecha de aprobación**: YYYY-MM-DD
**Evidencia de aprobación**: <link al mail o al PDF>
```

| | `Firmado por` | `Aprobado por cliente` |
|---|---|---|
| Qué afirma | el artefacto es correcto | el cliente acepta este alcance |
| Quién | el mantenedor | el cliente |
| Para qué sirve | destraba al siguiente agente | protege comercialmente |

**No se completan entre sí.** Un artefacto firmado sin aprobación es un documento correcto
que todavía nadie autorizó; uno aprobado sin firmar es un acuerdo que nadie revisó.

**Cómo se obtiene**: se exporta el artefacto a PDF, se manda por mail y el cliente
responde "aprobado". Ese mail es la evidencia: lleva timestamp de un tercero, es difícil de
repudiar y no cuesta nada. El PDF va a Drive; el mantenedor transcribe nombre, fecha y link
al encabezado. Notion no sirve para firmar — no tiene firma electrónica, y que el cliente
tilde algo del lado de Notion rompería la regla de que la copia derivada no se edita.

Las stories y los `spec.md` no llevan este bloque: al cliente no se le hace revisar
Gherkin. Un proyecto sin cliente externo borra el bloque y `product-gate` no bloquea.

## Dónde viven los artefactos

En git, y en ningún otro lado. `docs/product/` es la fuente de verdad porque es donde los
agentes leen, donde `product-gate` parsea el encabezado de firma, y donde el historial
prueba qué se aprobó exactamente el día que se firmó.

Drive y Notion cumplen otro rol: **la vista del cliente**, que no tiene el repo.

| Dónde | Qué | Quién lo genera | Se edita ahí |
|---|---|---|---|
| `docs/product/*.md` | El artefacto vivo y firmado | Los comandos `product-*` | **Sí** — es la fuente |
| Notion | Dashboard de avance, derivado de `STATUS.md` | Biz Agent (`knowledge-biz`), EPIC-02 | No |
| Drive | PDF del artefacto al momento de firmarlo, como constancia | Export manual al firmar | No |

**La copia derivada nunca se edita.** Un cambio empieza en el repo y se vuelve a exportar.
Editar del otro lado crea dos fuentes de verdad y la firma deja de significar algo.

## Estado de esta iniciativa

La primera iniciativa que usa la cadena es la que la construye:
[`PROJECT.md`](./PROJECT.md) → [`PRD.md`](./PRD.md) → [`stories/EPIC-01/`](./stories/EPIC-01/)
→ [`specs/005-agent-team-projects/`](../../specs/005-agent-team-projects/). El seguimiento
está en [`docs/STATUS.md`](../STATUS.md).
