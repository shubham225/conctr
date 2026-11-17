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

The gallery is now titled Conctr Showcase while keeping its Cargo package name.
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
