use dioxus::prelude::*;

#[css_module("/src/components/composed/empty_state/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/composed/empty_state/style.css",
    manganis::AssetOptions::css_module()
);

/// Empty content with an explanation and caller-supplied primary action.
#[component]
pub fn EmptyState(
    title: String,
    description: String,
    icon: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        section { class: "cn-empty-state {Styles::cn_empty_state.inner} grid justify-items-center text-center",
            if let Some(icon) = icon {
                div { class: "cn-empty-state-icon {Styles::cn_empty_state_icon.inner} grid place-items-center",
                    {icon}
                }
            }
            div { class: "cn-empty-state-copy {Styles::cn_empty_state_copy.inner} grid gap-2",
                h3 { "{title}" }
                p { "{description}" }
            }
            div { class: "cn-empty-state-actions {Styles::cn_empty_state_actions.inner} flex flex-wrap justify-center gap-3",
                {children}
            }
        }
    }
}
