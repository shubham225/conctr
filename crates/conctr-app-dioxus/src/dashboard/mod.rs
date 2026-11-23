//! Dashboard and its server details panel.
use conctr_api_dioxus::system_details;
use conctr_ui_dioxus::ui::{
    button::Button,
    card::{Card, CardContent, CardHeader},
};
use dioxus::prelude::*;

#[component]
pub(crate) fn Dashboard() -> Element {
    let mut details = use_resource(system_details);

    rsx! {
        div { class: "grid max-w-4xl gap-4",
            h1 { class: "text-2xl font-semibold", "Dashboard" }
            section { aria_labelledby: "system-details-heading",
                Card {
                    CardHeader {
                        div { class: "flex items-center gap-3",
                            span { class: "rounded-lg bg-accent p-2 text-accent-foreground", aria_hidden: "true",
                                icons::Server { class: "size-5" }
                            }
                            h2 { id: "system-details-heading", class: "text-lg font-semibold", "System details" }
                        }
                    }
                    CardContent {
                        match &*details.read() {
                            Some(Ok(system)) => rsx! {
                                dl { class: "grid gap-3 sm:grid-cols-3",
                                    for (label, value) in [
                                        ("Server version", system.version.as_str()),
                                        ("Operating system", system.os.as_str()),
                                        ("Architecture", system.architecture.as_str()),
                                    ] {
                                        div { key: "{label}", class: "grid gap-1 rounded-lg border bg-muted p-4",
                                            dt { class: "text-sm text-muted-foreground", "{label}" }
                                            dd { class: "break-all font-mono text-lg font-semibold", "{value}" }
                                        }
                                    }
                                }
                            },
                            Some(Err(_)) => rsx! {
                                div { class: "flex flex-wrap items-center gap-3",
                                    p { class: "text-sm text-destructive", role: "alert", "Could not load system details." }
                                    Button { onclick: move |_| { details.clear(); details.restart(); }, "Retry" }
                                }
                            },
                            None => rsx! { p { class: "text-sm text-muted-foreground", role: "status", "Loading system details…" } },
                        }
                    }
                }
            }
        }
    }
}
