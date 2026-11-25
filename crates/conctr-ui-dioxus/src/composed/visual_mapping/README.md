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

Rows show the field label, path, type and connection port. Filter by text or
show unmapped fields. Auto-map links unused targets to source fields with
matching normalized labels and types. Drag a source port to a target, or
activate the two ports in sequence. One source can feed multiple targets.
Replacing a target's source retains its link ID and transform pipeline.

With schema editing enabled, use a row's action menu to edit, ignore, restore
or delete the field. Drag a row to reorder its schema. Select a connection to
inspect its transforms. The resizable payload panels can show caller-supplied
previews.

Standalone mode previews and downloads the mapping as YAML. Studio mode hides
those export controls. Read-only mode supports inspection. The host owns
validation of type compatibility, persistence and transform execution. See
the [caller and import example](../demos/visual_mapping.rs) and
[YAML serializer](export.rs).

## Transformer pipelines and field editing

The connection inspector uses a searchable card picker rather than a long select.
Pass `transformers: Vec<TransformerDefinition>` to extend the catalog (stable unique
ID, label, category, description). Search matches all presentation fields. Each
selection appends a step to the ordered `FieldLink.transforms` list. The UI can move a step earlier or
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
never recreates them. Ignored fields stay selectable for restoration through their action menu.

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
