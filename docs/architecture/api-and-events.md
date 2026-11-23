# API and event contracts

## Implemented management API

The server feature of `conctr-api` exposes an Axum router. A host mounts the
router alongside its frontend. Authentication policy and account provisioning
belong to `conctr-auth`. The Dioxus client uses the same HTTP contract as other
clients. Send `X-Request-Client: conctr` on management requests. The built-in
client sends `dioxus`, which is also accepted. Browser origin checks apply to both.

| Endpoint | Successful response |
| --- | --- |
| `POST /api/v1/auth/login` | 200 with `{"data":"SignedIn"}` or `{"data":"SetPassword"}` |
| `POST /api/v1/auth/forgot-password` | 204 for a recovery request with a username. The operator delivers the private temporary password. |
| `POST /api/v1/auth/first-password` | 204 with a session cookie and empty body |
| `POST /api/v1/auth/session` | 200 with a username and remaining lifetime in `data`, or `{"data":null}` |
| `POST /api/v1/auth/logout` | 204 with an expired cookie and empty body |
| `GET /api/v1/system/details` | 200 with server build version, operating system and architecture in `data`. Requires a workspace session. |

The root router mounts feature routers under a shared version prefix. Each feature
registers its own HTTP methods and relative paths. The Rust client uses the same
`conctr_api::API_VERSION` through `api_path`. Deploy the client and server together
when migrating from unversioned paths. The credential demo API has been removed.

Success bodies use `application/json`. Optional `meta.page` and `meta.warnings`
are omitted when empty. Errors use `application/problem+json` with a relative
problem type URI, stable application code, title, safe detail and matching HTTP
status. Multiple invalid fields include `errors` with JSON Pointer locations.
Clients can supply a request ID of 1–128 ASCII letters, digits, periods, hyphens
or underscores. The server returns the accepted or generated ID in `X-Request-ID`
and includes it in the request tracing span. Login limits return 429 and
`Retry-After: 60`. See [Problem Details](https://www.rfc-editor.org/rfc/rfc9457.html)
and [JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901.html).

This pre-release migration replaces the Dioxus RPC wire format. Deploy the
client and server together. External clients must unwrap `data`, read Problem
Details on errors and skip JSON parsing on 204. The demo echo route was removed.
See the [setup guide](../../crates/conctr-api-dioxus/README.md).

## Proposed resources and events

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

Proposed run admission returns 202 after commit, with the queued run in `data`
and its resource URL in `Location`. Retrieving a failed or partially successful
run returns 200 with its execution state and record failures inside `data`.
Define idempotency-key
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
