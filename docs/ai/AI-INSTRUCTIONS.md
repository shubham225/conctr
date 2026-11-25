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
