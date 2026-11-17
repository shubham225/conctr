use crate::composed::{TagVariant, Tags};
use dioxus::prelude::*;
#[component]
pub fn TagsDemo() -> Element {
    let mut values = use_signal(|| vec!["Production".into(), "REST".into(), "Orders".into()]);
    rsx! {
        div { class: "grid gap-4",
            Tags {
                values: values(),
                variant: TagVariant::Primary,
                on_remove: move |index| {
                    values.write().remove(index);
                },
            }
            Tags {
                values: vec!["Secondary".into()],
                variant: TagVariant::Secondary,
            }
            Tags { values: vec!["Info".into()], variant: TagVariant::Info }
            Tags { values: vec!["Healthy".into()], variant: TagVariant::Success }
            Tags { values: vec!["Degraded".into()], variant: TagVariant::Warning }
            Tags { values: vec!["Failing".into()], variant: TagVariant::Danger }
        }
    }
}
