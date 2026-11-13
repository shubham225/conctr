//! Reusable presentation building blocks, organized by composition level.
pub mod composed;
pub mod primitives;
pub mod ui;

// Compatibility re-exports keep existing component paths and the prelude working.
pub use composed::*;
pub use primitives::{
    Align, Cluster, Container, Direction, Divider, Flex, Grid, Inline, Justify, LayoutBox, Space,
    Stack, cluster, container, divider, stack,
};
pub use ui::*;

/// Legacy registry namespace. Prefer `components::ui::<family>` for new code.
#[doc(hidden)]
pub mod dioxus {
    pub use super::ui::*;

    // The legacy registry root has a different API from the kit's convenience
    // pagination. Preserve both contracts while sharing the same implementation.
    pub mod pagination {
        pub use super::super::ui::pagination::*;
        pub use super::super::ui::pagination::{
            PaginationRoot as Pagination, PaginationRootProps as PaginationProps,
        };
    }
}
