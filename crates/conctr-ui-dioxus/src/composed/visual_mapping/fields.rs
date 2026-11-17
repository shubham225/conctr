use super::{MappingSchema, Styles};
use dioxus::prelude::*;

/// Selection is separate from the link handle: dragging a card reorders its column.
#[component]
pub(super) fn FieldPanel(
    schema: MappingSchema,
    selected: Option<String>,
    mapped: Vec<String>,
    source: bool,
    read_only: bool,
    on_select: EventHandler<String>,
    on_connect: EventHandler<String>,
) -> Element {
    rsx! {
        div { class:Styles::fields.inner, "data-column":if source { "source" } else { "target" },
            for field in &schema.fields {
                div { key:"{field.path}", class:Styles::field.inner,
                    "data-node-path":field.path.clone(),
                    "data-side":if source { "source" } else { "target" },
                    "data-ignored":field.ignored.to_string(),
                    "data-mapped":mapped.contains(&field.path).to_string(),
                    "data-selected":(selected.as_ref()==Some(&field.path)).to_string(),
                    button { r#type:"button", class:Styles::field_select.inner,
                        "data-field-path":field.path.clone(), "data-side":if source { "source" } else { "target" },
                        "aria-pressed":(selected.as_ref()==Some(&field.path)).to_string(),
                        "aria-label":format!("{} {}",if source {"Source"} else {"Target"}, field.path),
                        onclick:{ let path=field.path.clone(); move |_|on_select.call(path.clone()) },
                        span { "data-drag-handle":"", class:Styles::drag_handle.inner, icons::GripVertical { class:"size-4" } }
                        span { class:Styles::field_text.inner, strong { "{field.label}" } code { "{field.path}" } }
                        span { class:Styles::type_tag.inner, if field.ignored { "Ignored" } else { "{field.data_type}" } }
                    }
                    button { r#type:"button", class:Styles::port.inner,
                        "data-link-port":"", "data-side":if source {"source"} else {"target"},
                        disabled:read_only || field.ignored,
                        "aria-label":format!("Connect {} {}",if source {"source"} else {"target"},field.label),
                        title:if source {"Drag to a target or click to start a link"} else {"Connect selected source"},
                        onclick:{ let path=field.path.clone(); move |_|on_connect.call(path.clone()) },
                        icons::Plus { class:"size-3" }
                    }
                }
            }
            if schema.fields.is_empty() { p { class:Styles::empty.inner, "No nodes. Use New source or New target to begin." } }
        }
    }
}
