# Web, desktop, mobile and showcase

The table describes proposed product capabilities. Today the three product launchers
share a routed application, and the showcase documents the reusable UI.

| App | Primary job | Initial capability | Native-specific additions to consider |
| --- | --- | --- | --- |
| Web | Full authoring and administration | Connections, flow editor, runs, credentials, deployments and scoped settings | Browser session, deep links, downloads and SSR/hydration |
| Desktop | Full authoring against a configured server | Same routes/data/permissions as web; responsive shell | Window/menu shortcuts, server profiles, safe import/export and OS keychain for client session material |
| Mobile | Operate and inspect integrations | Overview, searchable runs, run details, connection health and permitted cancel/replay | Secure native session handling, deep links and later opt-in notifications |
| Design system | Document reusable UI | Component catalog, foundations, source examples and themes | Development tooling only; not a tenant administration app |

Desktop is not automatically an offline runtime. Client session material is not
an export of server-side connector credentials. A future local-worker product
needs an explicit deployment, credential, egress and recovery decision.

## Shared structure

The launchers mount the shared app from `conctr-app-dioxus`. It owns the routes,
workspace navigation, mapping and pipeline previews, queue inspection and
credential directory page. Desktop supplies its own window controls. Renderer
configuration belongs in each launcher.

## Screen behavior by width

Wide: 240px sidebar/icon rail, shared page header and resizable editor inspector.
Medium: compact navigation and optional inspector drawer; tables can scroll within
an accessible frame. Narrow: navigation drawer/bottom task navigation, stacked
run summaries and full-height detail sheets. Keep workspace/environment visible.
Do not compress the full editor into an unusable phone canvas; initially show a
read-only flow outline and direct users to web/desktop for authoring.

## Native actions and permission checks

A desktop keyboard shortcut invokes the same authorized command as its button.
Import/export uses validated contracts and never exports secrets. Mobile cancel
and replay require the same permissions and duplicate-effect warnings as web.
Unavailable features need an explanation; platform UI visibility must not replace
backend policy. Cross-app deep links re-check authentication and resource scope.
