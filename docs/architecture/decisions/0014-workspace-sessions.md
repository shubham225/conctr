# ADR 0014 — Workspace sessions for local and federated login

Status **Proposed for maintainer review** · 2026-11-23

The local password implementation and operator setup are documented in the
[sign-in guide](../../../crates/conctr-api-dioxus/README.md).

## Context

The shared app needs a login that works before an identity provider is available.
A later Google or Microsoft integration needs to preserve the same account IDs,
workspace permissions and API checks.

## Decision

Keep password verification and session issuance in the framework-independent
auth crate. Give each provisioned account a stable ID. Use the verified account
ID as the subject of a short-lived Conctr JWT. The server looks up the role
when processing each protected request. For a future OpenID Connect provider,
link its verified issuer and subject to that ID before issuing a session.

The management HTTP router owns cookie transport and origin checks. A process-local
registry of active session IDs gives logout immediate effect for subsequent
requests, including requests made with a copied token.

## Alternatives

Opaque session IDs would also support a server-side revocation check. Signed
JWTs give the API a standard format with explicit issuer, audience, subject
and expiration. Per-process session state still limits this first implementation
to one server.

## Consequences and review

Restarting invalidates logins and reloads account configuration. Review shared
revocation, key distribution, and throttling before adding replicas. Review
identity linking and provider callback validation before enabling federated login.

Regression checks cover password rejection, token validation, logout revocation,
cross-site request rejection, and anonymous access to workspace APIs.
