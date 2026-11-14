use dioxus::prelude::*;

#[css_module("/src/components/ui/pagination/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/pagination/style.css",
    manganis::AssetOptions::css_module()
);

/// Controlled page navigation. Page indices are one-based.
#[component]
pub fn Pagination(page: usize, pages: usize, onchange: EventHandler<usize>) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        nav { class: "cn-pagination {Styles::cn_pagination.inner} flex gap-1", aria_label: "Pagination",
            for index in 1..=pages {
                button {
                    r#type: "button",
                    aria_label: "Page {index}",
                    aria_current: if index == page { Some("page") } else { None },
                    onclick: move |_| onchange.call(index),
                    "{index}"
                }
            }
        }
    }
}
