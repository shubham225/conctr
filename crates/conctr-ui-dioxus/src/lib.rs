//! Conctr's themed presentation facade over a pinned, unmodified Rust UI snapshot.
pub mod assets;
pub mod composed;
pub mod ui;
#[doc(hidden)]
#[rustfmt::skip]
#[allow(clippy::all, dead_code)]
#[path = "rust-ui/mod.rs"]
mod rust_ui;
// Upstream absolute imports resolve through these integration seams.
#[doc(hidden)]
pub mod hooks;
#[cfg(feature = "showcase")]
pub use rust_ui::{demos, workflows};
/// Common app imports. Component families are available under `ui`.
pub mod prelude {
    pub use crate::assets::styles::ConctrStyles;
    pub use crate::composed::*;
    pub use crate::ui::{button::*, theme::*, theme_toggle::*};
}
