---
name: "product-gate"
description: "Verifica la compuerta de firma de un artefacto de la cadena de producto (PROJECT, PRD, story, spec) antes de derivar el siguiente. Responde PASA o SE DETIENE."
argument-hint: "<ruta-del-artefacto> [EPIC-xx]"
compatibility: "Requiere la capa de producto en docs/product/. En repos sin ella, informa y deja avanzar."
metadata:
  author: "knowledge"
  spec: "specs/005-agent-team-projects/ (EPIC-01, US-05)"
user-invocable: true
disable-model-invocation: false
---

## User Input

```text
$ARGUMENTS
```

Primer argumento: ruta del artefacto a verificar.
Segundo argumento opcional: ID de épica (`EPIC-xx`), que activa las verificaciones extra
de la sección "Verificación de épica".

### Invocación sin argumentos (desde un hook de spec-kit)

Los hooks de `.specify/extensions.yml` emiten `EXECUTE_COMMAND: product-gate` **sin
argumentos**: el mecanismo no los transporta. Cuando `$ARGUMENTS` esté vacío, **inferí el
objetivo del contexto en lugar de detenerte** — un gate que frena siempre se termina
desactivando, y entonces no protege nada.

Mirá qué comando te invocó:

| Invocado desde | Verificá |
|---|---|
| `before_plan` (`/speckit-plan`) | el `spec.md` de la feature activa: leé `feature_directory` de `.specify/feature.json` y verificá `<feature_directory>/spec.md` |
| `before_specify` (`/speckit-specify`) | `docs/product/PRD.md`, más la épica que corresponda a la feature que se está por crear |
| directamente por una persona, sin contexto | pedí la ruta: `SE DETIENE: product-gate necesita la ruta del artefacto a verificar.` |

En `before_specify` puede que todavía no sepas qué épica es —la feature no existe—. En ese
caso verificá solo que `docs/product/PRD.md` esté Aprobado, y sumá al resultado:

```
Recordá que la épica de esta feature tiene que estar en el PRD y tener stories. Si no las
tiene, corré /product-stories <EPIC-xx> antes de seguir.
```

Si `.specify/feature.json` no existe o apunta a un directorio inexistente, informá y dejá
avanzar: es un repo sin feature activa, no un incumplimiento.

## Qué es esto

El único lugar del repo donde vive la regla de compuerta. Los comandos `/product-prd`,
`/product-stories`, `/speckit-specify` y `/speckit-plan` la invocan en lugar de
reimplementarla, para que cambiarla sea editar un solo archivo (FR-006).

**No escribe nada.** Solo lee, decide y reporta. Quien lo invoca decide qué hacer con el
veredicto: si es `SE DETIENE`, aborta sin escribir el artefacto derivado.

## Contrato del encabezado de firma

Todo artefacto de la cadena lleva estas tres líneas en su encabezado (FR-008):

```markdown
**Estado**: Borrador | En revisión | Aprobado | Reemplazado
**Firmado por**: <nombre>
**Fecha de firma**: YYYY-MM-DD
```

Firmar es completar esos tres campos. El agente propone; el humano firma.

### Aprobación del cliente (solo `PROJECT.md` y `PRD.md`)

Esos dos artefactos llevan además un segundo bloque, porque son los que el cliente ve y
acepta: el brief ("esto entendí de tu problema") y el contrato de alcance ("esto construyo
y en qué orden").

```markdown
**Aprobado por cliente**: <nombre>
**Fecha de aprobación**: YYYY-MM-DD
**Evidencia de aprobación**: <link al mail, PDF o registro>
```

Son **dos gestos distintos y no intercambiables**:

| | Firma | Aprobación del cliente |
|---|---|---|
| Qué afirma | el artefacto es correcto | el cliente acepta este alcance |
| Quién | el mantenedor | el cliente |
| Para qué | destraba al siguiente agente | protege comercialmente |

Nunca completes uno con el otro. Un artefacto firmado sin aprobación del cliente es un
documento correcto que todavía nadie autorizó; uno aprobado sin firmar es un acuerdo que
nadie revisó.

Las stories y los `spec.md` **no** llevan este bloque: al cliente no se le hace revisar
Gherkin.

## Procedimiento

### 1. ¿El repo tiene capa de producto?

Comprobá si existe el directorio `docs/product/`.

- **No existe** → el proyecto es anterior a la capa de producto (o no la usa). Respondé:

  ```
  PASA (sin compuerta): este repo no tiene docs/product/, la capa de producto no está
  inicializada. Corré /product-discovery para empezar la cadena, o seguí sin ella.
  ```

  y terminá. **No bloqueás.** Es lo que mantiene válidas las iniciativas 001–004 (SC-005).

- **Existe** → seguí al paso 2.

### 2. ¿Existe el artefacto?

Leé el archivo de la ruta recibida.

- **No existe** → `SE DETIENE: <ruta> no existe. Hay que generarlo antes de derivar el
  siguiente artefacto.` Si la ruta es `docs/product/PROJECT.md`, agregá que se genera con
  `/product-discovery <input>`; si es `docs/product/PRD.md`, con `/product-prd`.
- **Existe** → seguí al paso 3.

### 3. Parseo del encabezado

Buscá en el archivo, en cualquier orden y en las primeras 40 líneas, las tres líneas que
empiezan con `**Estado**:`, `**Firmado por**:` y `**Fecha de firma**:`.

Distinguí dos casos, porque significan cosas distintas:

- **Faltan las tres** → el artefacto es **anterior a la capa de producto**. Existe desde
  antes de que hubiera compuertas, y bloquearlo sería romper trabajo válido:

  ```
  PASA (sin compuerta): <ruta> no tiene encabezado de firma; es anterior a la capa de
  producto. Si querés someterlo a la cadena, agregale el encabezado y firmalo.
  ```

  Es lo que mantiene válidas las iniciativas previas que conviven en un repo que **sí**
  tiene `docs/product/` — el caso de `specs/001-*` a `004-*` en este repo, y lo que exige
  SC-005 del spec. La escapatoria del paso 1 cubre repos enteros sin capa de producto;
  esta cubre artefactos sueltos anteriores dentro de un repo que ya la tiene.

- **Falta una o dos** → el encabezado existe pero está mal formado, y eso sí es un
  incumplimiento: alguien empezó a declarar la compuerta y la dejó a medias.

  ```
  SE DETIENE: <ruta> tiene el encabezado de firma incompleto. Le faltan: <campos>.
  Completalos según el contrato de docs/product/README.md.
  ```

Normalizá los valores: quitá espacios al borde. Un campo se considera **vacío** si su
valor es `—`, `-`, `TBD`, `<nombre>`, `YYYY-MM-DD` o la cadena vacía.

### 4. Veredicto

| `Estado` contiene | Veredicto |
|---|---|
| `Aprobado`, y `Firmado por` y `Fecha de firma` no vacíos | **PASA** |
| `Aprobado`, pero falta firmante o fecha | **SE DETIENE** |
| `Borrador` | **SE DETIENE** |
| `En revisión` | **SE DETIENE** |
| `Reemplazado` | **SE DETIENE** |
| cualquier otro valor | **SE DETIENE** (estado no reconocido) |

`Estado` se compara por contenido, no por igualdad: `Borrador — pendiente de firma`
cuenta como `Borrador`.

Formato exacto de las respuestas:

```
PASA: <ruta> está Aprobado (firmado por <nombre>, <fecha>).
```

```
SE DETIENE: <ruta> tiene Estado <x>; completar Firmado por y Fecha de firma, y poner
Estado en "Aprobado".
```

Cuando el estado es `Aprobado` pero falta firmante o fecha:

```
SE DETIENE: <ruta> dice Aprobado pero le falta <Firmado por | Fecha de firma>. Una firma
sin firmante no es una firma.
```

Cuando el estado es `Reemplazado`:

```
SE DETIENE: <ruta> está Reemplazado. Buscá el artefacto vigente que lo sucede.
```

### 5. Aprobación del cliente (solo `PROJECT.md` y `PRD.md`)

Se corre **además** de los pasos anteriores, y solo si el veredicto hasta acá es `PASA`.
Para cualquier otro artefacto, salteá este paso.

Buscá `**Aprobado por cliente**:`, `**Fecha de aprobación**:` y
`**Evidencia de aprobación**:`, con el mismo criterio de campo vacío del paso 3.

- **Faltan las tres líneas** → el artefacto es anterior a esta convención:

  ```
  PASA (sin aprobación de cliente): <ruta> está firmado pero no tiene el bloque de
  aprobación del cliente. Agregalo si el proyecto tiene un cliente externo.
  ```

  No bloquea: hay proyectos sin cliente externo, donde el mantenedor es su propio cliente.

- **Están pero vacías** → el bloque existe, así que el proyecto declaró tener cliente:

  ```
  SE DETIENE: <ruta> está firmado pero el cliente todavía no lo aprobó. Mandale el PDF y
  esperá su respuesta antes de arrancar el trabajo que este artefacto habilita.
  ```

- **Completas** → sumá al mensaje de PASA: `y aprobado por el cliente <nombre> el <fecha>`.

**No completes vos ninguno de esos campos.** La evidencia la registra el mantenedor
después de recibir la respuesta del cliente; un agente no puede afirmar que un cliente
aprobó algo.

### 6. Verificación de épica (solo si recibiste `EPIC-xx`)

Se corre **además** de los pasos anteriores, y solo si el veredicto hasta acá es `PASA`.
La invoca `/product-stories EPIC-xx` y el hook `before_specify`.

1. **La épica está en el PRD**: buscá `EPIC-xx` en la tabla de épicas de
   `docs/product/PRD.md`. Si no está:

   ```
   SE DETIENE: EPIC-xx no figura en la tabla de épicas de docs/product/PRD.md. Agregarla
   es un cambio de alcance: sumala al PRD y volvé a firmarlo antes de especificar.
   ```

2. **La épica tiene stories** (solo cuando quien invoca va a derivar un `spec.md`, es
   decir el hook `before_specify`): comprobá que exista `docs/product/stories/EPIC-xx/`
   con al menos un `US-*.md`. Si no:

   ```
   SE DETIENE: EPIC-xx no tiene stories en docs/product/stories/EPIC-xx/. Corré
   /product-stories EPIC-xx primero.
   ```

   `/product-stories` es justamente el comando que las crea, así que cuando **él** invoca
   esta verificación, saltea este punto 2.

3. Si pasa todo:

   ```
   PASA: docs/product/PRD.md está Aprobado (firmado por <nombre>, <fecha>) y EPIC-xx
   figura en el PRD[ con N stories].
   ```

## Cómo lo invocan los otros comandos

Es el **primer paso** de `/product-prd` y `/product-stories`, y el hook `before_prd` /
`before_specify` / `before_plan` de `.specify/extensions.yml`.

| Quien invoca | Argumentos | Qué exige |
|---|---|---|
| `/product-prd` | `docs/product/PROJECT.md` | discovery firmado |
| `/product-stories EPIC-xx` | `docs/product/PRD.md EPIC-xx` | PRD firmado y la épica en él |
| `/speckit-specify` (hook) | `docs/product/PRD.md EPIC-xx` | PRD firmado, épica en él, stories creadas |
| `/speckit-plan` (hook) | `specs/NNN-*/spec.md` | spec firmado |

> **Nota sobre los hooks de spec-kit** (research R1, verificado contra
> `.claude/skills/speckit-plan/SKILL.md`): un hook bloquea de verdad — el skill espera su
> resultado antes de seguir — **solo si se declara con `optional: false` explícito**. Sin
> ese campo, el skill no cae en la rama de hook obligatorio. Declarar siempre
> `optional: false` en `.specify/extensions.yml`.

## Reglas

1. **No escribas nada.** Ni el artefacto verificado ni el derivado. Solo leés y reportás.
2. **No firmes por decisión propia.** Ningún agente decide que un artefacto está listo.
   Si el veredicto es `SE DETIENE` porque falta la firma, decilo y terminá; no ofrezcas
   firmarlo vos.

   Lo único permitido es **transcribir**: el mantenedor te autoriza explícitamente, en esa
   misma sesión, a estampar su firma sobre un artefacto que él ya aprobó. En ese caso el
   campo tiene que dejar constancia de cómo se obtuvo:

   ```markdown
   **Firmado por**: <nombre> (firma transcrita por <agente> por autorización en sesión)
   ```

   La diferencia no es formal. Transcribir registra una decisión que el humano tomó;
   firmar por iniciativa propia inventa una que nadie tomó. Nunca transcribas la firma de
   un artefacto cuyo contenido el mantenedor no vio: la compuerta vale porque el registro
   es cierto, y una firma sobre algo no leído lo vuelve falso.
   Para `Aprobado por cliente` no hay transcripción posible: esa la registra el mantenedor
   después de recibir la respuesta del cliente.
3. **No inventes el estado.** Si el encabezado no se puede parsear, es `SE DETIENE`, no
   una suposición optimista.
4. **Un repo sin `docs/product/` nunca se bloquea.** Informar no es bloquear.
