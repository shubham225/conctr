# Information architecture and routes

Status: **Proposed**. All product routes below are unimplemented examples.

## Shell

Organization/workspace selector at the top of a collapsible navigation rail;
contextual header shows breadcrumb, environment and principal/session controls.
Navigation groups: Build (Flows, Connections), Operate (Overview, Runs), Manage
(Credentials, Environments, Settings). Respect the kit's maximum top-level item
guidance by grouping secondary items inside their parent/settings area.

The current sidebar component is the reusable foundation. Keep header dividers
aligned, icon-only controls named and tooltips anchored. Shell layout belongs in
`conctr-app-dioxus/layouts`, not in every page or launcher.

| Route sketch | Page | Primary action / feature owner |
| --- | --- | --- |
| `/w/:workspace/overview` | Environment summary and recent runs | Create flow; overview |
| `/w/:workspace/flows` | Search/filter/version/deployment summary | Create flow; flows |
| `/w/:workspace/flows/:flow` | Overview, versions and run history | Edit draft; flows |
| `/w/:workspace/flows/:flow/editor` | Canvas/outline and inspector | Save draft or publish by context; flows |
| `/w/:workspace/connections` | Endpoints and credential references | Create connection; connections |
| `/w/:workspace/connections/:connection` | Configuration, bindings and test history | Test connection with explicit scope; connections |
| `/w/:workspace/runs` | Run search/status/time/environment filters | Inspect run; runs |
| `/w/:workspace/runs/:run` | Timeline/node attempts and payload preview | Contextual safe recovery action; runs |
| `/w/:workspace/credentials` | Metadata and permitted credential actions | Create credential; credentials |
| `/w/:workspace/environments` | Bindings and deployment history | Review activation; deployments |
| `/w/:workspace/settings` | Members, policy, retention and integration settings | Contextual save; settings |

Workspace and environment context must survive navigation, search and deep links;
a page never silently switches production context. URLs carry resource IDs, not
secret values. Avoid storing sensitive payloads or tokens in search parameters.

Choose route layouts for authenticated shell and sign-in/error pages. Feature
modules own their page components and view state. API/server authorization remains
mandatory even if a route is not linked in navigation.
