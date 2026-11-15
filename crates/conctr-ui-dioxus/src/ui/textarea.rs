//! Conctr defaults applied outside the unmodified upstream implementation.
pub use crate::rust_ui::ui::textarea::*;
use dioxus::prelude::*;
/// Render the upstream control with Conctr defaults and caller overrides.
#[allow(non_snake_case)]
pub fn Textarea(mut props: TextareaProps) -> Element {
    props.class = Some(tw_merge::tw_merge!(
        "rounded-md disabled:opacity-45",
        props.class.as_deref().unwrap_or("")
    ));
    crate::rust_ui::ui::textarea::Textarea(props)
}
