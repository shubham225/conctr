use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};

#[css_module("/src/components/ui/label/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/label/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        label::Label {
            class: "cn-ui-label {Styles::cn_ui_label.inner} flex items-center text-foreground",
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
