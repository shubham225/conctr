use super::{FieldEdit, SchemaField, Styles};
use crate::ui::context_menu::{ContextMenuItem, ContextMenuLabel};
use dioxus::prelude::*;

/// Native popover dismissal and focus, with the shared menu facade's item styles.
#[component]
pub(super) fn FieldActions(
    field: SchemaField,
    enabled: bool,
    on_select: EventHandler<String>,
    on_edit: EventHandler<SchemaField>,
    on_change: EventHandler<FieldEdit>,
    children: Element,
) -> Element {
    rsx! {
        {children}
        if enabled {
        button {r#type:"button", "data-field-actions":"", class:"flex size-8 shrink-0 items-center justify-center rounded-md hover:bg-accent", "aria-label":format!("Actions for {}",field.label), "aria-haspopup":"menu", title:"Field actions", onclick:{let path=field.path.clone();move |_|on_select.call(path.clone())}, icons::Ellipsis {class:"size-4"}}
        div {class:Styles::field_menu.inner, popover:"auto", role:"menu", "data-field-menu":"", "aria-label":format!("Actions for {}",field.label),
            ContextMenuLabel {"{field.label}"}
            ul {
                ContextMenuItem {button {r#type:"button",role:"menuitem", "data-context-close":"",onclick:{let field=field.clone();move |_|on_edit.call(field.clone())}, icons::Pencil {class:"size-4"} "Edit field"}}
                ContextMenuItem {button {r#type:"button",role:"menuitem", "data-context-close":"",onclick:{let path=field.path.clone();move |_|on_change.call(FieldEdit::ToggleIgnored(path.clone()))},
                    if field.ignored {icons::Eye {class:"size-4"} "Restore field"} else {icons::EyeOff {class:"size-4"} "Ignore field"}
                }}
                ContextMenuItem {button {r#type:"button",role:"menuitem",class:"text-destructive", "data-context-close":"",onclick:{let path=field.path.clone();move |_|on_change.call(FieldEdit::Remove(path.clone()))}, icons::Trash2 {class:"size-4"} "Delete field"}}
            }
        }
        }
    }
}
