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

For Dioxus changes, use Rust components with `rsx!` and follow the theming
guide for Tailwind utilities and scoped styles. Check responsive layouts and
keyboard focus on the affected renderer. Start verification with a focused
`cargo check --message-format=short` or a relevant `dx check`.

## Optional AI tooling

Contributors can use the [development setup](../development/setup.md) with any
editor. AI assistants, RTK and skill installers are optional for all changes.

- [RTK](https://github.com/rtk-ai/rtk) is a personal command-output filter. Use
  standard shell commands by default, such as `rg` for search. If you use RTK
  with Codex, follow the upstream user-global setup so it applies from your own
  configuration.
- [Rewrite Rust skills](https://github.com/rewrite-rs/skills) are available from
  upstream. For a personal install, run
  `npx skills add rewrite-rs/skills --global` and select your agent and the
  skills you need.
- [UI Skills](https://github.com/ibelick/ui-skills) provides optional UI review
  guidance. If you choose to use the CLI, `npx ui-skills list` browses the
  registry and `npx ui-skills get baseline-ui` fetches a specific skill. The
  [design instructions](../../design/docs/AI-INSTRUCTIONS.md) cover project UI
  conventions and verification for everyone.

Codex reads the repository's `AGENTS.md` files without project-level
`.codex/` configuration. `CLAUDE.md`, `GEMINI.md` and
`.github/copilot-instructions.md` are entry points for other clients. Keep
shared instructions in `AGENTS.md` and follow scoped guidance before editing.
