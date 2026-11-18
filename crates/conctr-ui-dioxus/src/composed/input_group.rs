use crate::ui::input_group::{InputGroup as BaseInputGroup, InputGroupAddon, InputGroupAddonAlign};
use dioxus::prelude::*;
/// Kit input group; slots accept icons, text addons or action controls.
#[component]
pub fn InputGroup(
    children: Element,
    #[props(default)] prefix: Option<Element>,
    #[props(default)] suffix: Option<Element>,
) -> Element {
    rsx! {
        BaseInputGroup { class: "cn-composed-input-group h-auto min-h-10 overflow-hidden bg-card shadow-none focus-within:ring-2 focus-within:ring-ring [&>input]:flex-1 [&>input]:min-w-0 [&>input]:border-0 [&>input]:rounded-none [&>input]:shadow-none",
            if let Some(prefix) = prefix {
                InputGroupAddon { class: "self-stretch border-r border-border bg-muted px-3 text-sm text-muted-foreground",
                    {prefix}
                }
            }
            {children}
            if let Some(suffix) = suffix {
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd, class: "self-stretch border-l border-border px-3 text-sm text-muted-foreground",
                    {suffix}
                }
            }
        }
    }
}
