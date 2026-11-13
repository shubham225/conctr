# Current implementation evidence

Audit date: **2026-11-13**. Source/manifests and the working tree were inspected;
these observations do not assert that changes have been committed or released.

## Delivered foundation

- [x] Cargo workspace with four runnable Dioxus application packages, platform
  library scaffolds, a server-binary scaffold and a shared-app scaffold.
- [x] Framework-independent-core decision and component-layer decision recorded.
- [x] Shared `conctr-ui-dioxus` components, fonts, tokens and supplied brand assets.
- [x] Physical `components/primitives`, `components/ui`, `components/composed`
  organization with compatibility imports and adjacent component CSS.
- [x] `conctr-design-system` catalog: 62 live example entries, foundation pages,
  light/dark switching and source examples. Samples are illustrative.
- [x] Shared sidebar collapse/mobile-drawer behavior and component styling fixes.
- [x] Component/SSR/catalog tests plus a served CSS-selector regression checker.
- [x] `conctr-app-dioxus` is now a workspace member. Its implementation is still
  the starter `add()` function/test; it has no shared App/routes yet.

Evidence: [workspace manifest](../../Cargo.toml),
[UI source](../../crates/conctr-ui-dioxus/src/lib.rs),
[catalog](../../apps/conctr-design-system/src/catalog.rs),
[component tests](../../crates/conctr-ui-dioxus/tests/design_system.rs),
[stylesheet check](../../apps/conctr-design-system/scripts/check-component-styles.py).
`cargo test --workspace --offline` passed during this documentation audit. Starter
arithmetic tests are scaffold checks, not integration-runtime coverage. Prior UI
browser checks are useful component evidence, not a native application release test.

## Scaffold or demo only

| Item | Evidence | Not implemented yet |
| --- | --- | --- |
| core/auth/connectors/engine/transform/storage/gateway | Starter `add()` functions and tests | Their intended business/runtime/storage behavior |
| `conctr-api` | Hello-world `main.rs` | HTTP server, dependency wiring and health endpoints |
| `conctr-api-dioxus` | Echo server function | Authorized product use cases and real API contracts |
| web/desktop/mobile | Separate placeholder root components | Shared product routes/pages and native auth/backend integration |
| protocol/auth icons | Supplied visual artwork | Production protocol clients or credential implementations |

## No implementation evidence yet

- [ ] Canonical flow manifest/parser/compiler and version lifecycle.
- [ ] Real HTTP connector, mapping runtime or persisted flow execution.
- [ ] Database schema/migrations, worker claims, leases, durable retries/recovery.
- [ ] Authentication/authorization, tenant isolation, encrypted credential storage.
- [ ] Product flow editor, connection management, run inspector or deployment UI.
- [ ] Production telemetry, backup/restore, deployment automation or release hardening.

The new planning documents, provisional schemas and wireframes are documentation
artifacts. They do not complete any backend/product feature above.
