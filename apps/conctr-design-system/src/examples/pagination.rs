use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut page = use_signal(|| 1usize);
    rsx! {
    Stack { PaginationRoot { PaginationContent {
        PaginationItem { PaginationPrevious { href: "#", onclick: move |e: MouseEvent| { e.prevent_default(); page.set(page().saturating_sub(1).max(1)); } } }
        for index in 1..=3 { PaginationItem { PaginationLink { href: "#", is_active: index == page(), onclick: move |e: MouseEvent| { e.prevent_default(); page.set(index); }, "{index}" } } }
        PaginationItem { PaginationEllipsis {} }
        PaginationItem { PaginationNext { href: "#", onclick: move |e: MouseEvent| { e.prevent_default(); page.set((page() + 1).min(3)); } } }
    } } p { role: "status", "Page {page} of 3" } }
        }
}
