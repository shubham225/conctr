# Deployment and operations readiness

Status: **Proposed runbook requirements**. No production deployment, on-call
service, availability target or recovery objective has been implemented.

## Topology ladder

1. Local: one server process, disposable PostgreSQL and mock external endpoints;
   optional bounded filesystem blobs. No real credentials in fixtures.
2. Pilot: single authoritative control plane, managed backups/key storage, durable
   run/lease tables and tested restart recovery. Web and native clients use the
   same configured server; desktop is not an implicit production worker.
3. Scale: separate execution workers and shared blob store, admission limits and
   measured queue behavior. Add a broker only with evidence and outbox handling.
4. HA/SaaS: multi-instance admission/scheduling ownership, tenant isolation,
   database failover, compatible rollouts, capacity/abuse controls and tested DR.

## Operational contracts to implement

| Topic | Required evidence before pilot |
| --- | --- |
| Health | Separate process liveness from readiness to admit/claim work; failures visible without leaking secrets |
| Shutdown | Stop admission/claiming, cancel or finish bounded work, record/release leases safely |
| Telemetry | Correlated run/node/attempt logs, queue depth/age, failures, timeouts, rejected admission and lease expiry |
| Privacy | Redaction and payload retention apply to logs, traces, metrics labels and diagnostic bundles |
| Backup | Restore database, blobs and keys; verify references and a known run/version afterwards |
| Upgrade | Migration compatibility window and rollback plan; no older worker reads unsupported plans |
| Scheduling | Single durable occurrence ownership, clock drift/missed-fire/overlap policy |
| Recovery | Document stale leases, unknown effects, replay approval and dead-letter disposition |
| Capacity | Configured bounds for tenants, queues, payloads, requests and external connection pools |

Avoid tenant IDs, raw URLs or payload fields as unbounded metric labels. Traces
are useful but are not a substitute for durable run state. Choose measured pilot
SLOs, RPO and RTO in [open questions](../roadmap/open-questions.md); do not invent
numbers in product material.

## Incident workflow design

Locate correlation/run ID, determine scope, stop new admissions if needed,
classify confirmed failure versus unknown effect, inspect authorized redacted
evidence, choose reconciliation/replay, and record actor/reason/outcome. A restart
is not a safe universal retry strategy. Create executable recovery drills as the
runtime and storage adapters are implemented.
