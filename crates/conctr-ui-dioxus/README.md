# Conctr Dioxus design system

Editable Dioxus 0.7 components styled with Conctr's design tokens, fonts and UI kit.
The crate contains all 45 components listed by `dx components list`, plus the
Conctr-specific components in `design/ui-kit/`. It contains no platform services
or dependency on Conctr's application/API crates.

## Usage

Add the library alongside your application's Dioxus dependency:

```toml
[dependencies]
conctr-ui-dioxus = { workspace = true }
```

For an application outside this workspace, use a Cargo `path` dependency pointing
to `crates/conctr-ui-dioxus`. Keep the Dioxus renderer features in your launcher.
Mount a provider at the root and import components from the prelude:

```rust
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

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

`ThemeProvider` loads bundled fonts, tokens, shadcn-style themes and precompiled
Tailwind utilities. Components use utility classes in Rust and adjacent CSS modules
for specialized styling. Choose `Light`, `Dark`, or `System` (default); nested
providers reset their own defaults. No Tailwind setup or CSS build step is needed
in the consuming application.

Use typed props for component variants, sizes and state; native attributes,
`class`, `style` and event handlers pass through to the rendered control:

```rust,ignore
Button {
    variant: ButtonVariant::Secondary,
    size: ButtonSize::Sm,
    style: "--radius: 999px;",
    onclick: move |_| { /* application action */ },
    "Test connection"
}
```

To customize a whole subtree, set shadcn variables on its `ThemeProvider`, such as
`style: "--radius: 12px; --control-height: 32px;"`. Palette and scale defaults live
in `assets/colors/tokens.css`; semantic theme roles include `--background`,
`--foreground`, `--primary`, `--primary-foreground`, `--border` and `--ring`.
The UI crate compiles utilities from its own sources and workspace apps, including
`conctr-app-dioxus`. No app CSS build is needed. Standalone consumers receive the
shipped bundle; new external utility names require compilation. See [theming](../../docs/development/theming.md).

Browse complete, runnable usage examples with:

```sh
dx serve --package conctr-design-system --platform web
```

Each gallery page shows the exact source from
[`apps/conctr-design-system/src/examples`](../../apps/conctr-design-system/src/examples).

## Source ownership

```text
src/components/
  primitives/              # Box, Flex, Grid, Stack, Inline, Container, ...
  ui/                      # Base controls and presentation foundations
    button/
      mod.rs
      component.rs         # Locally owned implementation
      style.css            # Adjacent Conctr skin
    dialog/ ...
  composed/                # Reusable combinations; future complex components
    connection_flow/
    empty_state/
    modal/
    stat/
```

Use `components::ui::button::Button`, `components::primitives::Stack`, or
`components::composed::connection_flow::ConnectionFlow` for explicit imports.
`prelude::*` remains available. Existing flat paths such as
`components::button::Button` remain supported by re-exports, without duplicate
facade directories. The old `components::dioxus` namespace is a hidden
compatibility alias. Advanced families with overlapping names (`tag_group` and
`drag_and_drop_list`) remain available through their named modules.

Edit a control in `components/ui/<name>/component.rs` and its adjacent
`style.css`. Add reusable compositions to `components/composed/<name>/`;
product-specific pages, fetching and state belong in the product application.
Composed components may use UI controls and primitives; lower layers must not
import compositions. There is no runtime registry lookup or code generator.
Tailwind 4.1 uses shadcn-style variables (`--background`, `--primary`, `--ring`,
`--radius`, and standard surface/foreground pairs) with Conctr kit defaults.
Generic runtime CSS and Tailwind mappings live in `assets/styles/`. Component
utility classes stay in Rust; specialized CSS modules stay in adjacent
`style.css` files. See the [theming guide](../../docs/development/theming.md) for
consumer usage, scoped palettes, density, library tooling and new components.
`MAIN_CSS` is the plain runtime theme; `CONCTR_UI_CSS` remains an alias.
See [UPSTREAM.md](UPSTREAM.md) for provenance and
[presentation architecture](../../docs/architecture/dioxus-presentation.md)
for boundaries and the proposed shared multi-platform application structure.

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
| Layout primitives | `primitives::Box` (`LayoutBox` in the prelude), `Flex`, `Grid`, `Stack`, `Inline`, `Space`, `Align`, `Justify`, `Direction`; also `Container`, `Cluster`, `Divider` |
| Panel layouts | `ResizablePanelGroup`, `ResizablePanel`, `ResizableHandle`, `ResizableOrientation` |
| Actions | `Button`, `ButtonVariant`, `ButtonSize`; `loading` disables the action and exposes busy state |
| Forms | `Form`, `Field`, `Label`, `Input`, `Textarea`, `Select`, `Hint`, `FieldError`, `FileUploader` |
| Choices | `Checkbox`, `CheckboxState`, `RadioGroup`, `RadioItem`, `Switch`; native `Radio` also available |
| Badges/shapes | `Badge`, `BadgeVariant` (neutral, success, warning, info, error), `StatusBadge` (compatibility adapter), `Status`, `ProtocolBadge`, `Protocol`, `Shape` |
| Surfaces/messages | `Card` and its parts, `Alert` (`status: Status::…`), `ToastProvider`, `use_toast`, `ToastOptions` |
| Navigation | `Tabs`, `TabList`, `TabTrigger`, `TabContent`, `Breadcrumb`, `Pagination`, `Navigation`, `NavItem`, `NavGroup`, `Sidebar` |
| Data | `TableContainer`, `Table`, `CodeBlock`, `Stat`, `Avatar` and its parts, `Kbd`, `KbdGroup` |
| Feedback | `Progress`, `Spinner`, `Skeleton` |
| Overlays | `DropdownMenu` and its parts, `Menu`, `MenuItem`, `Tooltip` and its parts, `Dialog`, `Modal`, `ModalFooter`, `Command`, `CommandDialog` |
| Connection flow | `ConnectionFlow`, `ConnectionNode`, `FlowState` (healthy, paused, failing) |
| Empty state | `EmptyState` with caller-supplied action/icon |
| Branding | `Logo`, `Icon`, `UiIcon`, `IconName`, provided logo/icon assets |

`Dropdown`, `Menu` and `MenuItem` are simple native composition helpers. Use the
primitive-backed `DropdownMenu` family when you need menu keyboard behavior.
`Modal` is a convenience wrapper around `Dialog`, rather than a second dialog
implementation. Mount it conditionally and provide an explicit close action.

## New controls

`Alert { status: Status::Warning, ... }` is the semantic message API. It uses a
polite status announcement for neutral/info/success/warning and an alert for danger.
`StatusAlert` remains a compatibility adapter. `Kbd` renders a native `kbd` element;
`KbdGroup` groups shortcut hints. `KeyboardKey` remains an alias of `Kbd`.

```rust,ignore
KbdGroup { aria_label: "Control K", Kbd { "Ctrl" } Kbd { "K" } }
Alert { status: Status::Warning, "Review the connection configuration." }
FileUploader {
    id: "attachments", accept: ".json,.yaml,.csv", max_files: 3,
    max_file_size: 5 * 1024 * 1024,
    on_files_change: move |files: Vec<dioxus::html::FileData>| {
        // Full current selection. The app can read/stream files and upload them.
    },
}
```

The uploader supports native browsing and drag/drop, case-insensitive extension
and MIME filters, size/count limits, duplicate suppression, removal, disabled
state and accessible rejection messages. `on_reject` exposes structured reasons.
It retains renderer file handles without eagerly reading file contents. Transport,
server-side validation, upload progress and cancellation belong to the application.

```rust,ignore
ResizablePanelGroup {
    id: "workspace", default_sizes: vec![30.0, 70.0], min_sizes: vec![20.0, 30.0],
    style: "height: 280px;",
    ResizablePanel { index: 0, "Connections" }
    ResizableHandle { index: 0, with_handle: true }
    ResizablePanel { index: 1, "Details" }
}
```

Panel sizes are percentages, initialized when the group mounts. Use consecutive
panel indices and a handle between each pair. Groups support horizontal/vertical
orientation, nesting, RTL, pointer capture and `on_layout_change`. Focus a handle
and use arrows (1%), Shift+arrows (10%) or Home/End (bounds). Minimums clamp dragging
and keyboard updates. Give each group a stable, unique ID for SSR and ARIA.

```rust,ignore
Command { id: "actions", on_select: move |value: String| { /* run action */ },
    CommandInput {}
    CommandList { CommandEmpty { "No matching commands." }
        CommandGroup { heading: "Navigate",
            CommandItem { index: 0, value: "Connections", "Connections" }
            CommandItem { index: 1, value: "Runs", keywords: vec!["history".into()], "Runs" }
        }
    }
}
```

Command item indices are unique across groups and define keyboard order. Search
matches all query words against values/keywords, ignoring case. Arrows wrap among
matching enabled options, Home/End choose endpoints and Enter selects. Focus stays
in the combobox, linked to the active option with ARIA. `CommandSeparator` and
`CommandShortcut` complete the composition. `CommandDialog` accepts `open` and
`on_open_change`, using the shared focus trap, Escape handling and a visible close action;
applications own global shortcuts. See the gallery's [command](../../apps/conctr-design-system/src/examples/command.rs),
[resizable](../../apps/conctr-design-system/src/examples/resizable.rs),
[uploader](../../apps/conctr-design-system/src/examples/file_uploader.rs) and
[kbd](../../apps/conctr-design-system/src/examples/kbd.rs) examples.

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

## Styling ownership

Mount `ThemeProvider` (as above), or mount `ConctrStyles {}` once if you manage
scopes yourself. This loads bundled fonts, `tokens.css`, shadcn-style variables
and precompiled Tailwind 4.1.18 utilities. Importing a component loads its adjacent
CSS module and required visual dependencies. No app Tailwind configuration,
source scanning, npm install or copied assets are required.

All styling tools live here. For **library development only**, run `npm ci` once
in this directory. Cargo rebuilds then refresh the shared utility bundle
using the crate's `build.rs`; consumers without Node use the checked-in bundle.
For utility-only DX hot reload without a Rust rebuild, run `npm run watch:css`.
Before release, run `npm run check:css` and commit the updated bundle. Local
`style.css` modules need no separate compiler or generated companion file.
See the [theming guide](../../docs/development/theming.md) for token overrides
and the CSS-module pattern.

## SSR stylesheet workaround

Keep the `STYLES` asset declaration and local `document::Stylesheet` in components
with adjacent CSS. The locked Dioxus/Manganis **0.7.10** CSS-module macro uses a
process-wide `OnceLock`: automatic loading emits a link for the first SSR document
but misses later requests. Styles can appear only after browser hydration.
Upstream tracks the fix in [Dioxus #5707](https://github.com/DioxusLabs/dioxus/pull/5707).

Our components use `Styles::class_name.inner` for scoped class names and register
the CSS-module asset explicitly for each document. The constant is shared within
its component family; links are deduplicated per document. Utility-only components
need no local stylesheet. Applications need no additional workaround code.

See the [SSR guide](../../docs/development/theming.md#ssr-css-module-workaround)
for the implementation pattern, regression check and criteria for removing it.

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

## Component layers

The gallery separates **Primitives**, **Core** component categories and
**Composed** components. Primitives handle layout; core components handle a
single control or interaction. Composed components combine those pieces into
application patterns. `Stat`, `ConnectionFlow`, `EmptyState` and the convenience
`Modal` wrapper live in the composed catalog. Keep future data tables, flow
editors and dashboard compositions in their own public component modules, built
from core controls. Platform business logic stays outside this crate. Existing
public module paths remain available through compatibility re-exports.

### Layout primitives

```rust
use conctr_ui_dioxus::{components::primitives, prelude::*};
use dioxus::prelude::*;

#[component]
fn LayoutExample() -> Element {
    rsx! {
        primitives::Box { padding: Space::Xl,
            Stack { gap: Space::Xl,
                Flex { justify: Justify::Between, align: Align::Center, wrap: true,
                    Heading { level: HeadingLevel::H4, "Connection settings" }
                    Button { variant: ButtonVariant::Secondary, "Edit connection" }
                }
                Grid { columns: 3, gap: Space::Lg,
                    for name in ["Source", "Mapping", "Target"] {
                        primitives::Box { padding: Space::Lg, "{name}" }
                    }
                }
                Inline { gap: Space::Sm,
                    Badge { variant: BadgeVariant::Success, "Healthy" }
                    ProtocolBadge { protocol: Protocol::Rest }
                }
            }
        }
    }
}
```

`Space` maps to existing tokens: None=0, Xs=4, Sm=8, Md=12, Lg=16,
Xl=24, Xxl=32, Section=48px. `Grid` supports 1–12 equal columns and
collapses columns when they would be narrower than `min_item_width` (12rem by
default). Every primitive accepts native div attributes. Use qualified
`primitives::Box` or the prelude's `LayoutBox`: glob-importing a component named
`Box` shadows Rust's `Box` used by RSX macros.

`BadgeVariant::Error` and `Danger` share the danger treatment. `StatusBadge`
remains a compatibility adapter over `Badge`; new compositions should use the
semantic Badge variants directly. All twelve supplied protocol/authentication
icons are exported in both color and mono forms from `assets::icons`.
`Icon { monochrome: true, .. }` masks supplied mono artwork with currentColor
so it inherits the surrounding text color in both themes.
`UiIcon` uses the existing Lucide dependency for general interface actions;
these are not new branded protocol shapes.

`Sidebar` supports icon collapse, off-canvas collapse and fixed mode, with
`SidebarTrigger`, header/content/footer, groups, menus and nested menu parts.
The desktop rail is 240px; below 1024px navigation opens as a drawer. Give icon
rail items labels/tooltips, and use `is_active` to expose the current page.
The gallery demonstrates icon collapse and selectable navigation.

`Menu` is a static surface for composition. Use `DropdownMenu` with a labeled
trigger for interactive menus; this keeps the menu hidden until activated and
supplies arrow-key navigation, dismissal and focus restoration.

### Theme toggle

`ThemeToggle` is controlled by the same state passed to `ThemeProvider`:

```rust,ignore
ThemeToggle { dark: dark(), on_change: move |value| dark.set(value) }
```

Import it from `conctr_ui_dioxus::prelude` or `components::ui::theme_toggle`.
It requests the next light/dark value through `on_change`, supports `disabled`
and global attributes, and leaves theme storage/persistence to the caller.
Both themes expand outward from the button,
using browser view-transition snapshots. The callback still owns theme state;
no application animation code is needed. Keyboard activation uses the same
button-centered origin. Rapid activations are ignored until the reveal finishes.
Linux native WebKitGTK bypasses view transitions to avoid a native crash; its
sun/moon icon still animates. Reduced motion and browsers/WebViews without View Transition support update
immediately. Sun/moon icons animate with opacity and transforms unless reduced
motion is enabled. Its accessible label describes the next action.
