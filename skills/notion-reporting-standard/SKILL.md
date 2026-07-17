---
name: notion-reporting-standard
description: Stakeholder reporting standard with Notion integration and Markdown fallback for non-technical project dashboards
version: 1.0.0
tags:
  - reporting
  - notion
  - stakeholder
  - dashboard
  - business
---

# Notion Reporting Standard

Estandar para reportes de avance dirigidos a stakeholders no tecnicos. Define mapeos de estado, estructura de resumenes ejecutivos, y reglas de traduccion tecnica-a-negocio.

## Overview

Este skill define como convertir el estado tecnico de spec-kit (`specs/NNN-feature-name/{spec,plan,tasks}.md`) en reportes de negocio comprensibles para stakeholders que no tienen conocimientos de desarrollo. Soporta dos modos de output:

- **Modo Notion**: Actualiza dashboards en Notion via MCP
- **Modo Markdown** (fallback): Genera reportes en `/docs/reports/` cuando Notion no esta disponible

## Reglas de Mapeo: Spec-Kit → Reporte

### Estado de Iniciativas (carpetas `specs/NNN-feature-name/`)

No hay un campo de estado explicito como en un issue tracker — el estado se infiere de que
artefactos existen dentro de la carpeta de la spec y de cuantos checkboxes de `tasks.md`
estan marcados:

| Señal en `specs/NNN-feature-name/` | Estado Reporte | Emoji | Significado para Stakeholder |
|-------------------------------------|---------------|-------|------------------------------|
| Solo `spec.md` (sin `plan.md`/`tasks.md`) | En Planificacion | ⏳ | El equipo esta diseñando la solucion |
| `plan.md` + `tasks.md` generados, con checkboxes `[ ]`/`[x]` mezclados | En Desarrollo | 🏗️ | Se esta construyendo activamente |
| Reportado manualmente por un agente (sin señal automatica — ver nota) | Requiere Atencion | 🚨 | Necesitamos algo del cliente o hay un impedimento |
| Todos los checkboxes de `tasks.md` marcados `[x]` | Completado | ✅ | Listo para usar o revisar |

**Nota**: a diferencia de un issue tracker, spec-kit no tiene un estado "blocked" automatico.
El Biz Agent depende de que el agente que encuentra el impedimento lo señale explicitamente
(comentario de PR o entrada en `tasks.md`) para poder reportarlo como 🚨.

### Prioridad (orden de User Stories)

spec-kit no tiene un campo de prioridad numerico. La prioridad implicita es el orden de las
User Stories dentro de `tasks.md` (US1, US2, US3...): US1 es el corte minimo viable, las
siguientes son incrementales. Reportar como "Historia N de M" en vez de P0-P3:

| Posicion en `tasks.md` | Prioridad Reporte | Emoji | Significado |
|-------------------------|-------------------|-------|-------------|
| US1 (primera historia) | Alcance Minimo | ⭐ | Lo indispensable para la entrega |
| US2, US3... | Incremental | 📋 | Mejoras planificadas sobre el minimo |
| Tarea marcada `[P]` fuera de una user story | Paralelizable | 📌 | No bloquea al resto del plan |

### Traduccion de Terminos Tecnicos

Siempre traducir jerga tecnica a valor de negocio:

| Termino Tecnico | Traduccion para Stakeholder |
|-----------------|----------------------------|
| Refactor | Mejora interna de estabilidad y mantenibilidad |
| Hotfix | Correccion urgente de un error en produccion |
| CI/CD Pipeline | Sistema automatizado de pruebas y entregas |
| Middleware | Capa de procesamiento intermedio |
| API Endpoint | Punto de conexion del servicio |
| Database Migration | Actualizacion de la estructura de datos |
| Unit Tests | Verificaciones automaticas de calidad |
| Code Review | Revision de calidad por pares |
| Docker Container | Entorno aislado de ejecucion |
| Rate Limiting | Control de velocidad para proteger el servicio |
| JWT / Auth Token | Credencial de acceso seguro |
| WebSocket | Conexion en tiempo real |
| Cache | Almacenamiento rapido para mejor rendimiento |
| Load Balancer | Distribuidor de carga para alta disponibilidad |
| SSL/TLS | Cifrado de comunicaciones |
| Rollback | Reversion a version anterior estable |
| Staging | Entorno de pruebas pre-produccion |
| Dependency Update | Actualizacion de componentes base |
| Memory Leak | Problema de consumo de recursos (corregido) |
| Race Condition | Error de sincronizacion (corregido) |
| Breaking Change | Cambio que requiere adaptacion de integraciones |

## Estructura del Resumen Ejecutivo

Todo reporte debe seguir esta estructura de 3 secciones:

### 1. ¿Que logramos? (Progreso)

Maximo 3-5 hitos principales. Redactar en terminos de valor entregado, no de tareas tecnicas completadas.

**Formato:**
```
✅ [Hito en lenguaje de negocio]
   Detalle breve de que significa para el usuario/proyecto.
```

**Ejemplo:**
```
✅ Sistema de login seguro implementado
   Los usuarios pueden registrarse e iniciar sesion de forma segura.
   Incluye recuperacion de contraseña y proteccion contra intentos maliciosos.
```

**Anti-ejemplo (NO hacer):**
```
✅ Implementado JWT auth con refresh tokens, bcrypt hashing y rate limiting middleware
```

### 2. Estado de Salud del Proyecto (Semaforo)

Usar sistema de semaforo con criterios claros:

| Color | Significado | Criterio |
|-------|-------------|----------|
| 🟢 Verde | En camino | Sin blockers, avance segun plan |
| 🟡 Amarillo | Atencion | Riesgos identificados o retrasos menores |
| 🔴 Rojo | Problema | Blockers activos, requiere intervencion del stakeholder |

**Formato:**
```
## Estado de Salud: 🟢 En Camino

Resumen: El proyecto avanza segun lo planificado.
Velocidad del equipo esta semana: [X] tareas completadas de [Y] planificadas.
```

Si hay blockers que requieren accion del stakeholder, listarlos explicitamente:

```
## Estado de Salud: 🔴 Requiere Atencion

🚨 Blocker #1: Necesitamos acceso a [recurso/servicio/credencial]
   Impacto: Retrasa la entrega de [feature] en [N] dias
   Accion requerida: [Que necesitamos del stakeholder]
```

### 3. Proximos Pasos (Expectativas)

Comunicar que vera el stakeholder en la proxima entrega. Usar lenguaje futuro y concreto.

**Formato:**
```
## Proximos Pasos

📅 Para la proxima sesion:
1. [Feature/mejora en lenguaje de negocio]
2. [Feature/mejora en lenguaje de negocio]

📅 Esta semana:
1. [Objetivo de negocio]
```

## Reporte de Inversion (Seccion Financiera)

Si hay datos disponibles del agente de Finanzas (`knowledge-f1n`), incluir una seccion de inversion transparente.

### Formato

```
## 📊 Inversion del Periodo

| Concepto | Monto |
|----------|-------|
| Uso total del periodo | $XX.XX |
| Creditos restantes | $XX.XX |
| Porcentaje del presupuesto usado | XX% |

Distribucion por area:
- Desarrollo backend: XX%
- Desarrollo frontend: XX%
- Infraestructura y despliegue: XX%
- Pruebas y calidad: XX%
- Coordinacion y documentacion: XX%
```

**Regla**: NUNCA mostrar nombres de modelos de IA (claude-opus, haiku, etc.) al stakeholder. Traducir a areas funcionales.

## Notion Dashboard Structure

### Base de datos principal: Project Features

| Propiedad | Tipo | Descripcion |
|-----------|------|-------------|
| Feature Name | Title | Nombre de la feature en lenguaje de negocio |
| Status | Select | ⏳ En Planificacion / 🏗️ En Desarrollo / ✅ Completado / 🚨 Requiere Atencion |
| Progress | Number (%) | Porcentaje de checkboxes marcados en `tasks.md` |
| Priority | Select | ⭐ Alcance Minimo / 📋 Incremental / 📌 Paralelizable |
| Last Updated | Date | Fecha de ultima actualizacion |
| Spec Folder | Text | Ruta `specs/NNN-feature-name/` (referencia interna) |
| Notes | Rich Text | Resumen ejecutivo del estado actual |

### Pagina: Resumen Ejecutivo

Pagina de tipo "callout" o "page" que contiene el ultimo resumen ejecutivo generado.

### Pagina: Historial de Reportes

Tabla con todos los resumenes anteriores para tracking de progreso historico.

## Modo Markdown (Fallback)

Cuando Notion no esta disponible, generar reportes en `/docs/reports/` con este formato de nombre:

```
docs/reports/YYYY-MM-DD-session-report.md
docs/reports/YYYY-MM-DD-weekly-report.md
docs/reports/YYYY-MM-DD-sprint-report.md
```

### Estructura del archivo Markdown

```markdown
# Reporte de Avance - [Fecha]

**Proyecto**: [Nombre del proyecto]
**Periodo**: [Fecha inicio] al [Fecha fin]
**Estado**: 🟢 En Camino / 🟡 Atencion / 🔴 Problema

---

## ✅ Lo que logramos

1. [Hito 1]
2. [Hito 2]
3. [Hito 3]

## 📊 Estado de Features

| Feature | Progreso | Estado |
|------|----------|--------|
| [Nombre] | ██████░░░░ 60% | 🏗️ En Desarrollo |
| [Nombre] | ██████████ 100% | ✅ Completado |

## 🚨 Blockers (si aplica)

- **[Blocker]**: [Descripcion y accion requerida]

## 📅 Proximos Pasos

1. [Siguiente entrega]
2. [Siguiente entrega]

## 📊 Inversion del Periodo (si aplica)

[Datos financieros traducidos]

---
*Generado automaticamente por el agente de reportes*
```

## Reglas de Calidad

1. **Empatia**: Reconocer el progreso antes de comunicar problemas
2. **Honestidad**: No maquillar retrasos — comunicarlos con soluciones
3. **Brevedad**: Maximo 1 pagina de resumen ejecutivo
4. **Scannability**: Usar emojis, tablas y bullets para lectura rapida
5. **No tecnico**: Si un stakeholder necesita Google para entender un termino, esta mal escrito
6. **Accionable**: Todo blocker debe incluir que necesitamos del stakeholder
7. **Consistente**: Mismo formato y estructura en cada reporte
8. **Trazable**: Siempre referenciar la carpeta de spec (`specs/NNN-feature-name/`) internamente para que el equipo pueda cruzar datos

## Anti-patterns

| Anti-pattern | Correccion |
|-------------|-----------|
| Usar jerga tecnica sin traducir | Consultar tabla de traduccion arriba |
| Reportar tareas individuales | Agrupar en hitos de negocio |
| Solo reportar problemas | Siempre empezar con logros |
| Reportes de mas de 1 pagina | Resumir, linkear detalles si es necesario |
| Mostrar nombres de modelos IA | Traducir a areas funcionales |
| No incluir proximos pasos | Siempre cerrar con expectativas concretas |
| Omitir blockers que requieren al stakeholder | Destacarlos con 🚨 prominente |
