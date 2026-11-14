# Dioxus component sources

Source: https://github.com/DioxusLabs/components
Revision: `9a758255ea26e2b20c8cecf4c1c946feb9e71da7`
Catalog: all 45 entries returned by `dx components list`.
License: MIT OR Apache-2.0 (included alongside this file).

The CLI bulk-add failed during dependency resolution. Sources were copied from
its local registry cache, excluding examples, and module/CSS paths were adjusted.
Registry-derived implementations are owned, editable source under
`src/components/ui/<family>/`, alongside their CSS. They are not generated at
build time. Their CSS is customized with Conctr tokens and kit recipes. The
former separate registry and facade directories have been consolidated; the old
public module paths remain compatibility re-exports. Layout primitives live in
`components/primitives/` and reusable combinations in `components/composed/`.
Pagination keeps its composable registry implementation in `ui/pagination/parts.rs`
alongside the one-based convenience component.

When updating, diff the upstream revision against these local sources; do not
run `dx components add --force` over customized files. Review primitive API
changes and preserve Conctr CSS, accessibility behavior and local fixes.

## Tailwind customization

Components use Tailwind 4.1.18 utility classes in Rust with shadcn-style variables.
All tooling and the shipped utility bundle belong to this crate; applications
only mount `ThemeProvider` or `ConctrStyles`. Palette/scales come from `tokens.css`.
`main.css` maps semantic roles and shared interactions; `theme.css` supplies
compiler mappings. Adjacent `style.css` files use Manganis CSS modules, with
explicit per-component `document::Stylesheet` registration for repeated SSR.
There are no per-component generated files or central stylesheet registry.
Keep ownership comments accurate and preserve primitive behavior and licenses.

The explicit CSS-module registrations work around the resolved Dioxus/Manganis
0.7.10 SSR bug described in [Dioxus #5707](https://github.com/DioxusLabs/dioxus/pull/5707).
Preserve them when refreshing upstream sources. Removal criteria and regression
checks are in the [SSR guide](../../docs/development/theming.md#ssr-css-module-workaround).

## Locally implemented controls

`file_uploader`, `resizable`, `kbd` and `command` are local Rust implementations.
The [resizable](https://ui.shadcn.com/docs/components/base/resizable),
[kbd](https://ui.shadcn.com/docs/components/base/kbd) and
[command](https://ui.shadcn.com/docs/components/base/command) compositions informed
the public surface; no React runtime or JavaScript component package is required.
`KeyboardKey` aliases `Kbd`; `StatusAlert` forwards to the canonical `Alert` API.
