# Visual flow designer

Status: **Proposed design**. The existing ConnectionFlow summary is not a visual
editor. Implement the editor against the [flow contract](../architecture/flow-definition.md).

## Workspace composition

```text
[Back / flow name] [Draft revision / saved state] [Environment] [Validate] [Publish]
[Node palette]   [Canvas or keyboard outline                  ] [Inspector]
[Search nodes]   [Manual → HTTP source → Map → HTTP target      ] [Config]
[Triggers]      [Inline diagnostics attached to stable IDs     ] [Input/output]
[Operations]    [Zoom / fit / undo / redo / outline             ] [Policy]
                [Run drawer: fixture preview / saved run trace             ]
```

One visually primary action per context. Save draft/autosave is distinct from
Publish; activation is a separate reviewed environment action. A run control must
state whether it uses synthetic fixtures or real environment connections.

## Editing model

Represent edits as commands (add/remove/move/configure/connect/disconnect) applied
to a draft model. Undo/redo affects local draft edits, not deployed state or
external side effects. Keep pointer position and layout updates separate from
semantic changes. Stable node IDs survive reorder, rename and drag operations.

Select a node to open its schema-driven inspector. Show connection binding,
operation/version, fields, type hints, credential reference (metadata only),
output ports, error policy and timeout/retry safety. Inline diagnostics link
between canvas, inspector and an accessible issue list.

MVP graph is a DAG with a constrained node catalog. Explicit typed ports, not
spatial proximity, establish order. Unsupported nodes and patterns are disabled
with an explanation; an icon cannot make a pattern executable.

## Interaction and accessible alternatives

Support pointer add/connect/pan/zoom and an equivalent outline for keyboard users.
The outline allows add-after, configure, connect ports, reorder where semantics
permit and remove with dependency disclosure. Selection/focus must stay stable
after updates. Provide fit-to-view, zoom text and an action menu; drag-only
interaction and color-only error states are insufficient.

Small screens use outline/read-only inspection initially. On web/desktop, inspector
and execution drawer can be resized within sensible bounds, with labels and
keyboard controls. Preserve focus when drawers close.

## Validation and data preview

Validate locally for fast feedback, then use the server's canonical validator at
publish/activation. Never assume client validation authorizes an operation.
Fixture preview is deterministic and visibly labeled. Real tests require explicit
scope/permissions, bounded execution and warnings for unsafe side effects.

Show redacted, paginated input/output samples and schema summaries, not unlimited
payloads. A test snapshot is attached to the draft revision/config fingerprint;
editing a node marks it stale. Running an individual node must disclose upstream
fixture/input selection and any real external effect.

## Save, publish and conflict states

Display Unsaved, Saving, Saved, Conflict and Save failed distinctly. Optimistic
revision conflict preserves local edits and offers compare/reload/export-draft;
do not overwrite another author's version silently. Publishing lists blocking
issues; activating reviews environment bindings and affected trigger admission.
Leaving with unsaved work requires a clear disposition. Published versions are
inspectable and immutable.

## Implementation split

Reusable canvas viewport, node card/ports and graph outline may live under
`conctr-ui-dioxus/components/composed` once their props are generic. Conctr flow
editing commands, server validation, permission state, connection resolution and
routing belong to the app's flows feature. The runtime compiler stays in
`conctr-flow`; never make the engine depend on a UI graph library.
