use dioxus::prelude::*;

#[css_module("/src/composed/flow_link/style.css")]
struct Styles;
const CSS: Asset = asset!(
    "/src/composed/flow_link/style.css",
    AssetOptions::css_module()
);

/// Decorative animated connector; its orientation follows the responsive layout.
#[component]
pub fn FlowLink() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            "aria-hidden": "true",
            class: "cn-flow-link mx-auto h-8 w-[2px] bg-input sm:h-[2px] sm:w-full {Styles::flow_link.inner}",
            span { class: "cn-flow-packet {Styles::flow_packet.inner}" }
        }
    }
}
