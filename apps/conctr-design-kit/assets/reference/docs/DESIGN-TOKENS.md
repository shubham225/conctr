# Conctr design tokens

Version 1.1. Single reference for typography, spacing, radius, borders, elevation, layering, motion and sizing. The same values exist in four formats, generated from one source:

| Format | File | Use for |
|---|---|---|
| CSS variables | `colors/tokens.css` | Web, Dioxus web, docs, anything with CSS |
| JSON | `colors/design-tokens.json`, `colors/colors.json` | Tooling, design plugins, AI agents |
| Tailwind | `colors/tailwind.config.js` | Tailwind projects (reads the CSS variables) |
| Rust | `colors/conctr_tokens.rs` | egui, native or any UI without CSS |

**Naming:** `--cn-{category}-{name}`, for example `--cn-space-4`, `--cn-radius-md`, `--cn-fs-small`, `--cn-text-muted`. Semantic names (`text-muted`, `primary`, `border`) are for product work. Never use raw hex, px or ms values in components.

**Never invent a token inline.** If the scale is missing something, propose a new token in the kit, then use it.

---

## 1. Typography

### Font families
| Role | Family | CSS variable | Weights | Fallback stack |
|---|---|---|---|---|
| Display, headings | Sora | `--cn-font-display` | 500, 600, 700 | Inter, system-ui, sans-serif |
| UI, body | Inter | `--cn-font-ui` | 400, 500, 600 | system-ui, -apple-system, Segoe UI, Roboto, sans-serif |
| Code, data | JetBrains Mono | `--cn-font-mono` | 400, 500 | ui-monospace, SF Mono, Menlo, Consolas, monospace |

Fonts are bundled in `fonts/` (WOFF2, Latin subset, SIL Open Font License). Load `fonts/fonts.css` before `colors/tokens.css`.

**Weights:** regular 400 (body), medium 500 (labels, buttons, table headers), semibold 600 (headings, emphasis), bold 700 (Sora only, display moments). Use at most three weights on one screen.

### Type scale
Each style has four tokens: `--cn-fs-{name}` (size), `--cn-lh-{name}` (line height), `--cn-fw-{name}` (weight), `--cn-ls-{name}` (letter spacing). Utility classes `.cn-display`, `.cn-h1` to `.cn-h4`, `.cn-body-lg`, `.cn-small`, `.cn-caption` apply them.

| Token suffix | Size / line (px) | Rem | Weight | Tracking | Font | Use |
|---|---|---|---|---|---|---|
| `display` | 48 / 56 (below 640 px: 36 / 44) | 3rem / 3.5rem | 600 | -0.02em | Sora | Marketing hero, one per page |
| `h1` | 36 / 44 (below 640 px: 30 / 38) | 2.25rem / 2.75rem | 600 | -0.015em | Sora | Page title |
| `h2` | 28 / 36 (below 640 px: 24 / 32) | 1.75rem / 2.25rem | 600 | -0.01em | Sora | Section title |
| `h3` | 22 / 30 | 1.375rem / 1.875rem | 600 | -0.01em | Sora | Card or panel title |
| `h4` | 18 / 26 | 1.125rem / 1.625rem | 600 | -0.005em | Sora | Subsection, dialog title |
| `body-lg` | 18 / 28 | 1.125rem / 1.75rem | 400 | 0em | Inter | Intro paragraphs, marketing body |
| `body` | 16 / 26 | 1rem / 1.625rem | 400 | 0em | Inter | Default text |
| `small` | 14 / 22 | 0.875rem / 1.375rem | 400 | 0em | Inter | Dense UI, tables, form labels (500) |
| `caption` | 12 / 18 | 0.75rem / 1.125rem | 400 | 0.005em | Inter | Hints, metadata, badges (500) |
| `code` | 13 / 20 | 0.8125rem / 1.25rem | 400 | 0em | JetBrains Mono | Payloads, logs, endpoints, IDs |

### Typography rules
1. **Sentence case** for headings, buttons, menus, labels, tabs. No all caps (the `CONCTR` wordmark is the one exception).
2. **Line length** 45 to 75 characters for reading text (`max-width: 68ch`). Tables and forms are exempt.
3. **Headings:** Sora, tight tracking. Space above a heading should be about twice the space below it. Never skip levels (h1, then h3).
4. **Labels vs hints:** labels use `small` at weight 500. Hints and metadata use `caption` in `--cn-text-subtle`.
5. **Numbers in tables and stats** use tabular figures: `font-variant-numeric: tabular-nums`.
6. **Code, IDs, endpoints, headers, payloads, field paths** always use JetBrains Mono. Inline code gets a `--cn-sunken` background and `--cn-radius-xs`.
7. **Links** use `--cn-primary`, underlined with 3 px offset in running text. Navigation links drop the underline.
8. **Do not** letter-space body text, justify text, or set long passages in bold or italic.
9. **Minimum text size** is 12 px (`caption`), and only for secondary information. Body text on mobile is at least 16 px (this also stops iOS zooming into inputs).

---

## 2. Spacing

Base unit **4 px**. The token number times 4 is the size in pixels. Numbers that are skipped on purpose (7, 9, 11, 13 to 15 and so on) keep the scale short and decisions fast.

| Token | px | rem | Typical use |
|---|---|---|---|
| `--cn-space-0` | 0 | 0rem | No space |
| `--cn-space-1` | 4 | 0.25rem | Icon to label, tight inline gaps |
| `--cn-space-2` | 8 | 0.5rem | Between related controls, badge padding, menu padding |
| `--cn-space-3` | 12 | 0.75rem | Input padding, gap between a label and its field group |
| `--cn-space-4` | 16 | 1rem | Default gap, button padding, compact card padding |
| `--cn-space-5` | 20 | 1.25rem | Rarely needed, in-between steps |
| `--cn-space-6` | 24 | 1.5rem | Card and modal padding, gap between groups |
| `--cn-space-8` | 32 | 2rem | Between sections inside a panel, desktop page margin |
| `--cn-space-10` | 40 | 2.5rem | Large gaps in dense pages |
| `--cn-space-12` | 48 | 3rem | Between page sections (app) |
| `--cn-space-16` | 64 | 4rem | Between page sections (docs, marketing) |
| `--cn-space-20` | 80 | 5rem | Marketing section padding |
| `--cn-space-24` | 96 | 6rem | Marketing hero padding |

### Spacing rules
1. **Proximity shows relationship.** Related items are closer (space-2 to space-4) than unrelated groups (space-6 and up).
2. **Inside components** use space-1 to space-4. **Between components** use space-4 to space-8. **Between page sections** use space-12 and up.
3. **Page margins:** space-4 on mobile, space-6 on tablet, space-8 on desktop. **Grid gutter:** space-4 on mobile, space-6 from `md`.
4. **Component padding:** buttons 0 x space-4 (space-3 small, space-6 large), inputs space-3, cards space-6 (space-4 compact), modals space-6, table cells 14 px by space-4 (space-2 by space-4 dense), alerts space-3 by space-4, menus space-1.
5. Use `gap` for spacing between siblings (`.cn-stack`, `.cn-cluster`). Avoid margin on the last child.
6. Never use a value that is not on the scale, including 10 px or 14 px "just to fit". Change the layout instead.

---

## 3. Border radius

| Token | px | Use |
|---|---|---|
| `--cn-radius-xs` | 4 | Tiny chips, inline code, keyboard keys |
| `--cn-radius-sm` | 6 | Small controls, tags, tooltips |
| `--cn-radius-md` | 10 | Buttons, inputs, menus, alerts |
| `--cn-radius-lg` | 14 | Cards, panels, tables wrapper |
| `--cn-radius-xl` | 20 | Modals, large surfaces |
| `--cn-radius-pill` | 999 | Badges, switches, avatars (with 50%) |
| `--cn-radius-full` | 50% | Avatars, radio buttons, spinners, dots |

### Radius rules
1. **Nest radius correctly:** inner radius = outer radius minus the padding between them (a 14 px card with 6 px padding holds a 8 px element, so use `sm` or `md`).
2. One radius per level of hierarchy. Do not mix `md` and `lg` on siblings.
3. Controls inside the same row (button next to input) share the same radius (`md`).
4. Images inside cards follow the card radius at the corners they touch.
5. Never round only some corners unless the element is attached to another surface (tabs, a toast bar at the screen edge).

---

## 4. Borders and dividers

| Token | Value | Use |
|---|---|---|
| `--cn-border-1` | 1 px | Default border for cards, inputs, tables, dividers |
| `--cn-border-2` | 2 px | Focus outlines, selected tab underline, checkbox and radio edges |
| `--cn-border` (color) | see colors | Dividers and container edges (low emphasis) |
| `--cn-border-strong` (color) | see colors | Input edges, button edges (secondary), anything interactive |

Use a border or a shadow to separate a surface from its background, rarely both. In dark mode prefer a border; shadows are barely visible on navy.

---

## 5. Elevation (shadows) and layering

Four levels. Higher means closer to the user. Dark mode shadows include a 1 px white ring because black shadows disappear on dark surfaces.

| Token | Use | Light value | Dark value |
|---|---|---|---|
| `--cn-shadow-1` | Resting raised elements: cards, inputs on hover | `0 1px 2px rgba(18,25,51,.06), 0 1px 3px rgba(18,25,51,.08)` | `0 0 0 1px rgba(255,255,255,.04), 0 1px 2px rgba(0,0,0,.4)` |
| `--cn-shadow-2` | Sticky bars, floating cards | `0 2px 4px rgba(18,25,51,.06), 0 4px 16px rgba(18,25,51,.08)` | `0 0 0 1px rgba(255,255,255,.05), 0 4px 16px rgba(0,0,0,.45)` |
| `--cn-shadow-3` | Menus, popovers, tooltips, toasts | `0 4px 8px rgba(18,25,51,.06), 0 12px 32px rgba(18,25,51,.12)` | `0 0 0 1px rgba(255,255,255,.06), 0 12px 32px rgba(0,0,0,.55)` |
| `--cn-shadow-4` | Modals and dialogs | `0 8px 16px rgba(18,25,51,.08), 0 24px 64px rgba(18,25,51,.18)` | `0 0 0 1px rgba(255,255,255,.08), 0 24px 64px rgba(0,0,0,.65)` |
| `--cn-focus-ring` | Soft halo around focused fields (with the 2 px outline) | `0 0 0 3px rgba(79,70,229,.28)` | `0 0 0 3px rgba(165,176,255,.35)` |
| `--cn-scrim` | Backdrop behind modals | `rgba(11,16,32,.5)` | `rgba(0,0,0,.6)` |

### Elevation rules
1. **Flat by default.** Most UI sits at level 0 with a border. Add elevation only to show something is floating or interactive.
2. Elevation matches layering: cards 1, sticky bars 2, menus, popovers and toasts 3, modals 4.
3. Never stack two elevated surfaces of the same level on top of each other.
4. Use `--cn-raised` as the background of floating surfaces (it is slightly lighter than `--cn-surface` in dark mode).
5. Hover on a clickable card goes from border only to shadow-2, over `--cn-dur`.

### Z-index (layers)
| Token | Value | Layer |
|---|---|---|
| `--cn-z-base` | 0 | Normal flow |
| `--cn-z-sticky` | 100 | Sticky headers, sidebars |
| `--cn-z-dropdown` | 200 | Menus, selects, popovers |
| `--cn-z-overlay` | 300 | Drawers, side panels |
| `--cn-z-modal` | 400 | Modals and their scrim |
| `--cn-z-toast` | 500 | Toasts and banners |
| `--cn-z-tooltip` | 600 | Tooltips (always on top) |

Never use a raw z-index. Use the token for the layer.

---

## 6. Sizing

### Control heights
| Token | px | Use |
|---|---|---|
| `--cn-control-sm` | 32 | Dense tables, toolbars, secondary actions |
| `--cn-control-md` | 40 | Default for buttons, inputs, selects |
| `--cn-control-lg` | 48 | Marketing, auth screens, mobile primary actions |

Touch targets must be at least 44 x 44 px on touch devices. Use `lg` controls or extra padding on mobile.

### Icon sizes
16 px (inline with small text, badges), 20 px (buttons, menus), 24 px (nav, headers), 32 px (empty states, cards), 48 px (marketing, hero). Below 20 px use the `mono` icons. Icons align to the text baseline and share the text color unless they are protocol icons.

### Avatar sizes
24 (tables), 32 (menus, comments), 40 (default), 48 (profile headers).

### Containers
| Token | px | Use |
|---|---|---|
| `--cn-container-prose` | 720 | Articles, docs text, long forms |
| `--cn-container-content` | 1200 | Default page content |
| `--cn-container-wide` | 1440 | Dashboards, wide tables |

### Breakpoints
Mobile first, `min-width` queries. CSS variables cannot be used in media queries, so use the numbers.

| Name | Min width | Typical layout |
|---|---|---|
| `sm` | 640 px | Large phone, one column |
| `md` | 768 px | Tablet portrait, two columns |
| `lg` | 1024 px | Tablet landscape, small laptop, sidebar appears |
| `xl` | 1280 px | Laptop and desktop, full app shell |
| `2xl` | 1536 px | Wide desktop, extra whitespace only |

Grid: 12 columns, gutter space-4 below `md` and space-6 from `md`. The app shell has a 240 px sidebar from `lg`, a bottom or drawer navigation below it.

---

## 7. Interaction states

| State | Treatment |
|---|---|
| Hover | Background moves one step: `surface` to `sunken`, `primary` to `primary-hover`, ghost to `primary-soft`. Duration `--cn-dur-fast`. |
| Pressed | Same as hover (no scale or bounce). |
| Focus (keyboard) | 2 px `--cn-focus` outline with 2 px offset. Fields also get `--cn-focus-ring`. Never remove it. |
| Selected, current | `--cn-primary-soft` background with `--cn-primary` text (nav items, pagination, tabs use a 2 px underline). |
| Disabled | `--cn-opacity-disabled` (.45), `cursor: not-allowed`. Explain why in a hint when it is not obvious. |
| Error | `--cn-danger` border and message, `aria-invalid="true"`. The message says what to do. |
| Loading | A `.cn-spinner` or skeleton. Keep the control width so the layout does not jump. Set `aria-busy="true"`. |

---

## 8. Motion

| Token | Value | Use |
|---|---|---|
| `--cn-dur-fast` | `120ms` | Hover, press, color changes, switches |
| `--cn-dur` | `200ms` | Menus, tabs, cards, shadow changes |
| `--cn-dur-slow` | `320ms` | Large layout moves, drawers (rare) |
| `--cn-ease` | `cubic-bezier(.2,.7,.2,1)` | Default: things that move or appear |
| `--cn-ease-in` | `cubic-bezier(.4,0,1,1)` | Things leaving the screen |
| `--cn-ease-out` | `cubic-bezier(0,0,.2,1)` | Things entering from off-screen |

1. Motion explains change (something opened, moved, finished). It is never decoration.
2. Animate `opacity`, `transform`, `background-color`, `box-shadow`. Do not animate layout properties.
3. One signature animation: the amber packet along a connection link (`.cn-link`).
4. Respect `prefers-reduced-motion`: the kit already stops the packet and shimmer, and shortens transitions.

---

## 9. Opacity

| Token | Value | Use |
|---|---|---|
| `--cn-opacity-disabled` | .45 | Disabled controls |

Do not use opacity to create new text colors. Use `--cn-text-muted` and `--cn-text-subtle`. Allowed exceptions: the logo outline and faint decorative patterns.

---

## 10. Platform mapping

| Idea | CSS | Tailwind (with the provided config) | Rust (`conctr_tokens.rs`) |
|---|---|---|---|
| Primary color | `var(--cn-primary)` | `bg-cn-primary` | `light::PRIMARY`, `dark::PRIMARY` |
| Body size | `var(--cn-fs-body)` | `text-body` | `font_size::BODY` |
| Space 4 | `var(--cn-space-4)` | `p-4` | `space::S4` |
| Radius md | `var(--cn-radius-md)` | `rounded-md` | `radius::MD` |
| Shadow 2 | `var(--cn-shadow-2)` | `shadow-2` | n/a (draw a soft shadow with the same blur) |
| Control medium | `var(--cn-control-md)` | `h-10` | `control::MD` |

In egui, load the font files from `fonts/files/` (convert to TTF if needed), register family names from `fonts::*`, and build `egui::Visuals` from the `light` and `dark` color modules. Corner radius, spacing and font sizes map one to one.

---

## 11. Adding or changing tokens
1. Check that an existing token cannot do the job.
2. Name it by role, not by value (`--cn-text-subtle`, not `--cn-grey-500`).
3. Provide light and dark values if it is a color or shadow.
4. Check contrast (AA) for any text color against its backgrounds.
5. Update all four formats and this document in the same change, and bump the kit version.
