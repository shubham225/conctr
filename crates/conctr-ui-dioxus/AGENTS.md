# UI library instructions

Applies to this crate in addition to the root [AGENTS.md](../../AGENTS.md).
Read the [theming contract](../../docs/development/theming.md),
[design instructions](../../design/docs/AI-INSTRUCTIONS.md) and
[upstream notes](UPSTREAM.md) before changing styling or primitive adapters.

- `src/components/primitives/` owns layout; `ui/` owns base controls/interactions;
  `composed/` combines them into reusable patterns. Preserve compatibility exports.
  Product pages, fetching, permissions and business rules belong outside this crate.
- `ThemeProvider`/`ConctrStyles` loads fonts, tokens, runtime styles and bundled
  Tailwind utilities. Consuming apps require no Tailwind setup or asset copies.
- Keep complete Tailwind 4.1 class literals in Rust. The crate owns its npm dev
  dependencies and compiler inputs. Run `npm ci` once for library development;
  Cargo rebuilds refresh `assets/styles/tailwind.css`. Commit that bundle for
  consumers without Node. Use `npm run watch:css` for immediate utility-only edits.
- Use adjacent `style.css` for specialized rules in `@layer components`, with
  `#[css_module("/src/components/.../style.css")] struct Styles;` and `Styles::...`
  classes. Explicitly register the matching CSS-module asset per component for
  SSR. Keep source-path markers and intentional `:global` compatibility hooks.
- Palette and scales live in `assets/colors/tokens.css`, shadcn aliases and shared
  interactions in `assets/styles/main.css`, compiler mappings in `theme.css`.
  Preserve standard names and Conctr protocol/font/spacing extensions. Never send
  compiler directives to the browser in local CSS-module assets.
- Avoid empty stylesheets, a central registry and duplicated shared styling.
  Scope brand/density overrides to providers; verify nested and System modes.
  Preserve caller classes, and document explicit atomic utility overrides.
- Preserve primitive keyboard/focus/ARIA contracts, controlled/uncontrolled behavior,
  caller attributes and SSR/hydration consistency. Use current Dioxus 0.7 APIs.
- Update the matching gallery example/source when public behavior changes. Cover
  relevant hover, selection, invalid, disabled, loading and reduced-motion states.
  For sidebar changes, verify divider alignment and icon centering in expanded,
  collapsed and mobile states. For overlays, verify trigger anchoring and content fit.
- Run crate tests and relevant workspace checks for Rust changes. For CSS/layout,
  use the served-selector checker and actual browser checks from the root guidance.
  Compilation alone does not validate styling. Preserve upstream provenance/licenses.
