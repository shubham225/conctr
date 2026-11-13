use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-sidebar-demo",
        SidebarProvider { Sidebar { collapsible: SidebarCollapsible::None,
            SidebarHeader { strong { "Example workspace" } }
            SidebarContent { SidebarGroup { SidebarGroupLabel { "Platform" } SidebarMenu {
                SidebarMenuItem { SidebarMenuButton { is_active: true, "Connections" } }
                SidebarMenuItem { SidebarMenuButton { "Runs" } }
                SidebarMenuItem { SidebarMenuButton { "Settings" } }
            } } }
            SidebarFooter { Hint { "Development environment" } }
        }
        SidebarInset { div { class: "ds-sidebar-content", Heading { level: HeadingLevel::H4, "Connection overview" } p { "A sidebar composition scoped to this preview." } } }
        }
    }
        }
}
