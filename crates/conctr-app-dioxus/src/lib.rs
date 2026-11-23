//! Shared product UI and routing for Conctr launchers.
mod routes;

pub use routes::Route;

use conctr_ui_dioxus::{
    assets::images,
    ui::{
        avatar::*, breadcrumb::*, dropdown_menu::*, sidebar::*, theme::*,
        theme_toggle::BrandThemeToggle,
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
            div { class: "flex h-svh flex-col overflow-hidden bg-background text-foreground",
                if let Some(titlebar) = window_titlebar { {titlebar} }
                SidebarProvider { class: "flex min-h-0 flex-1",
                    Sidebar {
                        border_trigger: true,
                        footer: rsx! { UserControl {} },
                        collapsed_header: rsx! { img { src: images::HUB, alt: "Conctr", width: 32, height: 32 } },
                        header: rsx! { img {
                            src: if dark() { images::HORIZONTAL_COLOR_DARK_SVG } else { images::HORIZONTAL_COLOR_SVG },
                            alt: "Conctr", class: "h-9 object-contain object-left", width: 132, height: 36,
                        } },
                        SidebarGroup { title: "Workspace",
                            NavigationItem { to: Route::Dashboard {}, icon: rsx! { icons::LayoutDashboard {} } }
                        }
                        SidebarGroup { title: "Connectivity",
                            NavigationItem { to: Route::Credentials {}, icon: rsx! { icons::KeyRound {} } }
                            NavigationItem { to: Route::Connections {}, icon: rsx! { icons::Network {} } }
                            NavigationItem { to: Route::Endpoints {}, icon: rsx! { icons::Plug {} } }
                        }
                        SidebarGroup { title: "Message Transformation",
                            NavigationItem { to: Route::FieldMappings {}, icon: rsx! { icons::GitMerge {} } }
                            NavigationItem { to: Route::Transformers {}, icon: rsx! { icons::Braces {} } }
                        }
                        SidebarGroup { title: "Channels & Routing",
                            NavigationItem { to: Route::ChannelsQueues {}, icon: rsx! { icons::Layers {} } }
                            NavigationItem { to: Route::WorkflowPipeline {}, icon: rsx! { icons::Workflow {} } }
                        }
                        SidebarGroup { title: "Operations",
                            NavigationItem { to: Route::History {}, icon: rsx! { icons::History {} } }
                        }
                        SidebarGroup { title: "Administration",
                            NavigationItem { to: Route::Settings {}, icon: rsx! { icons::Settings {} } }
                            NavigationItem { to: Route::Help {}, icon: rsx! { icons::CircleHelp {} } }
                        }
                    }
                    main { class: "flex min-w-0 flex-1 flex-col",
                        header { class: "flex h-16 shrink-0 items-center gap-3 border-b border-border bg-card px-4 md:px-8",
                            SidebarTrigger {}
                            WorkspaceBreadcrumb {}
                            div { class: "ml-auto",
                                BrandThemeToggle { compact: true, dark: dark(), on_change: move |value| dark.set(value) }
                            }
                        }
                        div { class: "min-h-0 flex-1 overflow-y-auto p-4 md:p-5",
                            Outlet::<Route> {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UserControl() -> Element {
    rsx! {
        div { class: "@container relative",
            DropdownMenu {
                DropdownMenuTrigger { class: "h-11 w-full justify-center gap-2 border-0 bg-transparent px-0 @min-[9rem]:justify-start @min-[9rem]:px-2",
                    span { class: "sr-only", "User options for Shubham Shinde" }
                    UserAvatar {}
                    span { class: "hidden min-w-0 flex-1 text-left leading-tight @min-[9rem]:grid",
                        span { class: "truncate text-sm font-medium", "Shubham Shinde" }
                        span { class: "truncate text-xs text-muted-foreground", "Workspace operator" }
                    }
                    icons::ChevronsUpDown { class: "hidden size-4 text-muted-foreground @min-[9rem]:block" }
                }
                DropdownMenuContent { position: DropdownMenuPosition::Top,
                    class: "w-56 max-w-full rounded-lg shadow-lg data-[state=closed]:invisible md:absolute! md:top-auto! md:bottom-0 md:left-full! md:ml-4 md:max-w-none",
                    div { class: "flex items-center gap-2 px-2 py-2",
                        UserAvatar {}
                        div { class: "grid text-sm leading-tight",
                            strong { class: "font-medium", "Shubham Shinde" }
                            span { class: "text-xs text-muted-foreground", "Workspace operator" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        for (route, icon) in [(Route::Profile {}, rsx! { icons::UserRound {} }), (Route::Notifications {}, rsx! { icons::Bell {} })] {
                            DropdownMenuItem { key: "{route}", class: "p-0",
                                Link { to: route.clone(), class: "flex w-full items-center gap-2 px-2 py-2", "data-dropdown-close": "",
                                    {icon}
                                    "{route.label()}"
                                }
                            }
                        }
                    }
                    DropdownMenuSeparator {}
                    button { r#type: "button", disabled: true,
                        class: "flex w-full items-center gap-2 px-2 py-2 text-sm text-muted-foreground opacity-60",
                        title: "No active sign-in session", icons::LogOut { class: "size-4" } "Log out"
                    }
                }
            }
        }
    }
}

#[component]
fn UserAvatar() -> Element {
    rsx! {
        Avatar { class: "rounded-lg",
            AvatarFallback { class: "rounded-lg bg-secondary text-secondary-foreground", "SS" }
            AvatarImage { class: "rounded-lg bg-secondary", src: "https://api.dicebear.com/10.x/notionists/svg?seed=conctr-operator" }
        }
    }
}

#[component]
fn WorkspaceBreadcrumb() -> Element {
    let page = use_route::<Route>().label();
    rsx! {
        Breadcrumb { class: "min-w-0",
            BreadcrumbList { class: "flex-nowrap",
                BreadcrumbItem { class: "hidden sm:inline-flex",
                    Link { to: Route::Dashboard {}, class: "hover:text-foreground", "Workspace" }
                }
                li { class: "hidden sm:block", "aria-hidden": "true", "/" }
                BreadcrumbItem { class: "min-w-0",
                    BreadcrumbPage { class: "truncate font-medium", "{page}" }
                }
            }
        }
    }
}

#[component]
fn NavigationItem(to: Route, icon: Element) -> Element {
    let label = to.label();
    let active = use_route::<Route>() == to;
    rsx! {
        Link { to, class: "cn-sidebar-item", title: label,
            "aria-label": label, "aria-current": if active { Some("page") } else { None },
            {icon}
            span { "{label}" }
        }
    }
}
