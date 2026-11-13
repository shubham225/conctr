use dioxus::prelude::*;

/// Controlled page navigation. Page indices are one-based.
#[component]
pub fn Pagination(page: usize, pages: usize, onchange: EventHandler<usize>) -> Element {
    rsx! { nav { class: "cn-pagination", aria_label: "Pagination", for index in 1..=pages { button { r#type: "button", aria_label: "Page {index}", aria_current: if index == page { Some("page") } else { None }, onclick: move |_| onchange.call(index), "{index}" } } } }
}

/// Composable registry pagination parts, alongside the one-based convenience component.
pub use crate::components::dioxus::pagination::{
    Pagination as PaginationRoot, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationLinkKind, PaginationLinkProps, PaginationLinkSize, PaginationNext,
    PaginationPrevious,
};
