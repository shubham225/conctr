# ADR 0001: Framework-independent Core

- Status: Accepted

## Context

CONCTR initially uses Dioxus for its frontend applications.

The project may adopt other frontend technologies in the future,
such as GPUI, egui, or React.

Coupling business logic to Dioxus would make future frontend changes
expensive and difficult.

## Decision

Keep platform business logic independent of presentation frameworks.

Dioxus-specific functionality must remain in Dioxus-specific crates
and applications.

Platform crates must not depend on frontend frameworks.

## Consequences

### Positive

- Frontends can be replaced independently.
- Business logic can be reused.
- Platform functionality can be tested independently.
- Additional frontends can be introduced.

### Trade-offs

- Interfaces between layers require deliberate design.
- Framework adapters introduce some additional code.
- Cross-layer communication must be explicit.

## Implementation Guidelines

- Keep foundational types in conctr-core.
- Keep reusable platform functionality in platform crates.
- Keep Dioxus components in conctr-ui-dioxus.
- Keep Dioxus server functions in conctr-dioxus-api.
- Avoid framework-specific types in shared contracts.

## Alternatives Considered

### Put all logic inside the Dioxus application

Rejected because it couples the platform to one frontend framework.

### Share UI components across all frontend frameworks

Rejected because frontend frameworks have different rendering
and component models. Platform logic, not presentation code,
should be shared.
