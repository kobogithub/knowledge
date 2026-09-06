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

- `specs/005-agent-team-projects/` — 43 tareas (40 del plan + T041–T043 de follow-up), ninguna publicada como issue. Se publican
  con `/speckit-taskstoissues` después de la firma del spec (T004).

## Issues sin PR

Ninguno (no hay issues todavía).

## PRs sin review > 24 h

Ninguno.

## Riesgos abiertos

Ver tabla de riesgos en [`PRD.md`](./product/PRD.md). El más relevante hoy: que las
compuertas se salteen. Mitigación: US-05 hace que la verifique el agente, no la memoria.

## Preguntas al cliente pendientes

1. Idioma de los artefactos de producto (español fijo o por cliente).
2. Rol Architect separado del Planner, o no.
3. Proyecto piloto y fecha.
4. Dónde vive la guía comercial (sección 7 del input de discovery).

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
- **Beads en el binario, no en la doc (2026-09-06)**: por decisión del mantenedor, T015
  borró toda mención a beads de los README aunque `kn beads template` siga existiendo. El
  binario todavía instala `bd` desde `install.sh` (como dependencia obligatoria), lo
  verifica en `kn doctor` y expone el subcomando. Doc y binario están desalineados a
  propósito hasta que cierren T041–T043 de `specs/005-agent-team-projects/tasks.md`.
- **`analyst` en el `CLAUDE.md` generado**: T013 no agregó el rol a la lista del generador
  porque el rol se crea en T018/T019. Va en el mismo commit que lo cree.
