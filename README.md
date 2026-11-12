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

**CONCTR (pronounced Connector)** is an enterprise integration platform designed to connect applications, APIs, services, and data systems through a unified integration layer.

---

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
|   ├── conctr-api/
│   ├── conctr-api-dioxus/
│   ├── conctr-core/
│   ├── conctr-auth/
│   ├── conctr-connectors/
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
├── LICENSE-MIT
├── LICENSE-APACHE
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

## Project Status

CONCTR is in its initial development stage. The current repository establishes the workspace structure and foundational application scaffolding.

## License

CONCTR is dual-licensed under your choice of:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
