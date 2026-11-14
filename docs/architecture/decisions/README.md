# Architecture decision records

Accepted records document existing direction. Proposed records remain
recommendations for review; superseded records link to the replacing decision.

| ADR | Status | Topic |
| --- | --- | --- |
| [0001](0001-framework-independent-core.md) | Accepted | Framework-independent platform core |
| [0002](0002-dioxus-component-layers.md) | Accepted | Implemented UI component layers |
| [0003](0003-modular-platform.md) | Proposed | Modular platform before distributed services |
| [0004](0004-versioned-flow-definition.md) | Proposed | Versioned flow model independent of canvas |
| [0005](0005-durable-execution.md) | Proposed | Durable execution with explicit unknown outcomes |
| [0006](0006-scope-and-secrets.md) | Proposed | Server-enforced scope and separate credential handling |
| [0007](0007-persistence-and-queue.md) | Proposed | PostgreSQL first with a durable work queue |
| [0008](0008-application-and-transports.md) | Proposed | Shared product app and transport-independent use cases |
| [0009](0009-trusted-connectors-and-mapping.md) | Proposed | Trusted connectors and bounded declarative mapping first |
| [0010](0010-tailwind-component-styles.md) | Superseded by 0011 | Tailwind theme and component-local styles |
| [0011](0011-dx-managed-tailwind.md) | Superseded by 0012 | DX-managed Tailwind 4.1 and plain local CSS |
| [0012](0012-library-owned-tailwind.md) | Accepted | Library-owned Tailwind and component CSS modules |

## Record lifecycle

Use Context, Decision, Alternatives, Consequences and verification/review criteria.
A proposed record becomes Accepted only after maintainer review. Preserve prior
accepted records and supersede them with a linked new record when direction
changes. Keep ADR scope small enough to review independently.

New records: copy [the template](../../development/templates/adr.md), choose the
next unused number and link the implementation/roadmap work separately.
