use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);
    rsx! {
    Cluster { for direction in [SheetSide::Left, SheetSide::Right, SheetSide::Top, SheetSide::Bottom] {
        Button { variant: ButtonVariant::Secondary, onclick: move |_| { side.set(direction); open.set(true); }, "Open {direction.as_str()}" }
    } }
    Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": side().as_str(),
        SheetHeader { SheetTitle { "Connection settings" } SheetDescription { "Configure the example connection." } }
        div { class: "ds-sheet-body", Field { Label { html_for: "sheet-name", "Connection name" } Input { id: "sheet-name", placeholder: "Order sync" } } }
        SheetFooter { Button { onclick: move |_| open.set(false), "Save settings" } SheetClose { "Cancel" } }
        SheetContentClose {}
    }
        }
}
