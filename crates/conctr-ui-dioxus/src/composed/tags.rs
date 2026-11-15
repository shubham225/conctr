use dioxus::prelude::*;
/// Readable tags with optional application-owned removal.
#[component]
pub fn Tags(
    values: Vec<String>,
    #[props(default)] on_remove: Option<EventHandler<usize>>,
) -> Element {
    rsx! {div{class:"flex flex-wrap gap-2",for (index,value) in values.iter().enumerate(){span{class:"inline-flex items-center gap-2 rounded-sm border border-border bg-muted px-2 py-1 text-sm text-foreground","{value}",if let Some(remove)=on_remove{crate::ui::button::Button{variant:crate::ui::button::ButtonVariant::Ghost,class:"h-6 w-6 p-0",aria_label:format!("Remove {value}"),onclick:move |_|remove.call(index),icons::X{class:"size-3"}}}}}}}
}
