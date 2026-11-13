# User journeys and acceptance scenarios

Status: **Proposed**. Examples use fictional systems and synthetic data.

## Author a connection and first flow

1. Select a workspace and environment; create an HTTP connection with a scoped
   credential reference. The server validates endpoint policy and permission.
2. Create a draft flow, select a manual trigger and add source/map/target nodes.
3. Configure operations from connector schemas; validate the graph and bindings.
4. Run against local fixtures first. Preview only bounded, redacted payloads.
5. Publish an immutable version and activate it in development. Publishing alone
   does not enable production traffic.

Acceptance: invalid ports, missing fields or inaccessible credentials block
activation with a node-specific explanation. Canceling the editor does not alter
an existing deployment. Test runs are labeled and require permission for any real
external action; a preview does not imply that external calls are harmless.

## Diagnose an incident

Open a failed run from the activity view, identify its pinned version and failing
attempt, inspect the redacted error classification, and choose a permitted
retry/replay or disposition. Unknown external outcomes receive a distinct warning
and cannot be silently retried under an unsafe policy.

Acceptance: no raw token appears in list, detail, export or logs. A replay creates
a new linked run, records the actor and warns about possible duplicate writes.

## Promote a release

Validate an immutable version in development, bind production connections and
credentials separately, review the resulting plan, and activate it under the
required permission. Record old/new version and binding IDs in audit history.
Rollback activates an earlier compatible version for future triggers; existing
runs retain their original snapshots and are not rewound.

## Mobile operator

Open a run notification/deep link, verify workspace and environment, inspect
status and approve only supported scoped actions. Full canvas editing and bulk
credential changes are desktop/web authoring tasks in the initial design.
