# Current implementation evidence

**2026-11-22 product shell update.** The shared app now opens on Dashboard.
All eleven destinations are heading-only pages with grouped navigation, a desktop
border toggle, a compact Conctr mark, a mobile drawer and a theme toggle.
The mapping, pipeline, queue and credential screens described in the earlier
entries below are preserved in the pre-shell backup. See the
[active route inventory](../../crates/conctr-app-dioxus/README.md).

Baseline audit: **2026-11-15**. Package and asset inventory updated **2026-11-21**.

- `conctr-ui-dioxus` uses 86 unmodified Rust UI
  component modules, hooks, demos and workflows at pinned MIT source revision
  `4f5a516e76d75a15a587ab2169352cf34fd318d0`.
- The showcase exposes a dedicated `/icons` page and renders 65 pattern icons from `assets::icons`
  in eight families, plus six protocol and six authentication icons. The page displays labeled cards in responsive grids with a keyboard-accessible switch between color and monochrome SVGs. These are presentation assets for authoring.
- `ui/` is the app-facing facade; selected controls apply Conctr defaults.
  `composed/` combines controls. The former `components::*` API is removed.
- Palette/scales, fonts, logos and icons come from canonical `design/`.
  Library-owned Tailwind 4.1.18 ships compiled CSS; consumers mount ThemeProvider.
- `conctr-design-kit` provides brand foundations,
  78 primary component demos, eight workflow examples and light/dark switching.
  Its reference kit is a fresh copy of `design/`.
- Shared product App routes and unavailable-service pages remain presentation;
  their imports were migrated to the new facade. Native title-bar injection remains.
- The integration engine and connectors remain scaffolds. The credential reference
  directory uses server functions and a file-backed store. Workflow canvas state does not execute, persist or authorize flows.
- Upstream AutoForm is a shell, not automatic form generation. Rust UI is experimental.

Evidence: [UI source](../../crates/conctr-ui-dioxus/src/lib.rs),
[upstream inventory](../../crates/conctr-ui-dioxus/src/rust-ui/manifest.json),
[gallery catalog](../../apps/conctr-design-kit/src/catalog.rs),
[assessment/validation](../development/rust-ui-adoption.md).
The historical migration backup is not present in this checkout.
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

The Components gallery now includes Sidebar, a Conctr-owned navigation facade with independently collapsible
groups and a desktop icon rail. It uses the kit's 240 px desktop width, 36 px navigation
items, semantic selection, disabled items and a native modal drawer below 768 px.
Selection is caller-controlled; example destinations are placeholders. The upstream
sidenav snapshot remains unchanged.
Nine targeted sidebar browser checks pass for geometry, selection/disabled states,
dark tokens/logo, responsive visibility and native drawer focus/dismissal.
Workspace `cargo check`, formatting, the existing facade test and upstream integrity
checks pass.

The showcase now displays usage code with consumer imports, formatted compositions
and up to two additional registry examples per supported family. Its catalog and
navigation groups collapse; the Sidebar facade also offers desktop collapse.
Conctr-owned date and toast demos live in `src/demo/`. Calendar cells use native
buttons with visible selection, Shimmer supplies a CSS loading mask with inert
content and reduced-motion support, and OTP/phone controls use aligned kit borders.
Table/grid skins use quiet dividers and branded selection. Toasts reuse the facade
FeedbackMessage primitive; compositions re-export that primitive to preserve the
layer direction. These changes remain presentation demonstrations.

Validation for the showcase fixes: workspace check/test, formatting and Clippy
with warnings denied pass; upstream integrity verifies all 460 pinned files.
Sixteen targeted browser checks pass on the served web build, including mobile
containment of the grid and usage code. Documentation validation still reports
three existing links to the absent pre-replacement backup README.

The data showcase now uses Conctr-owned connection tables with integrated heading,
search/filter controls, status badges and selection summaries. Its mapping-grid
example edits target paths with independent cell drafts and reset; the original
advanced virtual grid remains an additional variant. Demo records are synthetic.
The shared Table facade adds branded table classes. ToastMessage provides a single
notification surface with semantic icon, feedback copy and inline dismissal; the
showcase separates trigger controls from message previews. These remain local UI
examples and do not connect to platform services.

Eighteen focused browser checks pass for the redesigned data/notification examples,
covering filtering, sorting, selection, empty states, mapping edit/commit/reset,
light/dark toast presentation, inline dismissal and mobile containment. Facade
tests, targeted Cargo check/Clippy, formatting and upstream integrity pass. Temporary
preview and browser processes used for this review are stopped after verification.

The showcase renders reusable composed demos from `composed/demos/`, and their
consumer usage code starts collapsed. Semantic tag variants and tag input match
the forms reference. Date, minute-precision time and combined local date/time
pickers expose caller-controlled values. Navigation collapses to a 56 px icon
rail on desktop and uses a native modal drawer on mobile. Command results include
keyboard hints, grouped actions and the kit's rounded/shadowed surface. Button
examples include ghost/icon variants; Button Action remains a separate page.
Drawer examples dismiss and restore focus. Repeated toast clicks append separate,
independently dismissible messages to a shared viewport inside ThemeProvider.
Thirty-three focused browser checks pass for these behaviors, including dark mode
and mobile containment. These are local presentation demos, not platform services.

The gallery is now titled Conctr Design Kit and ships from the `conctr-design-kit` package.
Its component explorer is a simple scrollable list, separate from the Sidebar
component. SidebarProvider shares state with SidebarTrigger placed in adjacent
main content; the Sidebar itself holds navigation and the mobile close control.
The composed FieldMappingEditor accepts caller-owned field schemas, links and
optional transform choices. Request/response examples show editable source →
transform → target rows with connectors, paths/types, add/remove controls and
container scrolling on narrow screens. Targets are offered once; sources may feed
multiple targets. Transform IDs configure a local presentation draft and are not
evaluated. Validation, persistence and executable mapping semantics remain with
the owning application/platform. Initial select values are serialized for SSR;
missing references remain explicitly visible.

Validation for the explorer/sidebar/mapping update: 52 focused showcase browser
checks and nine sidebar checks pass, including caller-owned mapping edits, initial
select labels, mobile containment and drawer focus/dismissal. Two new SSR tests
cover initial selections, read-only controls and missing schema references; the
existing facade test also passes. Workspace check, targeted Clippy with warnings
denied, formatting and all 460 upstream hashes pass. Documentation validation
continues to report the three pre-existing missing backup README links.

VisualFieldMapper now provides an additional interactive composed mapper; the
classic FieldMappingEditor and its showcase page remain intact. Its implementation
and scoped CSS module live together in composed/visual_mapping. Callers supply
serializable source/target MappingSchema inventories with JSONPath or XPath and
optional namespace bindings. A bounded JSON sample inference helper lives at the
caller import seam; raw XML parsing is the adapter's responsibility. Native field
buttons create/replace connections; wires and accessible chips select links for
transform configuration/removal. Read-only review allows inspection and download.
A versioned Conctr YAML mapping document mirrors the draft and downloads locally;
it is not an executable Apache Camel route. No transforms execute here.

Eighteen targeted browser checks pass for interactive mapping, read-only review,
JSON import errors, normalized XML-to-JSON links, local CSS, dark mode, narrow-screen
containment and actual YAML file download. The downloaded YAML parses back to the
edited draft. Six tests cover inference/limits, export validation/quoting/namespaces,
read-only markup and stylesheet registration in separate SSR documents.

The visual mapper now has a searchable, caller-extensible transformer catalog and
ordered transform pipelines with move-up/removal controls. Legacy single-transform
drafts load as the first pipeline step. Field inventory edits are reported through
an optional atomic MappingDraft callback: callers replace both schemas and links
together. Source and target fields can be added, ignored/restored or removed.
Ignoring/removing a mapped field prunes its links; restoration does not reconnect
it. YAML includes ignored paths and ordered transform IDs. The classic mapper is
unchanged. All new controls and styles remain in composed/visual_mapping.

Thirty browser checks pass for the updated mapper, including real pointer dragging,
searchable/reorderable/removable transform steps, source/target node creation,
ignore/restore, removal with link pruning, read-only protection and YAML download.
Pointer capture and node-owned listeners keep drag handling scoped to each mapper;
touch users can use grip handles or click-to-connect. Eight model/export/SSR tests
pass. Workspace check, targeted Clippy, formatting and upstream integrity pass.
The downloaded ordered transform YAML parses correctly.

The visual mapper now uses a ribbon editor layout with selection-aware node/link
commands and native new/edit node dialogs. Cards reorder vertically within their
source or target column; separate + handles drag connections between columns.
Root-delegated pointer listeners survive imported, renamed and reordered cards.
The former persistent three-dot menus have been removed. Node path edits retain
link IDs and pipelines; inventory removal/ignore continues to prune affected links.
`MappingDraft` is serializable and includes atomic Update/Move edits, a shared
node type vocabulary, and JSON/XML/YAML/TOML/text format provenance. Parsing remains
caller-owned; repeated imports replace inventories and clear old links in the demo.
Validation messages and an optional inspector element let callers extend pipeline
validation/script authoring without putting transform execution inside the editor.

Verification: 24 browser checks passed for real pointer reordering in both columns,
handle linking, dialogs, toolbar selection, transform order, import replacement,
read-only behavior, dark theme and mobile overflow. Nine mapper tests, workspace
check, targeted UI Clippy, formatting and the 460-file upstream integrity check
passed. The root-instruction served-style Python checker is absent from this
checkout; browser checks verified actual served component CSS instead. Documentation
validation still reports three existing missing backup README links.

Mapping Studio is now available in the shared product app at `/mappings`. The app
owns one local presentation document containing independent request and response
schemas/links, with bounded undo/redo across both directions. Reusable direction,
history toolbar chrome lives in `composed/mapping_workspace`; the existing
visual mapper supplies drag connections, field edits and ordered transform drafts.
Its optional studio layout provides a right inspector on wide screens and stacks
it below the canvas on smaller screens, without export controls. Classic mapping
rows and visual wires have focused modules; existing public editor APIs remain.
These are editable synthetic fields and session-local drafts, with no persistence,
transform execution or platform mapping contract implied.

Validation for the studio: targeted app/UI Cargo check and Clippy (warnings denied),
13 app/UI tests, workspace formatting and all 460 upstream hashes pass. The web
launcher builds successfully. Twelve browser checks verify mounted styles/controls,
hidden export, real pointer linking and selection, undo/redo, direction isolation,
desktop inspector placement, dark token propagation and mobile containment/stacking. Documentation checks
still report the three pre-existing missing backup README links.
The existing visual mapper gallery also passes 24 browser regressions, using a
temporary harness with `[data-editor-ready]` in place of its stale navigation
hydration selector. The prescribed Python served-style checker remains absent.


The subsequent Mapping Studio redesign replaces the visual mapper's former card
canvas/ribbon presentation with the supplied Studio prototype's compact 40 px
schema rows, dot ports, unified toolbar, persistent right tools/inspector and
bottom source/target panels. Both standalone visual-mapper consumers and the app
use this presentation. Dragging a connection renders a live cubic wire rather
than a floating card. Filters, unmapped-only and conservative name/type auto-map
update the controlled draft. The app supplies a synthetic source sample and an
explicit target mapping draft; no transform execution or persistence is claimed.
The classic row/select `FieldMappingEditor` remains a separate library control.

Validation: 18 studio browser checks pass for real pointer linking, curved drag
feedback, compact field geometry, inspector placement, bottom panes, filtering,
auto-map, history/direction isolation, dark theme and mobile containment. The 13
app/UI tests pass. Workspace check, targeted Clippy and formatting are checked
separately; upstream integrity still verifies 460 unchanged files. Earlier browser
counts describe earlier layouts, not this redesign.
Three gallery smoke checks also confirm the replacement field geometry, toolbar
and tools sidebar. Workspace compilation and targeted Clippy with warnings denied
pass; formatting and upstream integrity pass. Documentation validation retains
its three existing missing backup README links.

Mapping field edit, ignore/restore and deletion now live in field context menus,
opened by right-click, the row action button or Shift+F10. The menu uses native
popover dismissal with shared facade item styles and arrow-key focus navigation.
Read-only fields do not expose mutation menus. Source/target creation stays in
the tools panel; field ordering remains available through dragging.

Reusable `composed/resizable` handles provide pointer capture, bounded sidebar and
bottom resizing, separator semantics, arrows/Shift+arrows and Home/End controls.
The tools sidebar and payload panels have minimize, maximize and restore controls;
restoring retains the last resized size. Narrow layouts stack the panels and keep
mode controls available. These sizes are transient presentation state.

Validation for field menus and panels: 21 browser checks pass for pointer and
keyboard resizing, retained sizes, minimize/maximize/restore, right-click and
button menus, focus/navigation, edit/ignore/restore/delete and undo. The existing
18 Mapping Studio browser checks and 13 app/UI Rust tests pass. Workspace check,
targeted Clippy with warnings denied, formatting and all 460 upstream hashes pass.
Documentation validation retains the three existing missing backup README links.
Three gallery browser checks also pass for handle mounting, field menus and
keyboard resizing of the standalone payload panel.

The mapping field dialog now explicitly centers in the viewport (`inset: 0` and
`margin: auto`), correcting Tailwind reset margins that placed native dialogs in
a corner. Its header, schema context, field hints, monospace path input, validation
message and action footer follow the shared theme. The dialog has viewport-bounded
height with internal scrolling for short screens, plus a labeled close button.
Fourteen focused browser checks pass for desktop/mobile centering, validation,
focus, type selection, save/close/Escape, dark tokens and short-screen containment.
Nine mapper tests, targeted Cargo check/Clippy, formatting and upstream integrity
pass. Documentation validation retains the existing missing backup README links.
Two gallery checks also confirm centering within the usable viewport and the
served modal styling. Visual inspection confirms the updated dark dialog layout.


Resizable controls now follow the Rust component structure in the supplied
`temp/resizable` reference: `mod.rs`, `component.rs` and scoped `style.css`.
`resize.js` has been removed. The mapper owns pixel sizes in Rust signals and
renders its CSS variables normally. A `ResizablePanelGroup` provides independent
mounted-element measurement contexts; handles use Dioxus pointer/keyboard events
and a temporary drag shield instead of JavaScript listeners or imperative DOM
style writes. Generic handle/grip/control styling belongs to the resizable folder;
the mapper retains only placement and panel layout rules. The per-document SSR
stylesheet check now covers both mapping and resizable styles.

Validation: 26 panel/menu browser checks pass, including pointer resizing, focus,
overshoot bounds, release outside the group, cancellation, keyboard controls and
retained sizes after restore. All 14 app/UI Rust tests, workspace Cargo check,
targeted Clippy with warnings denied, formatting and diff whitespace checks pass.
Documentation validation retains the three existing missing backup README links.

### Pipeline Studio authoring preview

The shared app now exposes `/pipelines` in its workspace navigation. It uses the
existing workflow canvas, node wrappers, port connections, pan/zoom, minimap and
context menus inside a reusable `composed/pipeline_workspace` frame. The frame
owns scoped CSS, measured fit-to-canvas and a Rust-resizable inspector with
minimize/maximize/restore. Narrow layouts stack the panels.

Product-owned sample steps, palette filtering, inspector edits, duplicate/delete,
connection labels and basic connection diagnostics live in
`conctr-app-dioxus/src/pipeline_studio`. The sample follows the supplied studio
HTML's trigger, signature verification, mapping, condition, approval/direct API,
response and error-handler arrangement. Mapping steps link to `/mappings`.
Draft configuration and layout are transient presentation state. These are not
canonical executable flows: no requests, credential binding, expression execution,
persistence, test runs or deployment are implemented by this page.

Validation: 22 Pipeline Studio browser checks pass for node/port rendering,
selection, drag movement, add/duplicate/delete, disconnected-step diagnostics,
drag connections, inspector resizing and panel modes, step-name editing, palette
search, edge selection/label editing, dark theme and mobile overflow. Three gallery
checks confirm existing workflow ports and served positioning CSS. All 15 app/UI
Rust tests, workspace check, targeted Clippy with warnings denied, formatting and
460 upstream file hashes pass. Documentation validation still reports the three
existing missing backup README links.

### Authoring cleanup

Removed the unused `MappingEndpoints` composition; the mapper already renders
source/target summaries. The classic field mapper and workflow frame retain real
gallery/test consumers. Supplied `temp/` references and immutable upstream sources
remain intact. Root/UI/app instructions, presentation/theming docs and the AI
inventory now describe local authoring ownership and component-scoped CSS.
Historical backup references are plain provenance text because the archive is
absent; no restore capability is implied by this checkout.

Cleanup verification: documentation validation now passes (62 Markdown files,
191 local links and 15 workspace packages), resolving the historical broken backup
links. Workspace check, all 15 app/UI tests, formatting, whitespace and all 460
upstream hashes pass. No rendered behavior changed in this cleanup.

### Queue monitoring preview

The shared app exposes `/queues` with sample async traffic. The reusable
`composed::queue::QueueMonitor` displays snapshot totals, status-filtered history
and four exchange stages per request. A missing target response stays distinct
from a confirmed rejection. Display payloads come from the caller.

Pause and resume update local page state. Pause represents a halt to new dispatch
while existing requests retain their status. Connecting the monitor to live
counts, retained payloads and authorized queue controls requires runtime services.

### Workflow connection guide and code previews

The workflow facade measures pointer coordinates relative to the canvas viewport
during connection drags. Pipeline cards use the gallery's workflow node header.
The shared CodeBlock component provides Rust lexical highlighting for JSON,
YAML, TOML and XML. Queue exchanges and the Mapping Studio source sample use it.

The shared product app includes a credential reference directory at
`/credentials`. The Dioxus adapter invokes the framework-independent CRUD
service in `conctr-api`. The service stores name, purpose and vault alias in
a revision-checked, workspace-local metadata file. Configured deployments use administrator proxy authentication. An unconfigured
debug launcher on localhost supports local code sign-in. See the [adapter setup](../../crates/conctr-api-dioxus/README.md).
