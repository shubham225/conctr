use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut deleted = use_signal(|| false);
    rsx! {
    Button { variant: ButtonVariant::Danger, onclick: move |_| open.set(true), "Delete connection" }
    AlertDialog { open: open(), on_open_change: move |v| open.set(v),
        AlertDialogTitle { "Delete this example connection?" }
        AlertDialogDescription { "This preview only changes local example state." }
        AlertDialogActions { AlertDialogCancel { "Keep connection" } AlertDialogAction { on_click: move |_| deleted.set(true), "Delete connection" } }
    }
    if deleted() { StatusAlert { status: Status::Info, "Example connection deleted locally." } }
        }
}
