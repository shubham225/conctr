# Conctr Dioxus design system

Editable Dioxus 0.7 components styled with Conctr's design tokens, fonts and UI kit.
The crate contains all 45 components listed by `dx components list`, plus the
Conctr-specific components in `design/ui-kit/`. It contains no platform services
or dependency on Conctr's application/API crates.

## Usage

```rust
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut name = use_signal(String::new);
    rsx! {
        ThemeProvider { theme: Theme::Light,
            Card {
                CardHeader { CardTitle { "Create connection" } }
                CardContent {
                    Field {
                        Label { html_for: "name", "Connection name" }
                        Input {
                            id: "name",
                            value: name(),
                            oninput: move |event: FormEvent| name.set(event.value()),
                        }
                    }
                }
                CardFooter { Button { "Create connection" } }
            }
        }
    }
}
```

`ThemeProvider` loads bundled fonts, canonical kit CSS, tokens and the registry
compatibility aliases and explicit CSS-module links on every server-rendered
document. Choose `Light`, `Dark`, or `System` (default). Explicit
nested providers override inherited colors. `ConctrStyles` can be mounted directly
if an application already supplies its own `cn-body` shell. Fonts are bundled as
a directory so their relative WOFF2 URLs survive asset hashing.

## Source ownership

```text
src/components/
  dioxus/
    button/
      mod.rs
      component.rs      # locally owned Dioxus registry implementation
      style.css         # scoped Conctr skin and primitive states
    tabs/ ...           # all 45 registry families follow this structure
  button/
    mod.rs
    component.rs        # public facade re-exporting the themed implementation
  connection_flow/
    mod.rs
    component.rs        # Conctr-specific implementation
  empty_state/ ...
```

Use `components::button::Button` for explicit imports or `prelude::*` for common
components. Advanced families with overlapping names (`tag_group` and
`drag_and_drop_list`) are available through their named modules. The prelude does
not export the `components::dioxus` module, avoiding a collision with the Dioxus
crate name.

The editable source lives in this repository, as in shadcn's copy-in model. There
is no runtime registry lookup or code generator. Customize a registry component
in `components/dioxus/<name>/component.rs` and its adjacent `style.css`; every
public facade uses that same implementation. Add composition, defaults or a
product-specific wrapper in `components/<name>/component.rs`. Shared kit recipes
remain in `assets/styles/conctr-ui.css`; the CSS modules adapt those recipes to
the primitive markup. See [UPSTREAM.md](UPSTREAM.md) for provenance and upgrades.

## Component catalog

Registry families:

- accordion, alert_dialog, aspect_ratio, avatar, badge, button, calendar, card
- checkbox, collapsible, color_picker, combobox, context_menu, date_picker
- dialog, drag_and_drop_list, dropdown_menu, form, hover_card, input, item, label
- menubar, navbar, pagination, popover, progress, radio_group, scroll_area, select
- separator, sheet, sidebar, skeleton, slider, switch, tabs, tag_group, textarea
- toast, toggle, toggle_group, toolbar, tooltip, virtual_list

The upstream `form` registry entry was empty; this crate provides a native form
adapter there, with application-owned submission and validation.

Kit coverage:

| UI kit family | Public components |
| --- | --- |
| Typography | `Heading`, `HeadingLevel`; kit classes `cn-display`, `cn-body-lg`, `cn-small`, `cn-caption`, `cn-mono` |
| Layout | `Container`, `Stack`, `Cluster`, `Divider` |
| Actions | `Button`, `ButtonVariant`, `ButtonSize`; `loading` disables the action and exposes busy state |
| Forms | `Form`, `Field`, `Label`, `Input`, `Textarea`, `Select`, `Hint`, `FieldError` |
| Choices | `Checkbox`, `CheckboxState`, `RadioGroup`, `RadioItem`, `Switch`; native `Radio` also available |
| Badges/shapes | `Badge`, `StatusBadge`, `Status`, `ProtocolBadge`, `Protocol`, `Shape` |
| Surfaces/messages | `Card` and its parts, `StatusAlert`, `Alert`, `ToastProvider`, `use_toast`, `ToastOptions` |
| Navigation | `Tabs`, `TabList`, `TabTrigger`, `TabContent`, `Breadcrumb`, `Pagination`, `Navigation`, `NavItem`, `NavGroup`, `Sidebar` |
| Data | `TableContainer`, `Table`, `CodeBlock`, `Stat`, `Avatar` and its parts, `KeyboardKey` |
| Feedback | `Progress`, `Spinner`, `Skeleton` |
| Overlays | `DropdownMenu` and its parts, `Menu`, `MenuItem`, `Tooltip` and its parts, `Dialog`, `Modal`, `ModalFooter` |
| Connection flow | `ConnectionFlow`, `ConnectionNode`, `FlowState` (healthy, paused, failing) |
| Empty state | `EmptyState` with caller-supplied action/icon |
| Branding | `Logo`, `Icon`, provided logo/icon assets |

`Dropdown`, `Menu` and `MenuItem` are simple native composition helpers. Use the
primitive-backed `DropdownMenu` family when you need menu keyboard behavior.
`Modal` is a convenience wrapper around `Dialog`, rather than a second dialog
implementation. Mount it conditionally and provide an explicit close action.

## Controlled primitives

The API follows the upstream primitive contracts: `Label` uses `html_for`,
`Checkbox` uses `Option<CheckboxState>` and `on_checked_change`, `Select` uses
`SelectOption` children, and `Tabs` uses string-valued triggers/content. The
previous native-control prototypes' signatures have changed. Most state props
accept signals or values through Dioxus's `ReadSignal` conversion.

For SSR tabs, supply explicit trigger/panel IDs, `aria_controls` on triggers and
`aria_labelledby` on panels; upstream automatic registration runs after mount.
Supply visible labels, unique IDs, hint/error associations and meaningful status
text. A failing connection flow needs a caller-provided explanation. Native
attributes and custom classes remain available through extended attributes.
Calendar callers should pass a consistent `today`/`view_date` during SSR and
hydration, rather than relying on the time-dependent upstream defaults.

For toast messages, mount `ToastProvider` and call `use_toast()` from a descendant.
The default duration is five seconds; primitive behavior handles hover/focus
pausing. Use caller-owned values for all statistics and connection state.

## Development and distribution

```sh
cargo check --workspace
cargo test -p conctr-ui-dioxus
cargo clippy -p conctr-ui-dioxus --all-targets
cargo fmt --all
dx serve --package conctr-design-system --platform web
```

The gallery application is tracked under `apps/conctr-design-system`.
New dependencies support the complete catalog: `dioxus-icons` supplies upstream
Lucide controls, `time` supplies calendar types and WebAssembly-safe clock access, and `palette` supplies color
conversion. No new dependencies enter framework-independent platform crates.
Dioxus uses the workspace's `0.7.1` semver requirement (the lockfile may resolve a
newer 0.7 patch).

The package includes fonts and their OFL licenses, assets, editable components,
and upstream MIT/Apache licenses. `publish = true` opts this crate out of the
workspace's private default; it does not publish anything. Before a crates.io
release, replace the Git `dioxus-primitives` dependency with an available registry
release or publish an appropriately licensed primitive package. The upstream
project currently distributes primitives through Git. Keep the provenance and
licenses when distributing modified sources. This is a source-owned design
system crate, not a new `dx components` registry distribution.

The gallery includes every registry family plus Conctr kit examples, with
searchable navigation, foundation pages, light/dark previews and copyable source.
Run `dx serve --package conctr-design-system --platform web`.
When adding a new CSS module, register its asset in
`src/assets/registry_styles.rs` so later SSR requests also include its stylesheet.
