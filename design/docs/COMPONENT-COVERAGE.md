# Component coverage

Compared with three component libraries: [shadcn/ui](https://ui.shadcn.com/docs/components), the [Atlassian Design System](https://atlassian.design/components) and [Dioxus Components](https://dioxuslabs.com/components) (its list is loaded by script, so the entries below come from the DioxusLabs components repository). Lists checked November 2026; names are merged where libraries call the same thing differently. An "x" means the library has that component.

Statuses: **v1.1** existed before, **v1.2** added in this version, **Partly** composed from other parts, **Not included** with the reason.

| Reference component | shadcn/ui | Atlassian | Dioxus | Conctr component | Status |
|---|:-:|:-:|:-:|---|---|
| Accordion | x |  | x | Accordion | v1.2 |
| Alert | x | x |  | Alert, Banner, Inline message | v1.1 + v1.2 |
| Alert dialog | x |  | x | Alert dialog (`cn-modal--alert`) | v1.2 |
| Aspect ratio | x |  | x | Aspect ratio | v1.2 |
| Avatar | x | x | x | Avatar | v1.1 |
| Avatar group |  | x |  | Avatar group | v1.2 |
| Badge | x |  |  | Badge, shape badge, status badge | v1.1 |
| Banner |  | x |  | Banner | v1.2 |
| Breadcrumb | x | x |  | Breadcrumb | v1.1 |
| Button | x | x | x | Button | v1.1 |
| Button group | x | x |  | Button group | v1.2 |
| Calendar | x | x | x | Calendar | v1.2 |
| Card | x |  | x | Card | v1.1 |
| Carousel | x |  |  | - | Not included: marketing pattern, use a library if needed |
| Chart | x |  |  | Charts (bars, hbar, sparkline) and tokens for libraries | v1.2 |
| Checkbox | x | x | x | Checkbox | v1.1 |
| Collapsible | x |  | x | Collapsible | v1.2 |
| Combobox | x |  |  | Combobox and listbox | v1.2 |
| Command | x |  |  | Command palette | v1.2 |
| Comment |  | x |  | Item + timeline can be composed | Partly: no dedicated comment component |
| Context menu | x |  | x | Context menu | v1.2 |
| Data table | x |  |  | Table, sortable and selectable table, toolbar, bulk bar | v1.1 + v1.2 |
| Date picker | x | x | x | Calendar + native date input | v1.2 |
| Date time picker |  | x |  | Native `datetime-local` styled as input, calendar for dates | v1.2 |
| Dialog (modal) | x | x | x | Modal | v1.1 |
| Direction (RTL) | x |  |  | - | Not included: layouts are not RTL-tested yet |
| Drawer | x | x |  | Drawer and sheet | v1.2 |
| Dropdown menu | x | x | x | Menu (and `DropdownMenu` in the Dioxus crate) | v1.1 |
| Dynamic table |  | x |  | Sortable and selectable table | v1.2 |
| Empty state | x | x |  | Empty state | v1.1 |
| Field / Form | x | x | x | Field, Form layout | v1.1 + v1.2 |
| Flag |  | x |  | Toast | v1.1 |
| Focus ring |  | x |  | `cn-focus-ring` utility, `--cn-focus-ring` token | v1.2 |
| Heading / Text / Typography | x | x |  | Type scale classes, prose | v1.1 + v1.2 |
| Hover card | x |  | x | Hover card | v1.2 |
| Icon |  | x |  | Protocol and auth icon set, inline icons | v1.1 |
| Image |  | x |  | Aspect ratio, logo files | v1.2 |
| Inline dialog / Popup |  | x |  | Popover | v1.2 |
| Inline edit |  | x |  | Inline edit | v1.2 |
| Inline message |  | x |  | Inline message | v1.2 |
| Input / Textfield | x | x | x | Input | v1.1 |
| Input group | x |  |  | Input group, search, secret, endpoint input | v1.2 |
| Input OTP | x |  |  | One-time code (OTP) | v1.2 |
| Item | x |  |  | Item and item group | v1.2 |
| Kbd | x |  |  | Kbd | v1.1 |
| Label | x |  | x | Label (`cn-label`) | v1.1 |
| Link |  | x |  | Link (`cn-anchor`) | v1.2 |
| Lozenge |  | x |  | Status badge | v1.1 |
| Menu |  | x |  | Menu | v1.1 |
| Menubar | x |  | x | Menubar | v1.2 |
| Modal dialog |  | x |  | Modal, Confirm modal | v1.1 |
| Native select | x |  |  | Select | v1.1 |
| Navigation menu / Navbar | x |  | x | Navbar | v1.2 |
| Navigation system (side nav) |  | x |  | Sidebar nav, app shell | v1.1 + v1.2 |
| Page header |  | x |  | Page header | v1.2 |
| Pagination | x | x |  | Pagination | v1.1 |
| Popover | x |  | x | Popover | v1.2 |
| Popup select |  | x |  | Combobox | v1.2 |
| Progress | x | x | x | Progress | v1.1 |
| Progress indicator |  | x |  | Dots | v1.2 |
| Progress tracker |  | x |  | Stepper | v1.2 |
| Radio group | x | x | x | Radio, Choice card | v1.1 + v1.2 |
| Range / Slider | x | x | x | Slider | v1.2 |
| Rating |  |  |  | - | Not included: not needed for an integration platform |
| Resizable | x |  |  | Split view (drag script needed) | v1.2 |
| Scroll area | x |  | x | Scroll area | v1.2 |
| Section message |  | x |  | Alert | v1.1 |
| Select | x | x | x | Select, Combobox | v1.1 + v1.2 |
| Separator | x |  | x | Divider, vertical divider | v1.1 + v1.2 |
| Sheet | x |  | x | Drawer and sheet | v1.2 |
| Sidebar | x |  | x | Sidebar nav, app shell | v1.1 + v1.2 |
| Skeleton | x | x | x | Skeleton | v1.1 |
| Sonner / Toast | x |  | x | Toast | v1.1 |
| Spinner | x | x |  | Spinner | v1.1 |
| Spotlight |  | x |  | Spotlight | v1.2 |
| Switch / Toggle (on-off) | x | x | x | Switch | v1.1 |
| Table | x |  |  | Table | v1.1 |
| Tabs | x | x | x | Tabs, vertical tabs | v1.1 + v1.2 |
| Tag / Tag group |  | x |  | Tag, tag group, tag input | v1.2 |
| Textarea | x | x | x | Textarea | v1.1 |
| Tile |  | x |  | Tile | v1.2 |
| Time picker |  | x |  | Native `time` input styled as input | v1.2 |
| Toggle / Toggle group | x | x | x | Toggle, segmented control | v1.2 |
| Tooltip | x | x | x | Tooltip | v1.1 |
| Tree |  | x |  | Tree view, table tree | v1.2 |
| Visually hidden |  | x |  | `cn-sr-only` | v1.2 |
| Layout primitives (Box, Flex, Grid, Stack, Inline) |  | x |  | `cn-stack`, `cn-cluster`, `cn-grid` | v1.1 + v1.2 |
| AI elements (conversation, message, prompt input...) | x |  |  | - | Not included: not part of the product |


### Added beyond the three reference lists (integration-specific)
Endpoint input, secret input, key-value editor, mapping row, protocol choice cards, connection flow, status vocabulary, log viewer, JSON payload viewer, diff viewer, request trace (waterfall), test results and the file dropzone for WSDL, OpenAPI and `$metadata`.

## What "not included" means
- **Carousel, Rating:** not needed for a developer console. Add only if marketing pages need them.
- **Direction (RTL):** the kit uses physical left/right in places; a right-to-left pass is planned before localizing to Arabic or Hebrew.
- **AI elements:** conversation, prompt input and similar are for chat products. Conctr does not ship a chat surface.
- **Comment:** compose an `item` with an avatar and a timeline entry until a real need appears.
