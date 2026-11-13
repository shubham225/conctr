# Conctr brand guidelines

Version 1.1. This document is for designers, developers, marketers and partners. AI tools should read `AI-INSTRUCTIONS.md` first, then this file for detail.

## 1. Brand essence

**Name.** Conctr (pronounced "connector"). A play on *connector*, with the vowels removed the way an engineer would shorten a word.

**Tagline.** Where systems meet.

**What we do.** Conctr connects systems that speak different protocols (REST, SOAP, OData and more) and use different authentication methods, so teams can integrate without writing custom glue code.

**Personality.** Clear, precise, calm, engineer to engineer. We are the dependable layer in the middle, so we never shout.

**Positioning line.** Any system. Any protocol. One connection.

### Naming rules
- In running text write **Conctr** (capital C, lowercase rest).
- **CONCTR** in capitals appears only as the logo wordmark.
- Never write ConCTR, Con.ctr, conctr (in headings), or Conctr™ unless legal asks for it.
- Product areas use plain nouns: Conctr connections, Conctr mappings, Conctr run history.

## 2. The logo

The mark is a **hex orbit**. A solid hexagon at the centre is Conctr. Three nodes connect to it, each a different shape: a circle, a square and a triangle. Different systems, different protocols, one core.

### Files
| Use | File |
|---|---|
| Light backgrounds | `logo/svg/*-color.svg` |
| Dark backgrounds | `logo/svg/*-color-on-dark.svg` |
| One color, dark | `logo/svg/*-black.svg` |
| One color, white (photos, dark fills) | `logo/svg/*-white.svg` |
| Favicon | `logo/favicon/favicon.svg` and PNG sizes |
| App icon, social avatar | `logo/svg/app-icon.svg`, `logo/png/app-icon-1024.png` |

Lockups: **horizontal** (default), **stacked** (square spaces, posters), **icon** (avatars, favicons, small spaces), **wordmark** (only when the icon already appears nearby).

### Clear space and size
- Keep clear space of **2x** on every side, where x is the diameter of the circle node (about 17% of the icon width).
- Minimum sizes: icon 16 px, horizontal lockup 96 px wide, stacked lockup 80 px wide. In print: icon 6 mm, horizontal 25 mm.
- Below 24 px, use `favicon.svg`, which sits on a dark tile for legibility.

### Do not
- Recolor the nodes or swap their colors.
- Rotate, skew, outline, add shadows or gradients.
- Redraw the wordmark in a font. The letters are drawn as paths on purpose.
- Place the `color` version on a dark background, or `color-on-dark` on a light one.
- Put the logo on busy photos. Use the `white` version on a dark overlay.
- Change the shape of a node. Shapes carry meaning (see section 5).

## 3. Color

Four brand colors on a cool navy-tinted neutral scale. Every color has a light-mode and a dark-mode value. Always use tokens, never raw hex, in product work.

| Role | Light | Dark | Used for |
|---|---|---|---|
| Indigo (primary) | `#4F46E5` | `#818CF8` | Primary actions, links, selected states, the logo core |
| Teal | `#14B8A6` | `#2DD4BF` | REST nodes, success, positive data |
| Coral | `#F2603C` | `#FF8A6B` | SOAP nodes, attention, secondary data |
| Amber | `#F5A524` | `#FBBF24` | OData nodes, warnings, the "packet" accent |
| Ink | `#121933` | `#EEF0F7` | Text |

**Usage ratio.** About 70% neutrals, 20% indigo, 10% teal, coral and amber combined. Teal, coral and amber are accents. Do not use them for large fills.

**Text on color.** Raw teal, coral and amber are too light for small text on white. Use the `-text` tokens (`--cn-teal-text` and so on) for text, and the raw colors for shapes, fills and icons.

Full token list: `colors/tokens.css` (CSS), `colors/colors.json`, `colors/tailwind.config.js`. Preview: `colors/palette-preview.png`.

### Contrast (computed)
Normal text needs 4.5:1, large text and UI components 3:1 (WCAG 2.2 AA).

**Light mode**

| Foreground | Background | Ratio | Passes |
|---|---|---|---|
| `text` #121933 | `bg` #F7F8FC | 16.3:1 | AA text |
| `text-muted` #4B5578 | `bg` #F7F8FC | 6.9:1 | AA text |
| `text-subtle` #6B7494 | `surface` #FFFFFF | 4.6:1 | AA text |
| `primary` #4F46E5 | `surface` #FFFFFF | 6.3:1 | AA text |
| `on-primary` #FFFFFF | `primary` #4F46E5 | 6.3:1 | AA text |
| `teal-text` #0F766E | `surface` #FFFFFF | 5.5:1 | AA text |
| `coral-text` #C2410C | `surface` #FFFFFF | 5.2:1 | AA text |
| `amber-text` #B45309 | `surface` #FFFFFF | 5.0:1 | AA text |
| `danger` #DC2626 | `surface` #FFFFFF | 4.8:1 | AA text |
| `on-danger` #FFFFFF | `danger` #DC2626 | 4.8:1 | AA text |

**Dark mode**

| Foreground | Background | Ratio | Passes |
|---|---|---|---|
| `text` #EEF0F7 | `bg` #0B1020 | 16.6:1 | AA text |
| `text-muted` #AEB4CC | `bg` #0B1020 | 9.2:1 | AA text |
| `text-subtle` #8089AA | `surface` #121933 | 5.0:1 | AA text |
| `primary` #818CF8 | `surface` #121933 | 5.8:1 | AA text |
| `on-primary` #0B1020 | `primary` #818CF8 | 6.3:1 | AA text |
| `teal-text` #2DD4BF | `surface` #121933 | 9.3:1 | AA text |
| `coral-text` #FF8A6B | `surface` #121933 | 7.5:1 | AA text |
| `amber-text` #FBBF24 | `surface` #121933 | 10.4:1 | AA text |
| `danger` #F87171 | `surface` #121933 | 6.3:1 | AA text |
| `on-danger` #0B1020 | `danger` #F87171 | 6.8:1 | AA text |


### Data visualization order
Indigo, teal, coral, amber, then tints of indigo. Always pair color with the shape language or a label so charts work without color vision.

## 4. Typography

Full reference (all tokens, rules, platform mapping): `DESIGN-TOKENS.md`. Font files are bundled in `fonts/` (self-hosted WOFF2, open source licenses).

| Role | Font | Weights | Notes |
|---|---|---|---|
| Display and headings | Sora | 600, 700 (500 available) | Geometric, matches the drawn wordmark. Tight tracking. |
| UI and body | Inter | 400, 500, 600 | Interface text, docs, forms. |
| Code and data | JetBrains Mono | 400, 500 | Payloads, logs, endpoints, IDs. |

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

Key rules: sentence case everywhere; reading lines 45 to 75 characters; tabular numbers in tables; mono for anything a person might copy (IDs, URLs, field paths); no all caps (except the wordmark); body text is at least 16 px on mobile; display, h1 and h2 step down below 640 px.

## 5. Shape language and iconography

The three nodes in the logo are a system. The same shapes and colors identify protocols everywhere in the product, so a person can recognise a connection at a glance.

| Shape | Color | Meaning |
|---|---|---|
| Circle | Teal | REST / HTTP |
| Square (rounded) | Coral | SOAP / XML |
| Triangle | Amber | OData |
| Diamond | Teal | GraphQL |
| Hexagon | Indigo | gRPC, or Conctr itself |
| Ring | Coral | Webhooks and events |

Auth icons (OAuth 2.0, API key, Basic, JWT, mTLS, SAML) use indigo as the base with small teal, coral or amber details.

**Icon set** (`icons/`): 12 protocol and auth icons on a 64 px grid. Each sits inside a faint hexagon outline.
- `icons/color/` for marketing, docs and empty states.
- `icons/mono/` use `currentColor`, for toolbars and inline UI.
- Render sizes: 16, 20, 24, 32, 48 px. Below 20 px prefer the mono set.

**Adding new icons.** Keep the 64 px grid, the faint hexagon outline, one central glyph, 2.5 px strokes with round caps, and only brand colors. Choose a shape that is not already assigned. Register it in `icons/icons.json`.

## 6. UI principles

1. **Show the connection.** The source, hub and target flow (`.cn-flow`) is the signature component. Use it wherever a connection is summarised.
2. **Shape plus color.** Never use color alone to carry meaning. Add a shape, label or icon.
3. **Calm by default.** Neutral surfaces, one primary action per view, indigo only where the user can act.
4. **Say what happened and what to do.** Errors name the cause and the fix. They do not apologise.
5. **Density for operators.** Tables and logs are first class. Use 14 px body in dense views.

**Spacing.** 4 px base unit. Scale: 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96 (`--cn-space-1` to `--cn-space-24`). Related items are closer than unrelated ones. Page margin 16 / 24 / 32 px (mobile / tablet / desktop).

**Grid and breakpoints.** 12 columns. Breakpoints 640, 768, 1024, 1280, 1536 px (mobile first). Content width 1200 px, prose 720 px, wide 1440 px.

**Radius.** xs 4 (keys, inline code), sm 6 (tags, tooltips), md 10 (buttons, inputs, menus), lg 14 (cards), xl 20 (modals), pill (badges). Nested radius = outer radius minus padding.

**Elevation.** Flat with a 1 px border by default. Four shadow levels: 1 cards, 2 sticky bars, 3 menus and toasts, 4 modals. Dark mode adds a faint 1 px ring because black shadows vanish on navy.

**Control sizes.** 32, 40 (default) and 48 px heights. Touch targets are at least 44 px.

**Components** (`ui-kit/index.html`, `ui-kit/conctr-ui.css`, specs in `COMPONENTS.md`): buttons, inputs, textarea, select, checkbox, radio, switch, shape and status badges, cards, alerts, toasts, tabs, breadcrumb, pagination, sidebar nav, table, stat, code block, avatar, progress, spinner, skeleton, menu, tooltip, modal, connection flow, empty state. All states (hover, focus, disabled, error) are implemented.

**Accessibility.** Visible 2 px focus ring on every interactive element. Targets at least 32 px (40 px default). Respect `prefers-reduced-motion`. All token pairs listed in section 3 meet AA.

## 7. Dark and light mode

- Light is the default. Dark follows the OS (`prefers-color-scheme`) unless the user chooses a theme.
- Switch by setting `data-theme="light"` or `data-theme="dark"` on `<html>`.
- Dark mode is not an inversion. Brand colors get brighter variants so they hold contrast on navy.
- Logos: use `*-color.svg` in light and `*-color-on-dark.svg` in dark. The UI kit shows the swap pattern.

## 8. Motion

- Durations 120 ms (hover, press) and 200 ms (open, close). Easing `cubic-bezier(.2,.7,.2,1)`.
- One signature animation: a small amber **packet** travels along a connection line from source to target. Use it for live connections and loading states, nowhere else.
- Do not animate the logo outside of a loading state. Always honour reduced motion.

## 9. Voice and writing

**Tone.** Plain, exact and calm. We explain what the system does, not how impressive it is.

| We say | We avoid |
|---|---|
| Connect two systems | Unlock seamless synergy |
| Map fields between the source and target | Leverage our AI-powered transformation engine |
| The target returned 401. Check the credentials. | Oops! Something went wrong |
| Create connection | Submit |

**Vocabulary.** connection, source, target, mapping, run, protocol, credentials. Use the same word for the same thing everywhere. A button called "Create connection" leads to a message that says "Connection created".

**Style.** Sentence case. Active voice. Contractions are fine. No exclamation marks in product UI. No emoji in product UI.

### Approved copy
- **Tagline:** Where systems meet.
- **Short (under 100 characters):** Conctr connects systems that speak different protocols.
- **Boilerplate:** Conctr is an integration platform that connects systems using different protocols (REST, SOAP, OData and more) and authentication methods, without custom glue code.
- **Alternate headlines:** Any system. Any protocol. One connection. / Connect what doesn't connect. / One hub for every API.

## 10. Imagery and layout motifs

- Prefer diagrams of real flows (source, hub, target) over stock photos.
- Backgrounds: flat `bg` or `surface`. One restrained decoration is allowed: a faint hexagon outline pattern at 5% opacity.
- Screenshots: use the real UI in both themes, with data that looks real (named systems, plausible payloads).
- Social images: `assets/og-image.png` (1200 x 630) as the base.

## 11. File and folder reference

```
logo/svg, logo/png, logo/favicon   Logos in every lockup and variant
icons/color, icons/mono            12 protocol and auth icons
fonts/                             Sora, Inter, JetBrains Mono (WOFF2) + fonts.css + licenses
colors/                            tokens.css, design-tokens.json, colors.json,
                                   tailwind.config.js, conctr_tokens.rs, palette preview
ui-kit/                            conctr-ui.css and index.html (open in a browser)
assets/                            Social card
brand.json                         Machine-readable brand data
docs/                              BRAND-GUIDELINES, DESIGN-TOKENS, COMPONENTS, AI-INSTRUCTIONS
```

## 12. Quick start for developers

```html
<link rel="stylesheet" href="fonts/fonts.css">
<link rel="stylesheet" href="colors/tokens.css">
<link rel="stylesheet" href="ui-kit/conctr-ui.css">
<body class="cn-body">
  <button class="cn-btn cn-btn--primary">Create connection</button>
  <span class="cn-badge"><i class="cn-shape" data-s="circle" style="--c:var(--cn-teal)"></i>REST</span>
</body>
```

Use `var(--cn-*)` tokens in your own CSS. Never hard-code hex, px, radius or shadow values for UI. See `DESIGN-TOKENS.md` and `COMPONENTS.md`.
