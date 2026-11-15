# Conctr UI for Dioxus

Fresh Rust UI based design system. This replaces the former DioxusLabs-based
implementation; the previous source and gallery are in
[pre-migration backups](../../backups/pre-rust-ui-20261115T112238Z/README.md).
Old `components::*` APIs have been removed. The shared product app uses the new facade.

## Usage

```rust,ignore
use conctr_ui_dioxus::{prelude::*, ui::input::Input};
use dioxus::prelude::*;
#[component]
fn App() -> Element {
    rsx! { ThemeProvider { theme: Theme::Light,
        Input { placeholder: "Connection name" }
        Button { "Test connection" }
    } }
}
```

Use `conctr_ui_dioxus::ui::<family>` in applications. All 86 upstream UI modules
are exposed through this facade; Button, Input, Textarea and Card add Conctr defaults.
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
No customizations go into the upstream snapshot. See [provenance](UPSTREAM.md).

## Library development

Run `npm ci`, then `npm run build:css`. Cargo refreshes CSS when the local compiler
is installed. Commit `assets/styles/tailwind.css` so consumers need no Node.
The compiler scans only this library and its explicit shell-utility inventory.
There are no source paths into consuming apps. Additional application-specific
Tailwind classes require application-owned CSS compilation or a shared library composition.
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
