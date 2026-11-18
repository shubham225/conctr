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

Each base family has a Conctr-owned wrapper module under `src/ui/`, including
families that forward upstream defaults. All 86 upstream UI modules are exposed
through this facade. Button, Input, Textarea and Card add Conctr defaults.
InputOTP uses native editing over upstream slots because the upstream root references
a missing fixed-path JavaScript file. No app script setup is required.
`BrandThemeToggle` is controlled by the application. The upstream ThemeToggle
remains available for upstream demos; it manages the HTML class and storage itself.
Prefer the controlled Conctr switch with scoped ThemeProvider.

Conctr colors, fonts, logos and icons come from `design/`. Both palettes use
semantic CSS variables. Button heights are 32/40/48 px and default control radius
is 10 px. Caller Tailwind classes merge after wrapper defaults. Applications load
bundled styles through ThemeProvider and need no Node or Tailwind setup.

`src/rust-ui/` contains unchanged component, hook, demo and workflow files.
`src/ui/` holds facade adapters; `src/composed/` holds Conctr combinations.
Import `ui::input_group` for individual input-group controls, or use
`composed::InputGroup` for a field with prefix and suffix slots. Keep custom CSS
next to its component, as in `src/composed/resizable/`. Existing upstream skins
are compiled from adjacent stylesheets into the shipped utility bundle.
No customizations go into the upstream snapshot. See [provenance](UPSTREAM.md).

## Library development

Run `npm ci`, then `npm run build:css`. Cargo refreshes CSS when the local compiler
is installed. Commit `assets/styles/tailwind.css` so consumers need no Node.
The library compiler scans its own source, shell utilities and the shared app/gallery
consumer paths declared in `tailwind.input.css`. Keep additional scan paths here;
consuming apps do not own a Tailwind toolchain.
Run `python3 scripts/check-upstream.py` to verify vendored source integrity.

## Coverage and limits

The fresh [showcase](../../apps/conctr-design-system/README.md) includes 78 primary
component demos and eight workflow examples. This does not establish accessibility,
native-platform or production readiness for every upstream module. Upstream marks
Rust UI experimental. Dioxus AutoForm is only a form shell; it has no derive-driven
form generation. Browser-specific upstream controls may use JavaScript eval.
Workflow nodes, edges and canvas history are presentation state, with no executable
flow contract, persistence, authorization or connected runtime.

See [adoption assessment](../../docs/development/rust-ui-adoption.md) for licensing,
theming and the verification record.

## Brand compositions

`composed::{Protocol, ProtocolShape, ProtocolBadge, BadgesAndShapes,
FeedbackMessage, Feedback, EmptyState, ConnectionFlow}` expose the canonical kit
patterns. Protocol identity determines its shape and color; labels remain visible.
ConnectionFlow accepts source/target names and protocols and is a presentation
summary, separate from the interactive WorkflowBuilder and executable flow state.
EmptyState accepts action children so the application owns the action.
The gallery groups these examples under Composed.

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

Checkbox now supports omitted `checked` for local state, or a supplied value with
callbacks for controlled state. The theme switch uses accessible sun/moon icons.
ToastTrigger renders semantic Alert content; the connection summary's amber
packet moves along its links and stops with reduced motion. Table skins exclude
calendar day geometry; styling adapters for Command, phone inputs, menus and tabs
remain outside the vendored registry.

`ui::sidebar::{SidebarProvider, Sidebar, SidebarTrigger, SidebarGroup, SidebarItem}`
provides grouped navigation with a 56 px desktop icon rail. Wrap Sidebar and main
content together in SidebarProvider, and place SidebarTrigger in the main-content
header. Below 768 px, that trigger opens a native modal drawer supporting Escape,
contained focus and focus restoration. Providers own independent state. The
[gallery example](../../apps/conctr-design-system/src/examples/sidebar.rs) shows the
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
stylesheet registration. Product routes and drafts live in the
[shared app](../conctr-app-dioxus/README.md).

`ui::code_block::CodeBlock` renders read-only snippets. Pass a `CodeLanguage`
of `Json`, `Yaml`, `Toml`, or `Xml` for syntax colors. The default is
`PlainText`. Supply an optional label for the scrollable code region. The
component displays the supplied text verbatim with lexical coloring.
