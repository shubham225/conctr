# Trust, identity and credential boundaries

Status: **Proposed requirements**, not a completed security assessment.
Implement relevant controls with each vertical slice before exposing it.

## Trust model

Treat external messages, imported flows, endpoint URLs, node configuration and
client-supplied scope IDs as untrusted. The server authenticates a principal and
resolves organization/workspace permissions; UI visibility is not enforcement.
Every data query, event subscription, blob fetch, credential use, run admission,
replay and deployment activation must be scoped and authorized.

Start with a small action/resource policy (read/edit/publish/deploy/run/replay/
manage-credentials/admin) and deny by default. OIDC authentication and group
mapping are later implementations, not a replacement for authorization. Test
cross-tenant IDs in routes, filters, cache keys, references and bulk actions.

## Separate two authentication problems

`conctr-auth` owns who can use Conctr. `conctr-secrets` owns credentials Conctr
uses with external systems. Users may be allowed to use a credential in a flow
without being allowed to read or replace its material. A credential's metadata
and a short-lived worker resolution capability are different APIs.

Secrets are write-only from normal clients. Redact logs/errors/exports and test
fixtures by construction; do not depend solely on regex replacement after logging.
Use approved cryptographic libraries and external key/vault configuration rather
than designing encryption. Keys need rotation, revocation, backup and recovery
procedures. No real secret is stored in this documentation or prototype.

## Endpoint and payload threats

Connector URL tests and execution can otherwise become SSRF primitives. Enforce
protocol/port/host/IP/redirect policy, revalidate redirects and resolution, and
apply deployment network egress controls. Explicitly decide which private
networks a self-hosted worker is allowed to reach; do not globally block every
private address in a product meant to integrate private enterprise systems.
Never allow cloud metadata/link-local or loopback access just because a URL was
syntactically valid. OWASP's [SSRF guidance](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html)
is the implementation reference; application validation and network controls
must be tested together.

Apply inbound authentication/signatures, replay windows, rate/size/decompression
limits and explicit response modes to webhooks. Protect browser sessions against
CSRF; configure CORS deliberately. Escape stored names, errors, payload previews
and node labels; never inject imported HTML. Code/connector plugins remain
trusted compiled code until a sandbox decision is implemented.

## Auditing and release controls

Record actor/service principal, scope, action, resource/version, timestamp,
correlation and safe before/after identifiers for credential changes, grants,
publication, activation, cancellation and replay. Do not record secret values.
Audit storage/exports need restricted access, retention and tamper-detection
requirements appropriate to the chosen deployment. Do not advertise compliance
certifications without an actual assessed program.

Before multi-tenant SaaS: add adversarial isolation tests, quotas/fairness,
credential/key separation, incident handling, backup restore evidence, dependency
review, abuse controls and an independent security review. These are release gates,
not architecture promises.
