
# CONCTR Development Guidelines

You are an expert Rust and Dioxus 0.7 developer working on CONCTR, an enterprise integration platform.

Use the official [Dioxus 0.7 documentation](https://dioxuslabs.com/learn/0.7) for all Dioxus-related development. Do not use outdated Dioxus APIs.

Dioxus 0.7 does not use `cx`, `Scope`, or `use_state`. Use the current component, signal, and hook APIs.

Provide concise code examples with clear explanations.

# Project Architecture

CONCTR is designed as a framework-independent integration platform with replaceable frontends.

The repository is organized into:

- `apps/` — Runnable applications such as web, desktop, and mobile.
- `crates/` — Reusable Rust libraries containing platform logic and UI-specific code.
- `docs/` — Project documentation and architecture decisions.
- `assets/` — Shared branding, images, icons, and other assets.

## Architecture Rules

- Keep core business logic independent of UI frameworks.
- Do not introduce Dioxus dependencies into framework-independent crates.
- Do not couple domain or application logic to a specific frontend.
- Keep UI-specific code inside framework-specific crates.
- Treat frontends as adapters to the underlying platform.
- Prefer reusable, testable Rust libraries over duplicated application logic.
- Keep dependencies flowing toward the core, not from the core toward applications.
- Avoid unnecessary abstractions and dependencies.

## Crate Responsibilities

- `conctr-core` — Shared primitives, identifiers, and foundational types.
- `conctr-auth` — Authentication and authorization.
- `conctr-connectors` — External system integrations.
- `conctr-engine` — Integration and workflow execution.
- `conctr-transform` — Data transformation and mapping.
- `conctr-storage` — Persistence implementations.
- `conctr-gateway` — API gateway and HTTP server functionality.
- `conctr-ui-dioxus` — Reusable Dioxus UI components.
- `conctr-dioxus-api` — Dioxus-specific server functions.

Keep these responsibilities separate. Do not put business logic in UI crates.

# Rust Guidelines

- Prefer clear, idiomatic, and maintainable Rust.
- Use strong types instead of loosely typed values where practical.
- Handle errors explicitly; avoid unnecessary `unwrap()` and `expect()`.
- Use `Result` and `Option` appropriately.
- Keep functions focused and reasonably small.
- Avoid unnecessary cloning and allocations.
- Prefer composition over deeply nested abstractions.
- Write unit tests for business logic and integration tests for external boundaries.
- Document public APIs and non-obvious behavior.
- Run `cargo fmt` and `cargo clippy` when appropriate.
- Keep dependencies minimal and justify new dependencies.

# Dioxus Dependency

Use Dioxus 0.7.1.

```toml
[dependencies]
dioxus = { version = "0.7.1" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

In this workspace, prefer shared dependency declarations in the root `Cargo.toml`:

```toml
[workspace.dependencies]
dioxus = { version = "0.7.1" }
```

Then reference it from application and UI crates:

```toml
[dependencies]
dioxus = { workspace = true }
```

Enable only the features required by each application.

# Launching a Dioxus Application

Create a main function that launches the root component.

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        "Hello, CONCTR!"
    }
}
```

Run the web application from the workspace root:

```sh
dx serve --package conctr-web-dioxus
```

You can also specify a manifest directly:

```sh
dx serve --manifest-path apps/web-dioxus/Cargo.toml
```

# UI with RSX

Use `rsx!` to describe UI elements.

```rust
rsx! {
    div {
        class: "container",
        color: "red",
        width: if condition { "100%" },
        "Hello, CONCTR!"
    }

    for i in 0..5 {
        div { "{i}" }
    }

    if condition {
        div { "Condition is true!" }
    }

    {children}
}
```

Prefer direct RSX loops and conditionals when possible. Use braces for Rust expressions.

# Assets

Use the `asset!` macro to reference local assets.

Asset paths start with `/` and are relative to the asset root configured for the application.

```rust
rsx! {
    img {
        src: asset!("/assets/image.png"),
        alt: "An image",
    }
}
```

# Styles

Use `document::Stylesheet` to load a stylesheet into the document head.

```rust
rsx! {
    document::Stylesheet {
        href: asset!("/assets/main.css"),
    }
}
```

Keep styles organized and avoid duplicating shared styles across applications.

# Components

Components are functions annotated with `#[component]`.

- Component names must start with an uppercase letter or contain an underscore.
- Component props should use owned values rather than references.
- Props should implement `PartialEq` and `Clone` where required.
- Keep components focused and reusable.
- Keep business logic outside UI components.

```rust
#[component]
fn Greeting(name: String) -> Element {
    rsx! {
        div {
            "Hello, {name}!"
        }
    }
}
```

# State Management

Dioxus 0.7 uses signals for reactive state.

## Local State

Use `use_signal` for component-local state.

```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 {
            "Count: {count}"
        }

        button {
            onclick: move |_| *count.write() += 1,
            "Increment"
        }
    }
}
```

Use:

- `signal()` to read and clone the current value.
- `signal.read()` to borrow the current value.
- `signal.write()` to mutate the current value.
- `signal.with_mut()` to mutate through a closure.

Use `use_memo` for derived values that should be recalculated when their reactive dependencies change.

```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    let doubled = use_memo(move || count() * 2);

    rsx! {
        h1 { "Count: {count}" }
        h2 { "Doubled: {doubled}" }

        button {
            onclick: move |_| *count.write() += 1,
            "Increment"
        }
    }
}
```

## Context API

Use `use_context_provider` to provide shared state to descendants and `use_context` to consume it.

```rust
#[component]
fn App() -> Element {
    let theme = use_signal(|| "light".to_string());

    use_context_provider(|| theme);

    rsx! {
        Child {}
    }
}

#[component]
fn Child() -> Element {
    let theme = use_context::<Signal<String>>();

    rsx! {
        div {
            "Current theme: {theme}"
        }
    }
}
```

# Async Operations

Use `use_resource` for asynchronous operations managed by a component.

```rust
#[component]
fn DogInfo() -> Element {
    let mut dog = use_resource(|| async move {
        // Perform an asynchronous request.
        "Dog information"
    });

    rsx! {
        match dog() {
            Some(info) => rsx! {
                div { "{info}" }
            },
            None => rsx! {
                div { "Loading..." }
            }
        }
    }
}
```

Resources automatically track reactive dependencies read by their async closures.

For server-rendered applications, consider `use_server_future` when data must be available consistently during server rendering and initial client hydration.

# Routing

Use Dioxus Router for navigation.

Define routes in a Rust enum that derives `Routable`.

```rust
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            a {
                href: "/",
                "Home"
            }
        }

        Outlet<Route> {}
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
```

Enable the router feature:

```toml
dioxus = { version = "0.7.1", features = ["router"] }
```

# Fullstack

Dioxus Fullstack supports server rendering and server functions.

Enable the fullstack feature:

```toml
dioxus = { version = "0.7.1", features = ["fullstack"] }
```

Use Cargo features to separate server and client functionality.

## Server Functions

Use `#[get]` and `#[post]` to define server functions.

```rust
use dioxus::prelude::*;

#[post("/api/double/:number")]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
    Ok(number * 2)
}
```

Server functions should be used for Dioxus-specific fullstack communication.

Keep generic API contracts and platform services independent of Dioxus.

## Hydration

Hydration makes server-rendered HTML interactive on the client.

The initial UI rendered on the server must match the initial UI rendered on the client.

Important guidelines:

- Avoid relying on browser-specific APIs during server rendering.
- Use `use_server_future` for data that must be available during initial server rendering and hydration.
- Use `use_effect` for browser-specific side effects that should run after rendering.
- Do not access browser-only APIs directly from framework-independent crates.
- Handle loading and error states consistently.

# Testing

- Test core business logic independently of Dioxus.
- Prefer unit tests inside the crate that owns the functionality.
- Use integration tests for interactions between crates.
- Mock external services when testing connector behavior.
- Keep tests deterministic and avoid depending on live external services.

# Documentation

- Update relevant documentation when changing architecture or public behavior.
- Record significant architectural decisions in `docs/architecture/decisions/`.
- Keep README files accurate and avoid documenting planned functionality as implemented.
- Include examples for public APIs where useful.

# Development Commands

Run these from the workspace root:

```sh
cargo check --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets
```

Serve individual Dioxus applications:

```sh
dx serve --package conctr-web-dioxus
dx serve --package conctr-desktop-dioxus
dx serve --package conctr-mobile-dioxus
```

# General Instructions

- Follow the existing project structure and naming conventions.
- Inspect existing code before introducing new abstractions.
- Do not rename or move files without a clear reason.
- Do not add dependencies without justification.
- Do not mix UI concerns with platform logic.
- Prefer small, focused changes over unnecessary rewrites.
- Explain important architectural or dependency changes.
- Use the official Dioxus 0.7 documentation when uncertain about an API.


## Product architecture and planning context

For Conctr product/platform work, start with [docs/ai/AI-INSTRUCTIONS.md](docs/ai/AI-INSTRUCTIONS.md)
and read the relevant ownership/contracts from there. The dated implementation
inventory is [docs/roadmap/current-state.md](docs/roadmap/current-state.md).
Proposed ADRs and prototype screens are plans, not claims of working features.
Portable task-specific skills are indexed in [docs/ai/README.md](docs/ai/README.md).
Use this context within the user's requested scope; it does not require creating
all planned crates or seeking approval for every documented open question.
