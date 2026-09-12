---
name: "product-prd"
description: "Etapa 2 de la cadena de producto: deriva docs/product/PRD.md — el contrato de alcance, con épicas, roadmap y riesgos — de un PROJECT.md aprobado. Se detiene si el PROJECT no está firmado."
argument-hint: "(sin argumentos; lee docs/product/PROJECT.md)"
compatibility: "Requiere docs/product/PROJECT.md Aprobado y docs/product/templates/PRD.md. Rol: Planner (knowledge-x6e)."
metadata:
  author: "knowledge"
  spec: "specs/005-agent-team-projects/ (EPIC-01, US-03)"
user-invocable: true
disable-model-invocation: false
---

## User Input

```text
$ARGUMENTS
```

No lleva argumentos. Si el usuario pasó algo, tratalo como guía adicional sobre el corte de
épicas o el orden de sprints, nunca como permiso para saltear la compuerta.

## Qué hace

Genera `docs/product/PRD.md`, **etapa 2** de la cadena: el **contrato de alcance**. Lo que
está en el PRD es lo que se va a construir; lo que no está es una story nueva en un sprint
nuevo. Es el documento que el cliente aprueba y contra el que se frena el scope creep.

## Procedimiento

### 1. Compuerta (primer paso, siempre)

Invocá `product-gate docs/product/PROJECT.md`.

- **SE DETIENE** → repetí el mensaje del gate al usuario y **terminá sin escribir nada**:

  ```
  SE DETIENE: docs/product/PROJECT.md tiene Estado <x>. El PRD es el contrato y no se
  deriva de un discovery sin firmar: si el brief cambia después, el contrato queda
  apoyado en algo que nadie aprobó. Revisá y firmá el PROJECT primero.
  ```

  No ofrezcas generar "un borrador mientras tanto". El borrador es exactamente lo que la
  compuerta impide.

- **PASA** → seguí.

Después, si `docs/product/PRD.md` ya existe, invocá `product-gate docs/product/PRD.md`:

- Está **Aprobado** → detenete. Un PRD firmado no se regenera: los cambios de alcance se
  agregan como épicas nuevas y exigen volver a firmar. Decilo así y ofrecé listar qué
  habría que agregar.
- Está en Borrador o En revisión → seguí, avisando que lo vas a sobrescribir.

### 2. Leer el PROJECT entero

De acá sale todo. Prestá atención especial a:

- **Alcance** → es la materia prima de las épicas
- **Fuera de alcance** → se hereda tal cual; es lo que vas a citar cuando aparezca un
  pedido a mitad de sprint
- **Criterio de éxito** → cada criterio debería quedar cubierto por al menos una épica; si
  alguno no lo está, o falta una épica o el criterio no era del proyecto
- **Preguntas abiertas** → se heredan. Una pregunta sin responder no bloquea la firma del
  PRD, pero sí debería bloquear la épica que depende de ella: anotalo en esa épica.

### 3. Cortar en épicas

Una épica es **una unidad de alcance que mapea 1:1 a una carpeta `specs/NNN-*/`**. Ese es
el criterio de corte, no el tamaño.

- Si una épica no se puede describir como un spec con user stories propias, es demasiado
  chica: fusionala.
- Si necesitaría dos carpetas de spec, es demasiado grande: partila.
- Cada épica tiene que entregar algo visible por sí sola. "Refactor de la capa de datos" no
  es una épica; "el cliente puede ver su historial de pedidos" sí.

Prioridad: **P1** bloquea el objetivo, **P2** lo mejora, **P3** es deseable.

Sprint: las P1 primero, respetando dependencias. No metas dos épicas grandes en el mismo
sprint por optimismo.

### 4. Escribir `docs/product/PRD.md`

Desde `docs/product/templates/PRD.md`. El encabezado sale en **Borrador**, con el bloque de
aprobación del cliente vacío y link al `PROJECT.md`.

Tiene que llevar sí o sí:

- **Tabla de épicas**: ID, épica, prioridad, sprint, spec, estado. La columna Spec dice
  "se crea al entrar al sprint" hasta que exista la carpeta.
- **Una sección por épica**: valor (qué gana el cliente, en una frase), alcance previsto,
  y entregables visibles.
- **Roadmap por sprint**, con el hito de cada uno.
- **Plan de releases**: qué versión sale primero y cuáles siguen. Cada épica cerrada
  corresponde a un release. Cambiar el plan es cambio de contrato y exige volver a firmar.
- **Fuera de alcance**: heredado del PROJECT más lo que se descartó al armar las épicas.
- **Riesgos**: riesgo, probabilidad, impacto, respuesta, y qué épica lo cubre.
- **Preguntas abiertas** heredadas.

### 5. Autochequeo

- [ ] ¿Cada criterio de éxito del PROJECT está cubierto por alguna épica?
- [ ] ¿Cada épica entrega algo visible por sí sola?
- [ ] ¿Cada épica podría ser una carpeta `specs/NNN-*/`?
- [ ] ¿Hay alguna épica que el PROJECT no respalde? Sacala o abrí la pregunta.
- [ ] ¿El plan de releases dice cuál sale primero?
- [ ] ¿Las preguntas abiertas que bloquean una épica están anotadas en esa épica?

### 6. Reportar

```
✅ docs/product/PRD.md generado desde PROJECT.md (Aprobado por <firmante>, <fecha>).

N épicas, M sprints. Primer release: vX.Y.Z (EPIC-01).

Estado: Borrador — necesita tu firma, y la aprobación del cliente antes de arrancar.

Preguntas abiertas heredadas: N (bloquean: <cuáles>)

Siguiente paso: revisá y firmá el PRD. Después /product-stories EPIC-01.
```

## Reglas

1. **La compuerta es el primer paso, sin excepción.** Un PRD derivado de un PROJECT sin
   firmar no es un contrato.
2. **No firmes.** Sale en Borrador. La firma es del mantenedor; la aprobación, del cliente.
3. **No inventes alcance.** Si una épica no tiene respaldo en el PROJECT, sobra. Lo que el
   cliente no pidió no entra "porque sería útil".
4. **No regeneres un PRD aprobado.** Un cambio de alcance es una épica nueva y una firma
   nueva, no una regeneración silenciosa.
5. **Una épica, una carpeta de spec.** Es el criterio de corte.
