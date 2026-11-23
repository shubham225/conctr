use conctr_ui_dioxus::{assets::images, ui::sidebar::*};
use dioxus::prelude::*;
use icons::{CircleHelp, LayoutDashboard, Network, Settings};

#[component]
pub fn SidebarDemo() -> Element {
    rsx! {
        SidebarProvider { class: "flex min-h-[480px] flex-col overflow-hidden rounded-lg border border-border bg-background md:flex-row",
            Sidebar {
                label: "Example workspace",
                border_trigger: true,
                collapsed_header: rsx! { img { src: images::HUB, alt: "Conctr", width: 32, height: 32 } },
                header: rsx! {
                    img {
                        class: "cn-sidebar-logo-light",
                        src: images::HORIZONTAL_COLOR_SVG,
                        alt: "Conctr",
                        width: 140,
                        height: 36,
                    }
                    img {
                        class: "cn-sidebar-logo-dark",
                        src: images::HORIZONTAL_COLOR_DARK_SVG,
                        alt: "Conctr",
                        width: 140,
                        height: 36,
                    }
                },
                footer: rsx! {
                    p { class: "text-xs text-muted-foreground", "Where systems meet." }
                },
                SidebarGroup { title: "Workspace",
                    SidebarItem {
                        label: "Overview",
                        href: "#sidebar-overview",
                        active: true,
                        icon: rsx! {
                            LayoutDashboard {}
                        },
                    }
                    SidebarItem {
                        label: "Connections",
                        href: "#sidebar-connections",
                        icon: rsx! {
                            Network {}
                        },
                    }
                }
                SidebarGroup { title: "Manage",
                    SidebarItem {
                        label: "Settings",
                        href: "#sidebar-settings",
                        icon: rsx! {
                            Settings {}
                        },
                    }
                    SidebarItem {
                        label: "Help",
                        href: "#sidebar-help",
                        icon: rsx! {
                            CircleHelp {}
                        },
                    }
                    SidebarItem {
                        label: "Unavailable item",
                        href: "#",
                        disabled: true,
                    }
                }
            }
            section { class: "min-w-0 flex-1 p-6", id: "sidebar-overview",
                div { class: "flex items-center gap-3",
                    SidebarTrigger {}
                    h2 { "Workspace overview" }
                }
                p { class: "mt-4 text-sm text-muted-foreground",
                    "Grouped navigation with an active item and a disabled item. Below 768 px, open the navigation drawer. Example links are placeholders."
                }
            }
        }
    }
}
