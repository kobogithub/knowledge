---
name: "product-discovery"
description: "Etapa 1 de la cadena de producto: convierte un input de discovery (transcripción, brief, notas del cliente) en docs/product/PROJECT.md, sin inventar lo que el cliente no dijo."
argument-hint: "<ruta-del-input-en-docs/product/discovery/>"
compatibility: "Requiere docs/product/templates/PROJECT.md. Rol: Analyst (knowledge-an1)."
metadata:
  author: "knowledge"
  spec: "specs/005-agent-team-projects/ (EPIC-01, US-02)"
user-invocable: true
disable-model-invocation: false
---

## User Input

```text
$ARGUMENTS
```

Ruta del archivo de input, dentro de `docs/product/discovery/`.

Si está vacío, listá lo que haya en `docs/product/discovery/` y preguntá cuál usar. Si el
directorio no existe o está vacío, respondé:

```
SE DETIENE: no hay input de discovery. Guardá la transcripción, el brief o las notas del
cliente en docs/product/discovery/<fecha>-<tema>.md y volvé a correr el comando.
```

No trabajes desde el chat. Lo que no está en git no es evidencia.

## Qué hace

Genera `docs/product/PROJECT.md` — el **brief**, etapa 1 de la cadena — derivándolo del
input. Es el único artefacto que produce este comando.

## La regla que define todo

**Lo que no está en el input, no está en el PROJECT.** Va a "Preguntas abiertas", nunca
como decisión tomada.

No es una preferencia de estilo. Un dato inventado en un brief se vuelve un compromiso en
el contrato tres semanas más tarde, y el cliente lo va a leer como algo que vos prometiste.
Ante la duda entre afirmar y preguntar, preguntá.

## Procedimiento

### 1. Compuerta

Si `docs/product/PROJECT.md` ya existe, invocá `product-gate docs/product/PROJECT.md`:

- **PASA** (está Aprobado) → **detenete**:

  ```
  SE DETIENE: docs/product/PROJECT.md ya está Aprobado. Un discovery nuevo no pisa uno
  firmado: marcá el actual como Estado "Reemplazado", guardalo, y recién ahí generá el
  nuevo desde este input.
  ```

- **SE DETIENE** (está en Borrador o En revisión) → seguí: se puede regenerar sobre un
  borrador, avisando que lo vas a sobrescribir.
- **No existe** → seguí.

### 2. Leer el input entero

Leelo completo antes de escribir una línea. No lo edites nunca: es la evidencia de lo que
el cliente dijo.

Mientras leés, armá dos listas:

- **Lo afirmado**: lo que el cliente dice explícitamente. Cada ítem con la línea que lo
  respalda.
- **Lo ausente**: lo que la plantilla pide y el input no contesta. Esta lista se convierte
  en "Preguntas abiertas", una pregunta por ítem.

Si el input es muy corto o no tiene ninguna sección reconocible, **no abortes**: generá el
`PROJECT.md` con todas las secciones como preguntas abiertas y decilo explícitamente
arriba del archivo.

### 3. Cargar la plantilla

`docs/product/templates/PROJECT.md`. Respetá las diez secciones en su orden; no agregues,
no quites, no reordenes. Una sección que el input no cubre se completa con "no especificado
en el input" y su pregunta abajo, pero **la sección se queda**.

### 4. Escribir `docs/product/PROJECT.md`

Encabezado:

```markdown
**Estado**: Borrador
**Firmado por**: —
**Fecha de firma**: —
**Aprobado por cliente**: —
**Fecha de aprobación**: —
**Evidencia de aprobación**: —
**Agente autor**: Analyst (`<prefijo>-an1`)
**Input**: link relativo al input, p. ej. `[discovery/2026-09-06-tema.md](./discovery/2026-09-06-tema.md)`
```

Si el proyecto no tiene cliente externo, borrá el bloque de aprobación.

Sobre secciones específicas:

- **Objetivo**: una sola frase, con una condición verificable. Si el input no da con qué
  medirlo, dejá el objetivo cualitativo y mandá la métrica a Preguntas abiertas.
- **Fuera de alcance**: es la sección que después frena el scope creep. Poné lo que el
  cliente descartó explícitamente y lo que quedó para más adelante. Si el input no dice
  nada, preguntá qué queda afuera — no la dejes vacía en silencio.
- **Restricciones**: técnicas, tiempo, presupuesto, legales. **Sin presupuesto ni fechas en
  el input, no inventes cifras.** "Ninguna identificada" es válido en legales; "no
  especificado en el input" es lo correcto en presupuesto.
- **Criterio de éxito**: condiciones verificables, no aspiraciones. "El cliente está
  contento" no sirve; "un pedido se carga en menos de tres pantallas" sí.
- **Preguntas abiertas**: una por cada dato faltante, redactada para que el cliente la
  conteste en una frase. Nada de "definir alcance del módulo X".

### 5. Autochequeo antes de entregar

Recorré lo que escribiste contra el input:

- [ ] ¿Cada afirmación tiene respaldo en una línea del input?
- [ ] ¿Hay alguna cifra, fecha, nombre o integración que pusiste vos?
- [ ] ¿Restricciones dice "no especificado en el input" donde el input calla?
- [ ] ¿Hay una pregunta abierta por cada dato faltante?
- [ ] ¿Están las diez secciones, en orden?
- [ ] ¿El Objetivo es una frase y es verificable?

El segundo es el que más rinde: buscá activamente lo que agregaste de más.

### 6. Reportar

```
✅ docs/product/PROJECT.md generado desde <input>.

Estado: Borrador — necesita tu firma, y la aprobación del cliente antes de arrancar.

Preguntas abiertas (N):
  1. ...
  2. ...

Bloquean el PRD: <cuáles, o "ninguna">

Siguiente paso: revisá y firmá el PROJECT. Después corré /product-prd.
```

## Reglas

1. **No firmes.** El `PROJECT.md` sale en Borrador. La firma es del mantenedor y la
   aprobación es del cliente; son dos gestos distintos y ninguno es tuyo.
2. **No edites el input.** Es evidencia.
3. **No propongas soluciones.** El brief describe el problema, el alcance y el criterio de
   éxito. Arquitectura, épicas y tecnología son de otros roles y de etapas posteriores.
4. **No pises un PROJECT aprobado.** Paso 1.
5. **Preferí la pregunta a la afirmación.** Es todo el valor de este comando.
