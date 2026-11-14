# Tailwind and theming

`conctr-ui-dioxus` owns **Tailwind CSS 4.1.18**, its development dependencies,
compiler inputs and bundled utilities. Consuming applications only import
components and mount `ThemeProvider` or `ConctrStyles`. They require no Tailwind
configuration, source scanning, copied assets, Node installation or CSS build step.
The gallery follows this same consumer contract.

## Files and ownership

| File in `crates/conctr-ui-dioxus` | Responsibility |
| --- | --- |
| `assets/colors/tokens.css` | Canonical Conctr palette/scales, scoped to nested providers without changing kit values |
| `assets/styles/main.css` | shadcn semantic aliases, typography, focus, shared field states and reduced motion |
| `assets/styles/theme.css` | Tailwind `@theme inline` mappings and dark variant |
| `assets/styles/tailwind.input.css` | Compiler input scanning library and workspace consumer sources |
| `assets/styles/tailwind.css` | Shipped Tailwind Preflight and utilities; no component skins |
| `src/components/<layer>/<family>/component.rs` | Complete Tailwind class literals, typed props and interactions |
| Adjacent `style.css`, when needed | Component CSS module, compiled by Manganis, loaded only by its components |
| `package.json`, `package-lock.json`, `build.rs` | Library development tooling; consumers use shipped CSS |

The component layers remain `primitives → ui → composed`. The UI crate imports no
application crate or app asset. Its compiler input also scans workspace apps and `conctr-app-dioxus`; tooling stays in this crate.
Tailwind owns the reset and utilities; `main.css` contains only kit rules and
aliases. Importing a Button does not load Calendar CSS.

## Consumer usage

```rust,ignore
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::Light,
            Button { "Test connection" }
        }
    }
}
```

`ThemeProvider` includes `ConctrStyles`, which loads fonts, tokens, runtime styles
and the library's precompiled utilities. If managing theme scopes yourself,
mount `ConctrStyles {}` once. Each styled component explicitly registers its
CSS-module asset. Dioxus deduplicates stylesheet links in each document.

`MAIN_CSS`, `CONCTR_UI_CSS` and `PROVIDER_CSS` are compatibility exports for runtime
styles alone. Prefer `ConctrStyles` for the complete foundation. The obsolete
`REGISTRY_CSS` export and `registry.css` numbered palette aliases have been removed.
Update old imports to `ConctrStyles` and use the semantic variables below.

## Theme variables

The names follow [shadcn/ui](https://ui.shadcn.com/docs/theming), using Conctr's
palette and geometry:

- `--background` / `--foreground`
- `--card` / `--card-foreground`, `--popover` / `--popover-foreground`
- `--primary` / `--primary-foreground`, `--secondary` / `--secondary-foreground`
- `--muted` / `--muted-foreground`, `--accent` / `--accent-foreground`
- `--destructive`, `--destructive-foreground`, `--border`, `--input`, `--ring`, `--radius`
- `--chart-1` through `--chart-5`
- `--sidebar`, `--sidebar-foreground`, `--sidebar-primary`,
  `--sidebar-primary-foreground`, `--sidebar-accent`, `--sidebar-accent-foreground`,
  `--sidebar-border`, `--sidebar-ring`

Tailwind maps these to utilities such as `bg-primary`, `text-foreground` and
`border-input`. Conctr extensions cover readable status/protocol colors,
`--primary-hover`, elevation, fonts, spacing and density. All palette literals
belong in `tokens.css`; semantic aliases reference them in one direction.
Protocol shapes and readable `-text` colors retain their kit meanings.

`Theme::Light` and `Theme::Dark` set `data-theme`. System mode follows OS
preferences through CSS, preserving consistent initial SSR/client markup.
Each nested provider resets its palette and geometry. `.light`/`.dark` scopes
are supported too. Variables follow DOM inheritance, including overlays.

Customize with typed props, CSS variables or inline styles. For example:

```rust,ignore
ThemeProvider {
    theme: Theme::Light,
    style: "--primary: #0e7490; --primary-foreground: white; --primary-hover: #155e75; --ring: #0e7490; --radius: 12px; --control-height: 32px;",
    Button { "Test connection" }
}
```

Overrides can also live in consumer CSS on the provider. Supply corresponding
dark/OS rules for alternate brands. The default radius derives Conctr's
4/6/10/14/20px scale. Density uses `--control-height`, `--control-padding`,
`--field-padding`, `--card-padding` and dialog/choice extensions.

Caller classes and attributes are preserved. Utilities in library and workspace app sources are compiled by the UI crate.
Standalone external consumers receive the shipped bundle; new utilities outside
these scanned sources need additional compilation. See [workspace setup](setup.md#zed-tailwind-suggestions-and-colors-in-dioxus). Class-string order does not resolve conflicting atomic
utilities: prefer props, variables or inline styles. There is no runtime class
merging dependency.

## Library development

From `crates/conctr-ui-dioxus`, install tools once with `npm ci`. Cargo rebuilds
then run the pinned local compiler automatically through `build.rs`. Without
these development dependencies, Cargo simply uses the shipped stylesheet, without
network access or requiring Node. This keeps path and packaged consumers usable.

For changes that DX handles through RSX hot reload without rebuilding Rust,
`npm run watch:css` updates utility CSS immediately. Alternatively rebuild Rust.
Before distributing source changes, run `npm run check:css` and commit the updated
shared bundle. `npm run build:css` is available for explicit regeneration.
There are no `style.generated.css` files. Component CSS-module edits use the normal
Dioxus/Manganis asset pipeline.

Tailwind always needs compilation before browser use. Dioxus's
[automatic Tailwind integration](https://dioxuslabs.com/learn/0.7/guides/utilities/tailwind/)
builds an app's entry point. This library intentionally ships its own output to
avoid requiring that setup in every consumer.

## Adding components

Write complete Tailwind utility literals in Rust. Keep specialized selectors,
geometry and animations in the owning component's `style.css`:

```css
/* CSS module source: /src/components/ui/example/style.css */
@layer components {
  .panel[data-state="open"] { transform: translateY(0); }
}
```

```rust,ignore
#[css_module("/src/components/ui/example/style.css")]
struct Styles;

const STYLES: Asset = asset!(
    "/src/components/ui/example/style.css",
    manganis::AssetOptions::css_module()
);

rsx! {
    document::Stylesheet { href: STYLES }
    div { class: "{Styles::panel.inner} flex items-center gap-3", {children} }
}
```

Keep this explicit registration for the [SSR workaround](#ssr-css-module-workaround).
Source-path comments keep asset-cache identity correct after moves.
Stable `cn-*` hooks accompany scoped selectors for compatibility and inspection;
`:global(...)` is reserved for intentional compatibility modifiers or references
to another component. Utilities stay global in the shared bundle. Reuse shared
controls and recipes instead of copying their CSS. Utility-only components need
no empty stylesheet.

`@apply` and `@reference` require Tailwind preprocessing, so do not put them in
CSS modules loaded directly by Manganis. Use utility classes in Rust for Tailwind
features and ordinary CSS for specialized module rules. Compiler-only `@theme`
and package imports stay in the library compiler input/mappings.

## SSR CSS-module workaround

The workspace requires Dioxus `0.7.1` and currently resolves Dioxus/Manganis
**0.7.10**. In that version, `#[css_module]` normally inserts its stylesheet when
a generated class constant is formatted or dereferenced. Its process-wide
`OnceLock` allows this only once, while SSR creates a new document per request.
The first response contains the stylesheet link; later responses can be unstyled
until browser hydration. The upstream fix is
[Dioxus #5707](https://github.com/DioxusLabs/dioxus/pull/5707).

The library keeps stylesheet loading local to each component family:

1. Declare `#[css_module(...)] struct Styles;` and one matching `STYLES` asset,
   using `manganis::AssetOptions::css_module()` for both paths.
2. Render `document::Stylesheet { href: STYLES }` in every independently usable
   component that needs that stylesheet. Dioxus deduplicates repeated instances
   within each document. Visual dependencies register their own required assets.
3. Use `Styles::class_name.inner` in class expressions. Reading the scoped string
   directly bypasses the macro's process-wide automatic link and avoids a second
   link on the first render. Explicit registration handles all renderers.

The asset constant is declared once per family, including families with several
public parts such as Accordion. Utility-only components need neither the local
asset nor a CSS-module declaration. Consumers mount the usual style provider;
they do not maintain a stylesheet registry or add per-component imports.

[`tests/styles.rs`](../../crates/conctr-ui-dioxus/tests/styles.rs) renders separate
documents in the same process and checks that both receive the used stylesheet
exactly once. It also checks that unused Calendar/Spinner styles stay absent until
needed and that loading buttons and protocol badges include their dependencies.
Run this regression check with:

```sh
cargo test -p conctr-ui-dioxus --test styles
```

Remove the workaround only after the **resolved** Dioxus/Manganis version contains
the upstream fix and repeated SSR requests work before hydration. At that point,
switch `.inner` references back to automatic CSS-module class usage and remove
the explicit asset declarations/stylesheet nodes together. Ensure each styled
component actually uses a generated class constant, including modules with only
global selectors, so automatic registration runs. Rerun the per-document tests
and served-style checker; verify that component selection and link deduplication
still hold. A merged upstream PR alone does not change the lockfile.

## Verification

Run workspace checks and `npm run check:css` in the UI crate. Serve the gallery:

```sh
dx serve --package conctr-design-system --platform web
python3 apps/conctr-design-system/scripts/check-component-styles.py http://127.0.0.1:8080
```

Use the actual server URL. Check both themes, nested/System scopes, customization,
focus/error/disabled states, keyboard interactions, overlays, reduced motion and
responsive layouts in a browser. Verify separate SSR documents include only used
component sheets and required dependencies. Default workspace checks do not prove
native SDK builds or external integrations.
