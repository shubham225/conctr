use dioxus::prelude::*;
use dioxus_primitives::scroll_area::{self, ScrollAreaProps};

#[css_module("/src/components/ui/scroll_area/style.css")]
struct Styles;

const STYLES: Asset = asset!(
    "/src/components/ui/scroll_area/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn ScrollArea(mut props: ScrollAreaProps) -> Element {
    let base = dioxus_primitives::dioxus_attributes::attributes!(div {
        class: "cn-ui-scroll-area {Styles::cn_ui_scroll_area.inner}"
    });
    props.attributes =
        dioxus_primitives::merge_attributes(vec![base, std::mem::take(&mut props.attributes)]);
    rsx! {
        document::Stylesheet { href: STYLES }
        scroll_area::ScrollArea { ..props }
    }
}
