use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-virtual-frame",
        VirtualList { count: 1000usize, buffer: 6usize, estimate_size: |_| 64u32,
            render_item: move |index: usize| rsx! { div { class: "ds-virtual-row", strong { "Example connection {index + 1}" } span { class: "cn-caption", "Virtualized row {index}" } } },
        }
    }
        }
}
