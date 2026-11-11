# CONCTR

**Where systems meet.**

CONCTR (pronounced Connector) is an enterprise integration platform
designed to connect applications, APIs, services, and data systems
through a unified integration layer.

## Vision

One platform. Every integration.

## Goals

- Unified application and API integrations.
- Reusable connectors.
- Data transformation and mapping.
- Workflow orchestration.
- Authentication and authorization.
- Integration execution and monitoring.
- Extensible platform architecture.

## Technology

- Rust
- Cargo workspace
- Dioxus (initial frontend framework)

## Repository Structure

```text
CONCTR/
├── apps/
│   ├── web-dioxus/
│   ├── desktop-dioxus/
│   └── mobile-dioxus/
├── crates/
│   ├── conctr-core/
│   ├── conctr-auth/
│   ├── conctr-connectors/
│   ├── conctr-dioxus-api/
│   ├── conctr-engine/
│   ├── conctr-gateway/
│   ├── conctr-storage/
│   ├── conctr-transform/
│   └── conctr-ui-dioxus/
├── docs/
│   ├── architecture/
│   ├── development/
│   └── product/
├── assets/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── CONTRIBUTING.md
```

## Getting Started

Install the Rust toolchain and Dioxus CLI.

```bash
cargo install dioxus-cli
```

Check the workspace:

```bash
cargo check --workspace
```

Run the web application:

```bash
dx serve --package conctr-web-dioxus
```

## Architecture Principles

1. Framework-independent platform logic.
2. Modular and reusable Rust crates.
3. Replaceable frontend applications.
4. Clear separation of concerns.
5. Explicit dependency boundaries.

## Documentation

- [Product Overview](docs/product/overview.md)
- [Architecture](docs/architecture/README.md)
- [System Overview](docs/architecture/system-overview.md)
- [Development Setup](docs/development/setup.md)
- [Workspace Guide](docs/development/workspace.md)
- [Architecture Decision 0001](docs/architecture/decisions/0001-framework-independent-core.md)
- [Contributing](CONTRIBUTING.md)

## Project Status

CONCTR is in its initial development stage.
The current repository establishes the workspace structure and
foundational application scaffolding.

## License

To be determined.
