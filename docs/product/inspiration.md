# Inspiration and source register

Reviewed: **2026-11-13**. These are primary documentation sources, not dependencies
or compatibility commitments. Conctr's proposed design is an inference and
product recommendation informed by them. Edition availability can change.

| Source | Observed concept | Conctr recommendation |
| --- | --- | --- |
| [Camel architecture](https://camel.apache.org/manual/architecture.html) | Routes, processors and component-provided endpoints | Separate flow model, execution runtime and connector contracts |
| [Camel EIPs](https://camel.apache.org/components/4.22.x/eips/enterprise-integration-patterns.html) | Established routing/messaging patterns | Specify semantics before introducing visual node types |
| [Camel error handlers](https://camel.apache.org/manual/error-handler.html) | Configurable recovery/redelivery strategies | Explicit per-operation retry and terminal-failure policies |
| [Camel idempotent consumer](https://camel.apache.org/components/4.22.x/eips/idempotentConsumer-eip.html) | Persistent duplicate tracking | Scope deduplication keys and distinguish it from external exactly-once effects |
| [n8n workflow components](https://docs.n8n.io/build/understand-workflows/workflow-components.md) | Connected canvas nodes and annotations | Discoverable palette, inspector and a separate non-semantic layout document |
| [n8n executions](https://docs.n8n.io/build/understand-workflows/understand-executions.md) | Manual/production execution and run inspection | Separate draft tests from pinned deployed runs |
| [n8n queue mode](https://docs.n8n.io/deploy/host-n8n/configure-n8n/scaling/enable-queue-mode.md) | Main/worker split with Redis and database coordination | Keep execution separable, but do not require Redis in Conctr's first deployment |
| [n8n source control](https://docs.n8n.io/administer/use-source-control-and-environments.md) | Git-backed environment management | Promote immutable artifacts and bind environment secrets separately |
| [Dioxus platform guide](https://dioxuslabs.com/learn/0.7/guides/platforms/) | Shared UI with platform-specific features | Shared app crate and thin web/desktop/mobile launchers |
| [OWASP SSRF prevention](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html) | Endpoint/network validation and outbound controls | Connector egress policy and explicit private-network rules |
| [PostgreSQL SELECT](https://www.postgresql.org/docs/current/sql-select.html) | Row locking and SKIP LOCKED for queue-like workloads | Evaluate a persisted claim/lease queue before adding a broker |

## Deliberate differences

Conctr should own a Rust, framework-independent manifest and runtime. Do not embed
a Java runtime merely to reproduce Camel patterns, or claim n8n node/manifest
compatibility. Start with a constrained integration vocabulary and durable
execution contracts rather than matching either project's catalog size.

Conctr's proposed PostgreSQL queue and single-server topology are recommendations,
not descriptions of n8n internals. The node editor is a presentation of the
canonical flow model, not the source of execution semantics.

## Source and license boundary

Use public product concepts as inspiration; do not copy implementation, assets
or documentation into Conctr without checking their actual license and attribution
requirements. n8n's official [license guidance](https://support.n8n.io/article/can-i-use-your-license-for-my-use-case)
identifies a source-available community license and commercial-use conditions;
it should not be treated as an unrestricted code donor. Review any actual reuse
against the applicable license text. No licensing or product-edition equivalence
is asserted by this plan.
