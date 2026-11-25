# Application packages

- Keep launcher entry points thin. Put OS-specific behavior in the launcher
  that needs it. Desktop owns its native title bar. Shared product routes,
  pages and session UI belong in the [product app](../crates/conctr-app-dioxus/README.md).
- Keep renderer and server features explicit. Check the affected renderer and,
  for web changes, the transition from server markup to the interactive page.
  Web and desktop are the active targets. Mobile is pending removal.
- In the design kit, use library controls, local example state and the gallery's
  own demo styles. Update the catalog and usage example when adding a control.
  Use the [theming guide](../docs/development/theming.md) for shared assets and CSS.
