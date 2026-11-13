# Review questions and recommended defaults

Status: **Open**. Maintainer is the decision owner; no dates or approvals are
assumed. Record resolutions in ADRs and update affected contracts/checklists.

| ID | Question | Recommended initial default | Consequence if changed |
| --- | --- | --- | --- |
| Q01 | Self-hosted, managed SaaS, or both first? | Self-hosted pilot; keep tenant scope in model | Shared SaaS adds isolation/abuse/operations release gates |
| Q02 | Is workspace the collaboration/security boundary? | Organization owns workspaces; environment scopes bindings | Changes affect every key, query, permission and audit record |
| Q03 | What may native clients do offline? | No offline execution or production credential cache; mobile operations first | Offline authoring needs conflict/key/storage rules; embedded workers are a distinct product |
| Q04 | Which protocols and patterns are commercially essential first? | HTTP JSON/manual/map; then webhook/schedule/choice | SOAP/XML or broker-first changes payload/acknowledgement and test scope |
| Q05 | Retention, payload limits and sensitive-data policy? | Bounded previews; separate payload/run/audit retention; choose pilot limits explicitly | Impacts storage, exports, replay windows and capacity |
| Q06 | Identity provider and bootstrap model? | Scoped bootstrap administration for local development; choose OIDC for pilot | Token/session/CSRF/native-login and group mapping need concrete design |
| Q07 | Pilot load/SLO/RPO/RTO? | Measure a representative workload and agree targets before release | May justify independent workers, store changes or HA earlier |
| Q08 | Distribution and third-party reuse policy? | Own implementation, retain Conctr licenses and review dependency terms | Embedding restricted code may change product rights/architecture |
| Q09 | Need arbitrary scripting, loops or human approval in MVP? | No; bounded declarative nodes and DAG first | Requires sandboxing or new durable state/loop semantics |
| Q10 | Synchronous webhook request/reply required? | Durable async acceptance with run ID | Sync adds strict deadlines, disconnect and unknown-outcome handling |
| Q11 | Production approval and artifact promotion policy? | Distinct publish/activate; permission-gated activation | Mandatory multi-party approval adds workflow/auth/audit states |
| Q12 | Which private networks may workers reach? | Explicit administrator-controlled environment egress policy | Directly impacts legitimate integrations and SSRF isolation |

The provisional schema includes illustrative caps (100 nodes, 200 edges, bounded
attempts/timeouts). Review them with Q05/Q07; they are not measured product limits.
Unresolved questions should be explicit in implementation plans, not silently
converted into product promises or blockers for unrelated work.
