use dioxus::prelude::*;

#[css_module("/src/components/ui/form/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/form/style.css",
    manganis::AssetOptions::css_module()
);

/// Native form boundary. Submission policy and validation belong to the caller.
#[component]
pub fn Form(
    onsubmit: EventHandler<FormEvent>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = form)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        form { class: "cn-stack min-w-0 flex flex-col gap-4", onsubmit, ..attributes, {children} }
    }
}
