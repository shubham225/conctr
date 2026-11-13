# ADR 0009: Trusted connectors and bounded declarative mapping first

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

An extensible platform can unintentionally become an arbitrary-code execution service before it has isolation and resource controls.

## Recommended decision

Start with trusted compiled connector modules, versioned operation descriptors and a bounded mapping evaluator. Pin versions, classify retry safety and test cancellation/redaction/egress. Do not expose arbitrary code nodes or dynamic Rust plugins in the MVP.

## Alternatives

A JavaScript/Python marketplace or direct dynamic Rust ABI requires a separate sandbox/compatibility plan. Hard-coding all connector forms in the frontend blocks consistent API validation.

## Consequences and review criteria

Initial catalog breadth is intentionally limited. Rich expressions, XML mapping and community extensions need explicit semantics and dependency/license review. Extract per-connector crates only when their ownership or dependencies justify it.

Related review questions: Q04, Q08, Q09 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../connector-contract.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
