# Shared product app

- Add product destinations in `src/routes.rs` and place navigation in the shell
  or user menu as appropriate. Update the [route inventory](README.md) in the
  same change.
- Use `conctr-api-dioxus` for management requests. Keep session presentation in
  the app and authorization checks on the server. Pass OS controls in through
  the launcher's title-bar slot.
- Reuse library controls and compositions. For page-specific styles, follow the
  [sign-in example](src/sign_in/style.css) and [theming guide](../../docs/development/theming.md).
- Verify direct navigation through sign-in and the affected page after login.
  For shell changes, check the sidebar, narrow layout and theme switch. Confirm
  callers before removing routes or public app entry points.
