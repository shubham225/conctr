use crate::components::ui::status::Status;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/alert/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/alert/style.css",
    manganis::AssetOptions::css_module()
);

/// Inline feedback. Semantic status sets the palette and announcement priority.
#[component]
pub fn Alert(
    #[props(default)] status: Option<Status>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-alert {Styles::cn_alert.inner} flex gap-3",
        "data-status": status.map(Status::class),
        role: if status == Some(Status::Danger) { "alert" } else { "status" },
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div { ..merged,{children} }
    }
}
