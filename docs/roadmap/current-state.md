# Current implementation evidence

Audit: **2026-11-15**. Source evidence after user-directed Rust UI replacement.

- `conctr-ui-dioxus` was backed up, deleted and rebuilt around 86 unmodified Rust UI
  component modules, hooks, demos and workflows at pinned MIT source revision
  `4f5a516e76d75a15a587ab2169352cf34fd318d0`.
- `ui/` is the app-facing facade; selected controls apply Conctr defaults.
  `composed/` combines controls. The former `components::*` API is removed.
- Palette/scales, fonts, logos and icons come from canonical `design/`.
  Library-owned Tailwind 4.1.18 ships compiled CSS; consumers mount ThemeProvider.
- `conctr-design-system` was backed up, deleted and rebuilt with brand foundations,
  78 primary component demos, eight workflow examples and light/dark switching.
  Its reference kit is a fresh copy of `design/`.
- Shared product App routes and unavailable-service pages remain presentation;
  their imports were migrated to the new facade. Native title-bar injection remains.
- Backend platform crates remain scaffolds; demo echo functions are not integration
  services. Workflow canvas state does not execute, persist or authorize flows.
- Upstream AutoForm is a shell, not automatic form generation. Rust UI is experimental.

Evidence: [UI source](../../crates/conctr-ui-dioxus/src/lib.rs),
[upstream inventory](../../crates/conctr-ui-dioxus/src/rust-ui/manifest.json),
[gallery catalog](../../apps/conctr-design-system/src/catalog.rs),
[assessment/validation](../development/rust-ui-adoption.md),
[backups](../../backups/pre-rust-ui-20261115T112238Z/README.md).
Earlier test/browser counts belong to the removed implementation and do not
validate this replacement. Current checks are recorded in the adoption assessment.

Brand facade corrections add semantic Alert variants, pill Badge styling, native
ToastTrigger feedback and a provider-scoped ThemeToggle. Fifteen targeted browser checks verify the corrected theme controls and selected
kit styles; full visual parity across all families remains unverified.

The Composed gallery section now includes Badges and shapes, Feedback, Empty state
and Connection flow from the canonical kit. Reusable protocol shapes/badges,
feedback messages and source/hub/target summaries live in the library facade layer.
The dark Tailwind variant uses a standard attribute selector instead of CSS @scope
to avoid reliance on scoped-style invalidation during pointer hover.

The grid outside-click hook now resolves through a Conctr lifecycle adapter that
removes document listeners on unmount and safely reads the element signal.
Shared border defaults use brand tokens; table/grid surfaces and Command
selection styles are corrected outside upstream source.

The UI facade now supplies local-state Checkbox behavior, kit Chips, icon theme
switching and alert-style toast content. Compositions live in individual files;
Calendar, DatePicker, Tags, TagInput and InputGroup have Composed gallery examples.
Connection summaries animate the amber packet with reduced-motion support.
Calendar day geometry is isolated from data-table rules; phone input, menus,
tabs, Command and grid/table skins use the kit's sizing, surfaces and borders.
Thirty-two targeted browser checks pass, including new date/tag interactions.

The composed Calendar now has independent month/year header selectors, also used
by the composed DatePicker. Gallery dropdown examples reserve 480 px of preview
height with visible overflow so their menus can open outside the preview border.
