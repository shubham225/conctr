# Architecture Overview

## Objective

CONCTR follows a modular architecture built around a framework-independent
Rust platform.

Applications provide user interfaces and entry points. Reusable crates
provide platform functionality.

## Architectural Principles

1. Separation of concerns.
2. Framework independence.
3. Reusable platform capabilities.
4. Explicit interfaces.
5. Dependency direction toward stable abstractions.

## Main Components

- Applications: user-facing applications and runnable services.
- Core: foundational types and shared primitives.
- Domain and application logic: business rules and use cases.
- Infrastructure: storage, connectors, and execution implementations.
- UI adapters: framework-specific presentation components.

## Frontend Independence

Dioxus is the initial frontend framework.

Future frontends may use GPUI, egui, React, or other technologies.

Replacing a frontend should not require rewriting platform business logic.

See [System Overview](system-overview.md).
