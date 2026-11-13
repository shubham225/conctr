# API and event contracts

Status: **Proposed**. No stable public API exists; the current Dioxus adapter only
contains an echo demonstration.

## Boundary

`conctr-contracts` should contain serializable request/response models and safe
errors independent of Dioxus or a specific server framework. The generic HTTP
gateway and optional Dioxus server functions call the same application use cases.
Transport code maps identity, input and errors; it does not duplicate business rules.

## Resource sketch

| Proposed surface | Purpose / rule |
| --- | --- |
| `/api/v1/workspaces/{workspace}/flows` | List/create scoped drafts; cursor pagination |
| `.../flows/{flow}/draft` | Read/edit with revision precondition; conflict returns current revision |
| `.../flows/{flow}/versions` | Publish immutable validated versions; explicit permission |
| `.../deployments` | Validate/activate version with environment bindings; audit |
| `.../connections`, `.../credentials` | Manage public metadata/write-only secret inputs |
| `.../runs` | Admit an authorized manual run or query history |
| `.../runs/{run}` | Pinned version, attempt summaries and safe errors |
| `.../runs/{run}/cancel`, `.../runs/{run}/replays` | Explicit state transition / new linked run |
| `.../events` | Scoped resumable run-event subscription |
| Dedicated webhook ingress route | Signature/auth policy, durable acceptance and limits |

Paths are illustrative, not registered endpoints. A workspace path segment is
validated against authenticated scope; it does not confer access.

Admission returns an accepted run ID only after commit. Define idempotency-key
scope, expiry and request fingerprint conflict behavior for run admission and
other externally repeatable mutations. Editing uses optimistic concurrency.
Errors expose a stable code, safe message, optional node/config path and correlation
ID; hide storage details and credential material. Cursor tokens must not permit
cross-scope access or bypass sort/filter constraints.

## Event envelope

Propose event ID, event type/schema version, organization/workspace/run IDs,
monotonic per-run sequence, occurrence time, actor/service context and a safe
summary. Delivery may be repeated; clients deduplicate by ID and resume from a
cursor. On cursor expiry, reload a snapshot and continue from a defined boundary.
Do not promise global event ordering. Event rows are a projection/audit trail,
not the entire durable state machine unless event sourcing is explicitly chosen.

Prefer server-sent events for one-way run updates initially; use polling fallback.
WebSockets become useful for bidirectional collaboration only when that feature
exists. Use authorization on reconnect and cap retention/subscription count.
Binary/payload previews are authorized separate fetches, not broadcast event bodies.

## Contract evolution

Pin flow schema and connector versions separately from API version. Add fields
compatibly within a supported version; breaking behavior needs a migration and
compatibility policy. Publish OpenAPI/JSON Schema only after the first vertical
slice stabilizes. Use fixtures that round-trip across web/native clients and
reject unsupported or unsafe manifest features server-side.
