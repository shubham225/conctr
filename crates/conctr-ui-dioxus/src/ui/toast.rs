//! Native Conctr notification adapter; no external toast script is required.
pub use crate::rust_ui::ui::toast::*;
use dioxus::prelude::*;
#[allow(non_snake_case)]
pub fn ToastTrigger(props: ToastTriggerProps) -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
     crate::ui::button::Button { class: props.class.unwrap_or_default(), onclick: move |_| visible.set(true), {props.children} }
     if visible() {
      div { role: "status", class: "fixed bottom-4 right-4 z-[500] flex max-w-[420px] items-center gap-3 rounded-md border border-border bg-popover px-4 py-3 text-sm text-foreground shadow-lg",
       crate::ui::alert::Alert { variant: match props.variant { ToastType::Success=>crate::ui::alert::AlertVariant::Success,ToastType::Error=>crate::ui::alert::AlertVariant::Destructive,ToastType::Warning=>crate::ui::alert::AlertVariant::Warning,_=>crate::ui::alert::AlertVariant::Info },
        crate::ui::alert::AlertTitle { "{props.title}" }
        crate::ui::alert::AlertDescription { "{props.description}" }
       }
       crate::ui::button::Button { variant: crate::ui::button::ButtonVariant::Ghost, aria_label: "Dismiss notification", onclick: move |_| visible.set(false), "Dismiss" }
      }
     }
    }
}
