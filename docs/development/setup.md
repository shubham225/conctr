# Run Conctr locally

Use the Rust toolchain, Cargo and a Dioxus 0.7 CLI compatible with the workspace.
Run commands from the repository root. Limit build parallelism for this shell.

```sh
export CARGO_BUILD_JOBS=1
cargo metadata --no-deps --format-version 1
dx serve --package conctr-web-dioxus
```

Open the address printed by the server. For first login, use the local temporary
password file identified by the server and complete password setup. The
[sign-in guide](../../crates/conctr-api-dioxus/README.md) covers configuration,
account recovery and the API origin for native clients.

To work on desktop, use `dx serve --package conctr-desktop-dioxus --platform desktop`
with the native SDK installed. For component development, use
`dx serve --package conctr-design-kit --platform web`. Stop your previous
server before switching. Restart DX after changing workspace path dependencies
so it picks up the new dependency graph.

## Check a change

Start with the affected package. For example, the management API's server tests use
`cargo test -p conctr-api --features server`. For a workspace-wide Rust change,
run these commands sequentially with the build limit above.

```sh
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

For gallery changes, verify served styles and keyboard interactions in a browser.
The gallery checker requires Node 22+ and a dedicated Chrome instance with remote
debugging enabled. Substitute the actual server and debugging ports.

```sh
node apps/conctr-design-kit/scripts/check-browser.mjs http://127.0.0.1:8080 http://127.0.0.1:9225
```

For documentation changes, run `python3 docs/development/scripts/check_docs.py`.
