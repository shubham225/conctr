# Maintaining architecture documentation

Every new feature should update its owning contract, current-state evidence and
checklist as needed. Keep implementation observations separate from future scope.
Proposed ADRs need review; accepted records are superseded rather than silently
rewritten. Do not mark prototype controls or starter tests as product completion.

## Sources of truth

- Branding/tokens/component recipes: `design/`.
- Actual capabilities: source/tests, summarized in `docs/roadmap/current-state.md`.
- Ownership: crate map; decision status: ADR index and records.
- Executable contract: future flow crate, with provisional docs/schema until it exists.
- AI inventory: `docs/ai/project.json`, kept consistent with current workspace members.

After adding/moving docs, run:

```sh
python3 docs/development/scripts/check_docs.py
```

This validates local documentation links, structured JSON, workspace-inventory
membership, schema references and example graph references. When the optional
`jsonschema` library is available it also validates the complete provisional schema. It is not a runtime flow
compiler, security review or a check that every external reference still exists.

The portable skills can additionally be checked with the skill-creator validator
available in the AI environment. They live in docs for review and require explicit
reading/installation in tools that do not discover that location automatically.
The static experience prototype should be checked in light/dark and narrow/wide
browser layouts when its markup/styles change. No build dependency is required.
