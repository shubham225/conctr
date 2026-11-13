# States, accessibility and interaction rules

Status: **Proposed product rules**, building on the existing design kit.

| State | Required behavior |
| --- | --- |
| Initial loading | Layout-matched animated skeleton with reduced-motion alternative |
| Background refresh | Preserve data, focus and filters; show refresh/stale indication |
| Empty workspace | Explain the next permitted action; do not invent metrics |
| No search results | Preserve query and offer clear-filter control |
| Validation error | Field/node-specific message and programmatic association; move focus to summary when appropriate |
| Permission denied | Explain available action without leaking another tenant's metadata |
| Network/session loss | Preserve local draft, indicate offline/expired session and reauthenticate safely |
| Conflict | Preserve local changes and compare against the server revision |
| Unknown external outcome | Distinct warning and permitted reconciliation path; no casual retry button |
| Success | Past-tense confirmation; critical results also persist in the page/history |

Use native semantics, labels, visible focus, keyboard operation and contrast from
the design kit. Status has text/icon as well as color. Tooltip is supplemental,
not the only place instructions or errors exist. Dialogs restore focus; destructive
actions identify the resource/environment and consequence. Mobile touch targets
follow kit guidance. Motion must respect reduced motion.

The flow designer needs an equivalent ordered/structured outline. Tables expose
headers and captions; virtualized data preserves keyboard navigation and offers
a usable paginated alternative when needed. Live run updates must not announce
every event noisily; announce meaningful status changes and provide a readable
history. User-entered node names and external error strings are escaped text.
