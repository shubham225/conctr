# Fonts

| Family | Role | Weights included | License |
|---|---|---|---|
| Sora | Display and headings | 500, 600, 700 | SIL OFL 1.1 |
| Inter | UI and body | 400, 500, 600 | SIL OFL 1.1 |
| JetBrains Mono | Code and data | 400, 500 | SIL OFL 1.1 |

Files are WOFF2, Latin subset (covers English and most Western European languages), self-hosted so the product works offline and does not send visitor data to a font CDN.

**Use on the web:** `<link rel="stylesheet" href="fonts/fonts.css">` before `colors/tokens.css`. Keep the `files/` folder next to `fonts.css`.

**Use in desktop or mobile (Dioxus, egui):** bundle the `.woff2` files, or convert to `.ttf` and register them with your UI toolkit. Family names are in `colors/conctr_tokens.rs`.

**Need more languages?** Get extra subsets (latin-ext, cyrillic, greek, vietnamese) from the `@fontsource/*` npm packages and add matching `@font-face` rules with `unicode-range`.

**Print and design tools:** download the full families from Google Fonts. Licenses are in `licenses/`. The OFL allows embedding in apps and redistribution with the license text, but the fonts may not be sold on their own.

**Fallback stacks** are defined in `--cn-font-display`, `--cn-font-ui` and `--cn-font-mono`. Layouts must not break if a fallback renders.
