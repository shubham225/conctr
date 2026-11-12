# Conctr brand kit

Brand: **Conctr**, where systems meet. Logo concept: hex orbit.

## Start here
- **Designers and marketers:** `docs/BRAND-GUIDELINES.md`, then `logo/` and `colors/palette-preview.png`.
- **Developers:** `colors/tokens.css`, `ui-kit/conctr-ui.css`, and open `ui-kit/index.html` in a browser (it has a light/dark switch).
- **AI tools and agents:** give them `docs/AI-INSTRUCTIONS.md` and `brand.json`.

## What is inside
```
README.md
brand.json                      Machine-readable brand data
docs/
  BRAND-GUIDELINES.md           Full guide for people
  AI-INSTRUCTIONS.md            Rules and prompts for AI tools
logo/
  svg/                          icon, wordmark, horizontal, stacked in 4 variants
                                (color, color-on-dark, black, white) + app-icon.svg
  png/                          Raster exports of the main files
  favicon/                      favicon.svg and PNG 16, 32, 48, 180, 192, 512
icons/
  color/  mono/  icons.json     12 protocol and auth icons
colors/
  tokens.css                    Light + dark design tokens (CSS variables)
  colors.json                   Same tokens as JSON
  tailwind.config.js            Tailwind mapping
  palette-preview.svg / .png
ui-kit/
  conctr-ui.css                 Component styles
  index.html                    Live component showcase
assets/
  og-image.svg / .png           1200 x 630 social card
```

## Notes
- Fonts load from Google Fonts: Sora, Inter, JetBrains Mono. Install them locally for offline or print use. All three are open source.
- The logo wordmark is drawn as paths, so it needs no font.
- `assets/og-image` contains the tagline as live text in Sora, so install Sora before re-exporting it.
- The color tokens were contrast-checked for WCAG AA. See the table in the guidelines.
- This kit is a design starting point. It is not a trademark clearance. Check the name "Conctr" for availability before you commit to it publicly.
