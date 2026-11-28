//! Dashboard and its server details panel.
use conctr_api_dioxus::system_details;
use conctr_ui_dioxus::ui::{
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardHeader, CardSize},
};
use dioxus::prelude::*;

#[component]
pub(crate) fn Dashboard() -> Element {
    let mut details = use_resource(system_details);

    rsx! {
        div { class: "grid max-w-xl gap-4",
            h1 { class: "text-2xl font-semibold", "Dashboard" }
            section { aria_labelledby: "system-details-heading",
                Card { size: CardSize::Sm,
                    CardHeader {
                        div { class: "flex items-center gap-3",
                            span { class: "text-muted-foreground", aria_hidden: "true",
                                icons::Server { class: "size-5" }
                            }
                            h2 { id: "system-details-heading", class: "text-lg font-semibold", "System details" }
                        }
                    }
                    CardContent {
                        match &*details.read() {
                            Some(Ok(system)) => rsx! {
                                dl {
                                    for (label, value) in [
                                        ("Server version", system.version.as_str()),
                                        ("Operating system", system.os.as_str()),
                                        ("Architecture", system.architecture.as_str()),
                                    ] {
                                        div { key: "{label}", class: "grid grid-cols-2 gap-3 border-t border-border py-2",
                                            dt { class: "text-sm text-muted-foreground", "{label}" }
                                            dd { class: "wrap-break-word font-mono text-sm font-medium", "{value}" }
                                        }
                                    }
                                }
                            },
                            Some(Err(_)) => rsx! {
                                div { class: "flex flex-wrap items-center gap-3",
                                    p { class: "text-sm text-destructive", role: "alert", "Could not load system details." }
                                    Button { variant: ButtonVariant::Outline,
                                        onclick: move |_| { details.clear(); details.restart(); },
                                        "Reload system details"
                                    }
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
