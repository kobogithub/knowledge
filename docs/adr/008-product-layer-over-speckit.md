# ADR-008: A Product Layer Above spec-kit, With Signature Gates

## Status

Accepted

## Date

2026-09-06

## Context

[ADR-006](./006-adopt-speckit-remove-beads.md) adopted spec-kit as the workflow: a
`specs/NNN-feature-name/` folder per initiative, `spec.md` → `plan.md` → `tasks.md`, and
eleven agent roles working from it. That solved everything **from the spec down**.

Nothing sat above it. The Planner started `/speckit-specify` from a sentence in a chat, so
the agent filled the gaps with assumptions. Three problems followed:

- **Specs that answer the wrong question.** Nobody wrote down what the client actually
  said, so there was nothing to check the spec against.
- **No scope contract.** With no agreed statement of what is being built, there is nothing
  to point at when a request appears mid-sprint, and nothing to say "that is a new sprint".
- **No project state without reading code.** A maintainer running small client projects as
  a second income needs to answer "where is this?" without opening the repo.

The discovery input for this decision
([`docs/product/discovery/2026-09-06-metodologia-agentes.md`](../product/discovery/2026-09-06-metodologia-agentes.md))
described a chain of artifacts — Discovery → Scope → Epics → Stories → Specs → Issues → PRs
— where each document derives from the previous one and a human signs at each gate. The
question was how to fit that onto a repo that already had spec-kit and eleven roles,
without duplicating either.

## Decision

Add a **product layer** in `docs/product/`, above spec-kit and feeding it.

1. **Three artifacts before the spec.** `PROJECT.md` (the brief: problem, objective, scope,
   out of scope, success criteria), `PRD.md` (the scope contract: epics, roadmap, releases,
   risks), and `stories/EPIC-xx/US-NN.md` (user stories with Gherkin). Each declares which
   artifact it derives from.

2. **An epic maps 1:1 to a `specs/NNN-*/` folder.** The source guide proposed one spec per
   user story; spec-kit already groups several user stories under one spec, so the unit of
   spec is the epic, not the story.

3. **The story is the source; the spec derives from it.** A spec's "User Story N" sections
   carry the same ID and the same Gherkin scenarios as `US-NN.md`. Where they diverge, the
   story wins and the spec is corrected. This is what keeps two layers from becoming two
   truths.

4. **A signature header on every artifact.** `Estado` (Borrador | En revisión | Aprobado |
   Reemplazado), `Firmado por`, `Fecha de firma`. Signing is filling three fields. An
   approved artifact that is later edited returns to "En revisión".

5. **Client approval is separate from the signature.** `PROJECT.md` and `PRD.md` carry a
   second block — `Aprobado por cliente`, `Fecha de aprobación`, `Evidencia de aprobación`.
   The signature says "this artifact is correct" and the maintainer gives it; the approval
   says "I accept this scope" and the client gives it. Collapsing them would have the gate
   block the whole chain waiting on a client. Stories and specs carry the signature alone.

6. **The gate rule lives in one skill.** `product-gate` parses the header and answers PASA
   or SE DETIENE. Every command calls it instead of reimplementing the rule, so changing
   the rule is editing one file. In a repo with no `docs/product/`, it reports and lets
   through — initiatives 001-004 keep working unchanged.

7. **The commands are Claude Code skills**, `.claude/skills/product-*/`, which is how
   `speckit-*` already ships. They need their own `.gitignore` exception to be versioned.

8. **spec-kit's own commands are gated through `.specify/extensions.yml`**, using the
   `before_specify` and `before_plan` hooks, without editing the `speckit-*` skills —
   `specify-cli` regenerates those.

9. **One new role: Analyst**, owning discovery only. PM maps to the existing Planner. The
   Architect became its own role in a later decision, but not as part of this one.

10. **Artifacts live in git.** Notion and Drive carry derived, read-only copies for the
    client: a progress dashboard from `STATUS.md`, and the PDF exported at signature time.
    The derived copy is never edited.

## Alternatives Considered

### Adopt BMAD-METHOD or OpenSpec as the tooling

- Pros: ready-made role definitions and an agent workflow; less to write.
- Cons: replaces spec-kit, adopted six weeks earlier in ADR-006, and drags in a second
  spec layer.
- Why rejected: the gap was above the spec, not in it. Take the roles as reference, keep
  spec-kit.

### One spec per user story

- Pros: matches the source guide literally; smaller specs.
- Cons: spec-kit already groups user stories inside one `spec.md`; one folder per story
  would mean dozens of near-empty initiatives and a `tasks.md` per story.
- Why rejected: fights the tool. The epic is the natural unit.

### Put the gate check inside each `speckit-*` skill

- Pros: no extension file; the rule sits where it is enforced.
- Cons: `specify-cli` regenerates those skills and would silently drop the checks.
- Why rejected: not durable. Hooks in `.specify/extensions.yml` survive regeneration.

### Keep the product documents in Notion

- Pros: the client can read them without a repo.
- Cons: agents cannot read them, `product-gate` cannot parse a Notion property, and an
  edit destroys the signed version.
- Why rejected: it splits the source of truth. Notion gets a derived copy instead.

### No signature at all, just conventions

- Pros: zero friction.
- Cons: nothing stops an agent deriving a contract from a draft, which is the failure
  the layer exists to prevent.
- Why rejected: a convention nobody verifies is not a gate.

## Consequences

### Positive

- A spec can be checked against something: the story it derives from, and behind that the
  brief the client approved.
- Scope creep has a document to be refused against — the PRD is the contract.
- The Gherkin in a story becomes the acceptance criteria, QA's PR checklist, and the
  spec's scenarios, written once.
- A repo without `docs/product/` is unaffected, so existing initiatives keep passing.
- The rule sits in one skill, so tightening or relaxing it is one file.

### Negative

- Two more documents before any code gets written. On a genuinely small job that is
  overhead.
- Story and spec both hold the scenarios, so they can drift. Mitigated by the derivation
  rule and `/speckit-analyze`, not eliminated.
- The maintainer becomes a bottleneck: nothing advances without a signature.

### Risks

- **Gates get skipped under time pressure.** Mitigation: the agent verifies, not the
  human's memory — signing is a field, not a ceremony.
- **The layer becomes bureaucracy on small projects.** Mitigation: `product-gate` never
  blocks a repo without `docs/product/`, so the layer is opt-in per project. Revisit after
  the pilot.
- **Client approval never arrives and work stalls.** Mitigation: approval is a separate
  block from the signature, so internal work proceeds while the client is pending; only
  what the artifact authorizes waits.

## References

- [`specs/005-agent-team-projects/`](../../specs/005-agent-team-projects/) — spec, plan and tasks for this initiative
- [`docs/product/`](../product/) — the layer itself, and its README
- [`docs/product/discovery/2026-09-06-metodologia-agentes.md`](../product/discovery/2026-09-06-metodologia-agentes.md) — the input this decision derives from
- [ADR-006](./006-adopt-speckit-remove-beads.md) — adopting spec-kit, which this builds on
- [ADR-007](./007-personal-stacks-and-curated-catalog.md) — the curated skill catalog
