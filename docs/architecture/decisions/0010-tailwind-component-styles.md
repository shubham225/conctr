# ADR 0010: Tailwind theme and component-local styles

- Status: Superseded by [0011](0011-dx-managed-tailwind.md)
- Date: 2026-11-14

## Context

The UI library used vanilla CSS, hashed CSS-module selectors and a central
registry that loaded every registry skin. The maintainer requested Tailwind v4,
shadcn-style theme names, component-local styles and a small build setup.

## Decision

Use Tailwind 4.3.5 with CSS-first semantic variables and Conctr kit defaults.
Keep generic rules, theme mappings and compile-time recipes in `assets/styles/`.
Each styled family owns `style.css`, using `@reference` and `@apply`, and loads
its checked-in `style.generated.css` with `document::Stylesheet`. Dioxus handles
per-document deduplication. Use stable namespaced selectors and remove the central
registry. Keep recipes in the components layer so consumer utilities can override
them. Preserve public Rust imports, primitive interactions and platform boundaries.

## Alternatives

A single generated all-components bundle would simplify loading but would request
unused skins. Requiring Node during every Rust build would move CSS development
tooling into downstream builds. Neither meets the selected distribution model.

## Consequences and verification

CSS contributors run the crate's npm build/check commands; Rust consumers use
bundled CSS without Node. Additional consumer utility classes require that
consumer's Tailwind build; CSS-variable overrides do not. Generated files are
reviewable artifacts and must be regenerated after source edits.

Verify separate SSR documents, selective stylesheet loading, deterministic CSS
output, served selectors, browser states, themes, overrides and responsive layout.
See the [theming contract](../../development/theming.md).
