# Application experience design

Status: **Proposed**, with an illustrative documentation prototype. These screens
are not implemented product pages and make no backend capability claim.

- [Information architecture](information-architecture.md): navigation and routes.
- [Application surfaces](application-surfaces.md): web, desktop, mobile and showcase.
- [Flow designer](flow-designer.md): editing, configuration, validation and testing.
- [Screen blueprints](screen-blueprints.md): page layout, actions and data ownership.
- [States and accessibility](states-and-accessibility.md): consistent behavior.
- [Interactive wireframes](prototype/index.html): explore screens, themes and device
  modes; all values are synthetic and every action stays local.

Open the prototype directly in a browser, or serve the repository root with
`python3 -m http.server 8765 --bind 127.0.0.1` and visit
`http://127.0.0.1:8765/docs/experience/prototype/`. The server command is optional;
no prototype needs access to real credentials or a production API.

## Visual authority

Use the existing [design kit](../../design/README.md),
[AI visual instructions](../../design/docs/AI-INSTRUCTIONS.md) and
[component recipes](../../design/docs/COMPONENTS.md). Sora headings, Inter UI,
JetBrains Mono code; 4px spacing tokens; light and dark themes. Preserve supplied
logos and protocol shapes. The documentation prototype loads these actual assets
rather than recreating them.

Use the shared UI library for real product implementation. The static HTML
prototype is a review artifact, not a second frontend to maintain or copy verbatim.
It demonstrates priorities and relationships; pixel details remain governed by
the kit and responsive component behavior.
