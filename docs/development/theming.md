# Tailwind and theming

`conctr-ui-dioxus` owns Tailwind 4.1.18 tooling, compiler inputs, consumer scan
paths and the shipped CSS. Apps mount `ThemeProvider` or `ConctrStyles` and use
the shipped bundle without Node. Keep Tailwind configuration and generated CSS
in the library. Write shared utility classes directly in component Rust. See
[Rust UI adoption](rust-ui-adoption.md).

| Library file | Ownership |
| --- | --- |
| `assets/colors/tokens.css` | Palette/scales from `design/`, with provider-scoped selectors |
| `assets/fonts`, `logo`, `icons` | Supplied brand assets and license notices |
| `assets/styles/main.css` | Semantic role bridge, typography, focus and reduced motion |
| `assets/styles/theme.css` | Tailwind compiler mappings |
| `assets/styles/tailwind.input.css` | Library-owned consumer scan paths, compatibility skin imports and shell utilities |
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
Serve the gallery with `dx serve --package conctr-design-kit --platform web` and
verify actual computed tokens, controls, keyboard interactions and workflow canvas.

## Component CSS and SSR

Put component-specific selectors in `src/ui/<family>/style.css` or
`src/composed/<component>/style.css` beside the owning Rust module. Use
`#[css_module]` for scoped classes and register the asset with
`document::Stylesheet` so each SSR document receives the link. The
[resizable component](../../crates/conctr-ui-dioxus/src/composed/resizable/component.rs)
shows both declarations. Register styles in each component and keep ownership
comments accurate. Use the shared compiler for generated CSS and keep stylesheet
registration local to the component.

Product screens follow the same rule in their owning app crate. For example,
the [sign-in component stylesheet](../../crates/conctr-app-dioxus/src/sign_in/style.css)
owns the panel layout, brand artwork positioning and mobile breakpoint. Reuse
already-shipped utilities for common layout. Put one-off dimensions, gradients
and responsive treatments in the component's scoped `style.css`. Register it
with `document::Stylesheet` and keep it out of the Tailwind compiler imports.

For a component-only styling change, rebuild with
`npm --prefix crates/conctr-ui-dioxus run build:css` and inspect
`git diff -- crates/conctr-ui-dioxus/assets/styles/tailwind.css`. If the rebuild
adds a utility solely for that component, move the declaration to its stylesheet
and rebuild. Generate the bundle through the compiler.

Existing facade skins that target upstream DOM classes or data attributes live
beside their wrappers and are imported by `assets/styles/tailwind.input.css`.
The generated bundle ships those rules to consumers. Keep gallery layout and
demo-only selectors in the showcase stylesheet.

When CSS-module class mappings and served assets become inconsistent during
`dx serve` hot reload, restart the server and reload before judging the layout.
