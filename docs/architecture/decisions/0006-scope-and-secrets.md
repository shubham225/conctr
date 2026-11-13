# ADR 0006: Server-enforced scope and separate credential handling

- Status: **Proposed — for maintainer review**
- Date: 2026-11-13
- Implementation: not implied by this record

## Context

The platform receives untrusted URLs and payloads and uses sensitive credentials to reach enterprise systems. Hiding a button or encrypting a database alone does not enforce resource ownership.

## Recommended decision

Model organization/workspace ownership from the first store. Authenticate and authorize every application use case and subscription. Separate platform identity from external credential resolution; expose write-only secrets and safe metadata. Enforce scoped worker capability, redaction, egress policy and audit intent with protected changes.

## Alternatives

Retrofitting tenancy after shared storage risks ambiguous ownership. Putting external credentials in flow JSON or making desktop clients credential stores is rejected for the initial server-based product.

## Consequences and review criteria

Private-network integrations require explicit deployment policy rather than a blanket public-only rule. OIDC, key management and a multi-tenant release need implementation-specific reviews. No compliance certification is implied.

Related review questions: Q01, Q02, Q06, Q12 in [open questions](../../roadmap/open-questions.md).
Detailed contract: [design document](../security.md).

To accept, record the review date and resolved conditions here, then update the
checklist. Acceptance does not mark implementation tasks complete.
