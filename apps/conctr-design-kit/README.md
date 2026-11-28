# Conctr design kit

Run the gallery to inspect Conctr's Dioxus controls in light and dark themes,
try interactions with sample data, and copy Rust usage examples. Start it from
the repository root and open the URL printed by DX.

```sh
CARGO_BUILD_JOBS=1 dx serve --package conctr-design-kit --platform web
```

## Find an example

- Open `/icons` to browse the color and monochrome artwork.
- Filter the explorer by component name or open an example at
  `/components/<slug>`. The field mapping examples include a row editor and a
  visual mapper with sample import and transform selection. Workflow examples
  exercise canvas layout and connections.
- On the toast example, trigger several notifications. Hover over the stack or
  focus a dismiss button to expand the stack and pause the countdowns. Touch
  layouts show the cards expanded and let you scroll to older notifications.
- Use the [catalog](src/catalog.rs) to find an example's slug and
  [gallery demos](src/demo/) for gallery-specific behavior. Reusable controls
  and compositions belong to the [UI library](../../crates/conctr-ui-dioxus/README.md).
  The [reference snapshot](assets/reference/README.md) comes from the
  [canonical brand kit](../../design/README.md).

## Verify gallery changes

Use the existing server and one dedicated browser. The [development guide](../../docs/development/setup.md)
gives the Node and Chrome requirements and the general browser check.
The [scripts directory](scripts/) contains focused checks for the sidebar,
component controls, data examples and visual mapping.
