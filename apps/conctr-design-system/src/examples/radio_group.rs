use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    fieldset { class: "ds-fieldset", legend { "Environment" }
        RadioGroup { name: "environment", default_value: "development".to_string(),
            RadioItem { index: 0usize, value: "development".to_string(), "Development" }
            RadioItem { index: 1usize, value: "production".to_string(), "Production" }
            RadioItem { index: 2usize, value: "restricted".to_string(), disabled: true, "Restricted" }
        }
        Divider {}
        Radio { label: "Native radio alternative", name: "native-environment", value: "development", checked: true }
    }
        }
}
