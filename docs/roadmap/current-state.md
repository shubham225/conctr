# Current implementation evidence

Audit date: **2026-11-14**. Source/manifests and the working tree were inspected;
these observations do not assert that changes have been committed or released.

## Delivered foundation

- [x] Cargo workspace with four runnable Dioxus application packages, platform
  library scaffolds, a server-binary scaffold and a shared-app scaffold.
- [x] Framework-independent-core decision and component-layer decision recorded.
- [x] Shared `conctr-ui-dioxus` components, fonts, tokens and supplied brand assets.
- [x] Physical `components/primitives`, `components/ui`, `components/composed`
  organization with compatibility imports and adjacent component CSS.
- [x] `conctr-design-system` catalog: 66 live example entries, foundation pages,
  light/dark switching and source examples. Samples are illustrative.
- [x] Shared sidebar collapse/mobile-drawer behavior and component styling fixes.
- [x] Library-owned Tailwind 4.1.18 utilities, shadcn-style aliases backed by
  `tokens.css`, and selectively loaded adjacent CSS modules. Consumers mount
  `ThemeProvider`/`ConctrStyles` without Tailwind config, tooling or copied assets.
  Library development tools refresh one shipped utility bundle during Cargo
  rebuilds; there are no per-component generated files or global registry. See the
  [theming guide](../development/theming.md).
- [x] File uploader, nested resizable panels, Kbd/KbdGroup and inline/dialog command
  menus, with gallery examples. Upload transport remains application-owned.
- [x] Refined sidebar rail alignment, calendar/field focus states, slider colors,
  empty-state spacing and semantic `Alert` status variants.
- [x] Component/SSR/catalog tests plus a served CSS-selector regression checker.
- [x] Consumer setup and the Dioxus/Manganis 0.7.10 SSR workaround documented;
  obsolete numbered palette stylesheet/export and copied Tailwind reset removed. See
  [component usage](../../crates/conctr-ui-dioxus/README.md#usage) and
  [SSR details](../development/theming.md#ssr-css-module-workaround).
- [x] `conctr-app-dioxus` is now a workspace member. Its implementation is still
  the starter `add()` function/test; it has no shared App/routes yet.

Evidence: [workspace manifest](../../Cargo.toml),
[UI source](../../crates/conctr-ui-dioxus/src/lib.rs),
[catalog](../../apps/conctr-design-system/src/catalog.rs),
[component tests](../../crates/conctr-ui-dioxus/tests/design_system.rs),
[stylesheet check](../../apps/conctr-design-system/scripts/check-component-styles.py).
UI migration verification: workspace tests and strict Clippy passed, with formatting,
documentation validation and utility-bundle freshness checks. A new library
utility compiled during a normal Cargo rebuild without a CSS command; restoring
the source reproduced the verified bundle. An isolated copied
library consumer, with no Node modules, Tailwind config or app CSS, rendered styled
components and registered its assets in successive SSR documents. The README
quickstart also compiles unchanged as an independent consumer package. The gallery
serves bundled Tailwind 4.1.18 and scoped CSS modules; the served-style checker
covers all 66 pages. Thirty-nine browser checks cover nested light/dark/System scopes,
provider tokens and density, consumer customization, focus/error/disabled states,
keyboard menus, dialogs, calendar selection, reduced motion, responsive sidebar
and flow layout, virtual-list scrolling, nested pointer/keyboard panel resizing,
command filtering/dialog focus, native file browsing, file validation/removal and
drops, dark palettes and narrow layouts for the new controls. These checks do not establish native
SDK support or backend functionality; scaffold tests remain scaffold evidence.

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
