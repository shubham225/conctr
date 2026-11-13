# Connector and transformation contracts

Status: **Proposed**; no production connector is implemented.

## Descriptor and operation boundary

A connector descriptor should declare a stable ID/version, operations, input and
output ports/schemas, configuration schema, credential requirements, pagination
and streaming capabilities, idempotency characteristics and operational limits.
The UI renders forms from safe metadata; executable functions never travel to
the client. A descriptor is an executable compatibility contract, not just a
marketing catalog entry.

An operation call receives a validated configuration snapshot, input message,
scoped execution context, deadline/cancellation signal and short-lived credential
capability. It returns outputs or a safe classified error. It must not own flow
scheduling, publish versions, create an unrelated database transaction or decide
application authorization.

Start with trusted compiled Rust modules behind feature flags. Do not promise a
stable Rust dynamic-library ABI. Untrusted community connectors or code nodes
need a separate process/sandbox, versioned protocol, resource limits, signed
provenance and a security review before they are offered.

## First HTTP connector acceptance contract

- Separate connection base URL, allowed methods/paths and credential reference.
- Apply egress/redirect/DNS/TLS policy before connecting, including test requests.
- Bound connect/read/total deadlines, body size, decompressed size and redirects.
- Explicit query/header construction and content type; block credential-bearing
  headers from inherited untrusted metadata.
- Classify 4xx/5xx, timeout, rate limit and unknown write outcome deliberately.
  A 2xx response is not proof of a downstream business transaction's completeness.
- Pagination is bounded and resumable; it is not an unbounded loop in a node.
- Inject an HTTP client/mock boundary so contract tests never need live accounts.
- Produce redacted diagnostics and schema failures with actionable paths.

Add SOAP/OData/GraphQL/gRPC only with distinct behavior specs, fixtures and clear
payload semantics. An existing protocol badge does not prove protocol support.

## Transformation evaluator

Own a versioned mapping representation with selection, rename, defaults, explicit
coercion and validation first. A compiler validates field references and the
runtime enforces depth, output-size and instruction/time budgets. Treat missing,
null, invalid conversion and empty lists distinctly. Expressions do not get
network, process, filesystem, reflection or secret-store access by default.

Richer scripting and third-party expression engines need a separate decision on
sandboxing, determinism, licensing and version pinning. Do not invent a complex
expression language just to implement a field rename.

## Connector lifecycle and tests

Each connector needs an owner, version policy, compatible schema changes,
deprecation window and fixture-based conformance tests. Test authentication,
redaction, pagination, cancellation, retry safety, endpoint rejection and response
limits. Credential refresh must be single-flight/transactionally coordinated
when multiple workers share an OAuth credential. Never place credentials in
URLs, serialized flow manifests or screenshots.
