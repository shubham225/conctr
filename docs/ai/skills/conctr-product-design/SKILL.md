---
name: conctr-product-design
description: Design or implement Conctr product pages, shared app navigation, run inspection, and flow-editor interactions using the canonical design kit and product contracts. Use for Conctr application UX, not standalone artwork or unrelated website projects.
---

# Conctr product design

Locate the repository root. Read `design/docs/AI-INSTRUCTIONS.md` and the relevant
component/token specs in design/. Read `docs/experience/README.md` and the relevant
page/flow-designer blueprint. For state-changing UI, read the matching domain/API
contract and current-state evidence before presenting a capability as supported.

Use `conctr-ui-dioxus` components and its primitives/ui/composed ownership. Product
routes, data fetching, permissions and feature presentation state belong in
`conctr-app-dioxus`; native launchers supply actual OS capabilities. The shared
app is currently scaffolded, so verify source before assuming routes exist.

Keep draft save, publication, activation, testing and replay distinct. An external
test is not a harmless preview. Display workspace/environment and pinned version
where they matter. Include unknown outcomes, stale data, forbidden access,
conflicts, loading and empty states. Keep secrets out of previews and fixtures.

Follow supplied logos, fixed protocol meanings, typography/tokens, both themes,
visible focus and reduced motion. Provide an accessible outline for graph editing;
never make essential operations drag-only. Mobile starts with operations and
inspection unless the user changes that scope. The HTML documentation prototype
is illustrative; it is not a replacement product frontend or proof of a backend.

When moving CSS modules, keep source markers synchronized and verify served
selectors. Test actual layout, keyboard/focus and relevant interactions in a
browser when available; do not equate a successful build with visual correctness.
