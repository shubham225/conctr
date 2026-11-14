# ADR 0012: Library-owned Tailwind and component CSS modules

- Status: Accepted — explicitly requested by the maintainer
- Date: 2026-11-14
- Supersedes: [0011](0011-dx-managed-tailwind.md)

## Context

The UI crate must be reusable without Tailwind setup in each consuming app.
App-managed scanning/output couples consumers to library source paths. The
maintainer also requested the original adjacent `#[css_module]` authoring pattern
and a single palette/token source.

## Decision

Keep pinned Tailwind 4.1.18 development dependencies, compiler inputs and a shipped
utility bundle entirely in `conctr-ui-dioxus`. `ThemeProvider`/`ConctrStyles` loads
the foundation, including bundled utilities. Apps import components without
Tailwind configs, output assets or CSS build steps. The UI compiler scans workspace
consumer sources centrally.

Use complete utility classes in Rust and adjacent Manganis CSS modules for
specialized rules. Register each module explicitly in its components for repeated
SSR documents. Keep `tokens.css` as the palette/scale source, semantic shadcn
aliases in `main.css` and compiler mappings in `theme.css`. No central registry
or per-component generated stylesheet is introduced.

Library authors install npm development dependencies once. A small `build.rs`
refreshes the shared bundle on Cargo rebuilds when those tools are present;
otherwise consumers use the committed bundle without Node or network access.
An optional library watcher supports utility-only hot reload. CI/release checks
compare the shipped bundle with fresh compiler output.

## Alternatives

App-level DX compilation requires per-app setup and source-path knowledge.
Bundling all component CSS imports unused skins. Browser-time Tailwind adds runtime
tooling. Downloading a compiler during every consumer build creates a network/tool
requirement. The selected approach avoids those consumer dependencies.

## Consequences and verification

Workspace app utilities are scanned centrally by the UI compiler. Arbitrary
classes in standalone external consumers are outside that scan scope. Consumer customization uses props, variables, inline styles or shipped
utilities; optional app tooling can support additional app utilities independently.
Library authors must include the refreshed shared bundle when distributing edits.
CSS modules use plain browser CSS; `@apply` cannot be sent directly to Manganis.

Verify a consumer without Tailwind files/tools, per-document SSR stylesheet
selection/deduplication, bundle freshness, served scoped selectors, kit token
parity and actual light/dark/keyboard/responsive browser behavior. See the
[theming contract](../../development/theming.md).
