use dioxus::prelude::*;
#[component]
pub fn FlowLink() -> Element {
    rsx! { div { "aria-hidden":"true",class:"cn-flow-link mx-auto h-8 w-[2px] bg-input sm:h-[2px] sm:w-full", span { class:"cn-flow-packet" } } }
}
