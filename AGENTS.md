# Conctr development instructions

Conctr is an enterprise integration platform inspired by Apache Camel's integration
patterns and n8n's visual authoring. Its platform core is independent of UI
frameworks; frontends are replaceable adapters. Most backend packages are scaffolds; the shared product app provides navigation and
heading-only pages. A package name or showcase example is not a working feature.

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
| `conctr-app-dioxus` | Shared product App, routes, pages and presentation state |

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

## TOML formatting

Align assignment operators within each TOML table and use a space after each
`=`. Use inline tables with interior spacing for workspace inheritance,
as in `version = { workspace = true }`. Keep short arrays on one line and
format multiline arrays with four-space indentation, one item per line and
a trailing comma. See the [workspace guide](docs/development/workspace.md).

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
or CSS. Preserve the `upstream primitives → ui facade → composed` dependency direction: composed
components may depend on the lower layers; lower layers do not import compositions.

Use Tailwind 4.1 utility classes directly in component Rust. All Tailwind tooling,
compiler inputs and the bundled utility CSS belong to `conctr-ui-dioxus`; consuming
apps need only `ThemeProvider` or `ConctrStyles`. Keep consumer scan paths in the UI compiler; do not add app Tailwind configs
or generated assets. Library development dependencies refresh the
bundle during Cargo rebuilds; consumers use the shipped CSS without Node.
Keep palettes and design scales in `assets/colors/tokens.css`, semantic shadcn
aliases/shared interactions in `assets/styles/main.css`, and Tailwind compiler
mappings in `theme.css`. Put component-specific styling in `style.css` beside the owning component.
Use existing Tailwind utilities in Rust for shared layout and keep new component-specific selectors out of the shared Tailwind bundle. Keep gallery-only styles in the showcase. Adjacent `style.css` files use CSS modules and explicit
per-component `document::Stylesheet` registration for repeated SSR documents.
Keep ownership comments accurate; no global stylesheet registry or per-component
generated CSS. Preserve supplied assets, type/spacing scales, protocol meanings,
light/dark themes, keyboard focus, disabled/error states and reduced motion.
Reuse shared styles and avoid numbered palette aliases.

Add each shared application icon as matching color and monochrome SVGs under
`crates/conctr-ui-dioxus/assets/icons/{color,mono}/` in its family folder.
Follow the design kit, keep the icon inventory and design assets synchronized,
and export named handles through `conctr_ui_dioxus::assets::icons`. Reuse
those handles in application UI and include both editions in the showcase
icon library. Use monochrome for themed controls and color for brand artwork.
Verify the icon against both light and dark backgrounds.

## Local verification on memory-constrained workstations

- Start with the affected package and the smallest check that exercises the change.
  Run memory-intensive steps sequentially. On this 14 GiB workstation, start Rust
  builds with `CARGO_BUILD_JOBS=1`, including builds launched by `dx`.
- Reuse a healthy local server when possible. Run one Dioxus server and one
  dedicated verification browser at a time. Stop processes started for the task
  after verification or before switching apps, including failed server attempts.
  Preserve user-owned development sessions.
- If the desktop stalls or swap use grows while available memory falls, stop the
  agent-owned workload and inspect `free -h` and `/proc/pressure/memory` before
  retrying. After a restart, inspect the previous boot journal. Report deferred
  browser checks when rebuilding would put the session under pressure again.

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
dx serve --package conctr-design-kit --platform web
node apps/conctr-design-kit/scripts/check-browser.mjs http://127.0.0.1:8080 http://127.0.0.1:9225
```

Browser scripts require Node 22+ and Chrome started with
`--remote-debugging-port=9225`. Use the actual server URL/port. For product UI,
serve `conctr-web-dioxus` and verify navigation against
[the app route inventory](crates/conctr-app-dioxus/README.md). Product launchers are `conctr-web-dioxus`,
`conctr-desktop-dioxus` and `conctr-mobile-dioxus`. Default workspace checks do not
prove native SDK builds, browser geometry or external integrations work.

Update affected public API docs and current-state/checklist evidence when behavior
changes. Record significant architecture decisions in `docs/architecture/decisions/`;
keep Proposed and Accepted statuses distinct. Report what changed, checks actually
run and material limitations. Keep explanations and examples concise. Never invent
supported capabilities, customers, performance numbers or certifications.
