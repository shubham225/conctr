# ADR 0002: Dioxus component layers

- Status: Accepted

## Context

The UI crate mixed base controls, layout helpers and composed components at one
level, with a second mirrored directory for registry implementations. Future
complex components need a clear home without mixing in product pages or services.

## Decision

Organize `conctr-ui-dioxus/src/components/` into `primitives`, `ui` and `composed`.
Use `ui` rather than `core` to distinguish presentation controls from the
framework-independent `conctr-core` crate. Keep each component family's source
and optional CSS together, consolidating registry facades into their owning UI
module. Preserve existing import paths through re-exports.

Composed components depend on UI controls and primitives. The lower layers do
not depend on composed components. Product routes, pages, data fetching and
business rules remain outside the component library.

Keep the showcase application named `conctr-design-system`; the assets and HTML
reference under `design/` are its design kit.

## Consequences

- New generic data tables, flow editors and dashboard widgets have a clear home.
- Existing prelude and component imports continue working.
- CSS asset registrations and gallery source guidance follow the canonical paths.
- Legacy import aliases remain during migration; they contain no duplicate source.
- Platform launchers are unchanged. A shared product-app crate is proposed in
  [Dioxus presentation structure](../dioxus-presentation.md), not implemented by
  this component reorganization.
