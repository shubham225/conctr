use dioxus::prelude::*;
use dioxus_primitives::avatar::{self, AvatarState};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/ui/avatar/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/avatar/style.css",
    manganis::AssetOptions::css_module()
);

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarImageSize {
    /// The kit default: 40px.
    #[default]
    Default,
    /// Compact avatar: 24px.
    Small,
    Medium,
    Large,
}

impl AvatarImageSize {
    fn to_class(self) -> &'static str {
        match self {
            AvatarImageSize::Default => Styles::cn_ui_avatar_default.inner,
            AvatarImageSize::Small => Styles::cn_ui_avatar_sm.inner,
            AvatarImageSize::Medium => Styles::cn_ui_avatar_md.inner,
            AvatarImageSize::Large => Styles::cn_ui_avatar_lg.inner,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

impl AvatarShape {
    fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => "rounded-full",
            AvatarShape::Rounded => "rounded-md",
        }
    }
}

/// The props for the [`Avatar`] root component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Callback when image loads successfully.
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load.
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes.
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let class = format!(
        "cn-ui-avatar {} items-center relative overflow-hidden inline-flex shrink-0 justify-center rounded-full place-items-center inline-grid flex-none {} {}",
        Styles::cn_ui_avatar.inner,
        props.size.to_class(),
        props.shape.to_class()
    );
    let base = attributes!(span { class });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        avatar::Avatar {
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    pub src: String,

    #[props(default)]
    pub alt: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let base = attributes!(img {
        class: "cn-ui-avatar-image w-full h-full aspect-square",
        draggable: "false",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        avatar::AvatarImage {
            id: props.id,
            src: props.src,
            alt: props.alt,
            attributes: merged,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let base = attributes!(span {
        class: "cn-ui-avatar-fallback {Styles::cn_ui_avatar_fallback.inner} flex items-center w-full justify-center h-full",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        avatar::AvatarFallback { attributes: merged, {props.children} }
    }
}

/// The props for the [`ImageAvatar`] convenience component.
#[derive(Props, Clone, PartialEq)]
pub struct ImageAvatarProps {
    /// The image source URL.
    pub src: String,

    /// The image alt text.
    #[props(default)]
    pub alt: String,

    /// Callback when image loads successfully.
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load.
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes.
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn ImageAvatar(props: ImageAvatarProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        Avatar {
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            size: props.size,
            shape: props.shape,
            attributes: props.attributes,
            AvatarImage { src: props.src, alt: props.alt }
            AvatarFallback { {props.children} }
        }
    }
}
