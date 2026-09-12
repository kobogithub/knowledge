---
name: "product-stories"
description: "Etapa 3 de la cadena de producto: genera las user stories con Gherkin de una épica del PRD aprobado, en docs/product/stories/EPIC-xx/. El Gherkin es la fuente de los escenarios del spec y del checklist de QA."
argument-hint: "EPIC-xx"
compatibility: "Requiere docs/product/PRD.md Aprobado con la épica en su tabla, y docs/product/templates/US.md. Rol: Planner (knowledge-x6e)."
metadata:
  author: "knowledge"
  spec: "specs/005-agent-team-projects/ (EPIC-01, US-04)"
user-invocable: true
disable-model-invocation: false
---

## User Input

```text
$ARGUMENTS
```

El ID de la épica, con la forma `EPIC-xx`. Si está vacío o no tiene esa forma, listá las
épicas de la tabla de `docs/product/PRD.md` con su estado y preguntá cuál.

## Qué hace

Genera `docs/product/stories/EPIC-xx/US-NN.md`, una por story, **etapa 3** de la cadena.

El **Gherkin es el puente** entre producto y spec-kit. De cada bloque salen tres cosas:

1. las secciones "User Story N" del `spec.md` — mismo ID, mismos escenarios;
2. los tests que verifican la implementación;
3. el checklist con el que QA valida cada PR, un renglón por Scenario.

Por eso un Scenario mal escrito no es un problema de redacción: es un test que nadie va a
poder correr y un criterio que nadie va a poder verificar.

## Procedimiento

### 1. Compuerta (primer paso, siempre)

Invocá `product-gate docs/product/PRD.md EPIC-xx`, que verifica dos cosas: que el PRD esté
Aprobado y que la épica figure en su tabla.

- **PRD sin firmar** → repetí el mensaje del gate y terminá sin escribir nada:

  ```
  SE DETIENE: docs/product/PRD.md tiene Estado <x>. Las stories son el detalle del
  contrato; no se escriben sobre un contrato que nadie firmó.
  ```

- **La épica no está en el PRD** → terminá:

  ```
  SE DETIENE: EPIC-xx no figura en la tabla de épicas de docs/product/PRD.md. Agregarla
  es un cambio de alcance: sumala al PRD y volvé a firmarlo antes de escribir sus stories.
  ```

  Esto es lo que impide que aparezca trabajo que nadie acordó.

- **PASA** → seguí. (El gate saltea la verificación de "la épica tiene stories" cuando lo
  invoca este comando: justamente las va a crear.)

### 2. Leer la épica en el PRD

Su valor, alcance previsto y entregables visibles. Las stories son la descomposición de
ese alcance, **nada más**. Si al escribirlas aparece algo que el PRD no menciona, no lo
agregues: anotalo y decilo en el reporte final como candidato a cambio de alcance.

### 3. Numerar

Mirá `docs/product/stories/EPIC-xx/` y seguí desde el último `US-NN` existente. Los IDs no
se reciclan ni se renumeran: una story borrada deja su hueco, porque el spec, los PR y los
comentarios de QA la referencian por ID.

### 4. Escribir cada story

Desde `docs/product/templates/US.md`. Cada archivo lleva:

- **Encabezado**: Épica, Estado (`Borrador`), Firmado por (`—`), Fecha de firma (`—`),
  Prioridad, Rol ejecutor. Sin bloque de aprobación del cliente: al cliente no se le hace
  revisar Gherkin.
- **La frase**: "Como <rol> quiero <acción> para <valor>". El *para* es el que importa; si
  no sabés escribirlo, la story probablemente no vale la pena.
- **Contexto**, solo si hace falta.
- **Un bloque ```gherkin** con al menos un Scenario.
- **Notas / dependencias**: de qué otra story depende, o "no depende de ninguna otra".

#### Cómo escribir el Gherkin

- **Given** es estado verificable, no intención. "Given un usuario logueado", no "Given que
  el usuario quiere entrar".
- **When** es una sola acción.
- **Then** es un resultado **observable**: algo que se pueda mirar, medir o grepear. "Then
  el sistema funciona bien" no es verificable; "Then la respuesta tiene status 200 y el
  cuerpo trae el token" sí.
- Escribí también el escenario que la story debe **rechazar**, no solo el camino feliz.
- Si un Scenario necesita más de cinco líneas, probablemente son dos.

Cada Scenario tiene que poder responderse pasa/falla mirando el resultado, sin discutir.

### 5. Autochequeo

- [ ] ¿Cada story sale del alcance de la épica en el PRD?
- [ ] ¿Cada story tiene la frase Como/quiero/para completa?
- [ ] ¿Cada Scenario tiene un Then observable?
- [ ] ¿Hay al menos un escenario de rechazo o de borde, no solo el camino feliz?
- [ ] ¿Los IDs siguen desde el último existente, sin reciclar?
- [ ] ¿Quedó algo que el PRD no respalda? Va al reporte, no al archivo.

### 6. Reportar

```
✅ N stories generadas en docs/product/stories/EPIC-xx/

  US-01 <título> — N escenarios — <rol ejecutor>
  US-02 ...

Todas en Borrador: necesitan tu firma antes de /speckit-specify.

Fuera del alcance del PRD (candidatos a cambio de alcance): <lista, o "ninguno">

Siguiente paso: revisá y firmá las stories. Después /speckit-specify para la épica.
```

## La regla de derivación

Cuando el `spec.md` de la épica se genere, sus secciones "User Story N" **se derivan de
estos archivos**: mismo ID, mismos escenarios. Si divergen, **manda la story** y el spec se
corrige. `/speckit-analyze` reporta la divergencia.

Que la story mande no es arbitrario: es el artefacto que el humano revisó y firmó, y el
que QA usa para validar. El spec es una vista técnica de eso.

## Reglas

1. **La compuerta es el primer paso.** Sin PRD firmado y sin la épica en él, no escribís.
2. **No firmes.** Las stories salen en Borrador.
3. **No agregues alcance.** Lo que el PRD no respalda va al reporte como candidato, no al
   archivo.
4. **No renumeres IDs existentes.** Los referencian el spec, los PR y los comentarios de QA.
5. **Un Then que no se puede observar no es un criterio de aceptación.** Reescribilo.
