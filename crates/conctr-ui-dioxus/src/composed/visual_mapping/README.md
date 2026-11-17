# Visual field mapping

`VisualFieldMapper` is a controlled presentation component. Keep the original
`FieldMappingEditor` for the classic row editor. The showcase exposes both.

Files in this folder own the model/import helper (`model.rs`), interaction and
layout (`editor.rs`, `fields.rs`), YAML serialization (`export.rs`) and scoped CSS
module (`style.css`). The component explicitly registers its stylesheet for each
rendered document. There are no canvas coordinates in the exported contract.

## Caller contract

Pass `MappingSchema` for source and target, `Vec<FieldLink>` for the draft and
`on_change` to accept edits. Both schemas use the same serializable Rust/JSON
model, regardless of their original payload format:

```json
{
  "name": "Order service",
  "message": "Response body",
  "format": "xml",
  "namespaces": { "c": "urn:customer" },
  "fields": [
    { "path": "/c:Customer/@id", "label": "Customer ID", "data_type": "string", "depth": 0 }
  ]
}
```

JSON fields use bracket JSONPath, e.g. `$["customer"]["id"]`. XML fields use XPath
and optional namespace bindings. Paths are unique within a schema. `depth` is
only a presentation hint. Keep credentials and real sensitive samples out of demos.
`message` can identify any request/response direction; it does not change linking.
A JSON or YAML deserializer can construct these types. XML parsing and XML schema
inference belong to the calling adapter, which supplies normalized XPath fields;
this component does not parse raw XML or resolve external entities.

`schema_from_json` is a caller-side sample inference helper (not a JSON Schema
parser). It traverses all array elements into wildcard paths, unions discovered
fields, marks conflicting leaf types as `mixed` and retains empty containers.
It rejects inputs above 1 MiB, 32 nested levels or 2048 leaves. Presence in a sample
does not establish requiredness, constraints or production schema compatibility.

## Interaction and export

The editor has a ribbon toolbar and a dotted, two-column workspace inspired by
WorkflowBuilder. Select a card to enable Edit, Ignore/restore, Remove and Move
up/down. New source/target opens a native modal (focus trapping, Escape dismissal,
required fields and duplicate-path checks). There are no per-node popup menus.
Renaming a path updates attached link endpoints while preserving pipelines.

Drag a card or grip to reorder within its source/target column. Ordering is stored
in `MappingSchema.fields` and changes presentation only. Arbitrary x/y positioning
is deliberately constrained to these ordered columns. Drag a source **+ handle**
to a target card to connect. Alternatively click source/target handles, or choose
New link and then click source/target cards. Keyboard users can activate the same
buttons and Move up/down actions. Touch drags start from the grip or link handle.
Select a wire or connection chip to inspect and edit its pipeline. Each target
has one inbound link; replacing its source keeps its ID and pipeline. Sources
may fan out to multiple targets.

`read_only` disables field editing and removal but allows connection inspection
and download. A collapsed read-only YAML panel always mirrors the current draft.
`mapping_yaml` validates references, IDs and unique targets before export. It
emits a versioned `conctr.dev/v1alpha1` document containing path languages,
namespaces, links and optional transform IDs, with no sample values. Strings use
JSON-compatible quoted YAML scalars. Download uses the same serialization as preview.

This is a Conctr mapping configuration, **not an executable Apache Camel route**.
Camel YAML DSL requires route/EIP definitions and a supported expression language;
see the [official DSL](https://camel.apache.org/components/4.18.x/others/yaml-dsl.html).
An application/runtime adapter owns type compatibility, transform evaluation,
route generation, validation and persistence. No network requests run here.

See [caller usage and import demo](../demos/visual_mapping.rs). It shows editable
and read-only modes, JSON sample import and normalized XML-to-JSON mapping.

## Transformer pipelines and field editing

The connection inspector uses a searchable card picker rather than a long select.
Pass `transformers: Vec<TransformerDefinition>` to extend the catalog (stable unique
ID, label, category, description). Search matches all presentation fields. Each
selection appends a step to `FieldLink.transforms`; steps execute in their listed
order in the caller's future runtime adapter. The UI can move a step earlier or
remove it. Repeated transformers are allowed. Legacy `FieldLink.transform` is
retained on load as the first step; editing normalizes it into the ordered list.
YAML exports `transforms: [{type: ...}]` in that same order. These IDs are configuration,
not transform execution or type-compatibility guarantees.

## Inventory and extension boundaries

Accept `on_schema_change: EventHandler<MappingDraft>` and replace source, target and
links together. Without it, inventory toolbar buttons are disabled. The editor is
controlled: load/import in the caller, replacing the schemas **and clearing links**
on a subsequent import, as shown in the demo. No parser/import dialog is embedded
inside the editor. JSON/XML/YAML/TOML/plaintext adapters all supply the same model;
`PayloadFormat` records provenance, not a parser choice. YAML/TOML/text paths use
adapter-defined syntax. JSON sample inference remains a bounded leaf helper;
callers can explicitly supply object and array container nodes too.

`NODE_TYPES` defines the portable vocabulary: string, integer/int, number/float/
decimal, boolean, null, array, object, date, time, datetime, binary and mixed.
`MappingDraft` is serializable for host-owned persistence. Stable unique paths
identify nodes; labels are presentation. `FieldEdit` handles Add, Update, Move,
Remove and ToggleIgnored. Ignoring/removing prunes links atomically; restoring
never recreates them. Ignored fields stay selectable to restore from the toolbar.

The implementation is split into `editor.rs` (controlled orchestration),
`fields.rs` (cards/ports), `node_manager.rs` (modal form), `drag.rs` (scoped browser
gestures), `transform_picker.rs` (pipeline controls), `model.rs` (contract/reducer),
and `export.rs` (YAML). Drag listeners are delegated to the mapper root so new,
renamed and reordered cards work without rebinding stale node listeners.

Applications provide their catalog through `transformers`, report type/pipeline
issues through `validation_messages: Vec<String>`, and can render a custom script
or validation inspector through `inspector: Option<Element>`. Transform IDs are
configuration, never executed by this component. Runtime validation, parameters,
script compilation and persistence remain host responsibilities. Geometry and
column ordering never define transform execution semantics.
