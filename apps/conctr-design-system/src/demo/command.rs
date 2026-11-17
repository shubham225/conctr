use conctr_ui_dioxus::ui::{command::*, kbd::Kbd};
use dioxus::prelude::*;
#[component]
pub fn CommandDemo() -> Element {
    rsx! {
        Command { class: "cn-command",
            div { class: "cn-command__input",
                icons::Search { class: "size-5" }
                CommandInput { placeholder: "Search or run a command" }
            }
            CommandList { id: "command_demo", tabindex: "-1",
                CommandGroup {
                    CommandGroupLabel { "Connections" }
                    CommandItemLink { href: "#orders-erp",
                        icons::Plug { class: "size-4" }
                        span { "Orders to ERP" }
                        Kbd { "Enter" }
                    }
                    CommandItemLink { href: "#orders-warehouse",
                        icons::Plug { class: "size-4" }
                        span { "Orders to Warehouse" }
                    }
                }
                CommandGroup {
                    CommandGroupLabel { "Actions" }
                    CommandItemLink { href: "#create-connection",
                        icons::Plus { class: "size-4" }
                        span { "Create connection" }
                        Kbd { "C" }
                    }
                }
            }
            CommandFooter {
                span {
                    Kbd { "↑" }
                    " "
                    Kbd { "↓" }
                    " navigate"
                }
                span {
                    Kbd { "Esc" }
                    " close"
                }
            }
        }
    }
}
