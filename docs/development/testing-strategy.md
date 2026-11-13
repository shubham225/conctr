# Verification strategy

Status: **Proposed platform strategy**; UI tests/checks already exist.

| Boundary | Required tests as implemented |
| --- | --- |
| Flow compiler | Invalid/oversized graphs, missing ports, cycles, version migrations and deterministic fixtures |
| Mapping | Missing/null/coercion, output/depth budgets and no IO capability |
| Connectors | Local mocks for auth, redirects/egress, rate limits, pagination, cancellation and redaction |
| Application/auth | Every mutation/query with forbidden and cross-scope resources |
| Storage/runtime | Real disposable PostgreSQL, transaction races, leases/fencing, dedupe and restart recovery |
| External effects | Crash before send, after send, before commit; known-idempotent and unsafe unknown outcomes |
| Contracts | JSON schema/version round trips, stable errors, native/web client fixtures |
| Product UI | Keyboard outline, undo/redo, loading/error/permission states, publish versus activate, responsive navigation |
| Operations | Restore with keys/blobs, graceful shutdown, compatible migration/worker rollout and bounded load |

Unit tests use injected clocks, deterministic IDs and fixture data. Network tests
use local servers; no live credentials or external accounts in normal test runs.
A test that checks a starter arithmetic function is not runtime coverage.

Current workspace commands:

```sh
cargo check --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

Renderer validation is separate from default-feature workspace compilation.
Build/serve the gallery with `dx serve --package conctr-design-system --platform web`.
Then verify served selectors:

```sh
python3 apps/conctr-design-system/scripts/check-component-styles.py http://127.0.0.1:8080
```

CSS modules use path-derived selectors; preserve each stylesheet's source-path
marker when moving it. A successful build/file copy alone cannot prove selectors
match rendered markup. Use actual browser checks for geometry, focus and hover.
Native platform builds need their SDKs and explicit renderer features; do not
claim those were validated by a default `cargo test --workspace` run.
