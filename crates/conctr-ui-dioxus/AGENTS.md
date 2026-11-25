# Reusable Dioxus UI

- Keep the pinned Rust UI snapshot in `src/rust-ui/` intact. Refresh source,
  license and hashes together using the [upstream record](UPSTREAM.md).
- Expose each base family through a Conctr-owned module in `src/ui/`, including
  pass-through families. Build reusable combinations in `src/composed/`.
  Let callers supply product data and handle persistence and execution.
- Use Rust signals, Dioxus events and `MountedData` for interaction and
  measurement. Keep any necessary browser bridge local to the component.
- Take brand assets from the [design kit](../../design/README.md). Follow the
  [theming guide](../../docs/development/theming.md) for shared Tailwind, scoped
  component styles, existing upstream skins and per-document SSR registration.
- Run `python3 scripts/check-upstream.py` from this crate to check vendor
  integrity. Verify a changed control through its gallery example, including
  keyboard input and both themes.
