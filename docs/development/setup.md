# Development Setup

## Prerequisites

- Rust toolchain
- Cargo
- Dioxus CLI

## Install Rust

https://www.rust-lang.org/tools/install

## Install Dioxus CLI

```bash
cargo install dioxus-cli
```

## Verify Installation

```bash
rustc --version
cargo --version
dx --version
```

## Validate Workspace

```bash
cargo metadata --no-deps --format-version 1
```

## Build and Check

```bash
cargo check --workspace
```

## Run Tests

```bash
cargo test --workspace
```

## Format

```bash
cargo fmt --all
```

## Run Web Application

```bash
dx serve --package conctr-web-dioxus
```

## Run Desktop Application

```bash
dx serve --package conctr-desktop-dioxus
```

## Run Mobile Application

```bash
dx serve --package conctr-mobile-dioxus
```

Refer to the Dioxus documentation for platform-specific build requirements.
