# Workspace and crate map

The [workspace manifest](../../Cargo.toml) lists the package members.
`apps/` contains executable launchers and the component showcase. `crates/`
contains libraries shared by those programs. A library package can also ship a
small support binary, as authentication does for account provisioning.
`design/` owns the brand kit. Shared repository artwork lives in `assets/`.

## Implemented packages

| Location | Responsibility today |
| --- | --- |
| [apps/web-dioxus](../../apps/web-dioxus/) | Launch the shared web app and host its management API in the fullstack server. |
| [apps/desktop-dioxus](../../apps/desktop-dioxus/) | Launch the shared app with native window integration. |
| [apps/mobile-dioxus](../../apps/mobile-dioxus/) | Existing mobile launcher, planned for removal. |
| [apps/conctr-design-kit](../../apps/conctr-design-kit/) | Exercise reusable controls and themes with gallery examples. |
| [conctr-app-dioxus](../../crates/conctr-app-dioxus/) | Shared login UI, routes, navigation and Dashboard. Dashboard reads authenticated server details. Most destinations currently render a heading. |
| [conctr-ui-dioxus](../../crates/conctr-ui-dioxus/) | Reusable controls, compositions, themes, icons, fonts and bundled styles. |
| [conctr-api-dioxus](../../crates/conctr-api-dioxus/) | Dioxus client for the management HTTP API. Reuses the server's contract types. |
| [conctr-api](../../crates/conctr-api/) | Framework-independent management contracts and an optional Axum HTTP server. |
| [conctr-auth](../../crates/conctr-auth/) | Local accounts, password setup and recovery, session issuance and revocation. |

A working example crosses these boundaries on Dashboard. The page requests system
details through the Dioxus client. The HTTP handler checks the session and returns
a typed response. A second frontend could consume the same endpoint and supply
its own presentation state. See the [route definitions](../../crates/conctr-app-dioxus/src/routes.rs)
and [HTTP handler](../../crates/conctr-api/src/http/system.rs).

## Tentative responsibilities for the six scaffolds

Each package below currently contains starter code. These placements are
recommendations to revisit with the first working integration.

| Crate | Candidate responsibility | Example |
| --- | --- | --- |
| [conctr-core](../../crates/conctr-core/) | Domain values and business rules that can run without I/O | Validate that a mapping refers to a defined target field. |
| [conctr-engine](../../crates/conctr-engine/) | Coordinate a running integration over time | Invoke a translation step, dispatch its output and record the outcome. |
| [conctr-connectors](../../crates/conctr-connectors/) | Adapt a particular external system or protocol | Send the translated order using the partner's API and interpret its reply. |
| [conctr-gateway](../../crates/conctr-gateway/) | Expose integration endpoints to external systems | Accept an incoming order or webhook according to the sender's wire contract. |
| [conctr-transform](../../crates/conctr-transform/) | Convert a message's structure or representation | Translate JSON order lines into the partner's XML format. |
| [conctr-storage](../../crates/conctr-storage/) | Implement persistence required by a caller | Save and retrieve an integration definition using the selected store. |

Keep definitions and pure rules in core. Extract engine logic when a use case
has to coordinate steps, time or external effects. A small translation can begin
as a pure module in its owning crate. A separate transform library earns its
place when several integrations need the same evaluator or conversion tests.
Select a persistence interface around the caller's operations when they exist.

Defer `conctr-common`. A type with domain meaning belongs beside its rules,
and an API response belongs with its wire contract. Share an existing type from
its owner first. Consider extracting a narrowly named crate when independent
consumers repeatedly need the same stable functionality.
