# Application instructions

Applies under `apps/` in addition to the root [AGENTS.md](../AGENTS.md).
Read [presentation ownership](../docs/architecture/dioxus-presentation.md) and the
[crate map](../docs/architecture/crate-map.md) before changing application ownership.

- Web, desktop and mobile packages are launchers/adapters. Shared product routes,
  pages, feature state and API integration belong in `crates/conctr-app-dioxus`.
  That crate is currently a scaffold; inspect source before assuming an App exists.
- Put actual OS capabilities in platform adapters; use responsive layouts for screen
  width differences. Do not create three copies of the product UI or business rules.
- `conctr-design-system` is the interactive component showcase. Keep examples using
  the shared library and maintain catalog/source coverage when components change.
  Demo state and protocol artwork do not imply backend or connector support.
- Keep shared styles/assets in their owning library; app assets hold only app-specific
  presentation. Follow the [theming contract](../docs/development/theming.md).
- Keep renderer/server features explicit. Check SSR/hydration and browser-only work
  where applicable. Validate the renderer actually affected and state any SDK limits.
