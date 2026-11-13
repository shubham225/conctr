# Documentation instructions

Applies under `docs/` in addition to the root [AGENTS.md](../AGENTS.md).
Start with [AI context](ai/AI-INSTRUCTIONS.md) and
[documentation maintenance](development/documentation.md).

- Distinguish implemented, scaffold, demo, proposed and deferred capabilities.
  Verify source/manifests before updating dated inventories or checking off features.
- Preserve accepted ADRs; record significant changed decisions with explicit status
  and links. Proposed ADRs do not silently replace accepted policy.
- Keep `ai/project.json` aligned with actual workspace packages and valid file paths.
  Update relevant roadmap evidence/checklists only for observable deliverables.
- Keep architectural contracts independent of UI framework details. Use the canonical
  design kit for branding and the theming guide for library CSS ownership.
- Root `AGENTS.md` owns repository-wide coding policy; `ai/AI-INSTRUCTIONS.md` routes
  detailed context. Tool-specific entry files should import or point to these
  sources, not maintain competing copies. Portable skills under `ai/skills/` are
  task workflows, not universally auto-discovered instructions.
- Validate documentation and AI entry links with
  `python3 docs/development/scripts/check_docs.py` from the repository root.
  Describe only validation actually performed.
