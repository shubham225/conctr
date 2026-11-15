//! Conctr defaults applied outside the unmodified upstream implementation.
pub use crate::rust_ui::ui::card::*;
use dioxus::prelude::*;
/// Render the upstream control with Conctr defaults and caller overrides.
#[allow(non_snake_case)]
pub fn Card(mut props: CardProps) -> Element {
    props.class = Some(tw_merge::tw_merge!(
        "rounded-lg",
        props.class.as_deref().unwrap_or("")
    ));
    crate::rust_ui::ui::card::Card(props)
}
