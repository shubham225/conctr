use crate::{
    Route,
    catalog::{ENTRIES, GROUPS},
    foundations,
};
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Shell() -> Element {
    let mut dark = use_context::<Signal<bool>>();
    let mut query = use_signal(String::new);
    let mut navigation_open = use_signal(|| false);
    let route = use_route::<Route>();
    let filter = query().trim().to_lowercase();
    let results = ENTRIES
        .iter()
        .filter(|entry| {
            format!("{} {} {}", entry.name, entry.group, entry.description)
                .to_lowercase()
                .contains(&filter)
        })
        .count();
    rsx! {
        a { class: "ds-skip", href: "#main-content", "Skip to content" }
        header { class: "ds-header",
            Link { to: Route::Home {}, class: "ds-brand", Logo {} span { class: "cn-body-lg", "Design System" } }
            Cluster {
                a { class: "ds-reference cn-small", href: format!("{}/ui-kit/index.html", asset!("/assets/reference")), target: "_blank", rel: "noopener", "HTML reference ↗" }
                Label { html_for: "gallery-theme", "Dark theme" }
                Switch { id: "gallery-theme", checked: dark(), on_checked_change: move |value| dark.set(value) }
                Button { variant: ButtonVariant::Secondary, class: "ds-nav-toggle", r#type: "button", aria_expanded: navigation_open(), aria_controls: "catalog-navigation", onclick: move |_| navigation_open.toggle(), "Browse" }
            }
        }
        div { class: "ds-shell",
            aside { class: "ds-sidebar", id: "catalog-navigation", "data-open": navigation_open(),
                div { class: "ds-search", Label { html_for: "component-search", "Find a component" }
                    Input { id: "component-search", r#type: "search", placeholder: "Search components…", value: query(), oninput: move |event: FormEvent| query.set(event.value()) }
                    span { class: "cn-caption", role: "status", "{results} components" }
                }
                nav { aria_label: "Design system catalog",
                    div { class: "ds-nav-group", span { class: "ds-nav-heading", "Foundations" }
                        Link { to: Route::Home {}, class: "ds-nav-link", aria_current: if matches!(route, Route::Home {}) { Some("page") } else { None }, onclick: move |_| navigation_open.set(false), "Overview" }
                        for (topic, title) in [("typography", "Typography"), ("colors", "Color palette"), ("spacing", "Spacing & elevation"), ("branding", "Logo & icons")] {
                            Link { to: Route::Foundation { topic: topic.into() }, class: "ds-nav-link", aria_current: if matches!(&route, Route::Foundation { topic: current } if current == topic) { Some("page") } else { None }, onclick: move |_| navigation_open.set(false), "{title}" }
                        }
                    }
                    for group in GROUPS {
                        if ENTRIES.iter().any(|e| e.group == *group && format!("{} {} {}", e.name, e.group, e.description).to_lowercase().contains(&filter)) {
                            div { class: "ds-nav-group", span { class: "ds-nav-heading", "{group}" }
                                for entry in ENTRIES.iter().filter(|e| e.group == *group && format!("{} {} {}", e.name, e.group, e.description).to_lowercase().contains(&filter)) {
                                    Link { to: Route::ComponentPage { slug: entry.slug.into() }, class: "ds-nav-link", aria_current: if matches!(&route, Route::ComponentPage { slug } if slug == entry.slug) { Some("page") } else { None }, onclick: move |_| navigation_open.set(false), "{entry.name}" }
                                }
                            }
                        }
                    }
                    if results == 0 { p { class: "cn-small ds-no-results", "No components match. Try another name." } }
                }
            }
            main { class: "ds-main", id: "main-content", tabindex: -1, Outlet::<Route> {} }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Overview · Conctr design system" }
        div { class: "ds-page-head", span { class: "ds-eyebrow", "Conctr / Design system" }
            h1 { "Where systems meet." }
            p { class: "cn-body-lg", "The visual language and reusable components for Conctr. Inspect live states, switch themes, and copy real Rust examples into your application." }
            Cluster { Badge { "Dioxus 0.7" } Badge { "{ENTRIES.len()} live examples" } Badge { "Light & dark" } }
        }
        div { class: "ds-grid ds-overview-links",
            for (topic, name, description) in [("typography", "Typography", "Sora, Inter and JetBrains Mono. One shared type scale."), ("colors", "Color palette", "Semantic colors and surfaces for both themes."), ("spacing", "Spacing & elevation", "A 4px spacing grid, control sizes, radii and shadows.")] {
                Link { to: Route::Foundation { topic: topic.into() }, class: "ds-overview-card", h3 { "{name}" } p { "{description}" } span { class: "cn-small", "Explore foundation →" } }
            }
        }
        h2 { "Component catalog" }
        p { "Primitives handle layout. Core components provide reusable controls. Composed components combine them for richer application interfaces." }
        p { "Each page includes a live preview, its public import and the exact example source. All connection names and values are illustrative." }
        for group in GROUPS {
            section { class: "ds-catalog-group", h3 { "{group}" }
                div { class: "ds-catalog-grid", for entry in ENTRIES.iter().filter(|entry| entry.group == *group) {
                    Link { to: Route::ComponentPage { slug: entry.slug.into() }, class: "ds-catalog-link", "{entry.name}" span { aria_hidden: "true", "↗" } }
                } }
            }
        }
    }
}

#[component]
pub fn Foundation(topic: String) -> Element {
    match topic.as_str() {
        "typography" => rsx! { foundations::Typography {} },
        "colors" => rsx! { foundations::Colors {} },
        "spacing" => rsx! { foundations::Spacing {} },
        "branding" => rsx! { foundations::Branding {} },
        _ => rsx! { NotFound { segments: vec![topic] } },
    }
}

#[component]
pub fn ComponentPage(slug: String) -> Element {
    let Some(entry) = ENTRIES.iter().find(|entry| entry.slug == slug) else {
        return rsx! { NotFound { segments: vec![slug] } };
    };
    let source_path = entry.module.replace("::", "/");
    let import = if entry.module == "primitives" {
        "use conctr_ui_dioxus::{components::primitives, prelude::*};".to_string()
    } else {
        format!("use conctr_ui_dioxus::components::{}::*;", entry.module)
    };
    rsx! {
        document::Title { "{entry.name} · Conctr design system" }
        div { class: "ds-page-head", span { class: "ds-eyebrow", "Components / {entry.group}" } h1 { "{entry.name}" } p { class: "cn-body-lg", "{entry.description}" } }
        div { class: "ds-import", code { "{import}" } }
        section { class: "ds-example-section", aria_label: "Live example",
            div { class: "ds-section-label", h2 { "Preview" } span { class: "cn-caption", "Interactive · shared library components" } }
            div { class: "ds-preview", Preview { key: "{slug}", slug: slug.clone() } }
        }
        div { Source { key: "source-{slug}", source: entry.source.to_string() } }
        aside { class: "ds-guidance", h3 { "Using this component" }
            p { "Import from the shared library and mount ThemeProvider once at your application root. The theme provider loads the fonts and design tokens used in this preview." }
            code { "crates/conctr-ui-dioxus/src/components/{source_path}/" }
            p { class: "cn-small", "Edit the implementation and adjacent stylesheet in this component’s module. Base controls live in ui/, layout helpers in primitives/, and reusable combinations in composed/." }
        }
    }
}

#[component]
fn Preview(slug: String) -> Element {
    match ENTRIES.iter().find(|entry| entry.slug == slug) {
        Some(entry) => (entry.preview)(),
        None => rsx! { "Example not found." },
    }
}

#[component]
fn Source(source: String) -> Element {
    let mut copy_status = use_signal(String::new);
    rsx! {
        section { class: "ds-example-section",
            div { class: "ds-section-label", h2 { "Usage" }
                Cluster { span { class: "cn-caption", role: "status", "{copy_status}" }
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm,
                        onclick: {
                            let source = source.clone();
                            move |_| {
                                let source = source.clone();
                                spawn(async move {
                                    let mut script = document::eval("const text = await dioxus.recv(); try { await navigator.clipboard.writeText(text); dioxus.send(true); } catch { dioxus.send(false); }");
                                    let sent = script.send(source).is_ok();
                                    let copied = sent && script.recv::<bool>().await.unwrap_or(false);
                                    copy_status.set(if copied { "Copied" } else { "Select and copy the code below" }.into());
                                });
                            }
                        }, "Copy source"
                    }
                }
            }
            CodeBlock { class: "ds-source", code { "{source}" } }
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! { EmptyState { title: "Page not found", description: "Choose a component from the catalog.", Link { to: Route::Home {}, class: "cn-btn cn-btn--primary", "View catalog" } } }
}
