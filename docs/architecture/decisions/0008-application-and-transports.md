# ADR 0008: Shared product app and transport-independent use cases

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

Three platform launcher crates exist, and conctr-app-dioxus is now scaffolded. Product pages and backend use cases should not be copied into each launcher or into transport functions.

## Recommended decision

Implement routes/pages/presentation state once in conctr-app-dioxus; retain thin launchers. Keep reusable controls in conctr-ui-dioxus. Place server use cases in conctr-application and public DTOs in conctr-contracts. HTTP and optional Dioxus RPC call the same authorized use cases.

## Alternatives

One executable with target feature flags is valid but does not warrant a launcher migration now. Dioxus-only business contracts would impede other frontends. Product pages in the design-system crate would couple reusable UI to the product.

## Consequences and review criteria

Native API endpoint/auth configuration and SSR hydration need testing. Native clients are not worker processes. Prefer responsive layout for screen widths and platform adapters for true OS capabilities. Review the initial mobile feature boundary.

Related review questions: Q03, Q06 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../dioxus-presentation.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
