# AI project context and portable skills

Use [AI-INSTRUCTIONS.md](AI-INSTRUCTIONS.md) for shared product/architecture
context and [project.json](project.json) for the dated machine-readable inventory.
The repository root [AGENTS.md](../../AGENTS.md) owns development policy and
points here; explicit user instructions still take priority within the tool's
instruction hierarchy.

## Automatically discovered entry points

| File | Purpose |
| --- | --- |
| [AGENTS.md](../../AGENTS.md) | Canonical repository policy for agents supporting this convention, including Cursor |
| [apps/AGENTS.md](../../apps/AGENTS.md) | Launcher/shared-app ownership and showcase rules |
| [UI library AGENTS.md](../../crates/conctr-ui-dioxus/AGENTS.md) | Component layers, shared styles, theming and browser verification |
| [docs/AGENTS.md](../AGENTS.md) | Documentation status, inventories and maintenance |
| [CLAUDE.md](../../CLAUDE.md) | Claude Code compatibility entry importing root AGENTS.md |
| [GEMINI.md](../../GEMINI.md) | Gemini CLI entry directing it to canonical and scoped instructions |
| [Copilot instructions](../../.github/copilot-instructions.md) | Repository-wide GitHub Copilot entry, with essential guards and links |

Keep the tool entry files small. Update policy in root/scoped AGENTS.md and the
owning contract, rather than maintaining different rules for each assistant.
No duplicate Cursor rule is needed: Cursor supports root and nested AGENTS.md.
Claude's entry imports the root explicitly for compatibility; it also directs
the agent to read scoped AGENTS.md before edits. Gemini/Copilot links are reading
instructions, not a promise that linked documents are automatically inlined.

Discovery depends on the installed tool, trust/settings and session context.
Start a new session or refresh project context after edits and inspect the tool's
loaded instructions. These files do not change global configuration, permissions,
models, hooks or external connections.

For **Zed Agent**, the documented loader selects the first supported project
instruction file. In this repository `.github/copilot-instructions.md` takes
precedence over root AGENTS.md, so that entry explicitly directs it to the
canonical policy. External ACP agents use their own discovery mechanisms; Zed's
loader does not configure every external agent. Do not add a legacy `.rules` file
that silently shadows these entries.

Official discovery references, checked 2026-11-13:
[Cursor rules](https://cursor.com/docs/rules),
[Claude memory/imports](https://code.claude.com/docs/en/memory),
[Gemini project context](https://geminicli.com/docs/cli/gemini-md/),
[Copilot repository instructions](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/add-custom-instructions/add-repository-instructions),
[Zed instructions](https://zed.dev/docs/ai/instructions).

## Skills included

| Skill | Use |
| --- | --- |
| [conctr-platform-implementation](skills/conctr-platform-implementation/SKILL.md) | Implement/review flow, runtime, connector, API and persistence features with correct ownership and recovery contracts |
| [conctr-product-design](skills/conctr-product-design/SKILL.md) | Design product pages or flow-editor interactions using Conctr's kit and proposed product semantics |

These skills live under `docs/` for review and portability. Their presence does
not automatically register them with every AI tool. Ask an agent to read the
SKILL.md explicitly, or install/link it into the tool's supported project skill
location when desired. This task does not alter global AI configuration.

## Context to load for a task

Root/scoped instructions establish the rules. Then read AI-INSTRUCTIONS.md and
only the relevant ownership/contracts: current state for progress claims, crate
map for code placement, execution/security contracts for platform behavior, or
design/presentation/theming contracts for UI work. Follow relevant links rather
than loading the entire documentation tree into every prompt.

`project.json` records instruction entry paths as well as dated package evidence.
After changing an entry point, link or inventory, run:

```sh
python3 docs/development/scripts/check_docs.py
```

The checker verifies local Markdown links, standalone `@file` imports and declared
instruction entry paths. It does not launch each AI product or prove that a local
editor has enabled its instruction loader.

## Example requests

- Read the platform-implementation skill, then implement graph diagnostics for
  the first flow schema using synthetic fixtures and no UI dependencies.
- Read the product-design skill, then implement a run inspector in the shared
  app with explicit loading, forbidden and unknown-outcome states.
- Read AI-INSTRUCTIONS and current-state, then revise the roadmap from the actual
  source diff; do not infer completion from a crate name.

Maintain one source of truth per topic. Skills route to the design contracts;
they do not duplicate the full architecture manual or authorize external actions.
