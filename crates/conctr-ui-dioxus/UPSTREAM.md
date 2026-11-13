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

Shared appearance belongs in `assets/styles/main.css`, using semantic `--cn-*`
properties. Component CSS modules keep structural and primitive-specific rules;
do not copy the shared recipes or global focus/motion rules back from upstream.
The legacy `REGISTRY_CSS` asset retains numbered upstream palette aliases for
existing consumers; new styles use semantic names directly.
See the [theming guide](../../docs/development/theming.md).

## CSS module moves

Keep the first `CSS module source` comment in each `style.css` synchronized with
its crate-relative source path. Manganis scopes class names by source path,
while its processed asset cache can reuse an asset with identical contents after
a move. Updating the comment changes the source content/cache identity so the
served selectors agree with the generated Rust class names. A unit test guards
this invariant. Validate a running showcase with
`python3 apps/conctr-design-system/scripts/check-component-styles.py http://127.0.0.1:8080`;
this checks served CSS against rendered component classes, not just file presence.
