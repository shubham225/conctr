# Product vocabulary

Status: **Proposed**, except established Conctr branding.

| Term | Meaning and boundary |
| --- | --- |
| Organization / tenant | Security and administrative isolation boundary; IDs come from authenticated context |
| Workspace | Collaboration and authorization scope within an organization |
| Environment | Development/staging/production configuration and deployment scope |
| Connector | Versioned implementation exposing operations and configuration schemas |
| Connection | Workspace-owned endpoint configuration plus a credential reference; no workflow graph |
| Credential | Named authentication material with environment scope; never embedded in a flow export |
| Flow / workflow | The same integration definition; use **flow** in primary UI, workflow in explanatory text |
| Node | Stable-ID step with typed ports, operation, configuration and policy |
| Edge | Directed connection between declared ports; editor position is not execution order |
| Trigger | Entry event creating a run; source connector and trigger are distinct roles |
| Message / exchange | Payload reference plus approved metadata carried through processing |
| Draft | Mutable authoring document with optimistic revision checking |
| Flow version | Immutable published semantic snapshot plus pinned operation versions |
| Deployment | An environment's activation of a flow version with validated bindings |
| Run | One execution pinned to a version and deployment/binding snapshot |
| Node attempt | One bounded execution attempt, distinct from a node's logical invocation |
| Retry | Another permitted attempt of the same logical operation |
| Replay | A new run referencing an earlier run/input; explicitly may repeat side effects |
| Dead letter | Retained terminal failure requiring disposition; not automatic endless retry |
| Unknown outcome | External effect may have happened but success was not durably recorded |
| Mapping | Declarative transformation; not an unrestricted script by default |
| Runtime worker | Server-side executor; native desktop/mobile clients are not implicitly workers |

UI status labels may simplify details, but preserve exact machine states in APIs.
Keep **Healthy / Degraded / Failing / Draft / Paused** for connection summaries;
run states have their own vocabulary in the [execution model](../architecture/execution-model.md).
