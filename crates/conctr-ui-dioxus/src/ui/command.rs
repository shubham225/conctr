//! Kit palette framing around the immutable command/filtering primitives.
pub use crate::rust_ui::ui::command::*;
use dioxus::prelude::*;
/// Applies the rounded, elevated Conctr palette surface to inline and dialog commands.
#[allow(non_snake_case)]
pub fn Command(mut props: CommandProps) -> Element {
    props.class = Some(tw_merge::tw_merge!(
        "cn-command",
        props.class.as_deref().unwrap_or("")
    ));
    crate::rust_ui::ui::command::Command(props)
}
