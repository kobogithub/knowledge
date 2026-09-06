# Input de discovery — 2026-09-06

> **Tipo**: documento de trabajo del cliente (el propio mantenedor), pegado tal cual en la
> sesión que originó la iniciativa. Es el equivalente a la "grabación/transcripción de la
> reunión" de la etapa 1 de la cadena. No se edita: es evidencia. Lo derivado vive en
> [`PROJECT.md`](../PROJECT.md).

---

# Proyectos con equipo de agentes — Workflow, stack y seguimiento

> Guía para llevar proyectos chicos (segundo ingreso) con IA como harness: desde el relevamiento funcional hasta los issues, con vos en el rol de PM y un equipo de agentes por rol.

---

## 1. Principio rector

SDD (spec-driven development) resuelve **del spec para abajo**. El relevamiento funcional es una capa **anterior**, que también se documenta y se agentifica.

La forma de unir las dos capas es una **cadena de artefactos**: cada documento se deriva del anterior, todo vive en git como única fuente de verdad, y vos firmás en cada compuerta. El agente propone; vos validás.

```
Discovery → Objetivo/Alcance → Épicas → User Stories → Specs → Issues → PRs
   ▲            ▲                ▲           ▲            ▲
 firma        firma            firma       firma        firma
```

---

## 2. Cadena de artefactos (con agente responsable)

| # | Etapa | Artefacto | Agente | Qué produce | Compuerta |
|---|-------|-----------|--------|-------------|-----------|
| 1 | Discovery | `docs/product/PROJECT.md` | **Analyst** | Objetivo, problema, no-alcance, restricciones, criterio de éxito. Input: grabación/transcripción de la reunión con el cliente. | Vos corregís y firmás |
| 2 | Alcance | `docs/product/PRD.md` + épicas | **PM** | Épicas, roadmap, prioridades, riesgos. Se deriva del PROJECT.md. | Vos firmás; es el **contrato** con el cliente |
| 3 | Historias | `docs/product/stories/EPIC-xx/US-xx.md` | **PM** | User stories con criterios de aceptación en **Gherkin**. El Gherkin es el puente: se convierte en tests y en input del spec. | Vos revisás |
| 4 | Specs | `docs/specs/US-xx/` (spec → plan → tasks) | **Architect** | Acá entra SDD (spec-kit u OpenSpec). Decisiones relevantes van a `docs/adr/`. | Vos revisás |
| 5 | Issues | GitHub Issues / ClickUp | **PM** (publica) | Cada task del spec → un issue con link a la story y al spec. | Automático |
| 6 | Ejecución | PRs | **Dev** | Toma un issue, implementa, abre PR referenciando el issue. | Review humano |
| 7 | Validación | Tests / comentario en PR | **QA** | Valida el PR contra los criterios de aceptación de la story. | Merge |

### Plantillas mínimas

**PROJECT.md**
```
# Proyecto: <nombre>
## Cliente y contexto
## Problema que resuelve
## Objetivo (una frase medible)
## Alcance
## Fuera de alcance
## Restricciones (técnicas, tiempo, presupuesto, legales)
## Criterio de éxito
## Riesgos conocidos
## Preguntas abiertas
```

**User story**
```
# US-xx: <título>
Épica: EPIC-xx
Como <rol> quiero <acción> para <valor>.

## Criterios de aceptación
Feature: ...
  Scenario: ...
    Given ...
    When ...
    Then ...

## Notas / dependencias
```

---

## 3. Stack recomendado (sin gastar)

| Capa | Herramienta | Por qué |
|------|-------------|---------|
| Fuente de verdad | **Repo template** por proyecto: `docs/product/`, `docs/specs/`, `docs/adr/`, `CLAUDE.md`, `AGENTS.md` | Reutilizable en cada cliente; todo versionado |
| Tracker | **GitHub Issues + Projects** por defecto; **ClickUp** (vía MCP) solo si el cliente necesita una vista amigable | Gratis, integrado con los agentes |
| Agentes | **Claude Code / OpenCode** con subagentes por rol (analyst, pm, architect, dev, qa) | Ya lo usás; los roles se definen en `CLAUDE.md` / `.claude/agents/` |
| Método | **BMAD-METHOD** como referencia de roles; **spec-kit / OpenSpec** para la capa spec → plan → tasks | Es lo más cercano a "scrum con agentes"; tomar los roles sin adoptar todo |
| Automatización | **n8n** | Digest diario de issues/PRs, reporte semanal al cliente, alertas de PRs bloqueados |
| Entrega | **Railway + Supabase** | Deploy rápido, costo bajo, ya conectados |
| Conocimiento | Wiki en markdown (patrón LLM Wiki) | Lo aprendido por proyecto se realimenta a los agentes |

### Estructura del template repo
```
.
├── CLAUDE.md                 # reglas globales + roles
├── AGENTS.md                 # compat con OpenCode/otros
├── .claude/
│   ├── agents/               # analyst.md, pm.md, architect.md, dev.md, qa.md
│   └── commands/             # /discovery, /prd, /stories, /spec, /issues, /status
├── docs/
│   ├── product/
│   │   ├── PROJECT.md
│   │   ├── PRD.md
│   │   └── stories/EPIC-xx/US-xx.md
│   ├── specs/US-xx/{spec,plan,tasks}.md
│   ├── adr/ADR-xxxx.md
│   └── STATUS.md
├── src/
└── tests/
```

---

## 4. Rituales scrum adaptados a agentes

| Ritual | Cómo se hace | Quién |
|--------|--------------|-------|
| **Sprint** | 1 semana, alcance fijo acordado con el cliente | Vos |
| **Planning** | Vos + agente PM priorizan stories del PRD y generan issues del sprint | Vos + PM |
| **Daily** | El agente resume estado: issues abiertos, PRs esperando review, bloqueos. Vía n8n a tu mail/Slack. | PM (automático) |
| **Review** | Demo al cliente + changelog generado desde los PRs mergeados | Vos + PM |
| **Retro** | Lo aprendido va a `CLAUDE.md`, skills o ADRs — no a una minuta | Vos |

---

## 5. Seguimiento (vista PM)

`docs/STATUS.md` regenerado por el agente al cierre de cada día/sprint:

- Stories planificadas vs. cerradas en el sprint
- Specs sin issue (trabajo definido pero no planificado)
- Issues sin PR (trabajo planificado pero no arrancado)
- PRs sin review > 24 h
- Riesgos abiertos y preguntas al cliente pendientes
- Deuda técnica registrada

Con esto tenés el entendimiento completo del proyecto **sin leer código**. Si querés dashboard, GitHub Projects o una vista en ClickUp alcanzan.

---

## 6. Trampas a evitar

1. **El agente inventa requerimientos** para llenar huecos. Cada compuerta (objetivo, PRD, stories, spec) es tu firma; no se saltea.
2. **Scope creep**: vendé alcance fijo por sprint con el PRD como contrato. Cambios → nueva story → nuevo sprint.
3. **Documentación divergente**: si no está en el repo, no existe. Nada de specs en chats.
4. **Roles mezclados**: un agente que releva, diseña y codea en la misma sesión pierde criterio. Separar por rol y por sesión.
5. **Validación tardía**: QA valida contra Gherkin en cada PR, no al final del sprint.

---

## 7. Para segundo ingreso — modelo comercial

- **Oferta**: proyectos de alcance acotado (2–6 sprints), precio por sprint o por PRD cerrado.
- **Entregables visibles al cliente**: PROJECT.md firmado, PRD, demo semanal, changelog, deploy.
- **Reutilización**: el template repo + los agentes son tu activo; cada proyecto los mejora.
- **Contrato**: el PRD define alcance; el STATUS.md semanal es el reporte.

---

## 8. Próximos pasos

- [ ] Revisar el repo `kn knowledge` contra esta cadena y ver qué le falta (pendiente: pasar URL / conectar GitHub / pegar README + `tree`)
- [ ] Armar el template repo con la estructura de la sección 3
- [ ] Escribir `CLAUDE.md` con los cinco roles y los comandos por compuerta
- [ ] Definir el flujo n8n de daily digest y reporte semanal
- [ ] Elegir el primer proyecto piloto y correr un sprint de prueba
