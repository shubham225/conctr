//! Conctr defaults applied outside the unmodified upstream implementation.
pub use crate::rust_ui::ui::button::*;
use dioxus::prelude::*;
/// Render the upstream control with Conctr defaults and caller overrides.
#[allow(non_snake_case)]
pub fn Button(mut props: ButtonProps) -> Element {
    let dimensions = match props.size {
        ButtonSize::Default => "h-10 px-4",
        ButtonSize::Sm => "h-8 px-3",
        ButtonSize::Lg => "h-12 px-6",
        ButtonSize::Icon => "size-10",
        ButtonSize::IconSm => "size-8",
        ButtonSize::IconXs => "size-6",
    };
    let variant = match props.variant {
        ButtonVariant::Destructive => {
            "text-destructive-foreground dark:bg-destructive dark:focus-visible:ring-ring"
        }
        _ => "",
    };
    props.class = Some(tw_merge::tw_merge!(
        "rounded-md disabled:opacity-45 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
        dimensions,
        variant,
        props.class.as_deref().unwrap_or("")
    ));
    crate::rust_ui::ui::button::Button(props)
}
