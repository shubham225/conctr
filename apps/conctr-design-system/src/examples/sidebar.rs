use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let selected = use_signal(|| "Connections".to_string());
    rsx! {
        div { class: "ds-sidebar-demo",
            SidebarProvider {
                Sidebar { collapsible: SidebarCollapsible::Icon,
                    SidebarHeader {
                        SidebarMenu { SidebarMenuItem { SidebarMenuButton { tooltip: rsx! { "Example workspace" },
                            UiIcon { name: IconName::Folder } span { "Example workspace" }
                        } } }
                    }
                    SidebarSeparator {}
                    SidebarContent {
                        SidebarGroup { SidebarGroupLabel { "Platform" }
                            SidebarMenu {
                                NavEntry { name: "Connections", icon: IconName::Connections, selected }
                                NavEntry { name: "Runs", icon: IconName::Runs, selected }
                                NavEntry { name: "Settings", icon: IconName::Settings, selected }
                            }
                        }
                        SidebarGroup { SidebarGroupLabel { "Workspace" }
                            SidebarMenu { NavEntry { name: "Credentials", icon: IconName::User, selected } }
                        }
                    }
                    SidebarSeparator {}
                    SidebarFooter {
                        SidebarMenu { SidebarMenuItem { SidebarMenuButton { tooltip: rsx! { "Development environment" },
                            UiIcon { name: IconName::User } span { "Development" }
                        } } }
                    }
                    SidebarRail {}
                }
                SidebarInset {
                    div { class: "ds-sidebar-bar", SidebarTrigger {} span { class: "cn-small", "{selected}" } }
                    div { class: "ds-sidebar-content",
                        Heading { level: HeadingLevel::H4, "{selected}" }
                        p { class: "cn-small", "Use the toggle to collapse to icons. On smaller screens, it opens a navigation drawer." }
                        Badge { variant: BadgeVariant::Info, "Example workspace" }
                    }
                }
            }
        }
    }
}

#[component]
fn NavEntry(name: String, icon: IconName, mut selected: Signal<String>) -> Element {
    let active = selected() == name;
    let label = name.clone();
    rsx! {
        SidebarMenuItem {
            SidebarMenuButton { is_active: active, tooltip: rsx! { "{name}" },
                as: move |attributes: Vec<Attribute>| {
                    let label = label.clone();
                    rsx! { button { r#type: "button", onclick: move |_| selected.set(label.clone()), ..attributes,
                        UiIcon { name: icon } span { "{name}" }
                    } }
                },
            }
        }
    }
}
