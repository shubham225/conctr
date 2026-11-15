use dioxus::prelude::*;
/// Kit input group; slots accept icons, text addons or action controls.
#[component]
pub fn InputGroup(
    children: Element,
    #[props(default)] prefix: Option<Element>,
    #[props(default)] suffix: Option<Element>,
) -> Element {
    rsx! {div{class:"cn-composed-input-group flex min-h-10 items-center overflow-hidden rounded-md border border-input bg-card focus-within:ring-2 focus-within:ring-ring",
    if let Some(prefix)=prefix{span{class:"flex self-stretch items-center border-r border-border bg-muted px-3 text-sm text-muted-foreground",{prefix}}}
    {children}
    if let Some(suffix)=suffix{span{class:"flex self-stretch items-center border-l border-border px-3 text-sm text-muted-foreground",{suffix}}}
    }}
}
