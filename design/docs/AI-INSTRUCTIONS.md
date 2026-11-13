# Conctr brand instructions for AI assistants

Use this file as system or project context whenever you create anything for Conctr: UI, copy, slides, docs, emails, diagrams, code. Machine-readable data is in `brand.json`. Detail is in `BRAND-GUIDELINES.md`, `DESIGN-TOKENS.md` (typography, spacing, radius, shadow, motion, sizing) and `COMPONENTS.md` (every component spec).

## Who Conctr is
Conctr is an integration platform. It connects systems that use different protocols (REST, SOAP, OData and more) and different authentication methods. Tagline: **Where systems meet.** Audience: developers, integration engineers and IT teams. Personality: clear, precise, calm, engineer to engineer.

## Hard rules (always)
1. Name: write **Conctr** in text. **CONCTR** only as the logo wordmark. Never "ConCTR", "Conctr.io" or any variant you invent.
2. Logo: **use the provided SVG files. Never redraw, recreate or generate the logo**, and never describe it as something else. Pick the file by background:
   - light background: `logo/svg/*-color.svg`
   - dark background: `logo/svg/*-color-on-dark.svg`
   - single-color needs: `*-black.svg` (on light) or `*-white.svg` (on dark or photos)
3. Colors: use tokens, not raw hex. Support both themes. Never use raw teal, coral or amber for small text on light backgrounds, use the `-text` tokens.
4. Shapes carry meaning: circle = REST, square = SOAP, triangle = OData, diamond = GraphQL, hexagon = gRPC or Conctr, ring = webhooks. Do not reassign them.
5. Fonts: Sora (headings), Inter (UI and body), JetBrains Mono (code), bundled in `fonts/`. Sentence case. No all-caps labels. Only sizes from the type scale below.
6. Accessibility: AA contrast, visible focus ring, never color alone (pair with shape or label), respect reduced motion.
7. Do not invent features, customers, numbers, certifications or integrations. If a fact is not provided, leave a clearly marked placeholder.

## Tokens (CSS custom properties, light | dark)
| Token | Light | Dark |
|---|---|---|
| `--cn-bg` | #F7F8FC | #0B1020 |
| `--cn-surface` | #FFFFFF | #121933 |
| `--cn-raised` | #FFFFFF | #1A2244 |
| `--cn-border` | #E0E3EE | #2A3459 |
| `--cn-text` | #121933 | #EEF0F7 |
| `--cn-text-muted` | #4B5578 | #AEB4CC |
| `--cn-primary` | #4F46E5 | #818CF8 |
| `--cn-on-primary` | #FFFFFF | #0B1020 |
| `--cn-teal` / `--cn-teal-text` | #14B8A6 / #0F766E | #2DD4BF / #2DD4BF |
| `--cn-coral` / `--cn-coral-text` | #F2603C / #C2410C | #FF8A6B / #FF8A6B |
| `--cn-amber` / `--cn-amber-text` | #F5A524 / #B45309 | #FBBF24 / #FBBF24 |
| `--cn-danger` | #DC2626 | #F87171 |

Full list: `colors/tokens.css`. Load `fonts/fonts.css`, then `colors/tokens.css`, then `ui-kit/conctr-ui.css`. Set `data-theme="light|dark"` on `<html>`.

## Design token cheat sheet (all from `colors/tokens.css`)
**Type scale** (size / line height px, weight, font): display 48/56 600 Sora | h1 36/44 600 Sora | h2 28/36 600 Sora | h3 22/30 600 Sora | h4 18/26 600 Sora | body-lg 18/28 400 Inter | **body 16/26 400 Inter** | small 14/22 (labels 500) Inter | caption 12/18 Inter | code 13/20 JetBrains Mono. Classes: `cn-display cn-h1..cn-h4 cn-body-lg cn-small cn-caption cn-mono`. Display, h1, h2 shrink below 640 px.

**Spacing** (4 px base, token number x 4): space-1 4, 2 8, 3 12, 4 16, 5 20, 6 24, 8 32, 10 40, 12 48, 16 64, 20 80, 24 96. Inside components 1 to 4, between components 4 to 8, between sections 12+. Use `gap`. No off-scale values.

**Radius:** xs 4 (keys, inline code), sm 6 (tags, tooltips), md 10 (buttons, inputs, menus, alerts), lg 14 (cards, table wrapper), xl 20 (modals), pill (badges), full 50% (avatars). Nested radius = outer minus padding.

**Elevation:** flat + 1 px border by default. shadow-1 cards, shadow-2 sticky bars and hovered cards, shadow-3 menus, popovers, toasts, tooltips, shadow-4 modals. Layers: sticky 100, dropdown 200, overlay 300, modal 400, toast 500, tooltip 600.

**Sizes:** controls 32 / 40 (default) / 48 px, touch targets 44 px minimum. Icons 16, 20, 24, 32, 48. Containers: prose 720, content 1200, wide 1440. Breakpoints (min-width): 640, 768, 1024, 1280, 1536.

**Motion:** 120 ms (hover), 200 ms (open, close), easing `var(--cn-ease)`. Animate opacity and transform only. One signature animation: the amber packet on `.cn-link`.

**States:** hover = one step on the surface scale; focus = 2 px `--cn-focus` outline, 2 px offset; disabled = opacity .45; selected = `--cn-primary-soft` bg + `--cn-primary` text; error = `--cn-danger` + message.

## Component recipes
Use the kit classes, do not restyle them. Full specs in `COMPONENTS.md`.
- Button: `cn-btn cn-btn--primary|secondary|ghost|danger` (+ `--sm`, `--lg`, `--icon`). One primary per view. Label = verb + object.
- Field: `div.cn-field > label.cn-label + input.cn-input|select.cn-select|textarea.cn-textarea + .cn-hint|.cn-error`. Error: `aria-invalid="true"`.
- Choices: `label.cn-option > input.cn-check|cn-radio|cn-switch`.
- Protocol badge: `span.cn-badge > i.cn-shape[data-s=circle|square|triangle|diamond|hex|ring][style="--c:var(--cn-teal)"] + name`. Status badge: `cn-badge--success|warning|danger|info` with text (Healthy, Degraded, Failing, Draft).
- Card: `cn-card` (`--compact`, `--raised`, `--interactive`). No cards in cards.
- Messages: `cn-alert--info|success|warning|danger` (what happened + what to do); `cn-toast` for past-tense confirmations.
- Navigation: `cn-tabs`, `cn-breadcrumb`, `cn-pagination`, `cn-nav`.
- Data: `div.cn-table-wrap > table.cn-table`, `cn-stat`, `cn-code`, `cn-avatar`.
- Feedback: `cn-progress`, `cn-spinner` (with text), `cn-skeleton`.
- Overlays: `cn-menu`, `cn-tip[data-tip]`, `cn-scrim > cn-modal` (title = question, destructive button repeats the verb).
- Connection summary: `cn-flow` with `cn-node`, `cn-link`, `cn-hub`.

## Building UI
- Reuse classes from `ui-kit/conctr-ui.css`: `cn-btn cn-btn--primary|secondary|ghost|danger`, `cn-input`, `cn-select`, `cn-switch`, `cn-badge`, `cn-shape[data-s=circle|square|triangle|diamond|hex|ring]`, `cn-card`, `cn-alert`, `cn-tabs`, `cn-table`, `cn-code`, `cn-flow` with `cn-node`, `cn-link`, `cn-hub`.
- One primary button per view. Radius: 10 px controls, 14 px cards. 4 px spacing grid.
- Summarise a connection with the source, hub, target flow component.
- Icons: `icons/color/*.svg` for marketing, `icons/mono/*.svg` (currentColor) for UI. Do not draw new protocol icons unless asked, and follow the icon rules in the guidelines if you must.
- Dark mode is a real theme, not an inversion. Use the dark token values, and `*-color-on-dark` logos.

## Writing copy
- Plain verbs, active voice, sentence case, no exclamation marks, no emoji in product UI.
- Say what happened and what to do next. Errors do not apologise.
- Keep names consistent: connection, source, target, mapping, run, protocol, credentials.
- Buttons name the action ("Create connection", "Test connection"). Confirmations reuse the same verb ("Connection created").
- Avoid: seamless, unlock, leverage, supercharge, revolutionary, next-gen, "AI-powered" (unless true and specific).

Approved lines:
- Tagline: Where systems meet.
- Short: Conctr connects systems that speak different protocols.
- Boilerplate: Conctr is an integration platform that connects systems using different protocols (REST, SOAP, OData and more) and authentication methods, without custom glue code.

## Diagrams and slides
- Show flows as source, hub, target. Use the hex hub in indigo, with nodes in the protocol shapes and colors.
- Light slides on `--cn-bg`, dark slides on `#0B1020`. Titles in Sora 600, body in Inter.
- Logo bottom-left or top-left, horizontal lockup, with clear space of 2x the circle-node diameter.
- Max one accent color per chart series beyond the order: indigo, teal, coral, amber.

## Image generation
Do not ask an image model to draw the logo or the protocol shapes. Place the supplied SVG or PNG files. If you need atmosphere, request abstract dark navy backgrounds with faint hexagon outlines and no text.

## Pre-flight checklist
- [ ] Correct logo file for the background, unmodified, with clear space
- [ ] Tokens used (color, type, space, radius, shadow), both themes work
- [ ] Only type sizes and spacing values from the scale
- [ ] Components taken from the kit, states covered (hover, focus, disabled, error)
- [ ] Shape plus color for any protocol or status meaning
- [ ] Text contrast at AA, `-text` tokens for colored text on light
- [ ] Sentence case, no hype words, consistent vocabulary
- [ ] No invented facts

## Prompt snippet (paste into other tools)
> You are helping with Conctr, an integration platform that connects systems using different protocols and auth methods. Tagline: "Where systems meet." Voice: clear, precise, calm, engineer to engineer. Use the name "Conctr" in text. Use only the provided logo files. Brand colors: indigo #4F46E5 (dark mode #818CF8), teal #14B8A6, coral #F2603C, amber #F5A524, ink #121933. Shapes: circle = REST, square = SOAP, triangle = OData. Fonts: Sora headings, Inter body, JetBrains Mono code. Type scale 12/14/16/18/22/28/36/48 px. 4 px spacing grid. Radius 6 (small), 10 (controls), 14 (cards), 20 (modals). Controls 40 px high. Sentence case. Support light and dark mode. Never invent facts.
