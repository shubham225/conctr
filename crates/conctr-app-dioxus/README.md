# Shared Conctr app

The web, desktop and mobile launchers share a responsive sidebar and session theme toggle.
Use the header control to collapse the sidebar or open the drawer on narrow screens.
Dashboard loads server build details from `GET /api/v1/system/details`.
The header breadcrumb shows the current page and links back to Dashboard from other
destinations on larger screens. The first keyboard stop skips to the page content.
The sidebar footer identifies the signed-in user and provides Profile,
Notifications and Log out. The shared sign-in screen protects direct navigation
to every product page. See [account setup and API security](../conctr-api-dioxus/README.md).

| Navigation group | Destinations |
| --- | --- |
| Workspace | Dashboard (`/`) |
| Connectivity | Credentials (`/credentials`), Connections (`/connections`), Endpoints (`/endpoints`) |
| Message transformation | Field mappings (`/mappings`), Transformers (`/transformers`) |
| Channels and routing | Channels and queues (`/queues`), Workflow pipeline (`/pipelines`) |
| Operations | History (`/runs`) |
| Administration | Settings (`/settings`), Help (`/help`) |
| User menu | Profile (`/profile`), Notifications (`/notifications`) |

Dashboard displays live system details. The other named destinations say they are
under development. The login URL redirects to Dashboard, where the session gate handles
access. See the [routes](src/routes.rs) and [app entry points](src/lib.rs).

Session-check failures appear in the shared toast viewport on the sign-in screen.

The sign-in screen chooses one integration illustration per client mount. The selection
stays in place during login, password setup, recovery and theme changes. Each
[scene](src/sign_in/scenes/) owns its SVG geometry and animation timeline. To add
a scene, add an SVG there and register its copy and artwork in
[the catalog](src/sign_in/scenes/catalog.rs). The shared player handles pause,
window visibility and the system reduced-motion preference.
