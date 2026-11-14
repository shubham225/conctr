//! Shared product UI and routing for Conctr launchers.
mod pages;
mod routes;

pub use routes::Route;

use conctr_ui_dioxus::{
    assets::images,
    components::{
        Divider,
        ui::{
            icon::{IconName, UiIcon},
            sidebar::*,
            theme::Theme,
            theme_provider::ThemeProvider,
            theme_toggle::ThemeToggle,
        },
    },
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
                SidebarProvider {
                    Sidebar {
                        collapsible: SidebarCollapsible::Icon,
                        SidebarHeader {
                          class: "h-16 p-2",
                          SidebarBrand { dark: dark() }
                        }

                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Workspace" }
                                SidebarMenu {
                                    NavigationItem { to: Route::Connections {}, label: "Connections", icon: IconName::Connections }
                                    NavigationItem { to: Route::Runs {}, label: "Runs", icon: IconName::Runs }
                                }
                            }
                        }
                        SidebarFooter {
                            SidebarMenu {
                                NavigationItem { to: Route::Settings {}, label: "Settings", icon: IconName::Settings }
                            }
                        }
                        SidebarRail {}
                    }
                    SidebarInset {
                        if let Some(titlebar) = window_titlebar { {titlebar} }
                        header { class: "flex h-16 shrink-0 items-center justify-between gap-4 border-b border-border px-4 md:px-8",
                            div { class: "flex items-center gap-3",
                                SidebarTrigger {}
                                span { class: "text-sm text-muted-foreground", "Personal workspace" }
                            }
                            ThemeToggle { dark: dark(), on_change: move |value| dark.set(value) }
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
fn NavigationItem(to: Route, label: String, icon: IconName) -> Element {
    // Router lives around the complete shell so links and page state share history.
    let current = use_route::<Route>();
    let sidebar = use_sidebar();
    rsx! {
        SidebarMenuItem {
            SidebarMenuButton {
                is_active: current == to,
                tooltip: rsx! { "{label}" },
                r#as: move |attributes: Vec<Attribute>| rsx! {
                    Link { to: to.clone(), onclick: move |_| sidebar.set_open_mobile(false), attributes,
                        UiIcon { name: icon }
                        span { "{label}" }
                    }
                },
            }
        }
    }
}

#[component]
fn SidebarBrand(dark: bool) -> Element {
    let sidebar = use_sidebar();
    let collapsed = (sidebar.state)() == SidebarState::Collapsed && !(sidebar.is_mobile)();
    rsx! {
        if collapsed {
          div{
            img {
                src: if dark { images::ICON_COLOR_DARK_SVG } else { images::ICON_COLOR_SVG },
                alt: "Conctr",
                width: 24,
                height: 24,
                class: "mx-auto",
            }
          }
        } else {
            div { class: "px-2",
                img {
                    src: if dark { images::HORIZONTAL_COLOR_DARK_SVG } else { images::HORIZONTAL_COLOR_SVG },
                    alt: "Conctr",
                    width: 140,
                    height: 32,
                    class: "h-10 w-32",
                }
            }
        }
    }
}
