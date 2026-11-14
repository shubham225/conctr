# Conctr design system gallery

A runnable Dioxus application for inspecting the reusable components from
`conctr-ui-dioxus`. It replaces the former `dioxus-test` application.

From the workspace root:

```sh
dx serve --package conctr-design-system --platform web
```

The gallery provides:

- A searchable catalog of live examples, covering every Dioxus registry family
  in the shared crate and Conctr's kit-specific components, organized into
  Primitives, Core categories and Composed components.
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
and CSS match `design/`, and that library token values preserve the canonical kit.

```sh
cargo test -p conctr-design-system -p conctr-ui-dioxus
cargo clippy -p conctr-design-system -p conctr-ui-dioxus --all-targets -- -D warnings
```

The gallery is tracked source; it is no longer covered by the old test-app ignore
rule. The shared UI crate remains the library that applications consume.

Layout examples include Box, Flex, Grid, Stack and Inline. Stats, connection
flows, empty states and the composed Modal are grouped under Composed. Future
application patterns (data tables, flow editors, dashboards) belong alongside
these compositions, using the same core controls.

The branding foundations expose all supplied protocol and authentication icons
and a curated set of Lucide interface actions. The kit has no branded general
interface icon set, nor approved protocol shapes beyond the existing six.

The application name remains **Conctr Design System**: it documents the tokens,
components and usage rules. `design/` is the accompanying design kit of assets
and reference examples. Shared source is organized under
`components/primitives`, `components/ui`, and `components/composed`; the gallery's
import snippets and source guidance point to these canonical modules. See the
[presentation architecture](../../docs/architecture/dioxus-presentation.md).

`ThemeProvider` loads the UI library's bundled Tailwind 4.1 utilities and theme.
This app has no Tailwind dependency, input, config or build step. Component-specific
CSS modules stay in the UI library and load with their components.
See the library's [usage guide](../../crates/conctr-ui-dioxus/README.md#usage)
and [SSR workaround](../../docs/development/theming.md#ssr-css-module-workaround).
To catch styles that no longer match rendered markup, run the
served-styles regression check against the fullstack gallery:

```sh
python3 apps/conctr-design-system/scripts/check-component-styles.py http://127.0.0.1:8080
```

This fetches every catalog page and its linked CSS and checks that each rendered
component class has a matching selector. It catches asset-cache regressions that
compilation and source-path checks cannot detect.
