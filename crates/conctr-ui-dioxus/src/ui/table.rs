//! Conctr data-table surfaces over the immutable table primitives.
pub use crate::rust_ui::ui::table::*;
use dioxus::prelude::*;

/// A branded semantic table. Use `TableWrapper` for a scrollable rounded frame.
#[allow(non_snake_case)]
pub fn Table(mut props: TableProps) -> Element {
    props.class = Some(tw_merge::tw_merge!(
        "cn-table",
        props.class.as_deref().unwrap_or("")
    ));
    crate::rust_ui::ui::table::Table(props)
}
