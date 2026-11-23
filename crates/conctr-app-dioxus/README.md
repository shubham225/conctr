# Shared Conctr app

The web, desktop and mobile launchers share a responsive sidebar and session theme toggle.
The header breadcrumb links Workspace to Dashboard and shows the current page.
On small screens, it shows only the current page. The sidebar footer uses the Studio
demo identity with a DiceBear avatar. Its menu opens beside the desktop rail or above
the mobile trigger and links to Profile and Notifications. Log out is disabled while
the shell has no authenticated session.

| Navigation group | Destinations |
| --- | --- |
| Workspace | Dashboard (`/`) |
| Connectivity | Credentials (`/credentials`), Connections (`/connections`), Endpoints (`/endpoints`) |
| Message Transformation | Field Mappings (`/mappings`), Transformers (`/transformers`) |
| Channels & Routing | Channels/Queues (`/queues`), Workflow Pipeline (`/pipelines`) |
| Operations | History (`/runs`) |
| Administration | Settings (`/settings`), Help (`/help`) |

The [pre-shell archive](../../backups/conctr-app-dioxus-before-shell.tar.gz) contains
the previous app and its studio browser scripts. Those scripts target the earlier pages.
