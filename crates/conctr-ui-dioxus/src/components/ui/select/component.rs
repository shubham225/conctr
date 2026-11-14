use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectMultiProps, SelectOptionProps, SelectProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/select/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/select/style.css",
    manganis::AssetOptions::css_module()
);

pub use dioxus_primitives::select::SelectGroup;

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div {
        class: "cn-ui-select {Styles::cn_ui_select.inner} relative"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger { class: "cn-ui-select-trigger {Styles::cn_ui_select_trigger.inner} cn-select flex items-center gap-1 justify-between w-full cn-field-control",
                select::SelectValue {}
                ChevronDown {
                    class: "cn-ui-select-expand-icon text-muted-foreground shrink-0",
                    size: "20px",
                    stroke: "var(--cn-border-strong)",
                }
            }
            select::SelectList { class: "cn-ui-select-list {Styles::cn_ui_select_list.inner} cn-popup {Styles::cn_popup.inner} box-border min-w-full bg-popover text-popover-foreground", {props.children} }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div {
        class: "cn-ui-select {Styles::cn_ui_select.inner} relative"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger { class: "cn-ui-select-trigger {Styles::cn_ui_select_trigger.inner} cn-select flex items-center gap-1 justify-between w-full cn-field-control",
                select::SelectValue {}
                ChevronDown {
                    class: "cn-ui-select-expand-icon text-muted-foreground shrink-0",
                    size: "20px",
                    stroke: "var(--cn-border-strong)",
                }
            }
            select::SelectList { class: "cn-ui-select-list {Styles::cn_ui_select_list.inner} cn-popup {Styles::cn_popup.inner} box-border min-w-full bg-popover text-popover-foreground", {props.children} }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-select-group-label {Styles::cn_ui_select_group_label.inner} text-muted-foreground text-xs"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        select::SelectGroupLabel { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: "cn-ui-select-option {Styles::cn_ui_select_option.inner} flex items-center justify-between rounded-sm select-none"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check { size: "1rem", stroke: "var(--cn-text-muted)" }
            }
        }
    }
}
