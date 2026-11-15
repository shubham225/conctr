use dioxus::prelude::*;
/// Kit-sized selectable chips with native keyboard support.
#[component]
pub fn ChipsContainer(children: Element) -> Element {
    rsx! {div{class:"flex flex-wrap gap-2",{children}}}
}
#[component]
pub fn ChipItem(label: &'static str) -> Element {
    let mut selected = use_signal(|| false);
    rsx! {crate::ui::button::Button{variant:crate::ui::button::ButtonVariant::Outline,class:if selected(){"h-8 rounded-full border-primary bg-accent px-3 text-primary"}else{"h-8 rounded-full px-3 text-muted-foreground"},aria_label:format!("{label}: {}",if selected(){"selected"}else{"not selected"}),onclick:move |_|selected.set(!selected()),"{label}"}}
}
