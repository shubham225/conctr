# Component coverage

Use this inventory to find the owning module in `conctr-ui-dioxus`. The public
facade exposes base controls under `ui`. Reusable combinations live under
`composed`. The [component specifications](COMPONENTS.md) describe visual
treatments and the HTML reference recipes.

| Design need | Base control families under `ui` |
| --- | --- |
| Actions | `button`, `button_action`, `button_group`, `pressable`, `toggle_group`, `toolbar` |
| Form controls | `field`, `label`, `input`, `input_group`, `input_otp`, `input_phone`, `textarea`, `checkbox`, `radio_button_group`, `switch`, `slider` |
| Selection and dates | `select`, `multi_select`, `date_picker`, `chips` |
| Navigation | `breadcrumb`, `tabs`, `sidebar`, `sidenav`, `bottom_nav`, `pagination`, `stepper` |
| Menus and overlays | `command`, `menubar`, `dropdown_menu`, `context_menu`, `dialog`, `alert_dialog`, `drawer`, `sheet`, `popover`, `hover_card`, `tooltip` |
| Layout | `card`, `accordion`, `collapsible`, `aspect_ratio`, `scroll_area`, `separator` |
| Data display | `table`, `data_table`, `data_grid`, `charts`, `code_block`, `badge`, `status`, `avatar` |
| Feedback | `alert`, `feedback_message`, `empty`, `toast`, `progress`, `spinner`, `skeleton` |

See `crates/conctr-ui-dioxus/src/ui/mod.rs` for
the complete list of base families.

## Integration authoring and operations

| Design need | Library entry point |
| --- | --- |
| Identify a protocol | `composed::ProtocolBadge`, `composed::ProtocolShape` |
| Show a source, hub and target | `composed::ConnectionFlow` |
| Build a pattern palette | `assets::icons::PATTERN_GROUPS` and individual SVG asset constants |
| Inspect fields and edit mappings | `composed::VisualFieldMapper`, `composed::FieldMappingEditor` |
| Arrange workflow nodes | `composed::WorkflowBuilder`, `composed::pipeline_workspace::PipelineWorkspace` |
| Resize workspace panels | `composed::resizable` |
| Inspect queued requests | `composed::queue::QueueMonitor` |
