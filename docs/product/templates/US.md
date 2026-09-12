# US-NN: [título de la story]

**Épica**: EPIC-xx
**Estado**: Borrador
**Firmado por**: —
**Fecha de firma**: —
**Prioridad**: P1
**Rol ejecutor**: [Rol] (`<prefijo>-xxx`)

Como [rol] quiero [acción] para [valor].

## Contexto

[Solo si hace falta: qué encontró el discovery, qué está roto hoy, qué decisión previa
condiciona esta story. Si la frase "Como/quiero/para" se explica sola, borrar la sección.]

## Criterios de aceptación

```gherkin
Feature: [la capacidad que la story habilita]

  Scenario: [caso principal, en positivo]
    Given [estado inicial verificable]
    When [la acción del usuario o del agente]
    Then [el resultado observable]
    And [otra condición del mismo resultado]

  Scenario: [caso que la story debe rechazar o el borde que importa]
    Given [...]
    When [...]
    Then [...]
```

> Este Gherkin es **la fuente**. De acá salen las secciones "User Story N" del `spec.md`
> (mismo ID, mismos escenarios) y la validación que QA comenta en cada PR, un renglón por
> Scenario con pasa/falla. Si el spec y la story divergen, manda la story.

## Notas / dependencias

- [De qué otra story depende, o "No depende de ninguna otra story".]
- [Decisiones que esta story formaliza en un ADR, si las hay.]

---

**Plantilla**: etapa 3 de la cadena de producto. Las genera `/product-stories EPIC-xx`,
que exige `PRD.md` en Estado "Aprobado" y la épica en su tabla. Ver
[`../README.md`](../README.md).
