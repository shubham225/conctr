# Dioxus presentation structure

Conctr's fresh design system uses an immutable Rust UI source snapshot, a Conctr
facade, and reusable compositions. Product pages/state remain in `conctr-app-dioxus`.

```text
crates/conctr-ui-dioxus/src/
  rust-ui/       # Unmodified UI, hooks, demos, workflows; MIT notice and hashes
  ui/            # App-facing families, brand wrappers and theme provider
  composed/      # Reusable Conctr combinations built over ui
  assets/        # Dioxus asset handles and stylesheet loader
```

Dependencies go from compositions to facade controls to upstream primitives/hooks.
Upstream absolute imports are resolved through root facade re-exports, without
rewriting source. Neither layer imports product pages, services or platform logic.
Apps import `conctr_ui_dioxus::ui::<family>`; the former `components::*` namespace
was removed during the user-directed replacement. The shared app was migrated.

`apps/conctr-design-system` is the fresh runnable showcase with 78 component demos
and eight workflow examples. `design/` remains the canonical design kit; the
gallery's `assets/reference/` is its read-only snapshot. The previous gallery and
crate are [backed up](../../backups/pre-rust-ui-20261115T112238Z/README.md).
Canvas coordinates and interaction state are presentation only and must never
become the platform's executable flow semantics. Demos do not establish integration
support. See [adoption assessment](../development/rust-ui-adoption.md),
[theme ownership](../development/theming.md), and
[ADR 0013](decisions/0013-rust-ui-facade.md).
