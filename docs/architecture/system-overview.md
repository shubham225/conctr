# System Overview

## Applications

### web-dioxus

Dioxus-based web application.

### desktop-dioxus

Dioxus-based desktop application.

### mobile-dioxus

Dioxus-based mobile application.

## Platform Crates

| Crate | Responsibility |
|---|---|
| conctr-core | Foundational types and shared primitives |
| conctr-auth | Authentication and authorization |
| conctr-connectors | External system integrations |
| conctr-engine | Integration and workflow execution |
| conctr-gateway | Gateway and server functionality |
| conctr-storage | Persistence and storage |
| conctr-transform | Data transformation |

## Dioxus-specific Crates

| Crate | Responsibility |
|---|---|
| conctr-ui-dioxus | Reusable Dioxus UI components |
| conctr-dioxus-api | Dioxus-specific server functions |

## Conceptual Dependency Structure

```text
Applications
     |
     v
Presentation Adapters
     |
     v
Application Interfaces
     |
     v
Platform Logic
     |
     v
Infrastructure
```

This is a conceptual model. Actual crate dependencies should be
determined by explicit interfaces and must avoid circular dependencies.

## Future Frontends

Potential future applications:

- GPUI desktop application.
- egui desktop application.
- React web application.

All should reuse the same platform capabilities.
