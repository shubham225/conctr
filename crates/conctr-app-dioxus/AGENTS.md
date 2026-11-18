# Shared product app instructions

Read the [root instructions](../../AGENTS.md),
[presentation ownership](../../docs/architecture/dioxus-presentation.md) and
[UI instructions](../conctr-ui-dioxus/AGENTS.md) before changing product UI.

- Own product routes, pages, sample configuration and presentation drafts here.
  Launchers supply renderer/OS behavior; do not duplicate pages in `apps/`.
- Reuse `conctr-ui-dioxus::ui` controls and `composed` workspace components.
  Put reusable component geometry/CSS in the UI library, not an app CSS toolchain.
- `/mappings` owns separate request/response drafts and reversible draft history.
  `/pipelines` uses workflow state for canvas interaction and local step details.
  Neither page currently persists, executes transforms, sends requests or deploys.
- Keep future executable flow contracts independent of Dioxus and layout state.
  Label local checks/previews accurately; never turn a simulated run into a claim
  of backend support. Credentials are symbolic references, never fixture secrets.
- Preserve user-supplied design references. Confirm consumers before removing
  components, routes, public exports or browser regression scripts.
- Run focused Rust checks for app/UI changes and the relevant browser scripts
  from [README.md](README.md). These scripts share one Chrome tab; run sequentially.
