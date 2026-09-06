# STATUS

**Generado**: 2026-09-06 (a mano, por el Planner; EPIC-02 lo automatiza con `/product-status`)
**Sprint**: 0 — pre-sprint. `PROJECT.md` y `PRD.md` firmados; faltan las cinco stories y
el `spec.md` para que EPIC-01 entre a Sprint 1 y se pueda correr `/speckit-implement`.
**PRD**: [`docs/product/PRD.md`](./product/PRD.md) — **Aprobado** (incluye la decisión 9), Kevin Barroso, 2026-09-06

## Stories del sprint

| Story | Estado artefacto | Estado trabajo | Rol |
|---|---|---|---|
| US-01 Referencias rotas saneadas | Borrador | **Implementada, V1–V4 en verde** | Docs Writer, Rust |
| US-02 Discovery con rol Analyst | Borrador | No arrancada | Planner, Docs Writer |
| US-03 PRD derivado de PROJECT aprobado | Borrador | No arrancada | Planner |
| US-04 Stories con Gherkin | Borrador | No arrancada | Planner, QA |
| US-05 Compuertas de firma | Borrador | No arrancada | Planner, QA, mantenedor |

Planificadas: 5 · Cerradas: 0 (US-01 implementada; se cierra al firmarse los artefactos)

**Avance 2026-09-06**: Fase 2 (fundaciones: excepción de `.gitignore`, skill `product-gate`,
plantillas PROJECT/PRD/US) y Fase 3 (US-01 completa) implementadas en la rama
`claude/ultima-adicion-8yqkd3`. Las fases 4–7 siguen sin arrancar.

## Specs sin issue

- `specs/005-agent-team-projects/` — 45 tareas (40 del plan + T041–T045 del retiro de Beads), ninguna publicada como issue. Se publican
  con `/speckit-taskstoissues` después de la firma del spec (T004).

## Issues sin PR

Ninguno (no hay issues todavía).

## PRs sin review > 24 h

Ninguno.

## Riesgos abiertos

Ver tabla de riesgos en [`PRD.md`](./product/PRD.md). El más relevante hoy: que las
compuertas se salteen. Mitigación: US-05 hace que la verifique el agente, no la memoria.

## Decisiones del mantenedor (2026-09-06)

1. **Rol Architect separado del Planner**: sí, con arc42 recortado, ADR y LikeC4 como
   entregables. Nueva EPIC-06, Sprint 2.
2. **Flujo de ramas fijo**: `epic → dev → prod`, rc en `dev`, `vX.Y.Z` en `prod`, y el
   plan de releases como parte del contrato. Una rama `test` intermedia quedó **fuera por
   ahora**; vuelve si el piloto muestra que hace falta. Nueva EPIC-07, Sprint 2: la
   jerarquía ya está documentada pero `dev` no existe en el remoto. Primer release
   **v0.12.0** (EPIC-01); `v1.0.0` al cerrar el piloto.
3. **Beads se retira del producto**, no solo de la doc, con ADR propio (ADR-009).
   T041–T045, dentro de v0.12.0.
4. **Los artefactos viven en git; el cliente ve una copia derivada** (decisión 8 del PRD).
   Notion para el dashboard de avance (Biz Agent, EPIC-02) y Drive para el PDF de
   constancia al firmar. La copia derivada nunca se edita.
5. **El cliente aprueba el brief y el PRD, aparte de la firma** (decisión 9). Bloque
   propio en el encabezado, mecanismo mail + PDF, evidencia transcrita al `.md`. Notion
   queda descartado para firmar: no tiene firma electrónica.

> **La compuerta se aplicó a sí misma**: sumar la decisión 9 al PRD ya firmado lo devolvió
> a "En revisión", que es exactamente lo que la regla manda cuando un artefacto aprobado se
> edita. Kevin autorizó al Planner a transcribir la firma nueva, y el campo `Firmado por`
> lo deja registrado. `PROJECT.md` mantuvo su firma en todo momento: solo ganó el bloque de
> aprobación del cliente, sin cambio de contenido.
>
> La regla "no hay firmas delegadas a agentes" se reconcilió con la práctica: un agente
> puede **transcribir** una firma autorizada sobre contenido que el mantenedor ya revisó,
> y nunca decidir por su cuenta que algo está listo ni firmar lo que nadie leyó.

## Firmas

| Artefacto | Estado | Firmado por | Fecha |
|---|---|---|---|
| `docs/product/PROJECT.md` | **Aprobado** | Kevin Barroso | 2026-09-06 |
| `docs/product/PRD.md` | **Aprobado** | Kevin Barroso (transcrita) | 2026-09-06 |
| `stories/EPIC-01/US-01.md` … `US-05.md` | Borrador | — | — |
| `specs/005-agent-team-projects/spec.md` | Borrador | — | — |

La compuerta `product-gate` verificada contra estos encabezados: `PROJECT.md` y `PRD.md`
dan PASA; `spec.md` da SE DETIENE, así que `/speckit-implement` sigue bloqueado. `EPIC-06`
da SE DETIENE por falta de stories, que es lo correcto: se generan al entrar a Sprint 2.

**Pendiente de export**: los dos artefactos firmados todavía no tienen su PDF de
constancia en Drive (decisión 8). Se hace a mano hasta que EPIC-02 lo automatice.

## Preguntas al cliente pendientes

1. Idioma de los artefactos de producto (español fijo o por cliente).
2. Proyecto piloto y fecha.
3. Dónde vive la guía comercial (sección 7 del input de discovery).

## Deuda técnica registrada

- **Excepción de compuerta de la primera corrida (2026-09-06)**: `PROJECT.md`, `PRD.md`,
  las cinco stories, `spec.md`, `plan.md` y `tasks.md` se escribieron en una sola sesión
  como borradores, sin firma intermedia, para dejar la cadena completa y revisable de una
  vez. A partir de la firma rige la compuerta (US-05, escenario 5). Se cierra en T035.
- `.specify/memory/constitution.md` sigue siendo la plantilla sin ratificar (T040).
- ~~Referencias rotas conocidas hasta que cierre US-01~~ — **saneadas el 2026-09-06**
  (T012–T017). El chequeo encontró más de lo que la story listaba: seis skills fantasma en
  tres agentes (biz, devops, backend), no uno solo, y `docs/reports/README.md` también
  apuntaba al skill removido.
- **Beads en el binario, no en la doc (2026-09-06)**: T015 borró toda mención a beads de
  los README aunque el binario siga instalando `bd` desde `install.sh` (como dependencia
  obligatoria), verificándolo en `kn doctor` y exponiendo `kn beads`. El mantenedor
  decidió el retiro completo con ADR propio: T041–T045. Doc y binario están desalineados
  a propósito hasta que esas tareas cierren, dentro de v0.12.0.
- **`analyst` en el `CLAUDE.md` generado**: T013 no agregó el rol a la lista del generador
  porque el rol se crea en T018/T019. Va en el mismo commit que lo cree.
