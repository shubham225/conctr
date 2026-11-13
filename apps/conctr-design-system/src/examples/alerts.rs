use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack { for status in [Status::Info, Status::Success, Status::Warning, Status::Danger] {
        StatusAlert { status, div { strong { match status { Status::Info => "Connection is a draft.", Status::Success => "Connection saved.", Status::Warning => "Connection is degraded.", Status::Danger => "Connection test failed." } } br {} "Review the configuration before the next run." } }
    } Alert { "A neutral inline message." } }
        }
}
