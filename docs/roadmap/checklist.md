# Delivery checklist

Check only completed evidence. Proposed design decisions are not acceptance or
implementation. See [current state](current-state.md) for the dated source audit.

## Foundation already present

- [x] Workspace and platform package scaffolds.
- [x] Canonical design kit with fonts/tokens/branding.
- [x] Shared component library and layered source ownership.
- [x] Showcase catalog, source previews and theme switching.
- [x] Component tests and served stylesheet regression check.
- [x] Shared product-app package scaffold exists.
- [x] Product/architecture/experience/roadmap/AI documentation package drafted.
- [ ] Maintainer review and acceptance of the new product scope/ADRs.

## M1 — Contracts and shell

- [ ] Choose deployment/tenancy/mobile boundaries in Q01–Q03.
- [ ] Replace placeholder core helpers with scoped, strong identifiers/errors.
- [ ] Create `conctr-flow` with versioned definitions and pure graph diagnostics.
- [ ] Specify normalization, schema/version migration and fixture round trips.
- [ ] Create `conctr-application` and define scoped use cases/transaction ports.
- [ ] Create `conctr-contracts` for the first API; avoid leaking SQL/Dioxus types.
- [ ] Implement shared App, providers, Routable enum and feature page structure.
- [ ] Wire web/desktop/mobile launchers to the shared App and test renderer features.

## M2 — Reliable execution slice

- [ ] Implement PostgreSQL migrations, repository/unit-of-work and claim/lease ports.
- [ ] Add secrets boundary, approved key handling and write-only credential API.
- [ ] Implement HTTP connector descriptor, operation, egress and response limits.
- [ ] Implement bounded mapping/validation with precise missing/null behavior.
- [ ] Pin version/bindings/operation versions in admitted runs.
- [ ] Persist attempt intent/result/continuation; test all IO/commit crash windows.
- [ ] Implement bounded retry/deadline/cancel and unknown-effect state.
- [ ] Test cross-workspace access, redaction, idempotency and duplicate admission.
- [ ] Run the complete local synthetic integration demo.

## M3 — Authoring experience

- [ ] Build product connection list/detail and credential references.
- [ ] Implement command-based graph editing, undo/redo and separate layout persistence.
- [ ] Add schema-driven inspector, diagnostics and keyboard-accessible outline.
- [ ] Distinguish fixture preview from a real side-effecting test run.
- [ ] Publish immutable versions separately from activation.
- [ ] Build run list/detail with attempts, redacted payload preview and clear replay semantics.

## M4 — Controlled MVP

- [ ] Durable authenticated webhook admission, rate/body/signature/replay policy.
- [ ] Durable schedule occurrence/dedupe, timezone/missed-fire/overlap behavior.
- [ ] Environment binding validation and atomic deployment activation.
- [ ] Authorized replay/cancellation with actor/reason evidence.
- [ ] Retention/cleanup jobs, bounded queue/admission and health endpoints.
- [ ] Browser/native auth integration and mobile operations scope verified.

## M5/M6 — Enterprise and scale gates

- [ ] OIDC mapping, action/resource permissions and service identity lifecycle.
- [ ] Credential rotation/key restore and external vault plan.
- [ ] Audit export/access/retention controls and protected release workflow.
- [ ] Logs/traces/metrics with no secret or high-cardinality leakage.
- [ ] Backup/restore and migration/rollback drills with documented pilot targets.
- [ ] Independent security review before shared SaaS or untrusted extensions.
- [ ] Separate worker only when justified; test leases, fairness and compatible rollout.
- [ ] Add each protocol/pattern with conformance and recovery tests.
- [ ] Review licensing and compatibility before third-party code reuse/distribution.
