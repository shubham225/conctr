use super::{FieldEdit, MappingSchema, SchemaField, Styles};
use dioxus::prelude::*;

/// Compact schema table with keyboard selection and separate connection dot ports.
#[component]
pub(super) fn FieldPanel(
    schema: MappingSchema,
    selected: Option<String>,
    mapped: Vec<String>,
    source: bool,
    read_only: bool,
    on_select: EventHandler<String>,
    on_connect: EventHandler<String>,
    #[props(default)] editable: bool,
    on_edit: EventHandler<SchemaField>,
    on_field_change: EventHandler<FieldEdit>,
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
                        super::field_actions::FieldActions {enabled:editable && !read_only, field:field.clone(),on_select,on_edit,on_change:on_field_change,
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

                    }
                        }


                }
            }
            if schema.fields.is_empty() { p { class:Styles::empty.inner, "No visible fields. Adjust the filter or add a field." } }
        }
    }
}
