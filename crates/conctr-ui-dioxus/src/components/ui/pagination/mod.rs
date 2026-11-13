//! One-based convenience navigation and composable pagination parts.
mod component;
mod parts;
pub use component::*;
pub use parts::{
    Pagination as PaginationRoot, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationLinkKind, PaginationLinkProps, PaginationLinkSize, PaginationNext,
    PaginationPrevious, PaginationProps as PaginationRootProps,
};
