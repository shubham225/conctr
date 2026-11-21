mod catalog;
mod demo;
mod examples;
use conctr_ui_dioxus::{assets::images, prelude::*};
use dioxus::prelude::*;
#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/components/:slug")]
    ComponentPage { slug: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let dark = use_signal(|| false);
    use_context_provider(|| dark);
    rsx! {
        document::Stylesheet { href: asset!("/src/style.css") }
        document::Title { "Conctr Component Showcase" }
        document::Link { rel: "icon", href: images::FAVICON }
        ThemeProvider { theme: if dark() { Theme::Dark } else { Theme::Light }, Router::<Route> {} }
    }
}
#[component]
fn Shell() -> Element {
    let mut dark = use_context::<Signal<bool>>();
    rsx! {
        div { class: "min-h-screen",
            header { class: "sticky top-0 z-50 flex flex-wrap items-center justify-between gap-4 border-b border-border bg-background p-4 md:px-8",
                Link { to: Route::Home {},
                    img {
                        src: if dark() { images::HORIZONTAL_COLOR_DARK_SVG } else { images::HORIZONTAL_COLOR_SVG },
                        alt: "Conctr",
                        width: 140,
                        height: 36,
                    }
                }
                span { class: "text-sm text-muted-foreground", "Conctr Component Showcase" }
                BrandThemeToggle { dark: dark(), on_change: move |value| dark.set(value) }
            }
            div { class: "cn-gallery-frame",
                nav {
                    class: "cn-component-explorer",
                    "aria-label": "Component explorer",
                    strong { class: "cn-explorer-title", "Explore the kit" }
                    Link { to: Route::Home {}, class: "cn-explorer-link", "Brand foundations" }
                    h2 { class: "cn-explorer-group", "Composed" }
                    for entry in catalog::COMPOSED {
                        Link {
                            to: Route::ComponentPage {
                                slug: entry.slug.into(),
                            },
                            class: "cn-explorer-link",
                            {entry.slug.replace('-', " ")}
                        }
                    }
                    h2 { class: "cn-explorer-group", "Components" }
                    for entry in catalog::ENTRIES {
                        Link {
                            to: Route::ComponentPage {
                                slug: entry.slug.into(),
                            },
                            class: "cn-explorer-link",
                            {entry.slug.replace('-', " ")}
                        }
                    }
                }
                main { class: "cn-gallery-main min-w-0", Outlet::<Route> {} }
            }
        }
    }
}
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "grid gap-8",
            div { class: "grid gap-4",
                h1 { "Where systems meet." }
                p { class: "text-muted-foreground",
                    "Explore Conctr’s design kit, components and compositions."
                }
            }
            section {
                class: "grid min-w-0 gap-4",
                style: "grid-template-columns:minmax(0,1fr)",
                h2 { "Brand foundations" }
                div { class: "grid gap-4 sm:grid-cols-3",
                    for (name, color) in [
                        ("Indigo", "var(--cn-primary)"),
                        ("Teal", "var(--cn-teal)"),
                        ("Coral", "var(--cn-coral)"),
                        ("Amber", "var(--cn-amber)"),
                        ("Surface", "var(--cn-surface)"),
                        ("Text", "var(--cn-text)"),
                    ]
                    {
                        div { class: "overflow-hidden rounded-lg border border-border",
                            div { class: "h-16", style: "background:{color}" }
                            p { class: "p-3 text-sm", "{name}" }
                        }
                    }
                }
                p { "Sora headings · Inter interface · JetBrains Mono code" }
            }
            section {
                class: "grid min-w-0 gap-4",
                style: "grid-template-columns:minmax(0,1fr)",
                h2 { "Controls" }
                div { class: "flex flex-wrap gap-4",
                    Button { "Test connection" }
                    Button { variant: ButtonVariant::Outline, "View source" }
                    Button { disabled: true, "Unavailable" }
                }
                conctr_ui_dioxus::ui::input::Input { placeholder: "Connection name" }
            }
            section {
                class: "grid min-w-0 gap-4",
                style: "grid-template-columns:minmax(0,1fr)",
                h2 { "Workflow canvas" }
                p { class: "text-sm text-muted-foreground",
                    "Interactive upstream demonstration. Canvas state is presentation only; no flow execution or persistence is connected."
                }
                Link {
                    to: Route::ComponentPage {
                        slug: "conctr-workflow".into(),
                    },
                    class: "text-primary underline",
                    "Open workflow canvas"
                }
            }
            p { class: "text-sm text-muted-foreground",
                "Rust UI is experimental. AutoForm is an upstream shell, not automatic form generation. Examples illustrate controls, not available Conctr integrations."
            }
        }
    }
}
#[component]
fn ComponentPage(slug: String) -> Element {
    let entry = catalog::COMPOSED
        .iter()
        .chain(catalog::ENTRIES.iter())
        .find(|entry| entry.slug == slug);
    rsx! {
            if let Some(entry) = entry {
                div {
                    class: "grid min-w-0 gap-6",
                    style: "grid-template-columns:minmax(0,1fr)",
                    key: "{slug}",
                    h1 { {slug.replace('-', " ")} }
                    div {
                        class: if ["table", "data-table", "data-grid", "toast", "field-mapping"]
        .contains(&slug.as_str()) { "min-w-0" } else { "min-w-0 rounded-lg border border-border bg-card p-6" },
                        style: if [
        "date-picker",
        "composed-date-picker",
        "date-time-picker",
        "multi-select",
        "input-phone",
        "popover",
        "dropdown-menu",
        "select",
        "combobox",
    ]
        .contains(&slug.as_str()) { "min-height:480px;overflow:visible" } else { "overflow:auto" },
                        { (entry.preview)
                                                                            () }
                    }
                    UsageCode { source: entry.source.to_string() }
                    for variant in catalog::variants(&slug) {
                        section {
                            class: "grid min-w-0 gap-4",
                            style: "grid-template-columns:minmax(0,1fr)",
                            h2 { { variant.slug
                                                                                        .replace('-', " ") } }
                            div { class: if ["field-mapping", "visual-field-mapping"].contains(&slug.as_str()) { "min-w-0" } else { "min-w-0 overflow-auto rounded-lg border border-border bg-card p-6" },
                                { (variant
                                                                                                    .preview) () }
                            }
                            UsageCode { source: variant.source.to_string() }
                        }
                    }
                }
            } else {
                EmptyState {
                    title: "Example not found",
                    description: "Choose a component from the catalog.",
                }
            }
        }
}
#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        EmptyState {
            title: "Page not found",
            description: "Choose a component from the catalog.",
        }
    }
}
/// All examples show public consumer imports, never internal registry paths.
#[component]
fn UsageCode(source: String) -> Element {
    let source = source
        .replace("crate::ui::", "conctr_ui_dioxus::ui::")
        .replace("crate::composed::", "conctr_ui_dioxus::composed::")
        .replace("crate::{", "conctr_ui_dioxus::{")
        .replace("crate::hooks::", "conctr_ui_dioxus::hooks::")
        .replace("use super::", "use conctr_ui_dioxus::composed::");
    rsx! {
        details { class: "min-w-0",
            summary { class: "cursor-pointer text-sm font-medium text-primary", "Usage code" }
            pre {
                class: "mt-4 overflow-auto rounded-md border border-border bg-muted p-4 text-sm",
                style: "max-width:100%;tab-size:4;font-size:var(--cn-fs-code);line-height:20px;white-space:pre",
                code { class: "language-rust", "{source}" }
            }
        }
    }
}
