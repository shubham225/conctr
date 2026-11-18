# Conctr UI instructions

Read the root instructions, [brand kit](../../design/docs/AI-INSTRUCTIONS.md),
[theming](../../docs/development/theming.md) and [UPSTREAM.md](UPSTREAM.md).

- `src/rust-ui/` is an immutable MIT upstream snapshot. Never edit or format it.
- `src/ui/` is the app-facing facade. Apply customization here or in shared theme tokens.
- `src/composed/` combines UI controls. Keep services and business rules in the app/platform.
- `assets/colors/tokens.css` preserves kit values with provider-scoped selectors; fonts, logos and icons copy `design/`.
- Tailwind 4.1 tooling and the shipped CSS belong here. Consumers mount ThemeProvider.
- Run vendor integrity checks, crate/workspace checks and browser verification after changes.
- Update the source pin, license and hash manifest together when refreshing upstream.

- Keep component-specific CSS in the owning component folder under `src/ui/`
  or `src/composed/`, beside `mod.rs`. Use the resizable component as the
  CSS module and SSR registration example. Existing upstream compatibility skins
  use stable DOM selectors and are imported by the library Tailwind input.
  Reserve `assets/` for shared theme, fonts, artwork and compiler inputs/output.
  Gallery-only layout and demo styles belong to the gallery.
- Prefer Rust signals, Dioxus events and `MountedData` for interactions and
  measurements. Isolate browser bridges when required; do not add JavaScript for
  behavior already supported by Rust APIs. Resizable controls are Rust-owned.
- Mapping/pipeline workspace chrome belongs here; product drafts, routes, sample
  configuration and history belong in `conctr-app-dioxus`. Workflow state is layout
  and interaction data, not an executable flow definition.
