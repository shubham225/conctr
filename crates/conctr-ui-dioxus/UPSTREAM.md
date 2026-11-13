# Dioxus component sources

Source: https://github.com/DioxusLabs/components
Revision: `9a758255ea26e2b20c8cecf4c1c946feb9e71da7`
Catalog: all 45 entries returned by `dx components list`.
License: MIT OR Apache-2.0 (included alongside this file).

The CLI bulk-add failed during dependency resolution. Sources were copied from
its local registry cache, excluding examples, and module/CSS paths were adjusted.
`src/components/dioxus/` is owned, editable source, not generated at build time.
Its CSS is customized with Conctr tokens and kit recipes. Registry components
are re-exported through individual public component modules. Kit-only components
live directly under `src/components/`.

When updating, diff the upstream revision against these local sources; do not
run `dx components add --force` over customized files. Review primitive API
changes and preserve Conctr CSS, accessibility behavior and local fixes.
