# Proyecto: Metodología de proyectos con equipo de agentes

**Estado**: Borrador — pendiente de firma
**Firmado por**: —
**Fecha de firma**: —
**Agente autor**: Planner (`knowledge-x6e`), actuando como Analyst hasta que exista el rol
**Input**: [`discovery/2026-09-06-metodologia-agentes.md`](./discovery/2026-09-06-metodologia-agentes.md)

> Etapa 1 de la cadena. Este documento se deriva del input de discovery y **no** agrega
> requerimientos que el cliente no haya expresado. Lo que no está en el input aparece en
> "Preguntas abiertas", nunca como decisión tomada.

## Cliente y contexto

El cliente es el mantenedor de `knowledge`, que quiere usar el framework `kn` para llevar
proyectos chicos de terceros como segundo ingreso. Trabaja solo, con Claude Code / OpenCode
como harness y un equipo de agentes por rol. Hoy `knowledge` ya cubre la capa
spec → plan → tasks → PRs con spec-kit y once roles de agente, pero no tiene nada por
encima del spec: ni relevamiento funcional, ni alcance acordado con el cliente, ni
compuertas de firma.

## Problema que resuelve

Sin una capa de relevamiento y alcance documentada, el Planner arranca en
`/speckit-specify` desde una frase y el agente rellena los huecos con supuestos. Eso
produce specs que no responden al problema del cliente, scope creep sin contrato que lo
frene, y un PM que necesita leer código para saber en qué está el proyecto.

## Objetivo (una frase medible)

Que un proyecto nuevo de cliente pueda recorrer la cadena completa
**Discovery → PRD → Stories → Spec → Issues → PRs → QA** dentro de `knowledge`, con una
firma humana registrada en cada compuerta y un `STATUS.md` que muestre el estado sin leer
código, antes del primer proyecto piloto.

## Alcance

- Capa de producto sobre spec-kit: `docs/product/` con `PROJECT.md`, `PRD.md` y
  `stories/EPIC-xx/US-xx.md`, más el rol **Analyst** y los comandos por compuerta.
- Compuertas de firma explícitas en cada artefacto y regla para que ningún agente derive
  el siguiente artefacto sin la firma del anterior.
- Adaptación de los roles de la guía (analyst, pm, architect, dev, qa) a los once roles
  existentes de `knowledge`, sin duplicar agentes.
- Seguimiento PM: `docs/STATUS.md` regenerable con las métricas de la sección 5 de la guía.
- Publicación de tareas como issues (GitHub Issues por defecto) con link a story y spec.
- QA validando cada PR contra el Gherkin de la story.
- Scaffolding de la estructura de producto desde `kn init`, para que cada proyecto de
  cliente arranque con la cadena lista.
- Aplicar la cadena a esta misma iniciativa (dogfooding) para validarla antes del piloto.

## Fuera de alcance

- Adoptar BMAD-METHOD u OpenSpec como herramientas: se toman los roles como referencia,
  la capa de specs sigue siendo spec-kit (ADR-006).
- Integración con ClickUp. GitHub Issues + Projects es el tracker por defecto; ClickUp
  queda para cuando un cliente lo pida.
- Automatización con n8n (daily digest, reporte semanal). Se planifica como épica
  posterior porque depende de que exista `STATUS.md`.
- Cambios al modelo comercial (precio por sprint, contrato). Se documentan en la guía,
  no en el repo.
- Soporte de `kn` fuera de Apple Silicon macOS (ADR / CHANGELOG 0.11.0).

## Restricciones

- **Técnicas**: todo artefacto vive en git; nada en chats. spec-kit es la única capa de
  specs. Los agentes se definen en `agents/<rol>/AGENTS.md` y se instalan con `kn`.
- **Tiempo**: un mantenedor, sprints de una semana. La cadena tiene que ser operable por
  una persona sola sin dedicarle más que el tiempo de firma y revisión.
- **Presupuesto**: sin gasto nuevo. GitHub, Claude Code, Railway y Supabase ya están.
- **Legales**: ninguna identificada.

## Criterio de éxito

1. Un proyecto de prueba recorre las siete etapas de la cadena y cada compuerta queda
   registrada con firmante y fecha en el artefacto.
2. Ningún agente puede correr `/speckit-plan` sobre un spec sin estado "Aprobado", ni
   `/speckit-specify` sobre una épica que no está en un PRD aprobado.
3. `docs/STATUS.md` responde, sin abrir código, las seis preguntas de la sección 5 de la
   guía (stories del sprint, specs sin issue, issues sin PR, PRs sin review, riesgos,
   deuda).
4. `kn init` en un repo vacío deja `docs/product/`, `docs/adr/` y `docs/STATUS.md` listos.
5. Esta iniciativa se ejecuta con su propia cadena (este archivo, el PRD y las stories
   existen antes que el spec).

## Riesgos conocidos

| Riesgo | Impacto | Mitigación |
|---|---|---|
| Duplicar capas: stories en `docs/product` y user stories en `spec.md` divergen | Dos fuentes de verdad | Regla de derivación: el spec referencia el ID de la story y copia sus escenarios; la story manda |
| El Planner sigue haciendo PM + Architect en la misma sesión | Pierde criterio (trampa 4) | Separar por sesión primero; evaluar un rol Architect en una épica posterior |
| Las compuertas se vuelven burocracia y se saltean | La cadena deja de proteger | Firma = un campo en el encabezado; el agente la verifica, el humano solo la completa |
| `kn init` scaffoldea algo que después no se usa | Ruido en proyectos de cliente | Scaffolding mínimo (plantillas + STATUS vacío), sin contenido inventado |

## Preguntas abiertas

- ¿El idioma de los artefactos de producto es español siempre, o depende del cliente?
  (Los specs existentes en `knowledge` están en inglés; esta iniciativa se escribe en
  español.)
- ¿Se quiere un rol **Architect** separado del Planner, o el Planner sigue produciendo
  `plan.md`?
- ¿Qué proyecto es el piloto y cuándo arranca? Define la fecha límite real de la cadena.
- ¿Dónde vive la guía comercial (sección 7 del input)? Hoy queda solo en el input de
  discovery.
