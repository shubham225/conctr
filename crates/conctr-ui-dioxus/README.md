# Conctr UI for Dioxus

Use `conctr-ui-dioxus` as the UI dependency in product apps. Import individual
controls from `ui::<family>` and reusable combinations from `composed`.
The library owns the pinned Rust UI implementation.

## Usage

```rust,ignore
use conctr_ui_dioxus::{
    composed::InputGroup,
    ui::{input::Input, theme::{Theme, ThemeProvider}},
};
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::Light,
            InputGroup { prefix: rsx! { "https://" },
                Input { placeholder: "api.example.test" }
            }
        }
    }
}
```

## Component and style ownership

Base controls live in `src/ui/`. Put product-independent combinations in
`src/composed/`. The library ships fonts, semantic theme tokens and compiled
Tailwind CSS. Mount `ThemeProvider` at the application boundary and use the
controlled `BrandThemeToggle` for product theme state.

Use the [theming guide](../../docs/development/theming.md) when changing utilities
or component CSS. For changes to the vendored base controls, see the
[upstream record](UPSTREAM.md). Exercise changes through the
[interactive gallery](../../apps/conctr-design-kit/README.md).

## Brand compositions

`composed::{Protocol, ProtocolShape, ProtocolBadge, BadgesAndShapes,
FeedbackMessage, Feedback, EmptyState, ConnectionFlow}` expose the canonical kit
patterns. Protocol identity determines its shape and color; labels remain visible.
ConnectionFlow accepts source/target names and protocols and is a presentation
summary, separate from the interactive WorkflowBuilder and executable flow state.
EmptyState accepts action children so the application owns the action.
The gallery groups these examples under Composed.

### Loading indicator

Use `composed::loading::ConctrLoader` inline or pass `fullscreen: true` to center
the logo in the viewport. The caption defaults to `Loading...`. Pass `text` to
customize it or an empty string to hide it.

```rust,ignore
ConctrLoader {}
ConctrLoader { fullscreen: true, text: "Checking your session…", progress_bar: true }
ConctrLoader { text: "Importing connections", value: 42.0 }
```

A finite `value` shows percentage completion, including zero, and takes priority
over the looping `progress_bar`. Without either prop, the loader shows the logo
and caption alone.

## Form and date compositions

Each composition has its own file in `src/composed/`. `Calendar` accepts initial
`year`/`month`, optional selected `time::Date` and `on_change`. Its default display
is November 2026 for deterministic examples; supply the desired initial date in
applications. Separate month and year dropdowns in the header allow direct
navigation; arrow buttons step one month at a time. `DatePicker` composes this calendar, reports selection and supports
Escape dismissal. These are single-date controls, not range or timezone editors.
`Tags` accepts values and optional removal; `TagInput` accepts initial tags and
reports changes, supports Enter/comma, Backspace and removal, and rejects empty
or duplicate tags. `InputGroup` composes optional prefix/suffix slots with an
application-supplied input. All five have Composed showcase entries.

`ui::sidebar::{SidebarProvider, Sidebar, SidebarTrigger, SidebarGroup, SidebarItem}`
provides grouped navigation with a 56 px desktop icon rail. Wrap Sidebar and main
content together in SidebarProvider, and place SidebarTrigger in the main-content
header. Set `border_trigger: true` on Sidebar to place its desktop toggle on the
border. Supply a compact logo through `collapsed_header` for the icon rail.
Below 768 px, that trigger opens a native modal drawer supporting Escape,
contained focus and focus restoration. Providers own independent state. The
[gallery example](../../apps/conctr-design-kit/src/examples/sidebar.rs) shows the
layout and disabled items. Navigation visibility is not authorization.

## Visual field mapping

`composed::{FieldMappingEditor, MappingField, MappingLink, MappingTransform}`
provides a controlled editor for source → optional transform → target links.
Supply source/target field schemas, system labels, optional transform choices and
`mappings`; accept the updated `Vec<MappingLink>` through `on_change`. Stable link
IDs preserve rows while editing. Field IDs are unique per schema; transform IDs
are unique and nonempty. Each target can be selected once, while sources may feed
multiple targets. Add uses the first source and next free target. `read_only`
disables editing and hides add/remove controls. Unknown references display missing
selections. Callers validate type compatibility and own persistence/execution;
this UI does not evaluate transforms or establish a runtime mapping contract.

The request and response [usage examples](src/composed/demos/field_mapping.rs)
show caller-owned state. Connectors, field types/paths, transform choices and
keyboard-accessible controls use shared theme tokens. The board scrolls within
its container on narrow screens.

The interactive [VisualFieldMapper](src/composed/visual_mapping/README.md) adds
side-by-side schema fields, click-to-connect links, transform inspection, read-only
review and versioned YAML download. The classic FieldMappingEditor remains available.

## Authoring workspaces

- [Mapping workspace](src/composed/mapping_workspace/README.md): controlled direction/history toolbar.
- [Pipeline workspace](src/composed/pipeline_workspace/README.md): workflow frame, library and inspector slots.
- [Resizable controls](src/composed/resizable/README.md): Rust pointer/keyboard resizing and panel modes.

Component-specific geometry uses adjacent CSS modules and explicit per-document
stylesheet registration. The workspace components accept caller-owned state. Product routes live in the
[shared app](../conctr-app-dioxus/README.md).

`ui::code_block::CodeBlock` renders read-only snippets. Pass a `CodeLanguage`
of `Json`, `Yaml`, `Toml`, or `Xml` for syntax colors. The default is
`PlainText`. Supply an optional label for the scrollable code region. The
component displays the supplied text verbatim with lexical coloring.

## Icon assets

Import SVG assets from `assets::icons`. Each pattern has a named `Asset`
constant. Use `icons::color::SPLITTER` or `icons::mono::SPLITTER` for a specific
edition. The unqualified constants select monochrome. Both editions provide
`PATTERN_GROUPS` with the same 65 entries in the book’s eight chapter groups.
For protocols and authentication, use `BRAND_GROUPS` or named exports such as
`icons::color::REST` and `icons::mono::JWT`. Use an empty alt attribute when adjacent text already names the pattern.

```rust
use conctr_ui_dioxus::assets::icons;
use dioxus::prelude::*;

#[component]
fn SplitterLabel() -> Element {
    rsx! {
        span { class: "inline-flex items-center gap-2",
            img { src: icons::SPLITTER, width: 24, height: 24, alt: "", class: "dark:invert" }
            "Splitter"
        }
    }
}
```

## Toast notifications

`ThemeProvider` includes a shared toast viewport. In a descendant component,
get the handle with `ui::toast::use_toast()` and call `push` from an event or
async task. Pass `ToastType::Warning` for warnings and `ToastType::Error` for
errors.

```rust,ignore
use conctr_ui_dioxus::ui::toast::{ToastType, use_toast};

let toast = use_toast();
rsx! {
    Button {
        onclick: move |_| toast.push(
            "Sync delayed",
            "Check the target and try again.",
            ToastType::Warning,
        ),
        "Check sync"
    }
}
```

Use `ToastTrigger` for a button with a fixed message. Both ways of publishing
use the same compact stack. The newest toast sits in front of two visible card
edges. Each toast has a five-second countdown line and dismisses when it empties.
Hover over the stack or focus its close button to reveal every toast and pause
the countdowns. Each close button dismisses its own toast. Touch devices show
the toasts in a scrollable list. In a custom theme scope, mount `ToastProvider` once to create the
viewport.
