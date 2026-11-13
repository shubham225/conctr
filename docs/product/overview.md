# Product overview

Status: **Proposed product direction**. Implementation evidence is maintained in
[current state](../roadmap/current-state.md). Product name: **Conctr**;
CONCTR is reserved for the supplied logo wordmark. Tagline: **Where systems meet.**

## Purpose

Help integration engineers connect systems with different protocols and
credentials, express transformations and routing as inspectable flows, and
operate those flows with reliable execution records and controlled deployments.
The intended product combines a developer-friendly integration runtime with a
visual workflow authoring and debugging experience.

## Audience and value

| Persona | Job | Product outcome |
| --- | --- | --- |
| Integration developer | Connect an API, map data and route failures | A versioned, testable flow with understandable node configuration |
| Operator | Investigate failed or delayed runs | Inspect attempts, safely retry and distinguish unknown outcomes |
| Platform administrator | Control users, credentials and environments | Enforced scope, audit evidence and predictable operations |
| Reviewer/release engineer | Promote tested changes | Immutable versions, configuration bindings and a deployment history |

## Initial product boundary

Recommend a self-hosted-first server with web and desktop authoring clients and
a focused mobile operations client. Preserve organization/workspace boundaries
from the first persistence model; shared SaaS tenancy is a later release gate,
not an assumption that a single-tenant deployment is already secure for SaaS.
See [open questions](../roadmap/open-questions.md) for choices requiring review.

The first vertical slice is deliberately small: manually run an HTTP source,
validate/map a JSON payload, send it to an HTTP target, and inspect the saved
result. Then add authenticated asynchronous webhooks and schedules. A connector
is an implementation capability; a connection binds it to an endpoint and
credential reference; a flow composes operations; a run executes one pinned version.

## Enterprise direction

Progress toward governed environments, credential rotation, OIDC integration,
workspace authorization, audit trails, durable recovery, bounded concurrency,
operational telemetry, deployment controls and connector lifecycle management.
These are requirements to earn, not current certifications or guarantees.

## Non-goals for the first release

No Camel or n8n import compatibility promise, arbitrary untrusted code execution,
real-time collaborative editing, unlimited streaming, distributed transactions
across external systems, marketplace or built-in AI agent platform. Do not claim
exactly-once external effects. SOAP, OData, GraphQL and gRPC remain planned
protocol support even though their branded icons already exist.

## Success evidence to collect

A new user can author and run the documented sample without editing application
source; a worker crash has a documented recoverable outcome; operators can trace
an input to its node attempts without exposing credentials; a release can be
rolled back without silently changing in-flight runs. Set performance and
availability targets after measuring a representative workload. No invented
throughput, customer or uptime numbers are used as product claims.

[Capabilities](capabilities.md) · [Journeys](journeys.md) · [Inspiration](inspiration.md)
