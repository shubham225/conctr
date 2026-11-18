use conctr_ui_dioxus::ui::{drawer::*, input::Input};
use dioxus::prelude::*;
#[component]
pub fn DrawerDemo() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open drawer" }
            DrawerContent {
                DrawerHeader {
                    DrawerTitle { "Edit connection" }
                    DrawerDescription { "Update the name shown in your workspace." }
                }
                DrawerBody {
                    conctr_ui_dioxus::ui::label::Label { class: "mb-2 block text-sm font-medium", html_for: "drawer-connection", "Connection name" }
                    Input { id: "drawer-connection", value: "Orders to ERP" }
                }
                DrawerFooter {
                    DrawerClose { "Cancel" }
                    DrawerClose { "Save changes" }
                }
            }
        }
    }
}
#[component]
pub fn SideDrawerDemo() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Edit mapping" }
            DrawerContent { position: DrawerPosition::Right,
                div { class: "cn-drawer-title-row",
                    DrawerHeader {
                        DrawerTitle { "Edit mapping" }
                        DrawerDescription { "Match a source field to its destination." }
                    }
                    DrawerClose { class: "cn-drawer-icon-close",
                        icons::X { class: "size-4" }
                    }
                }
                DrawerBody {
                    div { class: "grid gap-4",
                        label { class: "grid gap-2",
                            "Source field"
                            Input { value: "customer.email" }
                        }
                        label { class: "grid gap-2",
                            "Target field"
                            Input { value: "account.email" }
                        }
                    }
                }
                DrawerFooter {
                    DrawerClose { "Cancel" }
                    DrawerClose { "Save mapping" }
                }
            }
        }
    }
}
