# Screen blueprints

Status: **Proposed**. [Interactive prototype](prototype/index.html) uses synthetic
values and local-only actions. No screen here is a production implementation.

| Page | Layout and content | Actions / permissions | Empty/error behavior |
| --- | --- | --- | --- |
| Overview | Header with environment; concise run/connection summary; recent runs table and actionable issues | Create flow for editors; inspect for viewers | No invented analytics; show empty state, stale-data timestamp and scoped errors |
| Flow list/detail | Search, tags, deployment/version state, owner; detail tabs for overview/versions/runs | Create/edit/publish per action; delete/archive disclose references | Empty onboarding; preserve filters; inaccessible resource does not leak another workspace |
| Editor | Palette, canvas/outline, inspector and run drawer | Save draft, validate, publish; activate elsewhere | Node diagnostics, unsupported schema and revision conflicts preserve draft |
| Connections | Protocol shape/name, endpoint in mono, environment, credential metadata, recent health | Create/configure/test; test explicitly may call endpoint | Unknown/unconfigured distinct from failing; redact auth/URLs as required |
| Run list | Status/time/flow/environment filters; virtualized/paginated rows and stable cursor | Inspect/cancel/replay with permission and state constraints | Loading skeleton, no-results guidance, delayed event indication |
| Run inspector | Summary with pinned version; step timeline; attempts; safe payload/schema preview; correlation | Recovery action shows duplicate-effect risk; download only permitted redacted evidence | Unknown outcome distinct from failed; expired payload reference explicit |
| Credentials | Name, type, scope, last change and reference count; no secret values | Create/replace/revoke with audit; use permission is separate from read metadata | Connection references explained; permission-denied does not reveal a token |
| Environments/deployments | Binding table, version comparison, validation summary and activation history | Review/activate/rollback with permissions | Missing production credential blocks activation; rollback does not rewind existing runs |
| Settings/admin | Members, role grants, session/providers, retention and worker configuration as implemented | Narrow contextual save and audit evidence | No nonfunctional security switches; unavailable plans/features labeled rather than simulated |

## Layout recipes

Use shared sidebar/header and content containers. Forms use a single readable
column with grouped sections and action footer. Dense tables use 14px Inter,
mono IDs/endpoints and status badges with labels. Avoid a grid of generic cards
for every page; reserve cards for meaningful groups. Do not add operational stats
to the reusable core merely to fill a dashboard.

Connection/flow status is separate from last-run status. A paused deployment can
have a previously successful run. Use precise timestamps, environment labels
and version IDs to prevent ambiguity. No success chart should be populated with
fictional production metrics when an environment has no data.

## Design review acceptance

Review each page in light/dark, empty/loading/error/success/forbidden/offline or
stale states, keyboard-only navigation and narrow width. Confirm its primary
job/action, resource scope, data owner, recovery path and what can cause an
external side effect. Cross-check all controls against the kit and domain/API
contracts before implementing visual polish.
