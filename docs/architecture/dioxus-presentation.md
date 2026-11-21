# Dioxus presentation structure

Conctr's design system uses an immutable Rust UI source snapshot, a Conctr
facade, and reusable compositions. Product pages/state remain in `conctr-app-dioxus`.

```text
crates/conctr-ui-dioxus/src/
  rust-ui/       # Unmodified UI, hooks, demos, workflows; MIT notice and hashes
  ui/            # App-facing families, brand wrappers and theme provider
  composed/      # Reusable Conctr combinations built over ui
  assets/        # Dioxus asset handles and stylesheet loader
```

Product apps depend on `conctr-ui-dioxus`. They import base controls from
`conctr_ui_dioxus::ui::<family>` and reusable combinations from
`conctr_ui_dioxus::composed`. Each base family has its own Conctr-owned module,
even when it forwards all upstream defaults. Add Conctr behavior to that module
so callers can keep their import paths.

For example, `composed::InputGroup` assembles the controls from `ui::input_group`
and accepts a field such as `ui::input::Input` as its child. The private
`rust_ui` module supplies the pinned implementation. Its internal absolute
imports resolve through the library facade. Product pages, services and business
rules stay in their owning app or platform crate.

`apps/conctr-component-showcase` is the runnable showcase with 78 component demos
and eight workflow examples. `design/` remains the canonical design kit; the
gallery's `assets/reference/` is its read-only snapshot. Integration pattern SVGs are
exported through `conctr_ui_dioxus::assets::icons`.
Canvas coordinates and interaction state are presentation only and must never
become the platform's executable flow semantics. Demos do not establish integration
support. See [adoption assessment](../development/rust-ui-adoption.md),
[theme ownership](../development/theming.md), and
[ADR 0013](decisions/0013-rust-ui-facade.md).

## Local authoring pages

The shared app owns `/mappings` (`src/mapping_studio`) and `/pipelines`
(`src/pipeline_studio`). Mapping request/response drafts and history are app state.
Pipeline sample configuration and authoring checks are app state; workflow nodes,
positions and edges remain presentation data. Both pages are transient local
previews without persistence, runtime execution or deployment.

The UI library supplies `composed/visual_mapping`, `mapping_workspace`,
`pipeline_workspace` and `resizable`. Reuse workflow facade components for canvas
interaction. Resizable handles use Rust signals/events and mounted measurements;
component geometry belongs in adjacent scoped CSS. The classic `field_mapping`
control and `WorkflowBuilder` remain supported gallery/library consumers.
