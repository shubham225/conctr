# Find the instructions for a change

The [root instructions](../../AGENTS.md) apply across the workspace.
Use the relevant row to find the owner and its sources before editing.

| Task | Start with |
| --- | --- |
| Understand the product, supported frontend targets or design direction | [Project overview](../README.md) |
| Place Rust types or choose a crate boundary | [Crate map](../architecture/crate-map.md), then the package's manifest and callers |
| Change authentication or a management endpoint | [HTTP contract](../architecture/api-and-events.md) and [sign-in guide](../../crates/conctr-api-dioxus/README.md) |
| Change product UI or the gallery | [Presentation ownership](../architecture/dioxus-presentation.md), [design instructions](../../design/docs/AI-INSTRUCTIONS.md) and [theming](../development/theming.md) |
| Format manifests, use temporary files or manage build memory | [Engineering learnings](../development/learnings.md) |
| Run or verify a change | [Development commands](../development/setup.md) |

Use a source path or a focused test to support claims about implementation.
Describe suggestions for the integration runtime as proposals until the team
adopts them. Keep user-facing behavior and design-kit demonstrations distinct.

# Token & Build Efficiency Rules

## CLI & Output Compression
- Use `rtk` when reading large files (`rtk read <file> -l aggressive` for signatures/structs first) or searching (`rtk grep`).
- Prefer `cargo check --message-format=short` or `dx check` over full builds when verifying RSX syntax or types.
- Never run `cargo tree`, `cargo build --verbose`, or dump full `Cargo.lock` contents.

## Dioxus & Rust Context Boundaries
- Never read or search inside `target/`, `dist/`, `.dioxus/`, `node_modules/`, or `assets/` unless explicitly asked.
- When editing Dioxus components (`rsx!`), only read the specific component module you are modifying rather than the entire `src/` tree.
- Keep responses concise: output diffs or targeted code blocks without conversational filler.

# Stack & Architecture
- Language/Framework: Rust + Dioxus (targeting both Web and Desktop).
- UI Syntax: Always write UI with `#[component]` and `rsx!` using Tailwind CSS utility classes (`class: "..."`). Never output React/JSX.
- Cross-Platform (Web + Desktop):
  - Avoid browser-only DOM hacks unless gated behind `#[cfg(target_arch = "wasm32")]`.
  - Ensure responsive layouts, keyboard navigation, focus states, and window resizing work cleanly on both desktop viewports and web browsers.

# Design Kit Priority (Strict)
1. **Your Design Kit is the source of truth:** Always use existing components, color tokens, spacing scales, typography, and Tailwind theme classes from this repo's design kit first. Do not invent new colors, arbitrary spacing values, or duplicate primitives when a design kit component exists.
2. **Use `ui-skills` for layout, polish, and UX rigor:**
   - Before building or refactoring UI, run `npx ui-skills list` (or check `ui-skills-root`) and fetch relevant skills via `npx ui-skills get <slug>` (such as `baseline-ui`, motion, or layout skills).
   - Apply `ui-skills` rules for hierarchy, alignment, micro-interactions, accessible states, and visual polish—while mapping all colors, radii, and typography to our existing design kit tokens.
