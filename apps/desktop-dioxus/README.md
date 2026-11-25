# Conctr desktop launcher

The desktop launcher supplies a native window, Conctr icon and custom title bar
to the shared product app. Window dragging, minimizing, maximizing and closing
are handled in the [desktop entry point](src/main.rs). Routes and session UI
live in the [shared app](../../crates/conctr-app-dioxus/README.md).

From the repository root, with the native desktop dependencies installed, run

```sh
CARGO_BUILD_JOBS=1 dx serve --package conctr-desktop-dioxus --platform desktop
```

The `desktop` feature enables the renderer and native API client. The `server`
feature hosts the workspace API for fullstack development. Packaged release
clients need the HTTPS `CONCTR_NATIVE_API_ORIGIN` at build time. See the
[configuration guide](../../crates/conctr-api-dioxus/README.md).
