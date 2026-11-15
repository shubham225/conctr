# Conctr design system showcase

Fresh gallery over Conctr's Rust UI facade, with brand foundations, light/dark
switching, 78 primary component demos and eight interactive workflow examples.
The previous gallery was archived before deletion; see
[backups](../../backups/pre-rust-ui-20261115T112238Z/README.md).
`assets/reference/` is a snapshot of the canonical `design/` kit; it is not a
second editable brand source. Refresh it from `design/` after brand updates.

```sh
dx serve --package conctr-design-system --platform web
```

The library supplies fonts, assets and compiled Tailwind CSS. Component routes
are `/components/<slug>`; workflow routes are `/components/workflow-1` through
`/components/workflow-8`. Source snippets are the exact upstream examples; their
`crate::ui` imports resolve to Conctr adapters. Use `conctr_ui_dioxus::ui` in apps.

These are upstream interaction demos, not working Conctr services. AutoForm is
an upstream shell. Workflow canvas state does not define executable flow semantics.
See [assessment and verification](../../docs/development/rust-ui-adoption.md).

Run the command from the repository root. The gallery defaults to the web renderer.
If port 8080 is occupied, append `--port 18081`. Use `dx serve` for the browser
asset pipeline; `cargo run` is not a replacement for bundling the web application.

The local `index.html` keeps startup independent of external fonts. Dioxus 0.7.10's
default development template imports Google Fonts before its scripts; a stalled
request can leave Chrome blank. The custom entry uses the library's bundled
fonts and keeps Dioxus's generated loader and hot reload. After pulling this fix,
restart `dx serve --package conctr-design-system` and reload the printed URL.

Dropdown examples, including both date pickers and multiselect, reserve at least
480 px of preview height and allow popups to extend beyond the preview border.
Other previews retain scrolling for wide content such as tables.
