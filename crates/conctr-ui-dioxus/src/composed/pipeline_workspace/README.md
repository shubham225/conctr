# Pipeline workspace

`PipelineWorkspace` composes the existing workflow canvas/minimap and Rust
resizable controls. Callers supply a `WorkflowState`, nodes, step library and
inspector. The component owns panel width/mode and registers its scoped CSS in
each document. Nodes remain the caller's `WorkflowNodeWrapper`/`WfNode` children.

The canvas retains upstream dragging, port connections, pan/zoom, selection and
context menus. Fit measures the available space using Dioxus `MountedData`.
The inspector supports pointer/keyboard resizing and minimize/maximize/restore;
narrow screens stack the library, canvas and inspector.

This is presentation infrastructure. Node configuration, validation, persistence,
credentials and execution belong to app/platform owners. The `/pipelines` page
in `conctr-app-dioxus` demonstrates local authoring without external requests.
