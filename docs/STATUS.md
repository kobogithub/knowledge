# STATUS

**Generado**: 2026-09-06 (a mano, por el Planner; EPIC-02 lo automatiza con `/product-status`)
**Última edición**: 2026-09-12 — enmienda 1 del PRD firmada (EPIC-08 de diseño, glosario en EPIC-02, errata de sprint); cadena de suministro de releases endurecida (#4); `release.yml` ya distingue prereleases, la regla del rc queda ejecutable (#34); constitución ratificada (v1.0.0, T040 cerrada)
**Sprint**: 1 — EPIC-01 en curso. Los ocho artefactos de la cadena están firmados y
`/speckit-implement` está habilitado desde el 2026-09-06.
**PRD**: [`docs/product/PRD.md`](./product/PRD.md) — **Aprobado** (incluye la decisión 9), Kevin Barroso, 2026-09-06

## Stories del sprint

| Story | Estado artefacto | Estado trabajo | Rol |
|---|---|---|---|
| US-01 Referencias rotas saneadas | **Aprobado** | **Implementada, V1–V4 en verde** | Docs Writer, Rust |
| US-02 Discovery con rol Analyst | **Aprobado** | **Implementada**; 2 escenarios sin verificar (corrida real) | Planner, Docs Writer |
| US-03 PRD derivado de PROJECT aprobado | **Aprobado** | **Implementada, V6–V7 en verde** | Planner |
| US-04 Stories con Gherkin | **Aprobado** | **Implementada**; escenario 4 (QA en PR real) para EPIC-02 | Planner, QA |

**QA 10/10 en el quickstart V1–V10** desde un árbol exportado con `git archive` (sin
`kn sync`). Contra el Gherkin: **18 de 21 escenarios verificados, 0 fallas, 3 no
verificables** en este entorno. Reporte completo en
[`docs/reports/2026-09-06-qa-005.md`](./reports/2026-09-06-qa-005.md).
| US-05 Compuertas de firma | **Aprobado** | **Implementada, V8–V9 en verde** | Planner, QA, mantenedor |

Planificadas: 5 · Implementadas: 5 · Pendientes de cierre formal: 2 (US-02 y US-04 esperan verificaciones que necesitan un PR o una corrida real)

**Avance 2026-09-06**: EPIC-01 implementada de punta a punta en
`claude/ultima-adicion-8yqkd3` — fases 2 a 8, más el retiro de Beads. La cadena está
operativa: cuatro skills (`product-gate`, `product-discovery`, `product-prd`,
`product-stories`), el rol Analyst, tres plantillas, ADR-008 y ADR-009, la regla de
derivación, la validación de QA contra Gherkin y los hooks de `.specify/extensions.yml`.
Sale en **v0.12.0**.

## Specs sin issue

- `specs/005-agent-team-projects/` — 46 tareas, 45 cerradas. Ninguna se publicó como issue:
  la iniciativa se ejecutó completa en una sesión, así que `/speckit-taskstoissues` no llegó
  a tener utilidad. Para EPIC-02 sí conviene publicarlas antes de empezar.

## Issues sin PR

Ninguno (no hay issues todavía).

## PRs sin review > 24 h

Ninguno. El PR de `claude/ultima-adicion-8yqkd3` → `dev` está pendiente de abrir.

## Ramas vivas

| Rama | Estado |
|---|---|
| `prod` | `d47ac31` |
| `dev` | `d47ac31` — sincronizada con `prod` el 2026-09-06 |
| `claude/ultima-adicion-8yqkd3` | EPIC-01 completa, pendiente de PR a `dev` |
| `epic/003-absorb-framework-ia` | Revisar: ¿su trabajo ya está en `prod`? |
| `epic/004-macos-arm64-homebrew` | Revisar: el PR #13 ya se mergeó |
| `task/restore-short-url` | Revisar: el PR #16 ya se mergeó |

Auditar y borrar las tres últimas es parte de EPIC-07.

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

## Decisiones del mantenedor (2026-09-12)

1. **v0.12.0 sale a `prod` sin release candidate previo en `dev`.** Excepción consciente y
   acotada a este release, decidida por Kevin Barroso el 2026-09-12.

   La decisión 2 de arriba, la decisión 7 del PRD y la sección "Restricciones adicionales"
   de la constitución dicen lo mismo: *ningún cambio llega a `prod` sin haber pasado por un
   rc en `dev`*. Esta vez no pasó.

   **Por qué**: hoy la regla no es ejecutable. `release.yml` dispara con cualquier tag `v*`,
   tiene `prerelease: false` escrito a mano y su job `publish-formula` corre sin condición.
   Un tag `v0.12.0-rc.1` se publicaría como release estable **y se empujaría al tap de
   Homebrew** — el mismo modo de falla que `tap-drift-check.yml` existe para detectar. Con
   esa herramienta, cumplir la regla hacía más daño que saltearla.

   **Alcance**: solo v0.12.0. La regla sigue vigente y no se enmendó.

   **Qué la destraba**: hacer que `release.yml` distinga prereleases — `prerelease: true`
   para tags con guión y saltear `publish-formula` en esos casos.

   **Destrabado el 2026-09-12** (issue #34): `release.yml` ya deriva la condición del tag.
   La excepción no se repite — v0.13.0 sale con `v0.13.0-rc.1` en `dev` primero.

   Precedente: la excepción de compuerta del 2026-09-06, registrada y cerrada el mismo día.

## Firmas

| Artefacto | Estado | Firmado por | Fecha |
|---|---|---|---|
| `docs/product/PROJECT.md` | **Aprobado** | Kevin Barroso | 2026-09-06 |
| `docs/product/PRD.md` | **Aprobado** | Kevin Barroso (transcrita) | 2026-09-06 |
| `docs/product/PRD.md` — **enmienda 1** (glosario en EPIC-02, EPIC-08, errata de sprint) | **Aprobado** | Kevin Barroso (transcrita) | 2026-09-12 |
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

## Firmas pendientes del mantenedor

| Artefacto | Estado | Qué destraba |
|---|---|---|
| [`stories/EPIC-02/US-01.md`](./product/stories/EPIC-02/US-01.md) — glosario de dominio | Borrador | **Tu firma.** El bloqueo de contrato se levantó: la enmienda 1 metió el glosario en el alcance de EPIC-02. Lo que falta ahora es revisar los escenarios Gherkin de la story y firmarla, como las cinco de EPIC-01. Origen: issue #24 |
| Stories de **EPIC-08** — capa de diseño | No existen | `/product-stories EPIC-08`. La épica quedó contratada con la enmienda 1; las stories se generan al entrar a Sprint 3. Origen: issue #25 |

## Deuda técnica registrada

- ~~`release.yml` no distingue prereleases (2026-09-12)~~ — **CERRADA el 2026-09-12**.
  `release.yml` deriva la condición del propio tag: un guión lo marca como prerelease
  (semver), el release se publica con `prerelease: true` y `publish-formula` no corre, así
  que el tap nunca sirve un candidato. El job también rechaza tags que no sean semver, y
  `tap-drift-check.yml` compara contra el último release **estable** para no reportar drift
  de un tap que está donde corresponde. **La regla del rc pasa a ser ejecutable**: desde el
  próximo release no hace falta excepción. Issue #34.

- ~~Excepción de compuerta de la primera corrida (2026-09-06)~~ — **CERRADA el mismo día**
  (T035, V10). Los ocho artefactos quedaron Aprobados y firmados, así que la cadena
  recorrió su propio circuito de punta a punta. Desde acá rige la compuerta sin excepción.
- ~~`.specify/memory/constitution.md` sin ratificar (T040)~~ — **CERRADA el 2026-09-12**.
  Ratificada en v1.0.0 con cinco principios movidos desde artefactos ya firmados, sin
  inventar ninguno: la story manda, una épica una carpeta de spec, todo artefacto vive
  en git, el agente propone y el humano firma, spec-kit es la única capa de specs.
- ~~Referencias rotas conocidas hasta que cierre US-01~~ — **saneadas el 2026-09-06**
  (T012–T017). El chequeo encontró más de lo que la story listaba: seis skills fantasma en
  tres agentes (biz, devops, backend), no uno solo, y `docs/reports/README.md` también
  apuntaba al skill removido.
- ~~Beads en el binario, no en la doc (2026-09-06)~~ — **CERRADA el mismo día**. T041–T045
  retiraron el subcomando, el chequeo de `kn doctor` y la instalación de `install.sh`, con
  la decisión en [ADR-009](./adr/009-remove-beads-from-the-product.md). Doc y binario
  vuelven a coincidir. Sale como breaking change en v0.12.0.
- ~~`analyst` en el `CLAUDE.md` generado~~ — **cerrada**: entró en T019 junto con el rol.
- **Tres escenarios sin verificar (2026-09-06)**: US-02 escenarios 2 y 3 necesitan una
  corrida real de `/product-discovery` sobre un input de prueba; US-04 escenario 4 necesita
  un PR abierto para que QA comente. Ninguno está marcado como pasado. Se cierran en
  EPIC-02, que es cuando habrá PRs y un proyecto sobre el cual correr el discovery.
- ~~**`.specify/memory/constitution.md` sin ratificar** (T040)~~ — **CERRADA el
  2026-09-12**, antes de abrir EPIC-02. Los dos principios candidatos que anotaba esta
  deuda (la convención de firma y "la story manda") quedaron como los principios IV y I.
  Se sumaron tres más que ya estaban decididos y sin lugar donde vivir. El
  `Constitution Check` de `.specify/templates/plan-template.md` dejó de ser un placeholder
  y ahora enumera las cinco compuertas, así que el próximo `plan.md` no puede pasar por
  vacuidad.
