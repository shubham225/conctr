# Data and storage

Status: **Proposed**. PostgreSQL is the recommended first durable server store;
no database adapter or migration currently exists.

## Persistence groups

| Group | Proposed records | Transaction boundary |
| --- | --- | --- |
| Identity and scope | organizations, workspaces, memberships, grants | Membership/policy changes with audit intent |
| Authoring | flows, draft revisions, layout revisions | Optimistic revision check and draft update |
| Artifacts | immutable versions and compiled plan metadata | Publication with unique version/checksum |
| Configuration | connection versions, credential metadata, environment bindings | Scoped references and permission checks |
| Activation | deployment revisions and active pointers | Swap active revision with audit event |
| Execution | runs, invocations, attempts, work items, leases | Admit input/run/work; complete attempt/next work |
| Reliability | dedupe keys, schedule occurrences, outbox/inbox | Atomic claim and idempotent state transition |
| Evidence | append-oriented audit events and redacted run events | Commit evidence intent with protected mutation |
| Payloads | metadata, bounded inline values, external blob references | Stage/verify blob, commit reference, garbage-collect orphans |

Names are provisional logical groups, not a generated migration specification.
Use workspace/organization scope in compound uniqueness/reference constraints
where applicable. A globally unique ID does not replace an ownership constraint.

## Store ports

Define repository and unit-of-work ports with the domain/use-case owner.
`conctr-storage` implements them and translates SQL errors into safe domain
outcomes. A single transaction boundary must be able to atomically persist
run/work/outbox state; separate repositories that each commit independently do
not satisfy this requirement. Keep SQL row structs and pool types out of UI and
core contracts.

A PostgreSQL work queue can begin with bounded batches of due rows claimed using
row locks and leases. [PostgreSQL documents SKIP LOCKED](https://www.postgresql.org/docs/current/sql-select.html)
for queue-like access, but it does not itself supply scheduling fairness, lease
renewal, retry safety or deduplication. Benchmark realistic contention and add
per-tenant limits before deciding to introduce a broker.

## Payload and secret storage

Keep large/binary bodies out of normal API/event rows. A payload reference records
scope, object ID, size, media type, checksum, retention class and access policy.
Bound previews and authorization checks. Shared workers need a shared blob adapter;
local filesystem storage is development/single-node only.

Store encrypted credential material separately from public metadata; keep keys
outside the database and repository. Encryption and backup/restore key lifecycle
are part of the design, not a claim that encrypting one column completes security.

## Evolution and deletion

Use ordered, reviewed migrations with forward/backward compatibility windows.
Expand then migrate then contract; do not remove columns needed by running older
workers. Test schema upgrades on realistic fixtures. Define retention for payloads,
run metadata, audit events and dedupe keys independently. Archive and tombstone
references before destructive cleanup. Test restore with keys and blobs, not only
a database dump. See [operations readiness](../operations/readiness.md).
