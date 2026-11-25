# Maintaining the documentation

Use [the index](README.md) to find the page that owns a topic. Update the
existing explanation and link callers to it. Keep runnable procedures in
`development/`, package and HTTP references in `architecture/`, and AI
navigation in `ai/`. Keep package startup instructions and caller examples in the owning README.
Link to them from the index. Verify examples against the current source. Label a
proposed responsibility explicitly and record the reason when it becomes a
decision.

Run `python3 docs/development/scripts/check_docs.py` from the repository root.
