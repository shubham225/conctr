# ADR 0011: DX-managed Tailwind 4.1

- Status: Superseded by [0012](0012-library-owned-tailwind.md)
- Date: 2026-11-14
- Supersedes: [0010](0010-tailwind-component-styles.md)

## Context

The previous migration introduced per-component generated CSS and npm build/watch
commands. The maintainer requested Dioxus's automatic Tailwind support and accepted
its managed Tailwind 4.1 version instead of the separately pinned 4.3 toolchain.

## Decision

Use Tailwind classes directly in Rust and DX-managed app-level compilation.
Dioxus CLI 0.7.10 installs and runs Tailwind 4.1.5. Each runnable app has a
`tailwind.css` input and `dioxus.toml` paths, scans both its own and the UI library's
Rust sources, and loads the output as an app asset. Only `dx serve`, `dx build` or
`dx bundle` is needed; no npm pipeline is retained.

The UI library's `main.css` is browser-ready runtime theme/typography/shared states.
Its `theme.css` provides the shadcn-style Tailwind mappings imported by apps.
Specialized ordinary CSS stays in each component's `style.css`, explicitly loaded
by that component. Remove all per-component generated CSS. Keep the existing Rust
APIs, semantic theme names, kit defaults and component-layer boundaries.

## Alternatives

Keeping a custom CSS watcher alongside DX would preserve the old authoring model
but leave the unwanted second command. Importing every component's CSS into one
bundle would make unused component skins part of every app. Browser-time Tailwind
compilation would add runtime tooling. None meets the selected workflow.

## Consequences and verification

DX performs Tailwind compilation during serving/building. Plain Cargo checks can
still run before generated app output exists because the stylesheet is loaded
only for renderer/server feature builds. The single generated file is ignored
by Git; a direct file asset supports DX's stylesheet hot reload. No Node dependency or
per-component generation script is shipped.

A consumer must scan the library's source classes and load its own Tailwind output.
Utility class ordering follows Tailwind's cascade; use typed props, theme variables,
inline styles or an explicit important utility to resolve a conflicting override.
There is no custom runtime class-merging engine.

Verify clean Cargo checks, automatic initial compilation and subsequent utility
updates with DX alone, separate SSR documents, served Tailwind 4.1 output and
component CSS, browser states, themes and responsive behavior. See the
[theming contract](../../development/theming.md).
