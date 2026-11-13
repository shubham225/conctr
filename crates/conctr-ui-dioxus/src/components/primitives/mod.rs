//! Layout building blocks using the Conctr spacing scale.
//! `Stack` flows vertically; `Inline` wraps horizontally. `Flex` and `Grid`
//! expose layout controls without coupling application/domain crates to Dioxus.
mod component;

pub use component::*;

/// Neutral layout container (named Box in the public design-system API).
pub use component::LayoutBox as Box;

pub mod cluster;
pub use cluster::*;
pub mod container;
pub use container::*;
pub mod divider;
pub use divider::*;
pub mod stack;
pub use stack::*;
