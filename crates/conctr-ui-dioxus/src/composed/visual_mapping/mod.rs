//! Interactive mapping presentation. Callers adapt payloads into the serializable
//! schema model and own persistence, type validation and transform execution.
mod editor;
mod export;
mod fields;
mod model;
use dioxus::prelude::*;
pub use editor::VisualFieldMapper;
pub use export::mapping_yaml;
pub use model::*;
#[css_module("/src/composed/visual_mapping/style.css")]
struct Styles;
// Register per rendered document; do not rely on the CSS macro's process-wide cache.
const CSS: Asset = asset!(
    "/src/composed/visual_mapping/style.css",
    AssetOptions::css_module()
);

mod node_manager;
mod transform_picker;

mod drag;
