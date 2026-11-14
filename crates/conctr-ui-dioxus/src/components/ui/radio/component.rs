use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/radio/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/radio/style.css",
    manganis::AssetOptions::css_module()
);

/// Labelled native radio control with browser keyboard behavior.
#[component]
pub fn Radio(
    label: String,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let base = attributes!(input {
        class: "cn-radio {Styles::cn_radio.inner} flex-none m-0 w-(--choice-size) h-(--choice-size) appearance-none",
        r#type: "radio"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        label { class: "cn-option items-center gap-2 text-sm text-foreground cursor-pointer inline-flex",
            input {
                onchange: move |e| {
                    if let Some(f) = onchange {
                        f.call(e);
                    }
                },
                ..merged,
            }
            "{label}"
        }
    }
}
