use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/ui/button/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/button/style.css",
    manganis::AssetOptions::css_module()
);
#[css_module("/src/components/ui/spinner/style.css")]
struct SpinnerStyles;

// Register for every document, including repeated SSR requests.
const SPINNER_STYLES: Asset = asset!(
    "/src/components/ui/spinner/style.css",
    manganis::AssetOptions::css_module()
);

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    /// Conctr danger action (alias for Destructive).
    Danger,
    Outline,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Destructive | ButtonVariant::Danger => "destructive",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "xs",
            ButtonSize::Sm => "sm",
            ButtonSize::Default => "default",
            ButtonSize::Lg => "lg",
            ButtonSize::Icon => "icon",
            ButtonSize::IconXs => "icon-xs",
            ButtonSize::IconSm => "icon-sm",
            ButtonSize::IconLg => "icon-lg",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] loading: bool,
    #[props(default)] disabled: bool,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: "cn-ui-button {Styles::cn_ui_button.inner} cn-btn {Styles::cn_btn.inner} items-center gap-2 inline-flex justify-center whitespace-nowrap",
        r#type: "button",
        disabled: disabled || loading,
        aria_busy: loading,
        "data-style": variant.class(),
        "data-size": size.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        button {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(f) = &onkeydown {
                    f.call(event);
                }
            },
            ..merged,
            if loading {
                document::Stylesheet { href: SPINNER_STYLES }
                span { class: "cn-spinner {SpinnerStyles::cn_spinner.inner} w-5 h-5", aria_hidden: "true" }
            }
            {children}
        }
    }
}
