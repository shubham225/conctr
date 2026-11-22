# Conctr Design Kit

Fresh gallery over Conctr's Rust UI facade, with brand foundations, light/dark
switching, 78 primary component demos and eight interactive workflow examples.
The previous gallery was archived before deletion; see
[backups](../../backups/pre-rust-ui-20261115T112238Z/README.md).
`assets/reference/` is a snapshot of the canonical `design/` kit; it is not a
second editable brand source. Refresh it from `design/` after brand updates.

```sh
dx serve --package conctr-design-kit --platform web
```

Open `/icons` from the sidebar to browse the 77 shared icons and switch between color and monochrome editions.

The library supplies fonts, assets and compiled Tailwind CSS. Component routes
are `/components/<slug>`; workflow routes are `/components/workflow-1` through
`/components/workflow-8`. Usage blocks show formatted Rust with public `conctr_ui_dioxus` imports.
The catalog adds up to two alternate examples where the registry provides them.
Gallery-specific examples live in `src/demo/`. Reusable composed examples live in
`crates/conctr-ui-dioxus/src/composed/demos/` and show public function usage.
Usage code is collapsed by default.
The component explorer is a simple scrollable list. The separate Sidebar component
demo uses a main-content trigger, a 56 px desktop icon rail and a mobile drawer
below 768 px. The Cargo package is `conctr-design-kit`.

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
restart `dx serve --package conctr-design-kit` and reload the printed URL.

Dropdown examples, including both date pickers and multiselect, reserve at least
480 px of preview height and allow popups to extend beyond the preview border.
Other previews retain scrolling for wide content such as tables.

Targeted browser regression checks (Node 22+, Chrome with debugging port 19222):

```sh
node apps/conctr-design-kit/scripts/check-showcase-browser.mjs http://127.0.0.1:18080
```

Use the actual running server URL. These checks exercise date selection, loading
animation and reduced motion, OTP entry, phone borders, feedback notifications,
usage code, variants, navigation collapse and data surfaces.

The primary Table and Data Table examples use connection records with integrated
headers, status badges and footers. Data Table supports local filtering, sorting
and visible-row selection. Data Grid shows editable field mappings with reset;
the original advanced virtual-grid example remains an additional variant.
Toast controls and static message previews occupy separate sections and share the
same `ToastMessage` surface as live notifications, including inline dismissal.

```sh
node apps/conctr-design-kit/scripts/check-data-browser.mjs <server-url> <chrome-debug-url>
```

Composed examples include semantic tags/tag input, date, time and combined local
date/time selection. Command examples include keyboard hints and grouped results.
Button examples include ghost and icon variants; Button Action has its own page.
Drawer examples support dismissal and focus restoration. Repeated toast triggers
append independently dismissible notifications to one shared stack.

```sh
node apps/conctr-design-kit/scripts/check-component-fixes-browser.mjs <server-url> <chrome-debug-url>
```

The focused suite exercises composed usage, tags, pickers, buttons, command styling,
sidebar navigation, drawers, stacked notifications and visual field mapping.

The `field-mapping` composition demonstrates a controlled source → optional
transform → target editor, with editable fields, paths/types, visible connectors
and add/remove controls. Request and response examples share the public
`FieldMappingEditor` API. The board scrolls within narrow screens. Mapping drafts
and transform IDs are presentation data; callers own validation and execution.

`visual-field-mapping` provides the interactive schema mapper and a read-only
variant. Its caller-side import demo accepts pasted JSON samples or a normalized
XML fixture. The existing `field-mapping` page retains the classic editor.

```sh
node apps/conctr-design-kit/scripts/check-visual-mapping-browser.mjs <server-url> <chrome-debug-url>
```
