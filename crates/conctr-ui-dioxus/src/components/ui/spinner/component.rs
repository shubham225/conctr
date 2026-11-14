use dioxus::prelude::*;

#[css_module("/src/components/ui/spinner/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/spinner/style.css",
    manganis::AssetOptions::css_module()
);

/// Loading indicator with an accessible visible label.
#[component]
pub fn Spinner(#[props(default = "Loading".to_string())] label: String) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        div { class: "cn-cluster flex flex-wrap items-center gap-3", role: "status",
            span { class: "cn-spinner {Styles::cn_spinner.inner} w-5 h-5", aria_hidden: "true" }
            "{label}"
        }
    }
}
