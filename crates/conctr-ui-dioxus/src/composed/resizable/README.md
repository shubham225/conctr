# Resizable panel controls

The implementation lives in `component.rs`, with scoped component styles in
`style.css`. Resizing uses Dioxus pointer/keyboard handlers, Rust signals and
`MountedData::get_client_rect` for group measurements. There is no resize script,
DOM listener registration or imperative CSS mutation.

Wrap the layout in `ResizablePanelGroup`. Give each `ResizableHandle` a controlled
pixel `size` and `on_change` callback. Apply the resulting size to the layout using
Rust-rendered styles. Set `vertical: false` for a trailing sidebar or `true` for a
bottom panel. Minimum sizes and maximum fractions bound the resizing operation.
A temporary drag shield keeps pointer events within the handle while crossing
adjacent panels. Arrows resize by 16px, Shift+arrows by 48px; Home/End use the bounds.
Escape cancels an active drag.

`PanelControls` exposes minimize, maximize and restore through a caller-owned
`PanelMode`. Panel sizes remain with the caller when handles unmount, so restoring
a panel retains its size. Nested groups use independent measurement contexts.
