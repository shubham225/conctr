# ADR 0004: Versioned flow model independent of canvas

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

A visual editor must not make screen coordinates or Dioxus state the executable workflow contract.

## Recommended decision

Use a versioned framework-independent manifest, stable node/port IDs and pinned operation versions. Validate and compile in conctr-flow. Keep layout separate, drafts revisioned, published versions immutable and activation a distinct environment-binding operation. Reject arbitrary cycles initially.

## Alternatives

Directly executing canvas JSON couples runtime behavior to the frontend. A general BPMN or Camel/n8n compatibility model is beyond the first scope. YAML may later be an authoring adapter, not a second semantic model.

## Consequences and review criteria

Schema migrations, deterministic checksums and diagnostic paths need deliberate design. Future fan-out/joins/loops require new semantics and fixtures. Review format limits and version-compatibility policy before treating the documentation schema as public.

Related review questions: Q04, Q09, Q11 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../flow-definition.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
