<p align="center">
  <picture>
    <source
      media="(prefers-color-scheme: dark)"
      srcset="assets/readme/logo/horizontal-color-on-dark.svg"
    >
    <source
      media="(prefers-color-scheme: light)"
      srcset="assets/readme/logo/horizontal-color.svg"
    >
    <img
      src="assets/readme/logo/horizontal-color.svg"
      height="64"
      alt="CONCTR Logo"
    >
  </picture>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.99.1-orange?style=flat-square&logo=rust" alt="Rust 1.99.1">
  <img src="https://img.shields.io/badge/Cargo-1.99.1-orange?style=flat-square&logo=rust" alt="Cargo 1.99.1">
  <img src="https://img.shields.io/badge/Dioxus-0.7.10-purple?style=flat-square" alt="Dioxus 0.7.10">
  <a href="LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=flat-square" alt="MIT / Apache-2.0 License">
  </a>
  <a href="https://github.com/shubham225/CONCTR">
    <img src="https://img.shields.io/github/last-commit/shubham225/CONCTR?style=flat-square" alt="Last Commit">
  </a>
</p>

<p align="center">
<b>CONCTR (pronounced Connector)</b> is an enterprise integration platform designed to connect applications, APIs, services, and data systems through a unified integration layer.
</p>

---

## Project status

Conctr is in early development. Users can sign in, recover a local account and
view server details on the Dashboard. Most other workspace pages currently show
only their headings. The component showcase includes interactive mapping and
workflow examples backed by sample data. The integration runtime and connector
crates still contain starter code.

## Goals and future work

The goal is to let teams connect systems that use different APIs and message
formats. For example, an order integration could receive JSON from a storefront,
translate it into the XML expected by a supplier, and record whether delivery
succeeded.

Gregor Hohpe and Bobby Woolf’s *Enterprise Integration Patterns* guides this
design. A **Message Translator** addresses the format change in the order
example. A **Content-Based Router** could select a supplier using a field in
the order. Future work includes reusable connections, mapping and routing,
and a run history that helps operators trace failures. The execution model,
connector interfaces and persistence needs for integration runs remain open.

## Technology

- **Rust and Cargo** provide the backend language and the workspace for shared libraries and application launchers.
- **Axum, Tokio and Serde** serve the management API and encode its typed requests and responses as JSON.
- **WebAssembly and native desktop builds** deliver the shared client to browsers and desktop users. Rust UI components and Tailwind CSS supply controls styled with Conctr’s brand tokens.

## Requirements

- Rust and Cargo with support for the 2024 edition
- A PostgreSQL database and role for the management server with the default configuration
- `dx` CLI 0.7.10
- `wasm32-unknown-unknown` Rust target for the web client

## Build and run

From the repository root, install the web target and build the web client and server.

```sh
rustup target add wasm32-unknown-unknown
dx build --package conctr-web-dioxus --web --fullstack --release
```

## Apps

For local development, run the command for the app you want from the repository root.

| App | Purpose | Run locally |
| --- | --- | --- |
| Design kit | Browse reusable components, icons and themes, and try mapping and workflow examples with sample data. | `dx serve --package conctr-design-kit --platform web` |
| Web | Open the workspace in a browser and run the management API alongside it. | `dx serve --package conctr-web-dioxus` |
| Desktop | Open the shared workspace in a native window with desktop window controls. | `dx serve --package conctr-desktop-dioxus --platform desktop` |

For the web app, create the database and configure the server connection using the [account storage guide](crates/conctr-api-dioxus/README.md#postgresql-account-storage). Then open the URL printed in the terminal. On first launch, the server prints the path
to a private temporary password file. Sign in as `admin` with that password and
choose a permanent one. The Dashboard shows the server version, operating system
and architecture.

## Configuration

Local startup creates a `.env` file using the [default settings](.env.example).
Environment variables take precedence over file entries. Restart the server after
changing a server setting.

| Variable | Used by | Purpose |
| --- | --- | --- |
| `CONCTR_DATABASE_URL` | Server | Required database URL. Use `postgresql://` for PostgreSQL or `mysql://` for MariaDB. |
| `CONCTR_ACCOUNTS_FILE` | Server | Optional legacy import path. Its directory holds temporary password handoffs. Leave blank to use the current OS user’s Conctr directory. Relative paths resolve from the directory containing `.env`. |
| `CONCTR_PUBLIC_ORIGIN` | Server | HTTPS origin used by browsers for remote access. Leave blank for local development. |
| `CONCTR_NATIVE_API_ORIGIN` | Desktop client build | HTTPS address of the server. Set before building a desktop release. |

## Architecture

The web and desktop clients share the workspace screens. Both call the versioned
management API over HTTP. For example, opening Dashboard requests
`GET /api/v1/system/details`. The server checks the session before returning its
build and platform details.

| Component | Responsibility |
| --- | --- |
| Application launchers | Start the web server or native window and load the shared workspace. |
| Shared client | Provide sign-in, navigation and Dashboard, and send management requests. |
| Management API (`conctr-api`) | Define request and response types and serve endpoints under `/api/v1`. |
| Authentication (`conctr-auth`) | Verify account credentials, handle password setup and recovery, and issue and revoke sessions. |

### Architecture principles

- Keep business rules and management API contracts independent of the frontend
  framework so another client can use the same operations.
- Share screens across clients and keep operating-system behavior in each
  launcher. The desktop launcher, for example, owns native window controls.
- Enforce authorization on the server for every protected operation, including
  requests made directly to the API.
- Choose crate boundaries around working integrations. Start with a module and
  extract a library when independent callers need the same behavior.

## Contributing

Start with the [development setup](docs/development/setup.md) and use the
[task guide](docs/ai/AI-INSTRUCTIONS.md) to find the relevant project rules.
Manual and AI-assisted contributions follow the same build and review workflow.
AI tools and skills are optional.

## Tests

```sh
cargo test --workspace
cargo test -p conctr-api --features server
```

## License

Conctr is available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
