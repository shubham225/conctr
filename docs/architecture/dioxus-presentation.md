# Presentation ownership

For a screen shared by web and desktop, put its route, loading state and UI
composition in `conctr-app-dioxus`. Put OS window controls in the desktop
launcher. For a new server operation, define its wire types in `conctr-api`
and adapt it to the UI in `conctr-api-dioxus`. Keep Dioxus signals, hooks and
renderer types on the frontend side of that boundary.

For a reusable control, look in `conctr_ui_dioxus::ui::<family>` first.
Combine base controls in `composed` when the result is useful across screens.
Exercise reusable interactions in the design-kit app. A gallery demo can use
in-memory sample data independently of product feature availability.

Use the [brand kit](../../design/README.md) for colors, typography, logos and
component appearance. Use the [Rust UI guide](../development/rust-ui-adoption.md)
for base controls and the [theming guide](../development/theming.md) for CSS.
