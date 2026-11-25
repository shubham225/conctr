# Working on Conctr

Start with the [task guide](docs/ai/AI-INSTRUCTIONS.md) and read the scoped
`AGENTS.md` files along each target's directory path before editing. Follow the
user's scope, preserve existing work, and check source and manifests before
describing a capability. Use the active docs for guidance. Files in `backups/`
preserve earlier approaches. When a change affects startup, navigation or a
public component API, update the README that developers use for that task.

- Keep domain rules, authentication policy and management API contracts independent
  of Dioxus. Put renderer and OS behavior at the edge. The six platform scaffolds
  have tentative responsibilities. Introduce dependencies when a feature needs them.
- Group Rust code by responsibility. Keep crate roots focused on exports and
  composition. Use explicit error handling and document public APIs. Use a
  replaceable interface when a caller needs one.
- Enforce access on the server and keep secrets out of logs, fixtures and UI
  previews. When an external write has an unknown outcome, establish whether it
  took effect before retrying.
- For Dioxus, inherit the workspace version and enable only the features in use.
  Use components, signals and current hooks. Match initial server and client
  markup. See the [Dioxus 0.7 guide](https://dioxuslabs.com/learn/0.7).
- Reuse the UI library and design kit. For shared icons, keep paired color and
  monochrome SVGs, named asset handles, design inventory and gallery in sync.
  Check keyboard access, light and dark themes, and reduced motion for UI changes.
- Use the manifest formatting, scratch directory and low-memory workflow in
  [engineering learnings](docs/development/learnings.md). Run the smallest
  relevant check first and report results and material limits.
