# Dependency-ordered milestones

Status: **Proposed**, with no calendar estimates. Each milestone needs reviewable
source, deterministic tests and an explicit demo; split work into small changes.

| Phase | Outcome | Dependencies | Exit evidence |
| --- | --- | --- | --- |
| M0 — UI foundation | Reusable design system and workspace | Existing work | Current-state evidence; keep regression checks healthy |
| M1 — Contracts and shared app shell | Core IDs, flow schema/validation, use-case/API boundaries, App/routes wired once | Review ADRs 0003–0009 | Invalid graph fixtures rejected; all launchers reference shared App; no domain Dioxus dependencies |
| M2 — First executable slice | Manual HTTP → map → HTTP run, persisted input/attempt/result, scoped credentials and recovery | M1 | Mock-server demo, unauthorized cases, crash injection, unknown-write handling and secret-redaction tests |
| M3 — Visual authoring | Palette/canvas/inspector, accessible outline, validation, draft/publish/run inspection | M2 | A user creates/tests/publishes the slice without editing source; round-trip manifest/layout fixtures |
| M4 — Controlled MVP | Authenticated async webhooks, durable schedule, deployment bindings, replay/cancel, run operations | M2/M3 | Restart-safe admission, trigger dedupe, permission/retention tests and a pilot runbook |
| M5 — Enterprise pilot | OIDC, stronger administration/audit, protected environments, rotation, telemetry and restore | M4 | Isolation review, restore drill, rollback compatibility and agreed pilot SLO/RPO/RTO |
| M6 — Scale and extensions | Separate workers and measured capacity; additional protocols and bounded patterns | M5 and measured demand | Fairness/load tests, compatible worker rollout, connector conformance and documented limits |

M0 is substantially implemented; it is not equivalent to a finished product.
M1–M6 remain incomplete. Credential security and scope checks start with M1/M2;
they do not wait until M5.

## First slice demonstration

Use synthetic local endpoints. Create scoped connections, save a validated draft,
publish/activate in development, admit one manual run, and inspect its persisted
node attempts. Inject a transient failure and a worker crash. Demonstrate that
an unsafe ambiguous write is surfaced rather than blindly repeated. No live SaaS
account is needed for the acceptance test.

## What to delay

Do not start with every protocol, a marketplace, code execution, simultaneous
editing, a broker cluster or a billing plane. Add one complete connector and
one explicit routing pattern at a time. Multi-region and untrusted plugins need
new design/review gates, not a feature flag around the MVP runtime.
