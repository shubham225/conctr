use crate::Route;
use conctr_ui_dioxus::components::{
    composed::empty_state::EmptyState,
    ui::{
        card::Card,
        icon::{IconName, UiIcon},
    },
};
use dioxus::prelude::*;

#[component]
pub fn Connections() -> Element {
    rsx! {
        div { class: "mx-auto grid w-full max-w-6xl gap-8",
            PageHeading { title: "Connections", description: "Bring your systems together. Manage connections in one workspace." }
            Card {
                EmptyState {
                    title: "A place for your connections",
                    description: "Your workspace is ready for its first connection. Connection creation will be available when the platform services are connected.",
                    icon: rsx! { UiIcon { name: IconName::Connections } },
                }
            }
            div { class: "grid gap-6 md:grid-cols-3",
                Guide { number: "01", title: "Connect your systems", description: "Define a source and target for the data you need to move." }
                Guide { number: "02", title: "Shape your data", description: "Plan how fields map between your systems." }
                Guide { number: "03", title: "Follow each run", description: "Inspect execution results once runtime services are available." }
            }
        }
    }
}

#[component]
pub fn Runs() -> Element {
    rsx! {
        div { class: "mx-auto grid w-full max-w-6xl gap-8",
            PageHeading { title: "Runs", description: "Follow the work moving through your connections." }
            Card {
                EmptyState { title: "No run history yet", description: "Execution services are not connected. Run history and results will appear here when available.",
                    icon: rsx! { UiIcon { name: IconName::Runs } },
                }
            }
        }
    }
}

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "mx-auto grid w-full max-w-6xl gap-8",
            PageHeading { title: "Settings", description: "Make this workspace your own." }
            Card {
                div { class: "grid gap-3 p-6",
                    h2 { class: "cn-h4", "Appearance" }
                    p { class: "text-sm text-muted-foreground", "Use the theme button in the header to switch between light and dark. Your selection applies to this session." }
                }
            }
            p { class: "text-sm text-muted-foreground", "Workspace, account and credential settings will be available when platform services are connected." }
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        EmptyState { title: "Page not found", description: "This address does not match a workspace page.",
            Link { to: Route::Connections {}, class: "text-primary underline", "Back to connections" }
        }
    }
}

#[component]
fn PageHeading(title: String, description: String) -> Element {
    rsx! { div { class: "grid gap-2", h1 { class: "cn-h2", "{title}" } p { class: "text-muted-foreground", "{description}" } } }
}

#[component]
fn Guide(number: String, title: String, description: String) -> Element {
    rsx! {
        div { class: "grid content-start gap-3",
            span { class: "cn-mono text-sm text-primary", "{number}" }
            h2 { class: "cn-h4", "{title}" }
            p { class: "text-sm text-muted-foreground", "{description}" }
        }
    }
}
