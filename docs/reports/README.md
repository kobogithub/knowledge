# Stakeholder Reports

This directory contains automatically generated project reports for stakeholders.

Reports are created by the **Biz Agent** (`knowledge-biz`) as a Markdown fallback when Notion is not available, or as a permanent record alongside the Notion dashboard.

## Report Types

| Type | Filename Pattern | Frequency |
|------|-----------------|-----------|
| Session Report | `YYYY-MM-DD-session-report.md` | After each work session |
| Weekly Report | `YYYY-MM-DD-weekly-report.md` | Weekly summary |
| Sprint Report | `YYYY-MM-DD-sprint-report.md` | End of sprint/milestone |

## Report Structure

Every report follows this **3-section standard**. It lives here, not in a skill — the
`notion-reporting-standard` skill was removed when the catalog was curated
([ADR-007](../adr/007-personal-stacks-and-curated-catalog.md)).

1. **What we accomplished** — 3 to 5 milestones in business language, never technical
   tasks. Translate: "payments endpoint with tests" → "we can charge customers now".
2. **Project Health** — traffic light: 🟢 on plan · 🟡 at risk, with the cause · 🔴 blocked,
   with what is needed to unblock it.
3. **Next Steps** — what the stakeholder will see in the next stretch.

Rules: one page maximum, always open with achievements rather than problems, and source
the numbers from `specs/NNN-*/tasks.md` checkboxes and the latest Finanzas Agent report
instead of estimating them.

## Notes

- Reports are written in **non-technical language** for stakeholders
- Financial data is sourced from the Finanzas Agent (`knowledge-f1n`)
- Blockers that require stakeholder action are highlighted with 🚨
- The Biz Agent restates this same standard in `agents/biz/AGENTS.md`; keep both in sync
