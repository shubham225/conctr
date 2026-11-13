# Architecture guide

The implemented foundation is a Rust workspace and layered Dioxus design system.
The integration runtime and product application remain largely scaffolds.
See [current-state evidence](../roadmap/current-state.md).

The following designs are **Proposed** unless an ADR explicitly says Accepted.

| Topic | Document |
| --- | --- |
| System boundaries and topology | [System overview](system-overview.md) |
| Package inventory and new crate recommendations | [Crate map](crate-map.md) |
| Domain ownership and lifecycle | [Domain model](domain-model.md) |
| Canonical manifest and graph validation | [Flow definition](flow-definition.md) |
| Durable execution, retries and recovery | [Execution model](execution-model.md) |
| Connector and transformation boundaries | [Connector contract](connector-contract.md) |
| API and event contracts | [API and events](api-and-events.md) |
| Persistence and transactions | [Data and storage](data-and-storage.md) |
| Trust, tenancy, credentials and egress | [Security](security.md) |
| Implemented UI layers and proposed shared pages | [Dioxus presentation](dioxus-presentation.md) |
| Decisions, alternatives and review status | [Decision records](decisions/README.md) |

The documentation recommends a direction; it does not automatically authorize
creating every planned crate or adding every suggested dependency. Implement
phase-sized vertical slices through explicit boundaries.
