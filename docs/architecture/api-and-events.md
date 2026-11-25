# Management HTTP API

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
`conctr_api::API_VERSION` through `api_path`. Coordinate client and server changes when the wire contract changes.

Success bodies use `application/json`. Optional `meta.page` and `meta.warnings`
are omitted when empty. Errors use `application/problem+json` with a relative
problem type URI, stable application code, title, safe detail and matching HTTP
status. Multiple invalid fields include `errors` with JSON Pointer locations.
Malformed request bodies receive 400. Field validation can return 422.
Clients can supply a request ID of 1–128 ASCII letters, digits, periods, hyphens
or underscores. The server returns the accepted or generated ID in `X-Request-ID`
and includes it in the request tracing span. Login limits return 429 and
`Retry-After: 60`. See [Problem Details](https://www.rfc-editor.org/rfc/rfc9457.html)
and [JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901.html).

For example, a request for system details without a valid session returns 401
with a Problem Details body.

```json
{
  "type": "/problems/session-required",
  "title": "Session required",
  "status": 401,
  "detail": "Sign in to continue.",
  "code": "SESSION_REQUIRED"
}
```

Reuse the [response types](../../crates/conctr-api/src/response.rs) and
[HTTP error conversion](../../crates/conctr-api/src/http/problem.rs). The
[management tests](../../crates/conctr-api/tests/management.rs) exercise the wire
contract. See [engineering learnings](../development/learnings.md) when extending it.
