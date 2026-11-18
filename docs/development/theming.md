# Tailwind and theming

The fresh `conctr-ui-dioxus` owns Tailwind 4.1.18 tooling, input and shipped CSS.
Apps use `ui::<family>` and mount `ui::theme::ThemeProvider`; no app Node setup.
The facade uses shared utilities; specialized compositions also use adjacent
CSS modules. See [Rust UI adoption](rust-ui-adoption.md).

| Library file | Ownership |
| --- | --- |
| `assets/colors/tokens.css` | Palette/scales from `design/`, with provider-scoped selectors |
| `assets/fonts`, `logo`, `icons` | Supplied brand assets and license notices |
| `assets/styles/main.css` | Semantic role bridge, typography, focus and reduced motion |
| `assets/styles/theme.css` | Tailwind compiler mappings |
| `assets/styles/tailwind.input.css` | Library-only scanning and explicit supported shell utilities |
| `assets/styles/tailwind.css` | Shipped compiled stylesheet |
| `src/rust-ui/` | Immutable MIT upstream snapshot |
| `src/ui/` | App-facing facade and theme wrappers |
| `src/composed/` | Reusable Conctr combinations |

```rust,ignore
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;
#[component]
fn App() -> Element {
    rsx! { ThemeProvider { theme: Theme::Dark, Button { "Test connection" } } }
}
```

Providers scope explicit light/dark palettes. Use application-controlled
`BrandThemeToggle`; the raw upstream ThemeToggle has independent global HTML-class
and storage behavior. Explicit initial theme state preserves SSR markup agreement.
Use full Tailwind literals; caller classes merge after wrapper defaults with
`tw_merge`. New customizations belong in wrappers or shared theme files, never
upstream. Specialized CSS lives beside its Conctr wrapper.

For library development run `npm ci` and `npm run build:css`. Cargo also refreshes
the bundle when the local compiler is installed; consumers use committed CSS
without Node. Run `python3 scripts/check-upstream.py` from the library directory.
Serve the gallery with `dx serve --package conctr-design-system --platform web` and
verify actual computed tokens, controls, keyboard interactions and workflow canvas.

## Component CSS and SSR

Mapping, resizable and pipeline workspace folders own their component-specific
`style.css`. Use `#[css_module]`, `asset!(..., AssetOptions::css_module())` and
explicit `document::Stylesheet` registration. Do not use a process-global registry:
each SSR document needs its own stylesheet links. The mapper SSR test covers both
mapping and resizable styles. Keep generic colors and interactions in shared tokens.

When CSS-module class mappings and served assets become inconsistent during
`dx serve` hot reload, restart the server and reload before judging the layout.
