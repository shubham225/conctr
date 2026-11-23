# Mapping workspace components

`MappingWorkspaceToolbar` provides controlled request/response switching and
undo/redo actions. It does not own a product document or store history.
Source/target summaries are rendered by the mapper footer.

Compose these with `visual_mapping::VisualFieldMapper { studio: true, ... }`
for a canvas with a persistent connection inspector. The mapper uses compact schema tables, smooth curved drag feedback, field filtering,
unmapped filtering, name/type auto-map and a persistent tools inspector. Studio mode
hides export; standalone consumers retain the existing YAML download.
The app owns document state, schemas, transform definitions and persistence.

Field rows offer right-click and an action button for edit, ignore/restore and
delete. Shift+F10 opens the same native popover; arrow keys navigate menu actions.
The tools sidebar and payload region use reusable `composed::resizable` handles
and panel controls, with retained sizes across minimize/maximize/restore.
