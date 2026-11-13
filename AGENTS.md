# Conctr development instructions

Conctr is an enterprise integration platform inspired by Apache Camel's integration
patterns and n8n's visual authoring. Its platform core is independent of UI
frameworks; frontends are replaceable adapters. Most backend packages and the shared
product app are scaffolds. A package name or showcase example is not a working feature.

## Start here

1. Read [shared AI context](docs/ai/AI-INSTRUCTIONS.md), then only the contracts
   relevant to the task. Use [docs/ai/README.md](docs/ai/README.md) for the instruction
   and portable-skill index.
2. Inspect existing source/manifests before introducing abstractions. Consult the
   [crate map](docs/architecture/crate-map.md) for ownership and
   [current-state evidence](docs/roadmap/current-state.md) before reporting progress.
   Verify dated observations against source.
3. Before editing, read any more specific `AGENTS.md` in the target file's ancestor
   directories, even if the current AI tool does not load nested instructions.
4. Follow explicit user instructions and the authorized task scope. Proposed ADRs
   and open planning questions do not require implementing every proposal or asking
   permission for routine choices. Explain assumptions and resolve blocking gaps.

## Ownership and dependency direction

- `apps/`: runnable launchers and platform adapters; avoid duplicating product pages.
- `crates/`: reusable platform libraries and framework-specific adapters.
- `docs/`: product plans, contracts, ADRs, implementation evidence and AI context.
- `design/`: canonical brand, visual tokens, component specs and HTML reference kit.
- `assets/`: shared assets; app-specific assets remain with the owning app.

| Existing package | Responsibility |
| --- | --- |
| `conctr-core` | Shared identifiers and foundational values |
| `conctr-auth` | Authentication and authorization policy |
| `conctr-connectors` | External integration descriptors, operations and adapters |
| `conctr-engine` | Execution, run/attempt state, recovery and runtime ports |
| `conctr-transform` | Bounded data transformation and mapping |
| `conctr-storage` | Persistence adapters implementing owner-defined ports |
| `conctr-gateway` | HTTP/API boundary and webhook ingress |
| `conctr-api` | Server composition root; currently under `crates/conctr-api` |
| `conctr-api-dioxus` | Dioxus-specific server-function adapter; this is the actual Cargo name |
| `conctr-ui-dioxus` | Reusable layout primitives, UI controls and generic compositions |
| `conctr-app-dioxus` | Shared product App, routes, pages and presentation state; currently scaffolded |

These are ownership boundaries, not claims of implementation. Add proposed crates
only when the selected feature needs their boundary. Prefer focused modules first.

Keep dependencies directed toward the core. Domain/use-case contracts must not
import Dioxus, frontends, concrete storage or HTTP server implementations. Keep
business rules outside UI and transport adapters. Storage implements ports owned
by the relevant application/runtime boundary; do not introduce dependency cycles.
Read the crate map before changing these relationships or adding dependencies.

## Platform invariants

- Executable flow semantics are independent of canvas coordinates and UI graph libraries.
- Runs pin immutable flow versions and binding/operation snapshots. Draft edits,
  promotion and rollback do not rewrite in-flight work.
- Authorization and resource scope come from authenticated server context, including
  queries, credentials, blobs and subscriptions; navigation visibility is insufficient.
- Keep external credentials separate from platform identity. Do not put secret
  material in flows, fixtures, exports, traces, logs or UI previews.
- Unknown external writes require reconciliation or attention. Do not promise generic
  exactly-once effects or blindly retry unsafe operations. Read the execution,
  connector and security contracts before changing retries or external effects.

## Rust and Dioxus

Write clear, idiomatic Rust with strong types, explicit `Result`/`Option` handling,
focused functions and documented public APIs. Avoid unnecessary `unwrap`, `expect`,
cloning, allocations, abstractions and dependencies. Preserve existing user changes.
Do not rename or move files without a task-based reason.

Use the workspace Dioxus **0.7.1 version requirement** through `dioxus = { workspace = true }`;
the lockfile may resolve a compatible 0.7 patch. Enable only required renderer,
router or server features. Do not enable every renderer by default. Use the
[official Dioxus 0.7 documentation](https://dioxuslabs.com/learn/0.7) for Dioxus work.

Use `#[component]`, `Element`, `rsx!`, signals, current hooks and `Routable` routes.
Do not use `cx`, `Scope` or `use_state`. Keep props owned and `Clone`/`PartialEq`
where required. Use `asset!` for bundled assets and `document::Stylesheet` for CSS.
Use `use_resource` for component async work, `use_server_future` when SSR requires
consistent initial data, and `use_effect` for browser-only side effects. Initial
server/client markup must agree. Dioxus server functions adapt authorized services;
generic API contracts remain framework-independent.

## UI and design

Read [design instructions](design/docs/AI-INSTRUCTIONS.md),
[presentation ownership](docs/architecture/dioxus-presentation.md) and, for CSS,
[theming](docs/development/theming.md). Reuse the UI library before adding markup
or CSS. Preserve the `primitives → ui → composed` dependency direction: composed
components may depend on the lower layers; lower layers do not import compositions.

Shared recipes and semantic `--cn-*` properties belong in the UI library's
`assets/styles/main.css`; adjacent CSS modules retain structure and primitive
behavior. Preserve source-path markers and explicit stylesheet registration.
Do not target generated class hashes, duplicate shared recipes or introduce new
numbered palette aliases. Use the supplied assets, type/spacing scales and protocol
meanings; cover light/dark, keyboard focus, disabled/error states and reduced motion.

## Verification and documentation

Test business logic in its owning crate and external boundaries with deterministic
local mocks/fixtures. Run checks appropriate to the change; documentation-only
changes need documentation validation, not a native renderer build.

```sh
cargo check --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
python3 docs/development/scripts/check_docs.py
```

For UI changes, serve the gallery and check actual browser behavior and served CSS:

```sh
dx serve --package conctr-design-system --platform web
python3 apps/conctr-design-system/scripts/check-component-styles.py http://127.0.0.1:8080
```

Use the actual server URL/port. Product launchers are `conctr-web-dioxus`,
`conctr-desktop-dioxus` and `conctr-mobile-dioxus`. Default workspace checks do not
prove native SDK builds, browser geometry or external integrations work.

Update affected public API docs and current-state/checklist evidence when behavior
changes. Record significant architecture decisions in `docs/architecture/decisions/`;
keep Proposed and Accepted statuses distinct. Report what changed, checks actually
run and material limitations. Keep explanations and examples concise. Never invent
supported capabilities, customers, performance numbers or certifications.
