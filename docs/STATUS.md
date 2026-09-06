# STATUS

**Generado**: 2026-09-06 (a mano, por el Planner; EPIC-02 lo automatiza con `/product-status`)
**Sprint**: 0 — pre-sprint. EPIC-01 entra a Sprint 1 cuando se firmen los artefactos.
**PRD**: [`docs/product/PRD.md`](./product/PRD.md) (Borrador)

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
