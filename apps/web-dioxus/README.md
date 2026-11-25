# Conctr web launcher

The browser build launches the shared product `App`. The server build calls
`serve_workspace` to mount the management API alongside the server-rendered
app. The `web` and `server` Cargo features select those entry points.
See the [launcher](src/main.rs) and [shared app](../../crates/conctr-app-dioxus/README.md).

Run from the repository root and open the URL printed by DX.

```sh
CARGO_BUILD_JOBS=1 dx serve --package conctr-web-dioxus
```

Use the [sign-in guide](../../crates/conctr-api-dioxus/README.md) for first login,
recovery and server settings.
