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

Every report follows the **3-section standard** defined in the `notion-reporting-standard` skill:

1. **What we accomplished** — Business value delivered (not technical tasks)
2. **Project Health** — Traffic light system (🟢🟡🔴)
3. **Next Steps** — What the stakeholder will see next

## Notes

- Reports are written in **non-technical language** for stakeholders
- Financial data is sourced from the Finanzas Agent (`knowledge-f1n`)
- Blockers that require stakeholder action are highlighted with 🚨
- See `notion-reporting-standard` skill for the full reporting standard
