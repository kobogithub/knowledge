# Specification Quality Checklist: Absorb framework_ia

**Purpose**: Validate specification completeness and quality before proceeding to planning
**Created**: 2026-08-04
**Feature**: [spec.md](../spec.md)

## Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

## Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Success criteria are technology-agnostic (no implementation details)
- [x] All acceptance scenarios are defined
- [x] Edge cases are identified
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

## Feature Readiness

- [x] All functional requirements have clear acceptance criteria
- [x] User scenarios cover primary flows
- [x] Feature meets measurable outcomes defined in Success Criteria
- [x] No implementation details leak into specification

## Notes

- Items marked incomplete require spec updates before `/speckit-clarify` or `/speckit-plan`

### Validation log

**Iteration 1** — three issues found and fixed:

1. *Requirements testable and unambiguous*: the draft's merge requirements said the result should be "the more complete version", which is not verifiable — completeness was being inferred from line count. Rewritten so FR-001/FR-005 require that no substantive guidance from either source is lost without a recorded decision, which can be checked. The Assumptions section now states explicitly that length is a signal, not proof, and each merge is verified.
2. *Success criteria technology-agnostic*: an earlier SC named specific skill filenames and line counts. Restated as outcomes (SC-001, SC-002) that hold regardless of how the catalog is organized. Concrete filenames and measurements were moved to the Context section, where they belong as observed baseline rather than as acceptance thresholds.
3. *Scope clearly bounded*: the draft did not pin what must not change. Added FR-021 through FR-024 covering CLI behavior, the rejected donor workflow/agents/script, the surviving spec-driven workflow, and the explicit decision not to rewrite the donor's history.

**Iteration 2** — one issue found and fixed:

4. *Edge cases identified*: the draft did not account for the donor's `object-storage` skill containing AWS's documented placeholder key (`AKIAIOSFODNN7EXAMPLE`), which a secret scanner will flag during the import. Added an edge case requiring genuine secrets to be distinguished from documentation placeholders, so the import does not either leak a real key or mangle example text.

**Iteration 3** — all items pass.

### Counting correction carried into this spec

An earlier verbal summary given to the maintainer stated "13 skills exist only in framework_ia". That count came from matching directory names and was wrong: it treated `supabase`, `postgresql` and `playwright` as new when each has a counterpart in the catalog, and it included `beads`, which is discarded.

The corrected classification, used throughout this spec, is **9 new / 7 to merge / 2 discarded** across the donor's 18 skills. In particular the donor splits the datastore topic into two skills (`supabase`, `postgresql`) that both collapse onto the single existing `supabase-postgres-best-practices`, which is why FR-002 exists.

### Judgment calls made without asking

- **Vetting is required but its outcome is not pre-decided.** The maintainer asked for each candidate to be evaluated against their real stacks. The spec mandates a recorded decision per candidate (FR-006..FR-009) rather than naming which nine to accept — that assessment belongs to `/speckit-plan`, where the skill content can actually be read.
- **The credential is treated as requiring rotation** even though the maintainer chose not to rewrite history. FR-020 requires the retirement record to say so. Removing a key from current files does not invalidate it, and recording otherwise would leave a false sense of resolution.
- **Retirement means marked retired, not deleted.** Stated in Assumptions; deleting the donor would destroy the record of what was deliberately left behind.
