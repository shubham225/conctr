# Shared Conctr app

Product routes and presentation state shared by web, desktop and mobile launchers.

| Route | Current behavior |
| --- | --- |
| `/` | Connections placeholder; services unavailable |
| `/mappings` | Local request/response drafts, field actions, transforms configuration and undo/redo |
| `/pipelines` | Local workflow canvas, step library, editable inspector and branch labels |
| `/queues` | Sample queue counts, status-filtered history, four exchange payloads and local pause/resume |
| `/runs` | Run-history placeholder; execution unavailable |
| `/settings` | Session appearance guidance; service settings unavailable |

Mapping drafts and pipeline state last for the mounted page session. No persistence,
transform evaluation, external requests, credential binding or deployment is wired.
Reusable controls and workspace frames live in `conctr-ui-dioxus`; its Tailwind
compiler scans this crate. Do not add app Tailwind configs or generated styles.

## Verification

From the repository root:

```sh
cargo check --workspace
cargo test -p conctr-app-dioxus -p conctr-ui-dioxus --lib --tests
cargo clippy -p conctr-app-dioxus -p conctr-ui-dioxus --all-targets -- -D warnings
cargo fmt --all --check
```

Serve the product app and start Chrome with a dedicated temporary profile:

```sh
dx serve --package conctr-web-dioxus --platform web --port 8092 --open false
google-chrome --remote-debugging-port=9225 --user-data-dir=/tmp/conctr-browser-checks about:blank
```

With Node 22+, run scripts sequentially against those actual ports:

```sh
node crates/conctr-app-dioxus/scripts/check-mapping-studio-browser.mjs http://127.0.0.1:8092 http://127.0.0.1:9225
node crates/conctr-app-dioxus/scripts/check-mapping-panels-browser.mjs http://127.0.0.1:8092 http://127.0.0.1:9225
node crates/conctr-app-dioxus/scripts/check-mapping-dialog-browser.mjs http://127.0.0.1:8092 http://127.0.0.1:9225
node crates/conctr-app-dioxus/scripts/check-pipeline-studio-browser.mjs http://127.0.0.1:8092 http://127.0.0.1:9225
```

Scripts exercise local browser behavior; they do not verify runtime integrations
or native SDK builds. See [current evidence](../../docs/roadmap/current-state.md).
