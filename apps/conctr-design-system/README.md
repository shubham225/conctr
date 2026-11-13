# Conctr design system gallery

A runnable Dioxus application for inspecting the reusable components from
`conctr-ui-dioxus`. It replaces the former `dioxus-test` application.

From the workspace root:

```sh
dx serve --package conctr-design-system --platform web
```

The gallery provides:

- A searchable catalog of 57 live examples, covering every Dioxus registry family
  in the shared crate and Conctr's kit-specific components.
- Direct URLs under `/components/<name>`, including `/components/toggle`,
  `/components/switch`, `/components/connection-flow` and `/components/dialog`.
- Foundation pages for typography, colors, spacing, radii, shadows, logos and icons.
- Light/dark theme switching, keyboard navigation, a mobile catalog menu and
  selectable Rust examples with a copy button.
- A bundled copy of the original HTML kit, linked in the header for comparison.

`src/catalog.rs` drives both navigation and previews. Each live example lives in
its own `src/examples/<component>.rs`; the exact file is displayed as its usage
example. Examples import the shared crate. Component styling and behavior fixes
belong in `crates/conctr-ui-dioxus`, rather than gallery-specific overrides.
Gallery layout alone lives in `assets/gallery.css`.

The catalog's preview functions return component nodes so each example owns its
Dioxus hook scope when navigating between pages. Keep this boundary when adding
examples.

Connection names, values, forms, deletion actions and toast notifications are
local illustrative examples; the gallery connects to no external systems.
The calendar uses a fixed sample date for deterministic SSR and hydration.

To refresh the bundled reference after editing `design/`:

```sh
sh apps/conctr-design-system/scripts/sync-reference.sh
```

The reference copy preserves relative font/image URLs. Tests check that its HTML
and CSS, and the shared crate's canonical CSS/tokens, match `design/`.

```sh
cargo test -p conctr-design-system -p conctr-ui-dioxus
cargo clippy -p conctr-design-system -p conctr-ui-dioxus --all-targets -- -D warnings
```

The gallery is tracked source; it is no longer covered by the old test-app ignore
rule. The shared UI crate remains the library that applications consume.
