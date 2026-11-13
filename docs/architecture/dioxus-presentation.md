# Dioxus presentation structure

## Implemented component library

`conctr-ui-dioxus` owns reusable presentation components. The source is organized
by responsibility rather than its upstream origin:

```text
crates/conctr-ui-dioxus/
  src/
    assets/                    # Fonts, tokens, branding and CSS registration
    components/
      mod.rs                   # Layers and compatibility re-exports
      primitives/              # Box, Flex, Grid, Stack, Inline, Container, ...
      ui/                      # Base controls and presentation foundations
        button/
          mod.rs
          component.rs
          style.css
        dialog/
        sidebar/
        table/
        theme_provider/
        ...
      composed/                # Reusable combinations of base components
        connection_flow/
        connection_node/
        empty_state/
        modal/
        stat/
  tests/
```

Choose `ui` instead of `core` here: `conctr-core` already names the
framework-independent foundation. `ui` clearly identifies presentation controls.
This is a project convention, not a requirement imposed by Dioxus.

New generic data tables, flow editors and dashboard widgets can be added to
`components/composed/<family>/` as they are implemented. Do not pre-create empty
modules for planned features. Their props should accept data and callbacks;
fetching Conctr data, permission decisions and workflows belong to the app and
platform layers. A reusable table renderer belongs here; a page that loads the
current user's connections belongs in the shared application.

Dependencies flow from composed components toward UI controls and layout
primitives. UI controls and primitives must not import composed components.
Keep Rust implementation and optional CSS together in the owning family.
Registry-derived controls are locally owned source in `ui/`; their provenance
is recorded in the crate's `UPSTREAM.md`.

Canonical imports:

```rust
use conctr_ui_dioxus::components::{
    ui::button::{Button, ButtonVariant},
    primitives::{Flex, Grid, Stack},
    composed::connection_flow::ConnectionFlow,
};
```

The prelude and existing `components::button`, `components::stack`, etc. paths
remain supported through re-exports. The old `components::dioxus` namespace is a
hidden compatibility namespace; new code should use the canonical layers.
`primitives::Box` is qualified to avoid shadowing Rust's Box in RSX expansion;
`LayoutBox` is its prelude name.

## Naming the design artifacts

Use **Conctr Design System** for the whole system: principles, tokens, assets,
implemented components, accessibility/interaction rules and usage guidance.
`apps/conctr-design-system` is the design system's runnable documentation and
component showcase. Keep its current package name.

Use **design kit** for the supplied assets and reference examples in `design/`,
including `design/ui-kit/`. The kit is one part of the design system.

## Recommended application structure (proposed)

The application crates currently contain placeholder App components. The shared
application crate now exists as a workspace scaffold (starter add function/test).
The structure below is still a recommendation: App, routes, product pages and
launcher wiring are not implemented.

Keep three small launchers while sharing one Dioxus application. Separate
executables support different packaging, feature flags and native configuration;
they do not require separate implementations of pages, routes or state.

```text
apps/
  web-dioxus/
    Cargo.toml                 # Web/server renderer features
    src/main.rs                # Launch the shared App; web host configuration
  desktop-dioxus/
    Cargo.toml                 # Desktop renderer features
    src/main.rs                # Shared App + window/menu configuration
  mobile-dioxus/
    Cargo.toml                 # Mobile renderer features
    src/main.rs                # Shared App + mobile bootstrap
  conctr-design-system/        # Independent component documentation app

crates/
  conctr-app-dioxus/            # Scaffold exists; proposed contents below
    Cargo.toml
    assets/app.css             # App layouts; component styles stay in UI crate
    src/
      lib.rs                   # Export App
      app.rs                   # Theme, providers, router, shared resources
      routes.rs                # One Routable enum for shared navigation
      layouts/
        mod.rs
        app_layout.rs          # Sidebar/header + Outlet<Route>
        auth_layout.rs         # Authentication page layout
      features/
        mod.rs
        connections/
          mod.rs
          pages/
            mod.rs
            list.rs
            detail.rs
            create.rs
          components/
            mod.rs
            connection_form.rs # Product-specific form composition
          state.rs             # Feature presentation state and async UI orchestration
        runs/
          mod.rs
          pages/
            mod.rs
            list.rs
            detail.rs
        settings/
          mod.rs
          pages/
            mod.rs
            profile.rs
      state/
        mod.rs                 # Only genuinely app-wide presentation state
      services/
        mod.rs                 # Calls shared API adapters; no business rules
  conctr-ui-dioxus/             # Reusable components; no product pages/API calls
  conctr-api-dioxus/            # Dioxus server-function transport adapters
  conctr-core/                 # Framework-independent types
  conctr-auth/                 # Authentication/authorization rules
  conctr-engine/               # Execution logic
  ...                          # Existing independent platform libraries
```

The feature/page filenames above are illustrative, not implemented product
features. Start with the pages needed now; create additional folders when they
have a concrete responsibility. Keep a small feature in one or two files until
it benefits from a split.

A launcher can initially be as small as:

```rust
use conctr_app_dioxus::App;

fn main() {
    dioxus::launch(App);
}
```

Put routes in `routes.rs`, page implementations beside their feature, and shared
shells in `layouts/`. Product-specific components used by one feature stay in
that feature. Promote a component to `conctr-ui-dioxus::components::composed`
when it has a reusable presentation API and no dependency on product services.
Do not import the application crate back into the UI library.

Use responsive CSS for compact navigation, page stacking and screen width
changes. A narrow desktop window should receive the same responsive layout as
a narrow browser. Use Cargo features or target configuration for actual platform
capabilities such as native window menus, OS dialogs, filesystem access and deep
links. Keep most platform setup in launchers; add a shared capability interface
only when a real cross-platform call site needs it.

The shared App should not enable every renderer. Each launcher enables its
renderer; forward only the shared server-function implementation features needed
by the server build. Native clients call the configured backend, with native
fullstack endpoint/authentication configuration handled at their boundary.
Backend secrets, persistence and authorization enforcement stay server-side.
Do not put browser-only initialization in the shared App's server render path;
initial SSR data and hydration must agree.

A single executable with web/desktop/mobile feature flags is also valid. Retain
the existing small launchers here because they provide clear packaging boundaries
and avoid an unnecessary app-path migration. Revisit that choice if those entry
points remain permanently identical.

The official [Dioxus 0.7 platform guide](https://dioxuslabs.com/learn/0.7/guides/platforms/)
describes sharing UI and enabling platform-specific code through Cargo features.
