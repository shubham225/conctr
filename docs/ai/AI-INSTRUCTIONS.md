# Conctr product and architecture context for AI

Use this context for Conctr platform/product work. Scope it to the user's task;
it is not a mandate to implement every proposal or create all suggested crates.

## Instruction ownership

[Root AGENTS.md](../../AGENTS.md) owns repository-wide development policy.
Read more specific `AGENTS.md` files along the target's directory path before
editing, including when the AI tool does not discover them automatically.
This file routes product/architecture context; the
[discovery index](README.md) explains tool entry points and portable skills.
Keep explicit user instructions authoritative within the tool's instruction
hierarchy. Source/manifests are evidence of implementation; Proposed documents
remain proposals. Do not copy all contracts into every always-loaded file.

## Load only the relevant sources

- Read [current state](../roadmap/current-state.md) before reporting progress.
- Read [crate map](../architecture/crate-map.md) before assigning new code ownership.
- Runtime/flow work: [flow definition](../architecture/flow-definition.md),
  [execution model](../architecture/execution-model.md) and relevant ADRs.
- Credential/connector work: [security](../architecture/security.md) and
  [connector contract](../architecture/connector-contract.md).
- API/storage work: [API](../architecture/api-and-events.md) and
  [storage](../architecture/data-and-storage.md).
- UI work: [experience](../experience/README.md),
  [presentation ownership](../architecture/dioxus-presentation.md) and the
  [canonical design instructions](../../design/docs/AI-INSTRUCTIONS.md).
  For CSS/theme changes, also read the [theming contract](../development/theming.md).

## Non-obvious project rules

1. A package is not a feature. Most platform crates and the shared app are
   scaffolds. The echo server function and showcase samples are demonstrations.
2. The canonical flow/compiler must not depend on Dioxus, canvas coordinates or
   a UI graph library. Layout and executable semantics are separate documents.
3. Pin immutable version and binding/operation snapshots in runs. Draft edits,
   promotion and rollback do not rewrite in-flight work.
4. Persist admission and attempt intent. Bounded retries require operation-level
   safety. Unknown external writes are not confirmed failures; never claim generic
   exactly-once effects or automatic compensation.
5. Authorization comes from authenticated server context. Scope queries, blobs,
   credentials and event subscriptions, not just navigation controls.
6. Platform identity and external credentials are different boundaries. Do not
   put secrets in flow JSON, app state, traces, exports or documentation fixtures.
7. Application services own administrative transaction ports; engine owns runtime
   ports; storage implements them. Do not create a storage/engine/app cycle.
8. Shared product pages go in `conctr-app-dioxus`; generic presentation goes in
   `conctr-ui-dioxus`. Preserve the primitives/ui/composed direction. Keep actual
   OS behavior in launchers/adapters and screen-width behavior responsive.
9. The UI library owns Tailwind 4.1 tooling, input and the shipped utility bundle.
   Apps import components and `ThemeProvider`/`ConctrStyles`; no app Tailwind setup
   or build step. Adjacent CSS modules load per component, with explicit document
   registration for SSR. No global registry or per-component generated files.
10. Palette/scales live in UI `assets/colors/tokens.css`; generic shadcn aliases
    and interactions in `assets/styles/main.css`; compiler mappings in `theme.css`.
    Keep local CSS in the components layer. Verify scoped themes, served module
    selectors, utility coverage and browser states. See the theming contract.
11. Use actual Cargo names: `conctr-api-dioxus` is the existing Dioxus transport
    package. Do not introduce `conctr-dioxus-api` as a second package to match old prose.

## Planning and evidence

Existing accepted ADRs remain the policy baseline. New Proposed ADRs are design
recommendations, not maintainer approval or completed implementation. A selected
feature can proceed within the user's authorization; document assumptions and
resolve only choices that block that task. Do not turn every open review question
into a permission request. New dependencies/crates need a concrete task-based
reason; prefer modules until the boundary warrants a package.

Update the checklist and current-state evidence when real functionality changes.
Use deterministic mocks/fake clocks and crash/isolation tests appropriate to the
boundary. Record checks actually run and distinguish default workspace checks
from native renderer/end-to-end validation. Read official Dioxus 0.7 documentation
for APIs and keep Dioxus out of the platform core.

## Product vocabulary and references

Use [the glossary](../product/glossary.md). Conctr is inspired by Camel patterns
and n8n authoring/operations concepts; it is not a compatible implementation or
an unrestricted copy of their code. See [the source register](../product/inspiration.md).
Never invent supported protocols, customers, performance results or certifications.
