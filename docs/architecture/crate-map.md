# Crate and application map

Audit: **2026-11-13**. Existing packages below were read from manifests and source;
proposed dependencies describe the target design, not today's Cargo graph.
Do not add all planned packages in one change.

## Existing packages

| Package and current path | Current evidence | Intended responsibility / exclusions |
| --- | --- | --- |
| `conctr-core` — `crates/conctr-core` | Scaffold: starter add function/test | Small shared IDs and foundational values; not a dumping ground for every domain type |
| `conctr-auth` — `crates/conctr-auth` | Scaffold | Principal, workspace grants and authorization policy; identity-provider adapters stay at a boundary; external connector credentials belong to secrets |
| `conctr-connectors` — `crates/conctr-connectors` | Scaffold | Connector descriptors, operation contracts, registry and initial HTTP implementation; no scheduler or UI logic |
| `conctr-transform` — `crates/conctr-transform` | Scaffold | Declarative mapping AST, bounded evaluation and schema/format conversion; no network or arbitrary filesystem access |
| `conctr-engine` — `crates/conctr-engine` | Scaffold | Plan execution, attempt/run states, retry/deadline policy, recovery and runtime persistence/clock ports; no SQL/HTTP framework dependency |
| `conctr-storage` — `crates/conctr-storage` | Scaffold | PostgreSQL repositories, migrations, durable work claiming and blob adapters; implement ports rather than defining business decisions |
| `conctr-gateway` — `crates/conctr-gateway` | Scaffold | HTTP routing, request identity, limits, API error mapping and webhook ingress; call use cases, not duplicate them |
| `conctr-api` — `crates/conctr-api` | Hello-world binary | Server composition root; config validation, wiring, lifecycle and health; later move directory to `apps/conctr-api` without changing package name |
| `conctr-api-dioxus` — `crates/conctr-api-dioxus` | Demo echo server function | Optional Dioxus transport adapter over the same authorized application services; generic contracts remain independent of Dioxus |
| `conctr-ui-dioxus` — `crates/conctr-ui-dioxus` | Implemented design-system components and tests | Reusable primitives/ui/composed components; no product pages, service calls or business rules |
| `conctr-app-dioxus` — `crates/conctr-app-dioxus` | Scaffold: starter add function/test; workspace member | Shared root App, routes, feature pages, view state and API integration; not yet wired to launchers |
| `conctr-web-dioxus` — `apps/web-dioxus` | Placeholder App | Web/server launch and renderer configuration; share product pages through app crate |
| `conctr-desktop-dioxus` — `apps/desktop-dioxus` | Placeholder App | Native bootstrap, window/menu/deep-link adapters; no duplicated product app |
| `conctr-mobile-dioxus` — `apps/mobile-dioxus` | Placeholder App | Mobile bootstrap and native capability adapters; shared responsive product UI |
| `conctr-design-system` — `apps/conctr-design-system` | Implemented 62-entry catalog, foundation pages and examples | Design system documentation/showcase; demos do not prove production connector support |

`conctr-api-dioxus` is the actual package name. Some earlier prose used
`conctr-dioxus-api`; use the existing Cargo name in new documentation and code.

## Recommended additions: create when their first feature starts

| Proposed package | Phase / creation trigger | Owns | Allowed inward dependencies |
| --- | --- | --- | --- |
| `conctr-flow` | M1, before building the editor/runtime | Versioned manifest, graph validation, typed ports and plan compiler | core; transform AST/config types if needed |
| `conctr-application` | M1, first real author/run use case | Authorize/orchestrate create/edit/publish/deploy/run/query use cases; admin repository and transaction ports | core, auth, flow, engine, connectors, secrets; no storage/UI/gateway |
| `conctr-contracts` | M1, before first supported API | Versioned public DTOs, safe error/event envelopes and schema/export contracts | core and deliberately public flow types; no server framework or storage |
| `conctr-secrets` | M1–M2, before using real credentials | Credential references/metadata, scoped resolver interface, redaction types and approved encryption/vault adapters | core; no UI, app service or gateway |

The application crate above is a **framework-independent use-case layer**;
`conctr-app-dioxus` is the **Dioxus product frontend**. Their similar names do not
make their responsibilities interchangeable. An application use case takes an
authenticated principal and explicitly checks authorization before side effects.

## Later additions only when justified

| Candidate | Add when | Keep initially |
| --- | --- | --- |
| `apps/conctr-worker` (binary) | Execution needs independent lifecycle, isolation or scale | Embedded engine runner in server |
| `conctr-client` | Multiple clients need the same stable generic HTTP API client | Small app service adapter / Dioxus RPC client |
| `conctr-connector-testkit` | Multiple connector implementations repeat the same conformance fixtures | Connector integration-test helpers |
| Per-connector packages | Independent dependency/feature/release ownership warrants them | Feature-gated connector modules; never load every SDK by default |
| `conctr-scheduler` | Durable trigger scheduling needs independent ownership or processes | Engine scheduling module and owner-defined clock/store ports |
| `conctr-observability` | Several binaries repeat substantial telemetry setup | Composition-root configuration and library instrumentation |
| `conctr-plugin-api` | A reviewed sandbox/compatibility model exists | In-process, trusted, compiled connectors only |

Do not add billing, AI, marketplace, separate services per domain, a generic
`utils` crate or a second `common` crate simply because the product is enterprise.
A Rust module is the default until a stable contract, independent dependencies,
reuse or testing boundary justifies a package.

## Target dependency rules

- `core` depends on no Conctr crate. Domain libraries do not import Dioxus.
- `flow` owns the executable definition; `engine` consumes validated plans.
- `engine` owns runtime store/clock/event/secret resolver requirements; storage
  implements them. Do not import storage back into engine.
- `application` owns administrative transaction/repository ports; storage may
  depend on application to implement them. Avoid the opposite dependency.
- Gateway and Dioxus server functions call application use cases. Neither owns
  authorization policy or execution rules.
- Contracts do not import application implementations. Wire DTOs are not SQL rows.
- App UI depends on UI components and transport/client contracts. UI never imports app.
- Executables select concrete adapters. Feature flags do not excuse a dependency cycle.

[System overview](system-overview.md) · [Flow contract](flow-definition.md) · [Roadmap](../roadmap/milestones.md)
