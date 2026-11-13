# UI library instructions

Applies to this crate in addition to the root [AGENTS.md](../../AGENTS.md).
Read the [theming contract](../../docs/development/theming.md),
[design instructions](../../design/docs/AI-INSTRUCTIONS.md) and
[upstream notes](UPSTREAM.md) before changing styling or primitive adapters.

- `src/components/primitives/` owns layout; `ui/` owns base controls/interactions;
  `composed/` combines them into reusable patterns. Preserve compatibility exports.
  Product pages, fetching, permissions and business rules belong outside this crate.
- Use `ThemeProvider`/`ConctrStyles` to load the complete style bundle. `MAIN_CSS`
  alone does not load fonts, palette or component modules. `CONCTR_UI_CSS` remains
  a compatibility alias; the kit's separate reference stylesheet keeps its name.
- Put shared recipes and semantic theme defaults in `assets/styles/main.css`.
  Keep placement, responsive structure and primitive-specific states in adjacent
  CSS modules. Reuse existing `cn-*` recipes and meaningful `--cn-*` properties.
  `registry.css` retains legacy aliases; new components must not use those aliases.
- Scope brand/density role overrides to the provider itself. Verify nested providers,
  both modes and System mode; custom properties follow DOM inheritance.
- Keep CSS-module source-path comments synchronized after moves. Register new styled
  modules in `src/assets/registry_styles.rs`; do not add empty stylesheets for
  components that need no styling. Check served selectors after structural changes.
- Preserve primitive keyboard/focus/ARIA contracts, controlled/uncontrolled behavior,
  caller attributes and SSR/hydration consistency. Use current Dioxus 0.7 APIs.
- Update the matching gallery example/source when public behavior changes. Cover
  relevant hover, selection, invalid, disabled, loading and reduced-motion states.
  For sidebar changes, verify divider alignment and icon centering in expanded,
  collapsed and mobile states. For overlays, verify trigger anchoring and content fit.
- Run crate tests and relevant workspace checks for Rust changes. For CSS/layout,
  use the served-selector checker and actual browser checks from the root guidance.
  Compilation alone does not validate styling. Preserve upstream provenance/licenses.
