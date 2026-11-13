# ADR 0007: PostgreSQL first with a durable work queue

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

Authoring versions, deployment bindings, run state and work scheduling require transactionally consistent persistence. The current storage crate has no implementation.

## Recommended decision

Use PostgreSQL as the first durable server store and evaluate bounded claimed work rows with leases. Store large payloads through a blob port. Keep state transitions and next work/outbox records atomic. Add Redis/brokers only for a measured requirement and with replay/reconciliation semantics.

## Alternatives

SQLite may suit a future offline/single-user mode but is not the initial multi-worker contract. A mandatory broker or full event-sourced/CQRS stack adds complexity before a vertical slice. An external durable workflow service remains an explicit future alternative.

## Consequences and review criteria

Database contention, retention, backup, blob consistency and fencing still need tests. A row-lock primitive is not a complete scheduler. Select supported database versions at implementation time and define retention/RPO/RTO before a production pilot.

Related review questions: Q03, Q05, Q07 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../data-and-storage.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
