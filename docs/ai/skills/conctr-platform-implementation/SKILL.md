---
name: conctr-platform-implementation
description: Implement or review Conctr flow, execution, connector, API, credential, and storage features with the repository's crate ownership and durable recovery contracts. Use for Conctr platform work, not generic Rust tasks or standalone visual styling.
---

# Conctr platform implementation

Find the repository root from Cargo.toml and docs/. Read `docs/ai/AI-INSTRUCTIONS.md`,
`docs/roadmap/current-state.md` and `docs/architecture/crate-map.md`. Source wins
when the dated inventory is stale; update evidence rather than repeating a plan
as implemented behavior.

Choose the owning boundary before editing. For graph/compiler work read
`docs/architecture/flow-definition.md`; for execution read `execution-model.md`;
for external operations read `connector-contract.md` and `security.md`; for
transport/persistence read `api-and-events.md` and `data-and-storage.md` in the
same architecture folder. Read only relevant ADRs. A Proposed ADR is a starting
recommendation, not a reason to block an otherwise authorized task.

Keep the change a coherent vertical slice. Preserve framework independence,
owner-defined ports and dependency direction. Do not create every proposed crate
or turn a feature task into a microservice migration. Keep credentials out of
fixtures and serialized flows; scope authorization on the server. Treat unsafe
unknown external effects separately from retryable failures.

Validate observable behavior: invalid graph fixtures, authorization/isolation,
transaction/recovery races or mock connector behavior as applicable. Prefer
injected time and local fixtures over live services. Report implementation,
checks actually run and remaining limitations; update the roadmap evidence for
completed capabilities only. Significant changed decisions belong in an ADR.
