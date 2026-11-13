# Theming the Dioxus component library

Status: **Implemented**. The default appearance follows the [design kit](../../design/README.md).
The Rust library owns the reusable CSS recipes and component adaptations below.

## Files and ownership

| File | Responsibility |
| --- | --- |
| [colors/tokens.css](../../crates/conctr-ui-dioxus/assets/colors/tokens.css) | Canonical palette, light/dark modes, spacing, type scale, radii, elevation and motion primitives; synchronized with the kit |
| [styles/main.css](../../crates/conctr-ui-dioxus/assets/styles/main.css) | Main stylesheet: semantic theme properties, shared recipes, typography, native helpers, global focus and reduced-motion behavior |
| [styles/provider.css](../../crates/conctr-ui-dioxus/assets/styles/provider.css) | Scoped light-mode palette, nested-provider branding and library adapters |
| [component CSS modules](../../crates/conctr-ui-dioxus/src/components/ui/) | Structure, anchoring, responsive layout and interaction states specific to each primitive |
| [styles/registry.css](../../crates/conctr-ui-dioxus/assets/styles/registry.css) | Compatibility aliases for older consumer CSS; current components use semantic properties directly |
| Application `theme.css` | Brand or density overrides, loaded after the library styles |

Use `ThemeProvider` at the application root. It loads bundled fonts, palette,
`main.css`, provider adapters, legacy aliases and component CSS modules through `ConctrStyles`.
`ConctrStyles` can also be mounted directly when the application owns its theme
surface. The `MAIN_CSS` asset exposes the main file for manual loading; it does
not include fonts, palette or CSS modules by itself. `CONCTR_UI_CSS` remains a
Rust compatibility alias for `MAIN_CSS`. The old library filename is removed;
the reference kit's `design/ui-kit/conctr-ui.css` keeps its original name.

## Property hierarchy

Values flow from foundation tokens into semantic roles, then shared recipes:

```text
--cn-primary → --cn-button-primary-background → .cn-btn → Button
--cn-radius-md → --cn-control-radius → buttons, toggles and fields
--cn-font-ui + type scale → --cn-font-label → navigation and control labels
```

Use `--cn-<family>-<purpose>` names. For example, `field-error-ring` explains
the intent; numbered aliases such as `primary-color-7` do not. New shared roles
belong in the first section of `main.css`. Reuse a foundation token directly
where a more specific role would add no useful customization boundary.

| Family | Main properties | Consumers |
| --- | --- | --- |
| Typography | `--cn-font-body`, `small`, `label`, `control`, `field`, `badge`, `title` (each with the `--cn-font-` prefix) | Body copy, navigation, controls, field values, badges and dialog headings |
| Control geometry | `--cn-control-height`, `radius`, `border-width`, `padding-inline`, `gap` | Buttons, toggles, fields; explicit small/large sizes retain their size tokens |
| Control states | `--cn-control-background`, `foreground`, `border`, `hover-background`, `selected-background`, `selected-foreground`, `selected-border`, `disabled-opacity` | Secondary actions and toggles; disabled states across controls |
| Action variants | `--cn-button-primary-background`, `primary-hover-background`, `primary-foreground`, `ghost-background`, `ghost-hover-background`, `ghost-foreground`, `danger-background`, `danger-foreground`, `danger-hover-filter` | Button variants and alert-dialog actions |
| Fields | `--cn-field-background`, `foreground`, `border`, `hover-border`, `placeholder`, `padding-inline`, `focus-border`, `focus-ring`, `error-border`, `error-ring`, `disabled-background` | Input, Textarea and Select trigger |
| Choices | `--cn-choice-size`, `radius`, `background`, `border`, `checked-background`, `checked-foreground` | Checkbox, RadioGroup items and native radio helper |
| Switch | `--cn-switch-width`, `height`, `thumb-size`, `thumb-inset`, `background`, `checked-background`, `thumb-background` | Native switch helper and Switch primitive |
| Cards | `--cn-card-background`, `border`, `radius`, `padding` | Card and native card helpers |
| Floating surfaces | `--cn-popup-background`, `foreground`, `border`, `radius`, `shadow` | Select list, DropdownMenu, Popover and HoverCard; native menu helper |
| Dialogs | `--cn-dialog-background`, `border`, `radius`, `shadow`, `padding`, `gap`, `width`, `viewport-gap` | Dialog, AlertDialog and the Modal composition |
| Focus/motion | `--cn-focus-outline`, `--cn-focus-outline-offset`, `--cn-motion-control`, `--cn-motion-overlay`, `--cn-overlay-background` | Keyboard focus, shared transitions and modal backdrops |

Unless the table explicitly states another prefix, abbreviated names in each
row use that row's first property's family prefix. `main.css` contains the full
authoritative list and defaults. Font recipes contain a complete CSS `font`
shorthand; ring properties contain a complete `box-shadow`. Motion recipes
contain a duration and timing function. `--cn-textarea-min-height` sets the
textarea's default minimum height.

## Create an application theme

Prefer an application stylesheet over editing library defaults for each app.
Change `main.css` defaults when the shared library's default design is changing.
This example creates an illustrative alternate brand; it is not a new canonical
Conctr palette. Supply matching light/dark values and review their contrast.

```css
/* apps/<app>/assets/theme.css, loaded after ConctrStyles */
.cn-body[data-brand="ocean"][data-theme="light"] {
  --cn-primary: #0e7490;
  --cn-primary-hover: #155e75;
  --cn-primary-soft: #ecfeff;
  --cn-on-primary: #ffffff;
  --cn-focus: #0e7490;
  --cn-focus-ring: 0 0 0 3px rgb(14 116 144 / 28%);
}

.cn-body[data-brand="ocean"][data-theme="dark"] {
  --cn-primary: #67e8f9;
  --cn-primary-hover: #a5f3fc;
  --cn-primary-soft: #164e63;
  --cn-on-primary: #083344;
  --cn-focus: #67e8f9;
  --cn-focus-ring: 0 0 0 3px rgb(103 232 249 / 28%);
}

/* Role overrides can apply to both modes without duplicating colors. */
.cn-body[data-brand="ocean"] {
  --cn-control-radius: var(--cn-radius-sm);
  --cn-dialog-width: 36rem;
  --cn-popup-shadow: var(--cn-shadow-2);
}

.cn-body[data-density="compact"] {
  --cn-control-height: var(--cn-control-sm);
  --cn-control-padding-inline: var(--cn-space-3);
  --cn-field-padding-inline: var(--cn-space-2);
  --cn-card-padding: var(--cn-space-4);
}
```

```rust
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::Light, "data-brand": "ocean", "data-density": "compact",
            document::Stylesheet { href: asset!("/assets/theme.css") }
            Button { "Create connection" }
            Input { aria_label: "Endpoint", placeholder: "https://api.example.test" }
        }
    }
}
```

`Theme::System` follows the OS using the default palette. An alternate brand
using System mode also needs a default palette and a
`prefers-color-scheme: dark` override scoped to
`.cn-body[data-brand="ocean"]:not([data-theme])`. Explicit light/dark providers
must win over that rule. The example intentionally uses an explicit mode.

## Scope, cascade and nested providers

Foundation colors inherit. Semantic aliases are declared on both `:root` and
every `.cn-body` so they resolve against that provider's palette. Put brand,
density and role overrides **on the provider itself**, as above. A role set only
on `html` can be reset by `.cn-body`; a role whose referenced token is changed
only further down the tree has already resolved at the provider. To customize
one control locally, override its role directly on that control instead.

Nested providers recalculate default roles for their own palette. Give a nested
provider its own `data-brand`/`data-density` if it should use the same preset.
Custom brand attributes do not propagate automatically. Use the provider that
contains an overlay's rendered content; inherited properties follow the DOM.
The default design-kit light/dark palettes and protocol colors stay unchanged.

## Extend the library

- Reuse stable shared recipe classes where markup has the same visual role:
  `cn-btn`, `cn-toggle`, `cn-input`, `cn-textarea`, `cn-select`, `cn-check`,
  `cn-switch-track`, `cn-switch-thumb`, `cn-badge`, `cn-card`, `cn-popup`,
  `cn-modal` and `cn-dialog-*`.
- Keep anchoring, dimensions particular to an interaction, keyboard behavior and
  state attributes in the owning component. A select list and a hover card share
  a surface recipe while retaining their own positioning logic.
- Keep global focus and reduced-motion rules in `main.css`. Retain local focus
  rules only when an interaction needs different placement or treatment.
- Do not target generated selectors such as `dx-button-<hash>` in themes. Preserve
  CSS-module source-path markers and register new styled modules in
  `src/assets/registry_styles.rs`; see [upstream notes](../../crates/conctr-ui-dioxus/UPSTREAM.md).
- Layout primitives keep caller-selected spacing in inline styles; customize
  spacing tokens rather than trying to override that inline declaration.
- No CSS preprocessor, runtime theme engine or extra dependency is required.

## Verify a theme

Run the [workspace checks](testing-strategy.md), serve the design-system app, and
run its served-selector checker. Compilation alone cannot validate the cascade.
In a browser, check the following for default and custom themes:

- Light, dark, System mode and nested explicit providers under the opposite OS mode.
- Button hover/disabled/loading states, toggle selection, checkbox and radio states.
- Input, Select and Textarea focus/invalid/disabled states; errors retain a red ring.
- Dialog/AlertDialog, popup anchoring and readable content at desktop/mobile widths.
- Sidebar expanded/collapsed dividers and icon alignment; text wraps with chosen fonts.
- Keyboard-visible focus, readable status colors and reduced-motion overlays/skeletons.

Keep touch targets accessible when choosing compact dimensions. Small/large
button variants and coarse-pointer minimum sizes are intentional local behavior.
