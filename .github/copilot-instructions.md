# Conctr repository instructions

Before changes, read the canonical [AGENTS.md](../AGENTS.md), then
[shared AI context](../docs/ai/AI-INSTRUCTIONS.md) and only task-relevant contracts.
Read scoped `AGENTS.md` files in each target's ancestor directories before editing.
Keep policy in those sources rather than expanding this compatibility entry.

- Keep domain/use-case contracts independent of Dioxus and concrete adapters.
- Use the workspace Dioxus 0.7.1 requirement and current signal/hook APIs.
- Reuse `conctr-ui-dioxus`; put shared CSS recipes and semantic theme properties in
  its `assets/styles/main.css`. Follow the [theming guide](../docs/development/theming.md).
- Verify source before claiming capability. Most backend packages and the shared
  app are scaffolds; proposed ADRs do not require creating every planned crate.
- Preserve user changes, keep edits focused, run relevant checks and report their
  actual results. Check documentation with `python3 docs/development/scripts/check_docs.py`.
