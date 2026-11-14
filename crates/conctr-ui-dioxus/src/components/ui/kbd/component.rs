use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/kbd/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/kbd/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `kbd`; accepts native attributes and arbitrary children.
#[component]
pub fn Kbd(
    #[props(extends = GlobalAttributes)]
    #[props(extends = kbd)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(kbd {
        class: "cn-kbd {Styles::cn_kbd.inner} text-muted-foreground bg-muted inline-flex items-center justify-center whitespace-nowrap"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        kbd { ..merged,{children} }
    }
}

/// A named group of keys forming a keyboard shortcut. This is display, not a binding.
#[component]
pub fn KbdGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span {
        class: "cn-kbd-group {Styles::cn_kbd_group.inner} inline-flex items-center gap-1",
        role: "group",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        span { ..merged,{children} }
    }
}
