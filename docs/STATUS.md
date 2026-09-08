# STATUS

**Generado**: 2026-09-06 (a mano, por el Planner; EPIC-02 lo automatiza con `/product-status`)
**Sprint**: 1 — EPIC-01 en curso. Los ocho artefactos de la cadena están firmados y
`/speckit-implement` está habilitado desde el 2026-09-06.
**PRD**: [`docs/product/PRD.md`](./product/PRD.md) — **Aprobado** (incluye la decisión 9), Kevin Barroso, 2026-09-06

## Stories del sprint

| Story | Estado artefacto | Estado trabajo | Rol |
|---|---|---|---|
| US-01 Referencias rotas saneadas | **Aprobado** | **Implementada, V1–V4 en verde** | Docs Writer, Rust |
| US-02 Discovery con rol Analyst | **Aprobado** | **Implementada**; falta corrida real de `/product-discovery` (V5) | Planner, Docs Writer |
| US-03 PRD derivado de PROJECT aprobado | **Aprobado** | **Implementada, V6–V7 en verde** | Planner |
| US-04 Stories con Gherkin | **Aprobado** | **Implementada**; escenario 4 (QA en PR real) para EPIC-02 | Planner, QA |
| US-05 Compuertas de firma | **Aprobado** | **Implementada, V8–V9 en verde** | Planner, QA, mantenedor |

Planificadas: 5 · Implementadas: 5 · Pendientes de cierre formal: 2 (US-02 y US-04 esperan verificaciones que necesitan un PR o una corrida real)

**Avance 2026-09-06**: fases 2 a 7 implementadas en `claude/ultima-adicion-8yqkd3`. La
cadena está completa y operativa: cuatro skills (`product-gate`, `product-discovery`,
`product-prd`, `product-stories`), el rol Analyst, las tres plantillas, ADR-008, la regla
de derivación, la validación de QA contra Gherkin y los hooks de `.specify/extensions.yml`.
Falta la Fase 8 (quickstart completo, índices, CHANGELOG, cierre).

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
   jerarquía ya está documentada y `dev` se sincronizó con `prod` el 2026-09-06. Primer release
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
| `stories/EPIC-01/US-01.md` … `US-05.md` | **Aprobado** | Kevin Barroso (transcritas) | 2026-09-06 |
| `specs/005-agent-team-projects/spec.md` | **Aprobado** | Kevin Barroso (transcrita) | 2026-09-06 |

Los ocho dan PASA. Las cinco stories y el `spec.md` se firmaron tras revisar los 21
escenarios Gherkin en sesión.

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

- ~~Excepción de compuerta de la primera corrida (2026-09-06)~~ — **CERRADA el mismo día**
  (T035, V10). Los ocho artefactos quedaron Aprobados y firmados, así que la cadena
  recorrió su propio circuito de punta a punta. Desde acá rige la compuerta sin excepción.
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
