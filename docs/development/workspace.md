# Cargo Workspace Guide

## Overview

CONCTR uses a Cargo workspace to manage multiple applications and
reusable Rust libraries in a single repository.

## Directory Responsibilities

### apps/

Runnable applications and application entry points.

### crates/

Reusable Rust libraries containing platform functionality and adapters.

### docs/

Project, architecture, product, and development documentation.

### assets/

Shared brand and other common assets.

Application-specific assets remain inside their respective app directories.

## Package Naming

Cargo package:

```toml
[package]
name = "conctr-core"
```

Rust import:

```rust
use conctr_core::SomeType;
```

Hyphens in Cargo package names become underscores in Rust imports.

## Shared Dependencies

Shared dependencies may be declared in the root Cargo.toml:

```toml
[workspace.dependencies]
dioxus = { version = "0.7.1" }
```

Workspace members can inherit them:

```toml
[dependencies]
dioxus = { workspace = true }
```

## Adding a Crate

1. Create a directory under crates/.
2. Add Cargo.toml.
3. Add src/lib.rs.
4. Register the package in workspace members.
5. Run cargo metadata.
6. Run cargo check --workspace.

## Adding an Application

1. Create a directory under apps/.
2. Add Cargo.toml and application entry point.
3. Register it in the workspace.
4. Connect it to platform functionality through defined interfaces.

## Dependency Rule

Framework-independent platform crates must not depend on
framework-specific UI crates.
