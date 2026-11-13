use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Tabs { value: None, default_value: "overview".to_string(), horizontal: true,
        TabList {
            TabTrigger { id: "overview-tab", aria_controls: "overview-panel", value: "overview".to_string(), index: 0usize, "Overview" }
            TabTrigger { id: "runs-tab", aria_controls: "runs-panel", value: "runs".to_string(), index: 1usize, "Runs" }
            TabTrigger { value: "disabled".to_string(), index: 2usize, disabled: true, "Disabled" }
        }
        TabContent { id: Some("overview-panel".to_string()), aria_labelledby: "overview-tab", value: "overview".to_string(), index: 0usize, p { "Connection overview. Use arrow keys to switch tabs." } }
        TabContent { id: Some("runs-panel".to_string()), aria_labelledby: "runs-tab", value: "runs".to_string(), index: 1usize, p { "Example run history is empty." } }
    }
        }
}
