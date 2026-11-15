//! Shared product UI and routing for Conctr launchers.
mod pages;
mod routes;

pub use routes::Route;

use conctr_ui_dioxus::{
    assets::images,
    ui::{sidenav::*, theme::*, theme_toggle::BrandThemeToggle},
};
use dioxus::prelude::*;

/// Shared application. Native launchers may supply their own window title bar.
#[component]
pub fn App() -> Element {
    rsx! { AppWithTitlebar {} }
}

/// Shared application with an optional platform-provided title bar.
#[component]
pub fn AppWithTitlebar(#[props(default)] window_titlebar: Option<Element>) -> Element {
    use_context_provider(|| window_titlebar);
    rsx! { Router::<Route> {} }
}

#[component]
fn Shell() -> Element {
    let window_titlebar = use_context::<Option<Element>>();
    let mut dark = use_signal(|| false);
    rsx! {
        document::Title { "Conctr · Where systems meet" }
        document::Link { rel: "icon", href: images::FAVICON }
        ThemeProvider { theme: if dark() { Theme::Dark } else { Theme::Light },
            div { class: "h-svh overflow-hidden bg-background text-foreground",
                // Platform chrome is supplied by the launcher, never by a shared page.
                SidenavWrapper {
                    Sidenav {
                        data_collapsible: SidenavCollapsible::Icon,
                        SidenavHeader {
                          class: "h-16 p-2",
                          SidenavBrand { dark: dark() }
                        }

                        SidenavContent {
                            SidenavGroup {
                                SidenavGroupLabel { "Workspace" }
                                SidenavMenu {
                                    NavigationItem { to: Route::Connections {}, label: "Connections" }
                                    NavigationItem { to: Route::Runs {}, label: "Runs" }
                                }
                            }
                        }
                        SidenavFooter {
                            SidenavMenu {
                                NavigationItem { to: Route::Settings {}, label: "Settings" }
                            }
                        }
                        SidenavResizeHandle {}
                    }
                    SidenavInset {
                        if let Some(titlebar) = window_titlebar { {titlebar} }
                        header { class: "flex h-16 shrink-0 items-center justify-between gap-4 border-b border-border px-4 md:px-8",
                            div { class: "flex items-center gap-3",
                                SidenavTrigger { "Menu" }
                                span { class: "text-sm text-muted-foreground", "Personal workspace" }
                            }
                            BrandThemeToggle { dark: dark(), on_change: move |value| dark.set(value) }
                        }
                        div { class: "min-h-0 flex-1 overflow-y-auto p-4 md:p-8",
                            Outlet::<Route> {}
                        }
                        footer { class: "flex shrink-0 items-center justify-between gap-4 border-t border-border px-4 py-2 text-xs text-muted-foreground md:px-8",
                            span { "Conctr · Where systems meet" }
                            span { "UI preview" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationItem(to: Route, label: String) -> Element {
    let current = use_route::<Route>();
    rsx! {
        SidenavMenuItem {
            Link { to: to.clone(), class: "flex min-h-10 items-center rounded-md px-3 text-sm hover:bg-accent", "aria-current": if current == to { "page" } else { "false" }, "{label}" }
        }
    }
}

#[component]
fn SidenavBrand(dark: bool) -> Element {
    rsx! { img {
        src: if dark { images::HORIZONTAL_COLOR_DARK_SVG } else { images::HORIZONTAL_COLOR_SVG },
        alt: "Conctr", width: 140, height: 32, class: "h-8 w-32",
    } }
}
