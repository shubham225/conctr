# Conctr brand kit

Brand: **Conctr**, where systems meet. Logo concept: hex orbit.

## Start here
- **Designers and marketers:** `docs/BRAND-GUIDELINES.md`, then `logo/` and `colors/palette-preview.png`.
- **Application developers** use the public controls and compositions in `conctr-ui-dioxus` and mount `ThemeProvider` for fonts, colors and utilities. See the [Dioxus component guide](docs/COMPONENTS.md#dioxus-application-components).
- **HTML kit users** load `fonts/fonts.css`, `colors/tokens.css` and `ui-kit/conctr-ui.css` in order. Open `ui-kit/index.html` for the light and dark reference pages.
- **AI tools and agents:** give them `docs/AI-INSTRUCTIONS.md` and `brand.json` (add `docs/COMPONENTS.md` for UI work).

## What is inside
```
README.md
brand.json                      Machine-readable brand data
docs/
  BRAND-GUIDELINES.md           Full guide for people
  DESIGN-TOKENS.md              Typography, spacing, radius, shadow, motion, sizing
  COMPONENTS.md                 Specs for every component and pattern
  COMPONENT-COVERAGE.md         Conctr control families and authoring components
  AI-INSTRUCTIONS.md            Rules, cheat sheets and prompts for AI tools
logo/
  svg/                          icon, wordmark, horizontal, stacked in 4 variants
                                (color, color-on-dark, black, white) + app-icon.svg
  png/                          Raster exports of the main files
  favicon/                      favicon.svg and PNG 16, 32, 48, 180, 192, 512
icons/
  {color,mono}/protocols/       Protocol artwork
  {color,mono}/authentication/  Authentication artwork
  {color,mono}/patterns/        65 paired pattern icons grouped by book chapter
  icons.json                    Icon inventory
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
  index.html, forms.html,       Live component showcase (5 pages), showcase.css/js
  navigation.html, overlays.html, data.html
assets/
  og-image.svg / .png           1200 x 630 social card
```

## Notes
- Fonts are bundled (self-hosted WOFF2, Latin subset, SIL OFL). Install the full families for print and design tools. See `fonts/README.md`.
- The logo wordmark is drawn as paths, so it needs no font.
- `assets/og-image` contains the tagline as live text in Sora, so install Sora before re-exporting it.
- Move `fonts/`, `colors/` and `ui-kit/` together: the CSS uses relative paths between them.
- Using Tailwind v4? Load `ui-kit/conctr-ui.layered.css` instead of `conctr-ui.css`. Unlayered CSS beats Tailwind's layered utilities, so with the plain file `p-4` or `rounded-none` on a component would be ignored (tested with the Tailwind CLI).
- The color tokens were contrast-checked for WCAG AA. See the table in the guidelines.
- This kit is a design starting point. It is not a trademark clearance. Check the name "Conctr" for availability before you commit to it publicly.
