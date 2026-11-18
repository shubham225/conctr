use super::{FieldEdit, NODE_TYPES, SchemaField, Styles};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

/// A native modal supplies focus trapping, Escape dismissal and a typed node form.
#[component]
pub(super) fn NodeDialog(
    source: bool,
    field: Option<SchemaField>,
    paths: Vec<String>,
    on_save: EventHandler<FieldEdit>,
    on_close: EventHandler<()>,
) -> Element {
    let original = field.clone();
    let mut name = use_signal(|| field.as_ref().map(|f| f.label.clone()).unwrap_or_default());
    let mut path = use_signal(|| field.as_ref().map(|f| f.path.clone()).unwrap_or_default());
    let mut kind = use_signal(|| {
        field
            .as_ref()
            .map(|f| f.data_type.clone())
            .unwrap_or("string".into())
    });
    let mut error = use_signal(String::new);
    rsx! {
        dialog {
            class: Styles::node_dialog.inner,
            "data-node-dialog": "",
            "aria-label": if original.is_some() { "Edit node" } else { "New node" },
            onmounted: move |_| { document::eval("requestAnimationFrame(()=>document.querySelectorAll('[data-node-dialog]').forEach(d=>{if(!d.open)d.showModal()}));"); },
            oncancel: move |_| on_close.call(()),
            form {
                onsubmit: move |event| {
                    event.prevent_default();
                    let next = SchemaField { label: name().trim().into(), path: path().trim().into(), data_type: kind(), depth: original.as_ref().map_or(0, |f|f.depth), ignored: original.as_ref().is_some_and(|f|f.ignored) };
                    if next.label.is_empty() || next.path.is_empty() {
                        error.set("Name and path are required.".into()); return;
                    }
                    if paths.iter().any(|p| p == &next.path && original.as_ref().is_none_or(|f| &f.path != p)) {
                        error.set("A field with this path already exists.".into()); return;
                    }
                    on_save.call(if let Some(ref old) = original { FieldEdit::Update { path:old.path.clone(), field:next } } else { FieldEdit::Add(next) });
                    on_close.call(());
                },
                header {class:Styles::dialog_header.inner,
                    span {class:Styles::dialog_icon.inner, icons::Braces {class:"size-5"}}
                    div {
                        span {class:Styles::dialog_eyebrow.inner, if source {"Source schema"} else {"Target schema"}}
                        h3 { if field.is_some() { "Edit field" } else { "Create field" } }
                        p {"Define how this field appears in your mapping."}
                    }
                    Button {button_type:"button",variant:ButtonVariant::Ghost,size:ButtonSize::IconSm,aria_label:"Close field dialog",onclick:move |_|on_close.call(()),icons::X {class:"size-4"}}
                }
                div {class:Styles::dialog_fields.inner,
                    label { span {"Field name"} input { autofocus:true, required:true, value:name(), oninput:move |e|name.set(e.value()), "aria-label":"Node name", placeholder:"e.g. Customer email" } }
                    label { span {"Field path"} input { class:Styles::path_input.inner, required:true, value:path(), oninput:move |e|path.set(e.value()), placeholder:"e.g. $['customer']['email']", "aria-label":"Node path" }
                        small {"Use the path syntax of your source or target schema."}
                    }
                    label { span {"Data type"} select { value:kind(), onchange:move |e|kind.set(e.value()), "aria-label":"Node datatype",
                        for datatype in NODE_TYPES { option { value:*datatype, selected:*datatype==kind(), "{datatype}" } }
                    } }
                    if !error().is_empty() { div {class:Styles::dialog_error.inner,role:"alert",icons::CircleAlert {class:"size-4"} p {"{error}"}} }
                }
                footer {class:Styles::dialog_footer.inner,
                    span {class:Styles::dialog_note.inner, icons::GitBranch {class:"size-4"} "Mapping draft"}
                    div {class:Styles::actions.inner,
                        Button { button_type:"button", variant:ButtonVariant::Outline, onclick:move |_|on_close.call(()), "Cancel" }
                        Button { button_type:"submit", if field.is_some() { "Save changes" } else { "Create field" } }
                    }
                }
            }
        }
    }
}
