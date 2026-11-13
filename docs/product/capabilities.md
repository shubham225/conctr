# Capability and scope matrix

Status: **Proposed**. None of the runtime/backend entries below is implemented.
Phases and completion evidence are defined in [milestones](../roadmap/milestones.md).

| Capability | First slice / MVP | Later enterprise expansion | Owning boundary |
| --- | --- | --- | --- |
| Flow authoring | Linear flows, validation, immutable versions; then explicit choice/filter | Bounded fan-out/join, subflows, version comparison | flow + app |
| Triggers | Manual; then authenticated async webhook and schedule | Broker consumers, polling and event subscriptions | gateway + engine/connectors |
| Connectivity | HTTP JSON operations, API-key/basic credentials, bounded response bodies | OAuth 2, SOAP/XML, OData, GraphQL, gRPC, files and brokers | connectors + secrets |
| Mapping | Select/rename/default/coerce fields, schema validation | XML mappings and richer bounded expressions | transform |
| Runtime | Persisted attempts, deadlines, classified errors, retries, crash recovery | Isolated workers, durable waits and compensation | engine + storage |
| Operations | Run list/detail, redacted failures, cancellation request, replay warning | Alerts, dead-letter management, fleet views | app + application |
| Credentials | Write-only secret input, scoped references, encryption/key separation | Rotation, vault integration, service identities | secrets + auth |
| Governance | Workspace authorization and audit for changes/runs | OIDC, policy-driven release approval, Git promotion | auth + application |
| Environments | Explicit deployment and binding snapshot | Promotion, drift detection and protected environments | application + storage |
| Observability | Correlation IDs, structured logs and run events | OTEL export, SLOs and capacity controls | engine/gateway + operations |
| Portability | One shared Dioxus app, thin launchers | Native capability adapters and managed updates | app + launcher apps |

## Integration-pattern ladder

Begin with pipes/filters, transformation, content-based choice and explicit error
routes. Add bounded split/join only after correlation, branch completion,
ordering and partial-failure semantics have tests. Later consider aggregation,
routing slip, subflows and compensation. Treat each pattern as a specified
runtime behavior, not merely a draggable canvas shape. Camel's
[EIP catalog](https://camel.apache.org/components/4.22.x/eips/enterprise-integration-patterns.html)
is a useful vocabulary reference.

## Product backlog rules

Every capability needs an owner, phase, authorization rule, persistence/failure
behavior, UI states and an acceptance scenario. No connector is advertised as
supported until its contract tests, credentials, timeouts and error behavior
work. Security controls must accompany the features they protect; they are not
all deferred to an enterprise phase.
