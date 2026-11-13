# ADR 0005: Durable execution with explicit unknown outcomes

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

External IO and local persistence cannot generally be committed atomically. A crashed worker may have performed a write that it did not record.

## Recommended decision

Persist run admission, attempt intent and continuation. Use bounded at-least-once processing, leases/fencing for store writes and operation-level idempotency. Unsafe unknown effects require attention/reconciliation, not unconditional automatic retry. Replay creates a new linked run.

## Alternatives

An in-memory task queue loses admitted work on restart. Claiming global exactly-once effects is not supported by generic external APIs. Adding a broker alone does not fix the side-effect/persistence gap.

## Consequences and review criteria

Recovery and cancellation are real state-machine features. Every IO/commit boundary needs crash tests. Choose the pilot retry, idempotency-window and recovery limits explicitly.

Related review questions: Q05, Q07, Q10 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../execution-model.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
