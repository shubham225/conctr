# Conctr component specifications

Live examples: `ui-kit/index.html`. Styles: `ui-kit/conctr-ui.css`. Token values: `DESIGN-TOKENS.md`.

**How to read each entry:** purpose, variants, spec (sizes and tokens), states, accessibility, do and don't, a code example. Class names use the `cn-` prefix; modifiers use `--` (BEM style): `cn-btn cn-btn--primary`.

## Dioxus application components

Mount `ui::theme::ThemeProvider` around the app to load the scoped theme tokens
and bundled utilities. Use the public Rust controls for application screens.
The HTML recipes below illustrate the corresponding visual treatments.

| Design need | Rust API | Usage |
| --- | --- | --- |
| Identify an endpoint protocol | `composed::ProtocolBadge` | Pairs its protocol shape with a name. |
| Identify a messaging pattern | `assets::icons` | Use a named SVG asset beside a visible label. Iterate over `PATTERN_GROUPS` to build the palette. |
| Edit a mapping | `composed::VisualFieldMapper` | Displays schema fields, links and transforms. Use `MappingWorkspaceToolbar` from `composed::mapping_workspace` for workspace actions. |
| Arrange pipeline steps | `composed::pipeline_workspace::PipelineWorkspace` | Frames a step library, workflow canvas and resizable inspector. The caller supplies the step definitions. |
| Inspect a queued request | `composed::queue::QueueMonitor` | Presents caller-supplied request history, status and safe exchange previews. |

The showcase’s Icons page presents 65 pattern icons across eight book chapters.
Each uses the kit’s 64-unit hexagonal frame. Switch between the color and
monochrome editions to see the same glyph in each. Use `dark:invert` for
monochrome files displayed as images on dark surfaces.
Pattern names follow Gregor Hohpe and Bobby Woolf’s
[Enterprise Integration Patterns catalog](https://www.enterpriseintegrationpatterns.com/patterns/messaging/toc.html).
The palette documents visual vocabulary for authoring previews.

## Index
| Need | Use |
|---|---|
| An action | Button `cn-btn` |
| Text input | Input `cn-input`, Textarea `cn-textarea`, Select `cn-select` |
| A choice | Checkbox `cn-check`, Radio `cn-radio`, Switch `cn-switch` |
| Show protocol, status | Badge `cn-badge`, Shape `cn-shape` |
| Group content | Card `cn-card` |
| Tell the user something | Alert `cn-alert`, Toast `cn-toast` |
| Move around | Tabs `cn-tabs`, Breadcrumb, Pagination, Nav `cn-nav` |
| Show data | Table `cn-table`, Stat `cn-stat`, Code `cn-code`, Avatar `cn-avatar` |
| Show progress | Progress `cn-progress`, Spinner `cn-spinner`, Skeleton `cn-skeleton` |
| Overlay | Menu `cn-menu`, Tooltip `cn-tip`, Modal `cn-modal` |
| Show a connection | Connection flow `cn-flow` |

## Global rules
1. **One primary button per view.** Everything else is secondary or ghost.
2. **Color is never the only signal.** Pair color with a label, icon or the shape language.
3. **Every interactive element has a visible focus state** and a name for screen readers.
4. **All values from tokens.** No hard-coded colors, px sizes (except where this document gives them), shadows or radii.
5. **Both themes.** Check every component in light and dark.
6. **Sentence case** and the vocabulary in the brand guide: connection, source, target, mapping, run, protocol, credentials.
7. **Text on tinted fills** uses the matching `-text` token (`--cn-teal-text` on `--cn-teal-soft`).

---

## Button
**Purpose:** trigger an action. **Element:** `<button>` (or `<a class="cn-btn">` for navigation).

| Variant | Class | Use |
|---|---|---|
| Primary | `cn-btn--primary` | The main action of the view. One per view. |
| Secondary | `cn-btn--secondary` | Other actions. Default for toolbars. |
| Ghost | `cn-btn--ghost` | Low emphasis, tertiary, inline. |
| Danger | `cn-btn--danger` | Destructive actions, usually after a confirm. |

**Sizes:** `--sm` 32 px (padding 0 12, text caption), default 40 px (padding 0 16, text small 500), `--lg` 48 px (padding 0 24, text body). Icon-only: `--icon` (square, same height).
**Spec:** radius `md`, border 1 px transparent (secondary uses `border-strong`), gap 8 between icon and label, icon 20 px (16 px in `sm`), no uppercase.
**States:** hover one step darker or tinted; focus 2 px outline offset 2; disabled opacity .45; loading shows a `cn-spinner` before the label, keeps width, sets `aria-busy="true"` and `disabled`.
**Content:** verb plus object ("Create connection", "Test connection"). Never "Submit", "OK" or "Click here". Max 3 words where possible.
**Accessibility:** icon-only buttons need `aria-label` and a tooltip. Do not use `<div>` as a button.
**Don't:** place two primary buttons side by side; use danger for non-destructive actions; disable a button without saying why.

```html
<button class="cn-btn cn-btn--primary" type="button">Create connection</button>
<button class="cn-btn cn-btn--secondary cn-btn--sm" type="button">Test connection</button>
```

## Input, textarea, select
**Spec:** height 32 / 40 / 48 (`--sm`, default, `--lg`), padding 0 space-3, radius `md`, border 1 px `border-strong`, background `surface`, text 16 px Inter (prevents iOS zoom), placeholder `text-subtle`. Textarea: min-height 96 px, resizable vertically, padding space-3.
**Anatomy:** label (above, `small` 500) , control, hint or error (below, `caption`). Wrap in `.cn-field` (gap 6 px).
**States:** hover border `text-subtle`; focus 2 px outline `focus` plus `focus-ring`; error `aria-invalid="true"` with `.cn-error` below; disabled .45 with `sunken` background.
**Content:** labels are nouns ("Endpoint URL"). Placeholders show an example, never replace the label. Errors say what is wrong and how to fix it ("Add the scheme, for example https://").
**Accessibility:** every field has a `<label for>`; link hints and errors with `aria-describedby`.
**Don't:** use placeholder as label; validate on every keystroke before the user finishes; clear the field on error.

```html
<div class="cn-field">
  <label class="cn-label" for="url">Endpoint URL</label>
  <input class="cn-input" id="url" aria-describedby="url-h">
  <span class="cn-hint" id="url-h">Include https://</span>
</div>
```

## Checkbox, radio, switch
**Spec:** checkbox and radio 18 px, border 2 px `border-strong`, checked fills `primary` with an `on-primary` mark; checkbox radius `xs`, radio fully round. Switch 40 x 24 px, thumb 18 px, pill radius. Label via `.cn-option` (gap 8, `small`).
**Use:** checkbox for independent choices, radio for one of a few (2 to 5), switch for an immediate on/off setting (no Save button needed). Use a select when more than 5 options.
**Accessibility:** always a visible label; group radios in a `<fieldset>` with a `<legend>`.

## Badge and shape
**Purpose:** show a protocol, a status or a short tag. Height 24 px, pill radius, padding 0 space-2, text caption 500.
**Protocol badge:** `.cn-badge` with a `.cn-shape` and the protocol name. Shapes and colors are fixed: circle teal REST, square coral SOAP, triangle amber OData, diamond teal GraphQL, hexagon indigo gRPC, ring coral Webhook.
**Status badge:** `--success` Healthy, `--warning` Degraded, `--danger` Failing, `--info` Draft, default (neutral) Paused or Disabled. Always include the text.
**Don't:** invent new colors for statuses; use badges for actions (they are not clickable); put more than 2 words in a badge.

```html
<span class="cn-badge"><i class="cn-shape" data-s="circle" style="--c:var(--cn-teal)"></i>REST</span>
<span class="cn-badge cn-badge--success">Healthy</span>
```

## Card
**Spec:** background `surface`, border 1 px `border`, radius `lg`, padding space-6 (`--compact` space-4). `--raised` adds shadow-1 and `raised` background. `--interactive` adds hover shadow-2 and a pointer; the whole card is then a link or button.
**Use:** group related content. Do not nest cards inside cards; use a divider or a `sunken` background for inner groups.
**Don't:** make every block a card; put two primary actions in one card.

## Alert
**Purpose:** an inline message about the current context that stays on screen. Radius `md`, padding space-3 by space-4, `small` text, tinted background (`-soft` token), no border.
**Variants:** `--info`, `--success`, `--warning`, `--danger`. **Content:** bold first line says what happened, the next line says what to do. Errors do not apologise and are never vague.
**Accessibility:** `role="alert"` for errors that appear after an action, `role="status"` for non-urgent updates.
**Don't:** stack more than two alerts; use alerts for marketing.

## Toast
**Purpose:** brief confirmation of an action that already happened. Radius `md`, `raised` background, border, shadow-3, max width 420 px, bottom center (mobile) or bottom right (desktop), layer `z-toast`.
**Behavior:** auto-dismiss after 5 s, pause on hover and focus, optional single action ("Undo"). Errors that need a decision are alerts or modals, not toasts.
**Content:** past tense, same verb as the button: "Connection created".

## Tabs
**Spec:** row with 1 px bottom border, tab padding 10 by space-4, `small` 500, selected tab `primary` text with a 2 px underline. `role="tablist"`, `role="tab"`, `aria-selected`, panels with `role="tabpanel"`. Arrow keys move between tabs.
**Use:** switch between views of the same object. Max 6 tabs. Do not use tabs for steps in a process.

## Breadcrumb and pagination
**Breadcrumb:** `<ol class="cn-breadcrumb">`, `small`, separator "/", the last item is `aria-current="page"` and not a link. Use from the third level down.
**Pagination:** `<nav aria-label="Pagination">` with 32 px buttons, the current page `aria-current="page"` on `primary-soft`. For logs and run history prefer "Load more" or infinite scroll with a visible count.

## Navigation (sidebar)
**Spec:** width 240 px, item height 36 px, radius `md`, `small` 500, icon 20 px. Current item `aria-current="page"` on `primary-soft` with `primary` text. Group titles are `caption` 500 in `text-subtle` (sentence case, no uppercase).
**Behavior:** collapses to a drawer below `lg`. Max 7 top-level items.

## Table
**Spec:** wrapper `.cn-table-wrap` (border, radius `lg`, overflow hidden). Header `sunken` background, `small` 500, `text-subtle`. Cells padding 14 px by space-4 (`--dense`: space-2 by space-4), 1 px row dividers, row hover `sunken`.
**Rules:** left-align text, right-align numbers (tabular figures), use mono for IDs and endpoints, show status as badges, put row actions in the last column (a menu, not 4 buttons). Provide an empty state and a loading skeleton. On mobile, switch to stacked cards or allow horizontal scroll inside the wrapper.

## Stat
**Spec:** `.cn-stat` inside a card: label (`small`, `text-muted`), value (Sora 600 at h2 size), optional context (badge or caption). One number per stat. Show the unit and the time range in the label ("Runs today").

## Code block
**Spec:** `.cn-code`, JetBrains Mono 13 / 20, `sunken` background, border, radius `md`, padding space-4, horizontal scroll (no wrapping). Syntax colors: keys `primary`, strings `teal-text`, comments `text-subtle`. Provide a copy button for blocks longer than one line.

## Avatar
**Spec:** round, sizes 24 / 32 / 40 / 48 (`--sm`, `--md`, default, `--lg`), initials 1 to 2 characters, `primary-soft` background with `primary` text. Use an image when available and keep initials as the alt fallback.

## Progress, spinner, skeleton
- **Progress:** 8 px high pill track, `primary` fill. Add `role="progressbar"` with `aria-valuenow`, and a text label with the percentage or step. Use for operations with a known length.
- **Spinner:** 20 px, for waits under 10 s with unknown length. Always pair with text ("Testing connection"). Replace with progress if it takes longer.
- **Skeleton:** same layout as the final content, shimmer stops with reduced motion. Use for first loads of tables, cards and stats.

## Menu
**Spec:** `.cn-menu`: min width 200 px, padding space-1, radius `md`, shadow-3, layer `z-dropdown`. Items 36 px high, radius `sm`, `small` text, hover `sunken`. Danger items use `danger` text and sit last behind a separator.
**Behavior:** `role="menu"` and `role="menuitem"`; Arrow keys move, Escape closes and returns focus to the trigger. Max 7 items; use verbs.

## Tooltip
**Spec:** `text` background with `bg` text (inverted), `caption` 500, radius `sm`, shadow-3, appears after a short delay on hover and on keyboard focus, above the element, layer `z-tooltip`.
**Use:** name icon-only buttons and explain truncated text. Never put essential information, links or buttons in a tooltip.

## Modal
**Spec:** `.cn-scrim` (`--cn-scrim` backdrop) with `.cn-modal`: width min(480 px, 100%), radius `xl`, padding space-6, shadow-4, `raised` background, layer `z-modal`. Title h4, body `body`, footer buttons right-aligned (secondary left of primary, gap space-3).
**Behavior:** `role="dialog"`, `aria-labelledby`, trap focus, Escape closes, focus returns to the trigger, scroll lock on the page. Clicking the scrim closes only non-destructive modals.
**Content:** the title is a question or an action ("Delete connection?"). The destructive button repeats the verb and the object ("Delete connection"). Say what will be lost and for how long it can be recovered.
**Don't:** open a modal from a modal; use a modal for long forms (use a page or a side panel).

## Connection flow (signature component)
**Purpose:** show how two systems connect through Conctr. Layout: source node, link, hub, link, target node.
**Spec:** nodes are `.cn-node` (surface, border, radius `md`, padding space-3 by space-4) with the system name (body, 600) and a protocol badge. The hub is the Conctr hexagon in `primary` (72 x 80 px) labelled CONCTR. Links are 2 px `border-strong` lines with an amber packet that travels from source to target. Below `sm` the flow stacks vertically.
**States:** healthy (animated packet), paused (packet hidden, link dashed), failing (link and failing node border in `danger`, with a text message under the flow).
**Rules:** the source is always on the left (or top), the target on the right (or bottom). Protocol badges use the shape language. Do not animate when `prefers-reduced-motion` is set (the kit handles this).

## Empty state
**Spec:** centered in a card, padding 48 by 24, 48 px icon (the Conctr icon or a protocol icon), h3 title ("No connections yet"), one sentence on what to do, one primary button.
**Don't:** use illustrations that are not from the kit; apologise; leave the user without an action.

---

## Patterns

### Form layout
Single column, labels above fields, groups separated by space-6, primary action bottom left, aligned with the fields. Show all errors in context, and move focus to the first one on submit. Save changes automatically only for switches.

### Destructive actions
Menu item (danger, last) opens a modal. The modal names the object and consequences, the confirm button repeats the verb. After success show a toast with "Undo" when it can be undone.

### Error messages
Structure: what happened, why if known, what to do. "Stock sync failed at the target. The SOAP endpoint returned 401. Check the credentials for this connection, then run it again." No "Oops", no exclamation marks, no blame.

### Loading
Under 1 s: nothing. 1 to 10 s: spinner with text. First load of lists: skeleton. Known length: progress with a label. Never block the whole page for a partial load.

### Status vocabulary
| Status | Badge | Meaning |
|---|---|---|
| Healthy | `--success` | Last runs succeeded |
| Degraded | `--warning` | Succeeding but slow or retrying |
| Failing | `--danger` | Last runs failed |
| Draft | `--info` | Not yet active |
| Paused | neutral | Stopped by a person |

### App shell
Sidebar 240 px (from `lg`), top bar 56 px with breadcrumb and a user menu, content in `--cn-container-wide` with padding space-6 to space-8. Page title (h1) and the primary action on one line at the top of the content.

## Building a new component (checklist)
- [ ] Is there an existing component or variant that fits? Reuse it.
- [ ] Uses tokens only; works in light and dark.
- [ ] Has hover, focus, disabled and error states where relevant.
- [ ] Keyboard operable; role, name and state exposed to screen readers.
- [ ] Meets AA contrast and a 24 px minimum target (44 px on touch).
- [ ] Respects `prefers-reduced-motion`.
- [ ] Added to `ui-kit/index.html` and to this document (purpose, variants, spec, states, a11y, do and don't).

## Notes for AI tools
- Generate markup with the class names above. Do not write custom CSS for things the kit already covers.
- Use the exact status and protocol vocabulary and shapes from this document.
- When unsure about a size or space value, pick the nearest token, never an arbitrary number.
- Always produce a light and dark friendly result by using `var(--cn-*)` tokens only.
- Keep copy short, in sentence case, with consistent verbs between buttons, modals and toasts.


---

## Form, navigation and data components

Live examples: `ui-kit/forms.html`, `navigation.html`, `overlays.html`, `data.html`. All of these follow the global rules at the top of this document (tokens only, both themes, states, accessibility).

## Buttons and toggles

### Button group
**Classes:** `cn-btn-group`

**Use for:** Join 2 to 4 related actions, or a primary action with its menu (split button).

**Spec:** Buttons share edges (1 px overlap); only the outer corners are rounded (`md`). Same size across the group.

**Accessibility:** Wrap in `role="group"` with a label when the buttons act on the same object.

**Do not:** Mix primary and secondary styles except in a split button.

### Toggle
**Classes:** `cn-toggle`

**Use for:** A button with an on/off state (mask values, wrap lines, bold).

**Spec:** 40 px high (use `cn-btn--sm` sizing via your layout for 32). On state: `primary-soft` background, `primary` text and border. Toggle `aria-pressed`.

**Accessibility:** `aria-pressed="true|false"`; the label never changes with state.

**Do not:** Use for one-time actions (that is a button), or for choosing between views (use segmented).

### Segmented control (toggle group)
**Classes:** `cn-segmented`

**Use for:** Choose one of 2 to 4 views or modes (Table, Cards, Graph).

**Spec:** Radio inputs hidden inside labels, sunken track, selected segment on `surface` with shadow-1. Segment height 28 px.

**Accessibility:** Use `role="radiogroup"` with a label. Arrow keys move the selection (native radios).

**Do not:** Use more than 4 segments (use tabs or a select), or use it for navigation between pages.

## Text input

### Input group
**Classes:** `cn-input-group`, `__addon`, `__icon`

**Use for:** Add a prefix, suffix, icon or button to an input: search, URL parts, units, copy and reveal.

**Spec:** One 1 px border and radius `md` around everything. Focus ring on the whole group. Addons use `sunken` background. Invalid: `cn-input-group--invalid` plus `aria-invalid` on the input.

**Accessibility:** Every input still needs a label (visible or `aria-label`). Icon-only buttons need `aria-label`.

**Do not:** Put long text in an addon, or stack more than two addons.

### Search input
**Classes:** `cn-input-group` with `__icon`

**Use for:** Filter a list or find an object. Include the shortcut hint (`cn-kbd`) when one exists.

**Spec:** Search icon at the start, placeholder says what is searched ("Search connections"). Clear button appears when there is text.

**Accessibility:** `role="search"` on the surrounding form, `aria-label` on the input.

**Do not:** Search on every keystroke without a debounce (200 to 300 ms).

### Secret input
**Classes:** `cn-input-group` with buttons

**Use for:** Credentials, API keys, client secrets.

**Spec:** Type `password` by default. Ghost icon buttons: show and copy. Never log or echo the value; show only the last 4 characters after saving.

**Accessibility:** Show/hide buttons use `aria-pressed` or change their `aria-label`.

**Do not:** Show a saved secret in full again.

### Endpoint input
**Classes:** `cn-input-group` with `cn-select--bare`

**Use for:** Integration pattern: method, URL and send in one control.

**Spec:** Method select (bold, `sunken`) then a mono URL input then a primary button. URL uses `cn-mono`.

**Accessibility:** Label the select ("Method") and the input ("URL").

**Do not:** Hide the method; people need it to read the request.

### Slider
**Classes:** `cn-slider`, `cn-slider-field`

**Use for:** Pick an approximate value in a range (timeout, batch size, retries).

**Spec:** 6 px track, filled part `primary`, 18 px thumb with a 2 px `primary` border. Set `--p` (percent) on the element for the fill in WebKit browsers (the kit script does it on input). Show the value next to the label.

**Accessibility:** Native `input[type=range]`: arrow keys, Home, End. Always show the current value as text.

**Do not:** Use for exact values (use a number input) or ranges where the exact number matters.

### One-time code (OTP)
**Classes:** `cn-otp`

**Use for:** Enter a 4 to 8 character verification code.

**Spec:** One 44 x 52 px box per character, mono 22 px, optional separator. Auto-advance, backspace-back and paste-split come from your framework.

**Accessibility:** `role="group"` with a label; each box has `aria-label="Digit n"`; `autocomplete="one-time-code"` on the first box.

**Do not:** Use for non-code input.

### Inline edit
**Classes:** `cn-inline-edit`

**Use for:** Edit a name or value in place without leaving the page.

**Spec:** Read mode is a button that looks like text with a hover background. Edit mode is a small input with Save (check) and Cancel (x) icon buttons. Enter saves, Escape cancels.

**Accessibility:** Read mode is a real `button` with the value as its name. Move focus to the input on edit and back on finish.

**Do not:** Use for long or multi-field edits (use a form or drawer).

## Choosing

### Combobox and listbox
**Classes:** `cn-combobox`, `cn-listbox`, `__option`, `__group`

**Use for:** Search and pick from many options (more than 7) such as systems, entities and fields.

**Spec:** Input plus a listbox panel (shadow-3, radius `md`, max 240 px). Active option `sunken`, selected option `primary` with a check. Show an empty message ("No systems found").

**Accessibility:** `role="combobox"` on the input with `aria-expanded`, `aria-controls`, `aria-activedescendant`; options `role="option"` with `aria-selected`. Up/Down move, Enter selects, Escape closes.

**Do not:** Use for fewer than 7 options (use a select or radios).

### Tag and tag group
**Classes:** `cn-tag`, `cn-tag-group`, `cn-tag--primary|success|warning|danger`

**Use for:** Label an object with a category or environment (production, beta). Tags can be removable.

**Spec:** 24 px high, radius `sm` (squarer than a badge, which is a pill), caption 500. Removable tags end with a `cn-tag__remove` button.

**Accessibility:** The remove button needs `aria-label="Remove <name>"`.

**Do not:** Use a tag for status (use a status badge) or make the tag itself a button.

### Tag input
**Classes:** `cn-taginput`

**Use for:** Enter several free-text values: scopes, allowed IPs, event names.

**Spec:** Border box with tags and a flexible input. Enter or comma adds, Backspace on empty input removes the last. Focus ring on the whole box.

**Accessibility:** Announce additions and removals with a polite live region.

**Do not:** Allow duplicates unless they mean something.

### Choice card
**Classes:** `cn-choice-group`, `cn-choice`

**Use for:** Pick one option that needs a title and a sentence of help: protocol, auth method, plan.

**Spec:** Radio input covers the card. Selected: `primary` border and `primary-soft` background. Disabled: opacity .45 with a reason in the text.

**Accessibility:** Wrap in `role="radiogroup"` with a label. Focus ring shows on the card.

**Do not:** Use for more than 6 options.

## Dates

### Calendar and date picker
**Classes:** `cn-calendar`, `__day`, `__dow`

**Use for:** Pick a date or a range. Use the native date input for simple dates and the calendar (in a popover) for ranges and rich states.

**Spec:** 7 columns of 36 px. Today: 1 px `primary` outline. Selected: filled `primary`. Range: `primary-soft`. Outside month: `text-subtle`. Disabled: .45.

**Accessibility:** Grid with `role="grid"`; days are buttons with full date as `aria-label`. Arrow keys move by day and week, PageUp/PageDown by month.

**Do not:** Use for birth dates and far-away dates without a year picker.

## Forms

### Form layout
**Classes:** `cn-form`, `cn-form-row`, `cn-form-section`, `cn-form-actions`

**Use for:** Lay out long forms in sections with a title and explanation on the left.

**Spec:** Max width 720 px. Section: 1/3 title column, 2/3 fields, 24 px padding, top border. Actions at the bottom, left aligned with the fields. Single column below 768 px.

**Accessibility:** Group related controls in `fieldset` with `legend`. Mark required fields with `cn-required` and say so at the top.

**Do not:** Spread labels beside fields on mobile, or use more than two columns of fields.

### Key-value editor
**Classes:** `cn-kv`, `cn-kv__row`, `cn-kv__head`

**Use for:** Edit headers, query parameters, environment variables.

**Spec:** Grid of key (1fr), value (2fr), remove button (40 px). Mono text. A blank row sits at the end; "Add row" below. Validate duplicate keys inline.

**Accessibility:** Each input needs a name ("Key 1"). The remove button names the row.

**Do not:** Hide values that are secrets without a reveal button.

### Mapping row
**Classes:** `cn-mapping`, `cn-mapping--error`

**Use for:** Link a source field to a target field with the transform between them.

**Spec:** Three columns: source field (path in mono, type below), arrow with transform name, target field. Error state: `danger` border and an inline message that says how to fix it.

**Accessibility:** The row is a group; the transform control has a label.

**Do not:** Show only the field names; the type is what prevents mismatches.

### File upload (dropzone)
**Classes:** `cn-dropzone`, `cn-file`

**Use for:** Upload WSDL, OpenAPI, OData `$metadata`, CSV.

**Spec:** Dashed 2 px border, hover and drag state `primary-soft`. Say what is accepted and the size limit. After a file is chosen show a `cn-file` row with name, size, progress and cancel.

**Accessibility:** Real `input type=file` inside the label (visually hidden, not `display:none`) so keyboard and screen readers work.

**Do not:** Rely on drag and drop only.

## Identity

### Count badge
**Classes:** `cn-count`, `--muted`, `--danger`

**Use for:** Show a number next to a label (alerts, items).

**Spec:** 20 px high pill, tabular numbers, cap display at 99+. Muted for totals, danger only for items that need action.

**Accessibility:** Add visible or hidden text ("3 alerts").

**Do not:** Use for statuses or as the only signal.

### Status dot
**Classes:** `cn-dot`, `--success|warning|danger|info`

**Use for:** A tiny status marker beside a label or in dense lists.

**Spec:** 8 px circle. Always next to the status word. For product status prefer the status badge.

**Accessibility:** Never the only indicator; include text.

**Do not:** Use a dot alone in a table cell.

### Date label
**Classes:** `cn-date-label`, `--soon`, `--overdue`

**Use for:** Show a date with its urgency (due dates, expiry, next run).

**Spec:** 24 px, calendar icon, caption 500. Soon uses warning tones, overdue uses danger tones. Say "Due tomorrow", "3 days overdue" rather than only a date.

**Accessibility:** Use `<time datetime>`.

**Do not:** Color a date without saying why.

### Tile
**Classes:** `cn-tile`, `--sm`, `--lg`

**Use for:** Put an icon in a rounded square for list rows and cards.

**Spec:** 32 / 40 / 48 px, radius = 25% of size, `primary-soft` background and `primary` icon. Protocol icons can replace the glyph.

**Accessibility:** Decorative: `aria-hidden` icon.

**Do not:** Use as a button.

### Avatar group
**Classes:** `cn-avatar-group`, `cn-avatar-group__more`

**Use for:** Show owners or members compactly.

**Spec:** Avatars overlap by 8 px with a 2 px `surface` ring. Show 3 to 5 then a "+N" avatar.

**Accessibility:** Give the group an accessible name ("Owners: Asha, Ravi and 4 more").

**Do not:** Overlap more than 8 px.

### Item and item group
**Classes:** `cn-item-group`, `cn-item`, `__body`, `__title`, `__desc`, `__meta`, `__actions`

**Use for:** A list row with media, title, description, status and actions. Use it for settings lists, connection lists and search results.

**Spec:** Padding 12 x 16, 1 px dividers, group radius `lg`. `--interactive` adds a hover background and a pointer; make the row a link or button.

**Accessibility:** If the row is clickable, the title is the link or the row has `role="link"`; one tab stop per row plus one per action.

**Do not:** Put more than two actions in a row (use a menu).

### Description list
**Classes:** `cn-dl`

**Use for:** Labeled facts about one object (owner, source, last run).

**Spec:** Two columns: label (`text-muted`, up to 200 px) and value. Mono for IDs and URLs. Stacks below 640 px.

**Accessibility:** Use real `dl`, `dt`, `dd`.

**Do not:** Use a table for a single object.

## Layout

### Accordion and collapsible
**Classes:** `cn-accordion`, `cn-collapsible`

**Use for:** Show and hide sections of content. Accordion for related sections, collapsible for one optional block.

**Spec:** Native `details` and `summary`: no script. 16 px padding, chevron rotates 180 degrees over 200 ms. First item can start open.

**Accessibility:** Native semantics give keyboard (Enter, Space) and screen reader support.

**Do not:** Hide required information or error messages inside.

### Grid
**Classes:** `cn-grid`, `--3`, `--4`, `--auto`

**Use for:** Equal columns of cards or stats.

**Spec:** Gap 16 px. Fixed column counts collapse to one column below 768 px; `--auto` fits columns of at least 240 px.

**Accessibility:** n/a

**Do not:** Nest grids more than once.

### Separator (vertical)
**Classes:** `cn-divider--vertical`, `cn-divider`

**Use for:** Divide groups in a row or a page.

**Spec:** 1 px `border` color, 12 px side margin when vertical.

**Accessibility:** Decorative: `role="separator"` only when it divides interactive groups.

**Do not:** Use as a heading underline.

### Aspect ratio
**Classes:** `cn-aspect`

**Use for:** Keep media (images, video, embeds) at a fixed ratio.

**Spec:** Set `--r` (for example `16/9`). Radius `md`, background `sunken`, content `object-fit: cover`.

**Accessibility:** Images need `alt`.

**Do not:** Stretch images.

### Scroll area
**Classes:** `cn-scroll`

**Use for:** Scroll inside a fixed-height container (logs, payloads, long lists) with a thin themed scrollbar.

**Spec:** Set a height. Thin 10 px scrollbar in `border-strong`. Make the container focusable (`tabindex="0"`) when it only scrolls.

**Accessibility:** Keyboard users must be able to scroll it.

**Do not:** Nest scroll areas.

### Panel (side panel)
**Classes:** `cn-with-panel`, `cn-panel`

**Use for:** Show details next to a list without covering it (non-modal).

**Spec:** Panel 360 px wide, left border. Below 768 px it moves under the list.

**Accessibility:** `aside` with a label. Do not trap focus; it is not a dialog.

**Do not:** Use for tasks that need full attention (use a drawer or page).

### Split view (resizable)
**Classes:** `cn-split`, `__pane`, `__handle`

**Use for:** Compare two areas (request and response) with a draggable divider.

**Spec:** Set `--left` for the first pane width. Handle is 6 px, `col-resize`, `primary` on hover and focus. Dragging needs script.

**Accessibility:** Handle has `role="separator"`, `aria-orientation`, `tabindex="0"`; arrow keys resize in 5% steps.

**Do not:** Allow panes to shrink below 200 px.

## Navigation

### Navbar
**Classes:** `cn-navbar`, `__links`, `__link`, `__actions`

**Use for:** Top navigation for marketing, docs and public pages.

**Spec:** 64 px high, `surface` with a bottom border. Logo, links, actions (Sign in secondary, Start free primary). Current link: `aria-current="page"` on `primary-soft`.

**Accessibility:** `nav` with `aria-label`. Collapse to a menu button below 768 px.

**Do not:** Use for the app console (use the sidebar shell).

### Menubar
**Classes:** `cn-menubar`, `__trigger`

**Use for:** Desktop-app style menus (File, Edit, View).

**Spec:** 32 px triggers in a bordered bar. Open trigger has `aria-expanded="true"` and `sunken`. Menus use `cn-menu`.

**Accessibility:** `role="menubar"` and `menuitem`; Left/Right move between menus, Down opens, Escape closes.

**Do not:** Use on the web unless the app is desktop-like.

### Link
**Classes:** `cn-anchor`, `--subtle`, `--external`

**Use for:** Links inside text and standalone.

**Spec:** `primary` text, underline with 3 px offset. External links get an arrow glyph.

**Accessibility:** Link text says where it goes ("connector guide", not "click here").

**Do not:** Style a button as a link or a link as a button.

### Vertical tabs
**Classes:** `cn-tabs--vertical`

**Use for:** Settings with many sections.

**Spec:** Same tab markup, 200 px wide, selected indicator on the right edge.

**Accessibility:** `aria-orientation="vertical"`; Up/Down move.

**Do not:** Use more than 8.

### Stepper (progress tracker)
**Classes:** `cn-stepper`, `--vertical`, `cn-dots`

**Use for:** Show progress through a multi-step flow (create connection, onboarding).

**Spec:** Steps: complete (filled `primary` with a check), current (`primary` ring), upcoming (outline), error (`danger`). Connectors fill as steps complete. On mobile only the current title shows. Dots for compact carousels.

**Accessibility:** `ol` with `aria-label`; current step has `aria-current="step"`. State must be readable, not color alone (check, number, "!").

**Do not:** Use more than 6 steps, or for non-linear navigation (use tabs).

### Tree view
**Classes:** `cn-tree`, `__leaf`

**Use for:** Hierarchies: API resources, OData entity sets, folders.

**Spec:** Rows 32 px, chevron rotates, children indented 12 px with a guide line. Selected row `primary-soft`.

**Accessibility:** `role="tree"`, `treeitem`, `aria-expanded`; arrow keys navigate, Right expands, Left collapses.

**Do not:** Nest deeper than 5 levels.

## Messaging

### Banner
**Classes:** `cn-banner`, `--warning|danger|success`, `__actions`

**Use for:** Page-wide message about something that affects everything: maintenance, billing, outage.

**Spec:** Full width, tinted background, icon, one sentence, optional action and dismiss. One banner at a time, at the very top.

**Accessibility:** `role="status"`, or `role="alert"` for urgent ones.

**Do not:** Stack banners or use them for page-specific messages (use an alert).

### Inline message
**Classes:** `cn-inline-message`, `--info|success|warning|danger`

**Use for:** A short hint or validation next to a field or value.

**Spec:** Icon plus one line of `small` text in the semantic color.

**Accessibility:** Link to the field with `aria-describedby`.

**Do not:** Write more than one sentence.

### Spotlight (tour step)
**Classes:** `cn-spotlight`

**Use for:** Introduce one new feature or step of a tour.

**Spec:** `primary` background, 320 px max, title, one sentence, step count, Skip and Next. Show once per person.

**Accessibility:** `role="dialog"` with a label; Escape skips; do not trap focus.

**Do not:** Block the interface or show more than 5 steps.

## Overlays

### Alert dialog
**Classes:** `cn-modal--alert`

**Use for:** Interrupt to confirm a risky action. Needs an answer.

**Spec:** 420 px wide. Icon tile, question title, consequence sentence, Cancel and a verb-and-object confirm button. No close icon, no backdrop dismiss.

**Accessibility:** `role="alertdialog"`, `aria-labelledby`, `aria-describedby`. Focus starts on Cancel for destructive actions.

**Do not:** Use for information (use a toast or alert).

### Popover
**Classes:** `cn-popover`, `cn-popover-wrap`

**Use for:** Small interactive content opened by a click: filters, quick settings, date picker.

**Spec:** Radius `lg`, shadow-3, 240 to 360 px, 8 px from the trigger. Closes on Escape and outside click.

**Accessibility:** Trigger has `aria-expanded` and `aria-controls`. Focus moves in and returns on close.

**Do not:** Put long forms or required decisions in it.

### Hover card
**Classes:** `cn-hover`

**Use for:** Preview an item (a person, a connection) on hover or focus.

**Spec:** Popover that opens on `:hover` and `:focus-within`. Read-only content.

**Accessibility:** Content must also be reachable another way (a hover card is a convenience). Not available on touch.

**Do not:** Put required actions or links in it.

### Command palette
**Classes:** `cn-command`, `__input`, `__list`, `__group`, `__item`, `__foot`

**Use for:** Keyboard-first search for pages, objects and actions (Ctrl K).

**Spec:** 560 px dialog, radius `xl`, shadow-4. Grouped results, selected row `sunken`, shortcuts as `cn-kbd`, footer with key hints.

**Accessibility:** `role="dialog"` with a combobox and listbox inside; Up/Down, Enter, Escape. Open with Ctrl or Cmd + K.

**Do not:** Make it the only way to reach something.

### Drawer and sheet
**Classes:** `cn-scrim--end|start|bottom`, `cn-drawer`, `__head`, `__body`, `__foot`

**Use for:** Create or edit without leaving the page (side drawer), or a mobile bottom sheet.

**Spec:** 420 px wide, full height, shadow-4. Header with title and close, scrolling body, footer with actions. Sheet: full width, radius `xl` on top, max height 85%.

**Accessibility:** `role="dialog"`, `aria-labelledby`, focus trap, Escape closes, focus returns to the trigger.

**Do not:** Stack drawers.

### Context menu
**Classes:** `cn-menu--context`

**Use for:** Right-click actions on rows and nodes.

**Spec:** The standard `cn-menu` positioned at the pointer, kept inside the viewport.

**Accessibility:** Always provide the same actions through a visible button or keyboard shortcut (Shift+F10 or the menu key).

**Do not:** Make it the only way to do something.

## Data

### Sortable and selectable table
**Classes:** `th[aria-sort]`, `tr[aria-selected]`, `cn-table--selectable|--sticky`, `cn-num`, `cn-table__check`, `cn-table__actions`

**Use for:** Tables where people sort, select and act on rows.

**Spec:** Clicking a header cycles `aria-sort` (ascending, descending). Selected rows `primary-soft`. Numbers right-aligned with tabular figures. Sticky header with `--sticky`. First column checkbox, last column row actions.

**Accessibility:** Header cells are buttons or have `aria-sort`; checkboxes have row-specific labels; announce selection count.

**Do not:** Hide the row actions menu behind hover only.

### Toolbar and bulk bar
**Classes:** `cn-toolbar`, `cn-bulkbar`, `cn-table-footer`

**Use for:** Filters above a table; actions on the selected rows; footer with count and pagination.

**Spec:** Toolbar: search, filters, spacer, actions. Bulk bar replaces the toolbar actions when rows are selected (inverse colors). Footer: count left, pagination right.

**Accessibility:** Bulk bar has `role="status"` and announces "2 selected".

**Do not:** Put destructive bulk actions first, or without confirmation.

### Table tree and empty table
**Classes:** `cn-tree-cell`, `cn-tree-toggle`, `cn-table__empty`

**Use for:** Nested rows, and a clear message when there are no rows.

**Spec:** Indent = `--depth` x 24 px. Toggle has `aria-expanded`. Empty table row spans all columns and offers a way out ("Clear filters").

**Accessibility:** Toggle names the row it expands.

**Do not:** Show an empty table without a message.

### Timeline (activity feed)
**Classes:** `cn-timeline`

**Use for:** Run history, audit log, comments over time.

**Spec:** Vertical line, 20 px marker column using a brand shape, title, time, optional detail. Newest first.

**Accessibility:** `ol` with `time` elements.

**Do not:** Use for tabular data (use a table).

### Metric and delta
**Classes:** `cn-metric`, `cn-delta`

**Use for:** One headline number with a change.

**Spec:** Label above, display-size value (tabular), delta in success or danger with text ("+8.2% vs last week"). Optional sparkline.

**Accessibility:** Delta includes a text description, not only color or an arrow.

**Do not:** Show more than one number per metric.

### Charts (bars, hbar, sparkline, legend)
**Classes:** `cn-chart`, `cn-bars`, `cn-hbar`, `cn-sparkline`, `cn-legend`

**Use for:** Simple built-in charts. Use a charting library for anything complex and take colors from the tokens.

**Spec:** Series order: indigo, teal, coral, amber. Axis baseline 1 px `border`. Always give a title, a legend or labels, and a text alternative.

**Accessibility:** `role="img"` with a summary `aria-label`, or a data table alternative.

**Do not:** Use color as the only difference between series (use shapes and labels).

## Developer views

### Log viewer
**Classes:** `cn-log`, `__line[data-level]`

**Use for:** Run logs and live tails.

**Spec:** Mono 13 / 20, time, level word, message. Error lines tinted `danger-soft`. Max height 320 px, scrolls. Provide search, level filter and copy.

**Accessibility:** `role="log"` with an accessible name. Levels are words, not just color.

**Do not:** Truncate messages without a way to expand.

### JSON payload viewer
**Classes:** `cn-json`, `.k .s .n .b .z`

**Use for:** Show request and response payloads.

**Spec:** Keys `primary`, strings `teal-text`, numbers `amber-text`, booleans `coral-text`, null `text-subtle`. Collapsible objects (`details`). Add copy and "mask secrets".

**Accessibility:** Collapsible nodes are native `details`.

**Do not:** Show secrets unmasked by default.

### Diff viewer
**Classes:** `cn-diff`, `[data-op=add|del]`

**Use for:** Compare two versions of a mapping or request.

**Spec:** Line numbers, an op column (+, -), tinted backgrounds, mono text.

**Accessibility:** The +/- sign carries the meaning, color reinforces it.

**Do not:** Rely on color alone.

### Request trace (waterfall)
**Classes:** `cn-trace`, `__row`, `__track`, `__span`

**Use for:** Show where time went in one run.

**Spec:** Rows: name (indent by `--depth`), track with a span at `--start` and `--w`, duration. Span colors follow the protocol shape colors.

**Accessibility:** `role="table"`; durations are text.

**Do not:** Draw spans without a text duration.

### Test results
**Classes:** `cn-results`, `__summary`, `__item[data-state=fail]`

**Use for:** Outcome of "Test connection" or mapping checks.

**Spec:** Summary counts (passed, failed, total time), then a list. Failed items use a `danger` border and say what failed and how to fix it. Shapes: circle (pass), square (fail).

**Accessibility:** Each item states its result in text.

**Do not:** Show only a pass/fail count.

## Content

### Long-form text (prose)
**Classes:** `cn-prose-content`

**Use for:** Docs, release notes, help articles.

**Spec:** Max 720 px, heading margins 32 px above, lists and quotes styled with `primary` rule.

**Accessibility:** Real headings in order.

**Do not:** Use outside long-form content.

## Interaction patterns

### Table with filters and bulk actions
Toolbar above (search, filter buttons, spacer), table with a checkbox column and row-actions column, footer with count and pagination. When rows are selected, a bulk bar replaces the toolbar actions. Always confirm destructive bulk actions in an alert dialog that names the count ("Delete 2 connections?").

### Create connection flow
Stepper (Source, Target, Mapping, Review) above a form or choice cards. One decision per step, Back and Next at the bottom, errors at the step where they happen, a test step before activation. Save as a draft automatically.

### Debugging a run
Run header (status badge, duration), trace waterfall, then tabs for Request, Response, Log. Payloads in the JSON viewer with secrets masked, failed step expanded by default, test results for the last test.

### Settings page
Vertical tabs on the left, form sections on the right, one save bar at the bottom. Switches apply immediately; everything else saves with a button.

### Empty, loading and error states for each data view
Empty: icon, one sentence, one action. Loading: skeleton of the final layout. Error: alert with cause and fix, and a retry button.
