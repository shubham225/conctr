use dioxus::prelude::*;

/// Semantic tag treatments from the Conctr forms kit.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TagVariant {
    Primary,
    #[default]
    Secondary,
    Info,
    Success,
    Warning,
    Danger,
}
impl TagVariant {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}
/// Compact kit tags with optional caller-owned removal and semantic color.
#[component]
pub fn Tags(
    values: Vec<String>,
    #[props(default)] variant: TagVariant,
    #[props(default)] disabled: bool,
    #[props(default)] on_remove: Option<EventHandler<usize>>,
) -> Element {
    rsx! {
        div { class: "cn-tag-group", "data-disabled": disabled.to_string(),
            for (index, value) in values.iter().enumerate() {
                span { class: "cn-tag", "data-variant": variant.name(),
                    "{value}"
                    if let Some(remove) = on_remove {
                        button {
                            r#type: "button",
                            class: "cn-tag__remove",
                            disabled,
                            "aria-label": format!("Remove {value}"),
                            onclick: move |_| remove.call(index),
                            icons::X { class: "size-3" }
                        }
                    }
                }
            }
        }
    }
}
