# AI project context and portable skills

Use [AI-INSTRUCTIONS.md](AI-INSTRUCTIONS.md) for shared product/architecture
context and [project.json](project.json) for the dated machine-readable inventory.
The repository root AGENTS.md points here; user instructions still take priority.

## Skills included

| Skill | Use |
| --- | --- |
| [conctr-platform-implementation](skills/conctr-platform-implementation/SKILL.md) | Implement/review flow, runtime, connector, API and persistence features with correct ownership and recovery contracts |
| [conctr-product-design](skills/conctr-product-design/SKILL.md) | Design product pages or flow-editor interactions using Conctr's kit and proposed product semantics |

These skills live under `docs/` for review and portability. Their presence does
not automatically register them with every AI tool. Ask an agent to read the
SKILL.md explicitly, or install/link it into the tool's supported project skill
location when desired. This task does not alter global AI configuration.

## Example requests

- Read the platform-implementation skill, then implement graph diagnostics for
  the first flow schema using synthetic fixtures and no UI dependencies.
- Read the product-design skill, then implement a run inspector in the shared
  app with explicit loading, forbidden and unknown-outcome states.
- Read AI-INSTRUCTIONS and current-state, then revise the roadmap from the actual
  source diff; do not infer completion from a crate name.

Maintain one source of truth per topic. Skills route to the design contracts;
they do not duplicate the full architecture manual or authorize external actions.
