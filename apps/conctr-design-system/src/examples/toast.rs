use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let toast = use_toast();
    rsx! {
    Cluster {
        Button { onclick: move |_| { toast.success("Connection saved".to_string(), ToastOptions::new().description("Example notification; no external action was performed.")); }, "Show success" }
        Button { variant: ButtonVariant::Secondary, onclick: move |_| { toast.info("Test queued".to_string(), ToastOptions::new()); }, "Show information" }
    }
        }
}
