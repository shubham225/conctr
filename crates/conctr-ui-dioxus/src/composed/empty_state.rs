use crate::ui::empty::{Empty, EmptyDescription, EmptyHeader, EmptyTitle};
use dioxus::prelude::*;
/// Consistent presentation for an empty or unavailable product surface.
#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] children: Element,
) -> Element {
    rsx! { Empty { class: "rounded-lg border-solid bg-card px-6 py-12",
    img { src: crate::assets::images::HUB, alt: "", width: 48, height: 48 } EmptyHeader { EmptyTitle { "{title}" } EmptyDescription { "{description}" } } {children} } }
}
