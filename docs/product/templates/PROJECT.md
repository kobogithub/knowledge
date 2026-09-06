# Proyecto: [nombre del proyecto]

**Estado**: Borrador
**Firmado por**: —
**Fecha de firma**: —
**Agente autor**: Analyst (`<prefijo>-an1`)
**Input**: [`discovery/[fecha]-[tema].md`](./discovery/[fecha]-[tema].md)

> Etapa 1 de la cadena. Este documento se deriva del input de discovery y **no** agrega
> requerimientos que el cliente no haya expresado. Lo que no está en el input aparece en
> "Preguntas abiertas", nunca como decisión tomada.

## Cliente y contexto

[Quién es el cliente, a qué se dedica, qué tiene hoy funcionando y qué harness/stack usa.
Solo lo que está en el input.]

## Problema que resuelve

[El dolor concreto, en los términos del cliente. Qué pasa hoy sin este proyecto: qué
cuesta tiempo, qué se rompe, qué no se puede saber.]

## Objetivo (una frase medible)

[Una sola frase, con una condición verificable y un horizonte. Si el input no da con qué
medirlo, la métrica va a Preguntas abiertas y acá queda el objetivo cualitativo.]

## Alcance

- [Entregable 1: qué se construye, en una línea.]
- [Entregable 2.]
- [...]

## Fuera de alcance

- [Lo que explícitamente NO entra, y por qué (otra etapa, otro presupuesto, decisión
  tomada). Esto es lo que frena el scope creep más adelante.]

## Restricciones

- **Técnicas**: [stack obligado, integraciones existentes, plataformas soportadas.]
- **Tiempo**: [fechas, cadencia de sprints, disponibilidad del equipo.]
- **Presupuesto**: [tope de gasto, servicios ya pagos, "sin gasto nuevo".]
- **Legales**: [regulación, datos personales, licencias. "Ninguna identificada" es una
  respuesta válida.]

> Si el input no menciona presupuesto o fechas, **no inventes cifras**: escribí "no
> especificado en el input" y abrí la pregunta abajo.

## Criterio de éxito

1. [Condición verificable 1: cómo sabemos, mirando algo concreto, que el proyecto cumplió.]
2. [Condición 2.]
3. [...]

## Riesgos conocidos

| Riesgo | Impacto | Mitigación |
|---|---|---|
| [Qué puede salir mal] | [Qué pasa si sale mal] | [Qué hacemos para que no pase] |

## Preguntas abiertas

- [Una pregunta por cada dato que el proyecto necesita y el input no trae. Redactada para
  que el cliente pueda contestarla en una frase.]

---

**Plantilla**: etapa 1 de la cadena de producto. La genera `/product-discovery <input>`.
Ver [`../README.md`](../README.md). Las diez secciones (título + nueve) son las de la guía
de discovery; no se quitan ni se reordenan, aunque queden con "no especificado".
