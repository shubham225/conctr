use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use dioxus_primitives::tag_group::{
    self, TagGroupEmptyProps, TagGroupLabelProps, TagGroupMultiProps, TagGroupProps, TagListProps,
};

#[css_module("/src/components/ui/tag_group/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/tag_group/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn TagGroup(props: TagGroupProps<String>) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagGroup {
            class: "cn-ui-tag-group {Styles::cn_ui_tag_group.inner} flex gap-2 flex-col gap-3",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            selectable: props.selectable,
            allow_empty_selection: props.allow_empty_selection,
            escape_clears_selection: props.escape_clears_selection,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagGroupMulti(props: TagGroupMultiProps<String>) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagGroupMulti {
            class: "cn-ui-tag-group {Styles::cn_ui_tag_group.inner} flex gap-2 flex-col gap-3",
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            selectable: props.selectable,
            allow_empty_selection: props.allow_empty_selection,
            escape_clears_selection: props.escape_clears_selection,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagGroupLabel(props: TagGroupLabelProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagGroupLabel {
            class: "cn-ui-tag-group-label {Styles::cn_ui_tag_group_label.inner} text-sm font-medium text-foreground",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagGroupEmpty(props: TagGroupEmptyProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagGroupEmpty { class: "cn-ui-tag-group-empty {Styles::cn_ui_tag_group_empty.inner} text-center text-sm", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn TagList(props: TagListProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagList { class: "cn-ui-tag-list flex flex-wrap gap-1 gap-2", attributes: props.attributes, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    pub value: ReadSignal<String>,
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,
    pub index: ReadSignal<usize>,
    #[props(default)]
    pub id: ReadSignal<Option<String>>,
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagOption::<String> {
            class: "cn-ui-tag {Styles::cn_ui_tag.inner} flex items-center gap-1 max-w-fit gap-2 min-h-(--cn-control-sm)",
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RemoveButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        tag_group::TagRemoveButton { class: "cn-ui-remove-button {Styles::cn_ui_remove_button.inner} flex items-center p-0 cursor-pointer shrink-0 justify-center overflow-visible w-6.5 h-6.5 ml-2.5 ml-1 m-0 min-w-6 min-h-6", attributes,
            {children}
            X { size: "12px" }
        }
    }
}
