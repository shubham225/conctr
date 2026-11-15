# Conctr UI instructions

Read the root instructions, [brand kit](../../design/docs/AI-INSTRUCTIONS.md),
[theming](../../docs/development/theming.md) and [UPSTREAM.md](UPSTREAM.md).

- `src/rust-ui/` is an immutable MIT upstream snapshot. Never edit or format it.
- `src/ui/` is the app-facing facade. Apply customization here or in shared theme tokens.
- `src/composed/` combines UI controls. Keep services and business rules in the app/platform.
- `assets/colors/tokens.css` preserves kit values with provider-scoped selectors; fonts, logos and icons copy `design/`.
- Tailwind 4.1 tooling and the shipped CSS belong here. Consumers mount ThemeProvider.
- Run vendor integrity checks, crate/workspace checks and browser verification after changes.
- Update the source pin, license and hash manifest together when refreshing upstream.
