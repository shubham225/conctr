# Conctr documentation

Conctr is a planned enterprise system integration platform with a visual flow
designer. This documentation is a reviewable product and architecture proposal,
not a claim that the platform capabilities already exist.

**Repository audit:** 2026-11-13. [Current implementation](roadmap/current-state.md)
is the evidence baseline. New ADRs are **Proposed** until reviewed; existing
accepted decisions remain accepted. No proposed packages were created by this
planning work.

## Start here

1. [Product overview](product/overview.md) — purpose, users and first release.
2. [Capabilities](product/capabilities.md) — first slice, MVP and enterprise scope.
3. [Crate map](architecture/crate-map.md) — existing packages and staged additions.
4. [System overview](architecture/system-overview.md) — boundaries and deployment.
5. [Flow definition](architecture/flow-definition.md) and
   [execution model](architecture/execution-model.md) — the central contracts.
6. [App experience](experience/README.md) and [interactive wireframes](experience/prototype/index.html).
7. [Milestones](roadmap/milestones.md), [checklist](roadmap/checklist.md) and
   [questions to resolve](roadmap/open-questions.md).
8. [Verification strategy](development/testing-strategy.md) and
   [documentation maintenance](development/documentation.md).
9. [Decision records](architecture/decisions/README.md) — review and accept deliberately.
10. [UI themes](development/theming.md) — shared CSS properties, custom brands and density.

## Document map

| Folder | Purpose |
| --- | --- |
| [product/](product/overview.md) | Product scope, vocabulary, journeys and inspiration |
| [architecture/](architecture/README.md) | Crates, contracts, runtime, storage, security and ADRs |
| [experience/](experience/README.md) | App navigation, page designs, visual editor and prototype |
| [roadmap/](roadmap/README.md) | Evidence of progress, delivery gates and unresolved choices |
| [development/](development/workspace.md) | Workspace conventions, tests and documentation maintenance |
| [operations/](operations/readiness.md) | Proposed deployment and operational readiness requirements |
| [ai/](ai/README.md) | AI instructions, machine-readable inventory and portable skills |

## Status vocabulary

- **Implemented:** working source exists; scope and limitations are stated.
- **Scaffold:** package/files exist, but the intended capability is not implemented.
- **Demo:** local illustrative behavior, not a production integration.
- **Proposed:** a recommendation awaiting review/implementation.
- **Deferred:** intentionally outside the current phase; revisit on a stated trigger.

An accepted architecture decision is not evidence that its feature is implemented.
Checkmarks track observable deliverables, not directory presence or intended behavior.

The [design kit](../design/README.md) remains the authority for branding and visual
tokens. These docs govern proposed product behavior and technical boundaries.
