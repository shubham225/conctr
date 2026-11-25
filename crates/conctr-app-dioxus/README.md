# Shared Conctr app

The web, desktop and mobile launchers share a responsive sidebar and session theme toggle.
Dashboard loads server build details from `GET /api/v1/system/details`.
The header breadcrumb links Workspace to Dashboard and shows the current page.
On small screens, it shows only the current page. The sidebar footer identifies the signed-in user and provides Profile,
Notifications and Log out. The shared sign-in screen protects direct navigation
to every product page. See [account setup and API security](../conctr-api-dioxus/README.md).

| Navigation group | Destinations |
| --- | --- |
| Workspace | Dashboard (`/`) |
| Connectivity | Credentials (`/credentials`), Connections (`/connections`), Endpoints (`/endpoints`) |
| Message Transformation | Field Mappings (`/mappings`), Transformers (`/transformers`) |
| Channels & Routing | Channels/Queues (`/queues`), Workflow Pipeline (`/pipelines`) |
| Operations | History (`/runs`) |
| Administration | Settings (`/settings`), Help (`/help`) |
| User menu | Profile (`/profile`), Notifications (`/notifications`) |

Dashboard displays live system details. The other named destinations currently render
headings. The login URL redirects to Dashboard, where the session gate handles
access. See the [routes](src/routes.rs) and [app entry points](src/lib.rs).
