# Canonical flow definition

Status: **Proposed contract**, not an implemented parser or compatibility promise.
Review with [ADR 0004](decisions/0004-versioned-flow-definition.md).

## One executable model

`conctr-flow` should own a versioned, serializable definition and pure validation
and compilation. The designer, API import/export, tests and future CLI use the
same contract. Canvas coordinates, zoom, colors and selection live in a separate
layout document; moving a node must not change execution behavior or require a
new deployment.

Provisional examples: [HTTP flow](examples/http-flow.json),
[layout](examples/http-flow.layout.json) and [structural schema](schemas/flow.schema.json).
These are documentation fixtures only. The schema checks shape, not operation
availability, authorization, endpoint safety or graph semantics.

## Required semantics

| Field / concept | Contract |
| --- | --- |
| `schemaVersion` | Explicit format version; unsupported versions fail with an actionable error |
| Stable node IDs | Survive moves, renames and serialization; never derive from array index |
| Operation kind/version | Immutable version pins; no silent upgrade to the newest connector |
| Ports and edges | Named source/target ports validated against descriptors and payload schemas |
| Connection references | Symbolic binding keys resolved in an environment; never raw credentials |
| Retry/deadline policy | Bounded defaults and per-operation safety; immutable within a run |
| Layout | Separate non-executable artifact keyed by the draft/version identifier |
| Extensions | Explicit namespaced/versioned mechanism later; reject unknown semantic fields initially |

Use deterministic serialization for checksums and diffing. Define normalization
and numeric encoding before relying on a content digest; hash maps' iteration
order and ordinary pretty JSON are not a canonicalization strategy. Draft revision,
published version ID and content checksum are different values. A checksum detects
content change but is not an authorization decision or signature.

## Validation pipeline

1. Enforce document byte/depth/node/edge/config limits before expensive work.
2. Validate schema version, unique IDs, allowed fields and version syntax.
3. Resolve pinned descriptors; validate config, available ports and bindings.
4. Validate reachable nodes, exactly one entry trigger for the first model,
   edge references, permitted fan-in/fan-out and payload compatibility.
5. Reject cycles in the MVP. Later loops must be explicit bounded operators,
   not arbitrary graph back-edges. Reject unsupported join/subflow semantics.
6. Produce a deterministic plan and diagnostics with node ID, config path,
   error code and message. No external network calls in pure compilation.
7. Activation additionally checks scoped permissions, credential availability,
   endpoint policy and deployment limits. Metadata validation is not proof that
   an external endpoint is reachable.

MVP nodes: manual trigger, HTTP operation, declarative mapping, then webhook,
schedule, filter and choice. Initially a choice schedules only its selected
branch; it does not imply parallelism or a join. Parallel split/join requires a
separate contract for ordering, correlation, empty branches and failure behavior.

## Data contract

An engine message contains content type, bounded approved headers, correlation
and causation IDs, schema reference and either bounded inline data or a blob
reference. JSON is the first format; binary/XML use explicit content types and
blob storage later. Do not stringify arbitrary binary payloads into every event.

A node receives a logical input and returns typed output-port messages or a
classified failure. Mapping has explicit missing/null/coercion rules; unavailable
fields do not silently become successful empty values. Secrets and unsafe
transport headers are excluded from default previews and propagated metadata.

## Version and import/export rules

A successful publish creates an immutable validated version. Export references
connections symbolically and excludes credential material, payload history and
local UI state. Import is a draft operation with size limits and validation,
not code execution. Version migrations must be explicit, repeatable and covered
by fixtures; retain old versions needed by historical runs. In-flight execution
uses its pinned compatible plan, not the latest draft.
