# Rust UI adoption assessment

Reviewed 2026-11-15 against source commit
`4f5a516e76d75a15a587ab2169352cf34fd318d0` of
[Rust UI](https://github.com/rust-ui/ui).

## Theme fit

Yes. Rust UI uses Tailwind 4 semantic roles such as `primary`, `background`,
`foreground`, `border` and `ring`. Conctr maps these to the canonical
[brand tokens](../../design/colors/tokens.css), including light/dark palettes,
Sora, Inter and JetBrains Mono. The app-facing facade adds sizing and radius
classes without changing upstream code. Upstream components also contain fixed
sizes, arbitrary values and illustrative color classes; the brand bridge alone
does not guarantee every demo meets every brand/accessibility rule. Review and
adapt additional families through wrappers as product features adopt them.

## Cost and license

The pinned [MIT license](https://github.com/rust-ui/ui/blob/4f5a516e76d75a15a587ab2169352cf34fd318d0/LICENSE)
allows commercial use, copying, modification, distribution and sale without a
license fee or royalty. Retain its copyright and permission notice in copies or
substantial portions. It carries no warranty. No subscription is needed to use
this public snapshot. This finding covers this source, not unspecified premium
services, future releases, trademarks or a guarantee against every legal claim.
Third-party dependencies and assets retain their own licenses. Brand fonts include
SIL OFL notices; Rust UI's `icons` package includes its own icon licensing terms.

## Source and workflows

The [upstream README](https://github.com/rust-ui/ui/blob/4f5a516e76d75a15a587ab2169352cf34fd318d0/README.md)
describes Dioxus 0.7, Tailwind 4 and a copy-source registry rather than a component
crate dependency. The website's installation prose still contains Leptos-only
statements; the pinned Dioxus source is the implementation evidence.
The requested https://rust-ui.com/workflows page could not be retrieved by the
research browser. Its workflow component, state hook and eight examples are
present in the MIT repository and are included in the new gallery.

AutoForm is explicitly a Dioxus shell in source, not automatic form generation.
Rust UI describes itself as experimental and expects breaking changes. All 86 UI
module names are exposed, not all are equivalent to the Leptos implementations.

## Replacement and recovery

At the user's explicit direction, the old UI crate, Tailwind setup and gallery
were deleted after source backup, then recreated. The original `design/` remains
canonical and unchanged by this migration. The migration backup was recorded at
`backups/pre-rust-ui-20261115T112238Z`, but the archive and inventory are not
present in this checkout. Recovery instructions depend on obtaining that archive;
do not assume it is available locally.

## Validation

Verification results are recorded here as checks complete. Native SDK behavior,
backend integration and a full accessibility audit are outside these demo claims.

Initial verification passed workspace tests, upstream integrity and documentation
validation. Chrome verified brand colors/fonts, 40px controls, disabled states,
dark switching, nested light palettes, narrow layout, OTP editing, reduced motion,
and Conctr workflow rendering/zoom. The full catalog sweep remains incomplete.
Carousel exposed an upstream inline-script/SSR marker syntax error; the showcase
now defaults to client rendering. SSR compatibility of all upstream demos is not
established. Upstream doctests contain a non-compilable VirtualFor fragment, so
library doctests are disabled without editing that source; the facade has an
independent consumer/document registration test.

Final Rust validation: workspace check, workspace tests, formatting, strict Clippy,
documentation validation, vendor integrity and all four backup checksums passed.
The client-rendering change has compiled; its complete browser catalog sweep
remains unverified after the interrupted Chrome session.

Brand feedback corrections: the Conctr facade now uses pill badges with soft
semantic colors and 24px default height, semantic alert backgrounds with the
kit's 10px radius, and native toast rendering on the raised surface. The showcase ThemeToggle controls application-provided theme state instead of
the upstream global HTML class. ThemeProvider remains application-controlled.
All upstream hashes remain unchanged.

Correction verification: 15 targeted Chrome checks passed for light/dark palettes
and logos, nested light scope, control sizing/fonts, narrow layout, OTP, workflow
zoom, reduced motion, pill badge sizing, soft alert surface and toast open/dismiss.
Facade tests, targeted strict Clippy, formatting, documentation validation and
460-file upstream integrity checks passed. The verification servers, Chrome
profiles and temporary download helpers were confirmed stopped. This is targeted
evidence, not a full visual-parity audit of all 86 upstream families.

Pointer/theme and compositions verification: 19 targeted Chrome checks pass,
including a real mouse click while keeping the pointer over the theme button,
dark palette/logo updates, and all four Composed routes. The dark variant no
longer depends on CSS @scope. Targeted gallery compilation and strict Clippy,
formatting, docs and upstream integrity checks pass. Verification processes
were stopped through the shell's cleanup trap. This does not establish native
WebView rendering behavior or complete interaction coverage for every upstream
component.

Grid/Command corrections: the Conctr root hook adapter removes its document
mousedown listener on unmount, avoiding the dropped-signal panic without editing
upstream. Border defaults now use the kit token, tables/grids use kit header/cell
surfaces and spacing, and Command selection explicitly sets soft-primary
background with primary text. Ten Chrome regression checks pass with no recorded
exceptions, including three dark SPA grid unmount/remount click cycles and dark
Command selection. Targeted compilation/strict Clippy, formatting, docs and
upstream integrity checks pass. All temporary browser/server processes stopped.

Control and composition refinement: 32 targeted Chrome checks pass for kit
geometry, theme switching, chips, checkbox local state, Command surface, phone
height, tabs, custom calendar/month navigation, date selection, tag add/remove,
input-group borders, connection packet animation and reduced motion. Upstream
source remains unchanged. Compositions now have separate files; optional local
Checkbox state is implemented in the facade. Calendar tables are explicitly
excluded from data-table cell padding. Strict targeted Clippy and formatting pass.
These checks establish selected behavior/geometry, not visual parity of every
upstream variant or a full accessibility audit.

Final control-refinement verification: workspace tests passed, targeted strict
Clippy passed, and the shipped Tailwind bundle matches a clean regeneration.
All 460 upstream files retain their original hashes. The verification browser
and server exited; an orphaned server child from an earlier run was also stopped.

Calendar navigation refinement: composed Calendar and DatePicker now offer
separate month and year dropdowns. Popup showcase previews reserve 480 px and
allow visible overflow; wide data examples retain scrolling. UI crate tests,
targeted strict Clippy, formatting and upstream integrity checks pass.
The expanded browser script includes leap-day selection and popup-space checks,
but this run could not complete: headless Chrome stalled during initial gallery
navigation, including with fresh profiles. Browser verification of these latest
changes remains pending; earlier browser results above cover the previous state.
