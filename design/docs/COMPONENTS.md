# Conctr component specifications

Version 1.1. For designers, developers and AI tools. Live examples: `ui-kit/index.html`. Styles: `ui-kit/conctr-ui.css`. Token values: `DESIGN-TOKENS.md`.

**How to read each entry:** purpose, variants, spec (sizes and tokens), states, accessibility, do and don't, a code example. Class names use the `cn-` prefix; modifiers use `--` (BEM style): `cn-btn cn-btn--primary`.

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

## Dioxus layout primitives

The Dioxus adapters extend the existing Stack/Cluster layout helpers with Box,
Flex, Grid and Inline. They reuse the spacing scale and add no colors or type
styles. Box is a neutral div with token-based padding. Stack is a vertical flex
layout; Inline is a wrapping horizontal layout. Flex exposes row/column,
alignment, distribution and wrapping. Grid lays out up to twelve equal columns
and reduces the column count when the available width cannot fit the configured
minimum item width. Use gap for siblings; avoid padding on both parent and
children unless both levels need it. All accept native attributes and semantic
labels as needed. Layout containers have no implicit interactive semantics.

The gallery groups these as Primitives, individual controls as Core, and Stats,
connection flows, empty states and composed Modal as Composed. This organization
leaves a home for future dashboard, data-table and flow-editor compositions.

General interface action icons are supplied by the existing Lucide dependency
in the Dioxus adapter. The branded kit currently contains protocol and
authentication artwork only. Keep its six protocol shape meanings unchanged;
new protocol shapes require an explicit design specification.
