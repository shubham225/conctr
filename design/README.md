# Conctr brand kit v1.1

Brand: **Conctr**, where systems meet. Logo concept: hex orbit.

## Start here
- **Designers and marketers:** `docs/BRAND-GUIDELINES.md`, then `logo/` and `colors/palette-preview.png`.
- **Developers:** load `fonts/fonts.css`, `colors/tokens.css`, `ui-kit/conctr-ui.css` in that order, and open `ui-kit/index.html` in a browser (light/dark switch). Rust UIs: `colors/conctr_tokens.rs`. Specs: `docs/DESIGN-TOKENS.md`, `docs/COMPONENTS.md`.
- **AI tools and agents:** give them `docs/AI-INSTRUCTIONS.md` and `brand.json` (add `docs/COMPONENTS.md` for UI work).

## What is inside
```
README.md
brand.json                      Machine-readable brand data
docs/
  BRAND-GUIDELINES.md           Full guide for people
  DESIGN-TOKENS.md              Typography, spacing, radius, shadow, motion, sizing
  COMPONENTS.md                 Specs for every component and pattern
  AI-INSTRUCTIONS.md            Rules, cheat sheets and prompts for AI tools
logo/
  svg/                          icon, wordmark, horizontal, stacked in 4 variants
                                (color, color-on-dark, black, white) + app-icon.svg
  png/                          Raster exports of the main files
  favicon/                      favicon.svg and PNG 16, 32, 48, 180, 192, 512
icons/
  color/  mono/  icons.json     12 protocol and auth icons
fonts/
  fonts.css, files/*.woff2      Sora, Inter, JetBrains Mono (self-hosted), licenses/
colors/
  tokens.css                    All design tokens (CSS variables), light + dark
  design-tokens.json            Every token as JSON (type, space, radius, shadow...)
  colors.json                   Color tokens only
  tailwind.config.js            Tailwind mapping
  conctr_tokens.rs              Rust constants (egui or native UIs)
  palette-preview.svg / .png
ui-kit/
  conctr-ui.css                 Component styles
  index.html                    Live component showcase
assets/
  og-image.svg / .png           1200 x 630 social card
```

## Notes
- Fonts are bundled (self-hosted WOFF2, Latin subset, SIL OFL). Install the full families for print and design tools. See `fonts/README.md`.
- The logo wordmark is drawn as paths, so it needs no font.
- `assets/og-image` contains the tagline as live text in Sora, so install Sora before re-exporting it.
- Move `fonts/`, `colors/` and `ui-kit/` together: the CSS uses relative paths between them.
- The color tokens were contrast-checked for WCAG AA. See the table in the guidelines.
- This kit is a design starting point. It is not a trademark clearance. Check the name "Conctr" for availability before you commit to it publicly.

## Changelog
- **1.1** Added bundled fonts, full token set (type scale, spacing, radius, shadows, z-index, motion, sizing), responsive type, Rust and JSON token files, 15 more components (textarea, checkbox, radio, avatar, progress, spinner, skeleton, menu, tooltip, modal, toast, breadcrumb, pagination, sidebar nav, stat), `DESIGN-TOKENS.md` and `COMPONENTS.md`.
- **1.0** Logo set, icons, color tokens, base UI kit, guidelines, AI instructions.
