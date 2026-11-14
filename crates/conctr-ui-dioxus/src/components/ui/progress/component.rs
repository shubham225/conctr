use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressProps};

#[css_module("/src/components/ui/progress/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/progress/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        progress::Progress {
            class: "cn-ui-progress {Styles::cn_ui_progress.inner} relative box-border overflow-hidden min-w-50",
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            progress::ProgressIndicator { class: "cn-ui-progress-indicator {Styles::cn_ui_progress_indicator.inner} h-full bg-primary" }
        }
    }
}
