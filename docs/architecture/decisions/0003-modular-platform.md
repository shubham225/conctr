# ADR 0003: Modular platform before distributed services

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

The workspace has clear names but almost no platform implementation. Splitting every responsibility into a deployed service now would create operational work before establishing contracts.

## Recommended decision

Start with a modular server and PostgreSQL. Introduce conctr-flow, conctr-application and conctr-contracts for the first real vertical slice, and conctr-secrets before real credentials. Keep execution replaceable behind engine-owned ports. Extract a worker binary only when isolation/lifecycle/load requires it.

## Alternatives

A microservice per crate was rejected as the initial default. Putting everything in the UI or one shared common crate would obscure ownership. Reusing an existing workflow engine is an alternative that needs a separate operational, semantic and licensing evaluation.

## Consequences and review criteria

The server is initially simpler to operate, but boundaries must still be tested. More Cargo packages do not automatically create isolation. Review the self-hosted-first assumption and whether independent workers are needed for the pilot.

Related review questions: Q01, Q07 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../crate-map.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
