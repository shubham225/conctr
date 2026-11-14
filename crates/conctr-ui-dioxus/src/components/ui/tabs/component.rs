use dioxus::prelude::*;
use dioxus_primitives::tabs::{self, TabContentProps, TabListProps, TabTriggerProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/tabs/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/tabs/style.css",
    manganis::AssetOptions::css_module()
);

/// The props for the [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The class of the tabs component.
    #[props(default)]
    pub class: String,

    /// The controlled value of the active tab.
    pub value: ReadSignal<Option<String>>,

    /// The default active tab value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the tabs are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the tabs are horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// The variant of the tabs component.
    #[props(default)]
    pub variant: TabsVariant,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// The variant of the tabs component.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    /// The default variant.
    #[default]
    Default,
    /// The ghost variant.
    Ghost,
}

impl TabsVariant {
    /// Convert the variant to a string for use in class names
    fn to_class(self) -> &'static str {
        match self {
            TabsVariant::Default => "default",
            TabsVariant::Ghost => "ghost",
        }
    }
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let base = attributes!(div {
        class: format!("{} cn-ui-tabs {} flex gap-2 flex-col", props.class, Styles::cn_ui_tabs.inner),
        "data-variant": props.variant.to_class(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tabs::Tabs {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-tabs-list {Styles::cn_ui_tabs_list.inner} flex gap-1 box-border flex-row flex-1"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tabs::TabList { attributes: merged, {props.children} }
    }
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    let base = attributes!(button {
        class: format!(
            "cn-ui-tabs-trigger {} {}",
            Styles::cn_ui_tabs_trigger.inner,
            props.class.unwrap_or_default()
        )
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tabs::TabTrigger {
            class: None,
            id: props.id,
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    let base = attributes!(div {
        class: format!(
            "cn-ui-tabs-content {} cn-ui-tabs-content-themed {} w-full box-border p-8 {}",
            Styles::cn_ui_tabs_content.inner,
            Styles::cn_ui_tabs_content_themed.inner,
            props.class.unwrap_or_default()
        )
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tabs::TabContent {
            class: None,
            value: props.value,
            id: props.id,
            index: props.index,
            attributes: merged,
            {props.children}
        }
    }
}
