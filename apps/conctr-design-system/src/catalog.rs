use dioxus::prelude::*;
pub struct Entry {
    pub slug: &'static str,
    pub source: &'static str,
    pub preview: fn() -> Element,
}
pub static COMPOSED: &[Entry] = &[
    Entry {
        slug: "calendar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/calendar.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::Calendar{}},
    },
    Entry {
        slug: "composed-date-picker",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/date_picker.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::DatePicker{}},
    },
    Entry {
        slug: "tags",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/tags.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::Tags{values:vec!["Production".into(),"REST".into(),"Orders".into()]}},
    },
    Entry {
        slug: "tag-input",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/tag_input.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::TagInput{initial:vec!["Production".into()]}},
    },
    Entry {
        slug: "composed-input-group",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/input_group.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::InputGroup{prefix:rsx!{"https://"},suffix:rsx!{"Endpoint"},conctr_ui_dioxus::ui::input::Input{placeholder:"api.example.test"}}},
    },
    Entry {
        slug: "badges-and-shapes",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/badges_and_shapes.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::BadgesAndShapes{}},
    },
    Entry {
        slug: "feedback",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/feedback.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::Feedback{}},
    },
    Entry {
        slug: "empty-state",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/empty_state.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::EmptyState{title:"No connections yet",description:"Connect two systems to start moving data between them.",conctr_ui_dioxus::ui::button::Button{"Create connection"}}},
    },
    Entry {
        slug: "connection-flow",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/connection_flow.rs"),
        preview: || rsx! {conctr_ui_dioxus::composed::ConnectionFlow{source:"SAP S/4HANA",target:"Salesforce",source_protocol:conctr_ui_dioxus::composed::Protocol::OData,target_protocol:conctr_ui_dioxus::composed::Protocol::Rest}},
    },
];
pub static ENTRIES: &[Entry] = &[
    Entry {
        slug: "conctr-workflow",
        source: include_str!("examples/workflow.rs"),
        preview: || rsx! { crate::examples::workflow::ConctrWorkflow {} },
    },
    Entry {
        slug: "accordion",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_accordion.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_accordion::DemoAccordion {} },
    },
    Entry {
        slug: "alert",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_alert.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_alert::DemoAlert {} },
    },
    Entry {
        slug: "alert-dialog",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_alert_dialog.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_alert_dialog::DemoAlertDialog {} },
    },
    Entry {
        slug: "animate",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_animate.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_animate::DemoAnimate {} },
    },
    Entry {
        slug: "aspect-ratio",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_aspect_ratio.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_aspect_ratio::DemoAspectRatio {} },
    },
    Entry {
        slug: "attachment",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_attachment.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_attachment::DemoAttachment {} },
    },
    Entry {
        slug: "auto-form",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_auto_form.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_auto_form::DemoAutoForm {} },
    },
    Entry {
        slug: "avatar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_avatar.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_avatar::DemoAvatar {} },
    },
    Entry {
        slug: "badge",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_badge.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_badge::DemoBadge {} },
    },
    Entry {
        slug: "bottom-nav",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_bottom_nav.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_bottom_nav::DemoBottomNav {} },
    },
    Entry {
        slug: "breadcrumb",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_breadcrumb.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_breadcrumb::DemoBreadcrumb {} },
    },
    Entry {
        slug: "bubble",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_bubble.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_bubble::DemoBubble {} },
    },
    Entry {
        slug: "button",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_button.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_button::DemoButton {} },
    },
    Entry {
        slug: "button-action",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_button_action.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_button_action::DemoButtonAction {} },
    },
    Entry {
        slug: "button-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_button_group.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_button_group::DemoButtonGroup {} },
    },
    Entry {
        slug: "callout",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_callout.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_callout::DemoCallout {} },
    },
    Entry {
        slug: "card",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_card.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_card::DemoCard {} },
    },
    Entry {
        slug: "card-carousel",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_card_carousel.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_card_carousel::DemoCardCarousel {} },
    },
    Entry {
        slug: "carousel",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_carousel.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_carousel::DemoCarousel {} },
    },
    Entry {
        slug: "checkbox",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_checkbox.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_checkbox::DemoCheckbox {} },
    },
    Entry {
        slug: "chips",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_chips.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_chips::DemoChips {} },
    },
    Entry {
        slug: "collapsible",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_collapsible.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_collapsible::DemoCollapsible {} },
    },
    Entry {
        slug: "command",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_command.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_command::DemoCommand {} },
    },
    Entry {
        slug: "context-menu",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_context_menu.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_context_menu::DemoContextMenu {} },
    },
    Entry {
        slug: "data-grid",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_data_grid.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_data_grid::DemoDataGrid {} },
    },
    Entry {
        slug: "data-table",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_data_table.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_data_table::DemoDataTable {} },
    },
    Entry {
        slug: "date-picker",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_date_picker.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_date_picker::DemoDatePicker {} },
    },
    Entry {
        slug: "dialog",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_dialog.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_dialog::DemoDialog {} },
    },
    Entry {
        slug: "direction-provider",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_direction_provider.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_direction_provider::DemoDirectionProvider {} },
    },
    Entry {
        slug: "drag-and-drop",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_drag_and_drop.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_drag_and_drop::DemoDragAndDrop {} },
    },
    Entry {
        slug: "drawer",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_drawer.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_drawer::DemoDrawer {} },
    },
    Entry {
        slug: "dropdown-menu",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_dropdown_menu.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_dropdown_menu::DemoDropdownMenu {} },
    },
    Entry {
        slug: "dropzone",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_dropzone.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_dropzone::DemoDropzone {} },
    },
    Entry {
        slug: "empty",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_empty.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_empty::DemoEmpty {} },
    },
    Entry {
        slug: "field",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_field.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_field::DemoField {} },
    },
    Entry {
        slug: "form",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_form.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_form::DemoForm {} },
    },
    Entry {
        slug: "hover-card",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_hover_card.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_hover_card::DemoHoverCard {} },
    },
    Entry {
        slug: "image",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_image.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_image::DemoImage {} },
    },
    Entry {
        slug: "input",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_input::DemoInput {} },
    },
    Entry {
        slug: "input-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_group.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_input_group::DemoInputGroup {} },
    },
    Entry {
        slug: "input-otp",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_otp.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_input_otp::DemoInputOtp {} },
    },
    Entry {
        slug: "input-phone",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_phone.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_input_phone::DemoInputPhone {} },
    },
    Entry {
        slug: "input-prompt",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_prompt.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_input_prompt::DemoInputPrompt {} },
    },
    Entry {
        slug: "item",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_item.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_item::DemoItem {} },
    },
    Entry {
        slug: "kbd",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_kbd.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_kbd::DemoKbd {} },
    },
    Entry {
        slug: "label",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_label.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_label::DemoLabel {} },
    },
    Entry {
        slug: "marker",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_marker.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_marker::DemoMarker {} },
    },
    Entry {
        slug: "marquee",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_marquee.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_marquee::DemoMarquee {} },
    },
    Entry {
        slug: "mask",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_mask.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_mask::DemoMask {} },
    },
    Entry {
        slug: "menubar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_menubar.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_menubar::DemoMenubar {} },
    },
    Entry {
        slug: "message",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_message.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_message::DemoMessage {} },
    },
    Entry {
        slug: "multi-select",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_multi_select.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_multi_select::DemoMultiSelect {} },
    },
    Entry {
        slug: "navigation-menu",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_navigation_menu.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_navigation_menu::DemoNavigationMenu {} },
    },
    Entry {
        slug: "pagination",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_pagination.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_pagination::DemoPagination {} },
    },
    Entry {
        slug: "popover",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_popover.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_popover::DemoPopover {} },
    },
    Entry {
        slug: "pressable",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_pressable.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_pressable::DemoPressable {} },
    },
    Entry {
        slug: "progress",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_progress.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_progress::DemoProgress {} },
    },
    Entry {
        slug: "radio-button",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_radio_button.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_radio_button::DemoRadioButton {} },
    },
    Entry {
        slug: "radio-button-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_radio_button_group.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_radio_button_group::DemoRadioButtonGroup {} },
    },
    Entry {
        slug: "scroll-area",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_scroll_area.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_scroll_area::DemoScrollArea {} },
    },
    Entry {
        slug: "select",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_select.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_select::DemoSelect {} },
    },
    Entry {
        slug: "separator",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_separator.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_separator::DemoSeparator {} },
    },
    Entry {
        slug: "sheet",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_sheet.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_sheet::DemoSheet {} },
    },
    Entry {
        slug: "shimmer",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_shimmer.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_shimmer::DemoShimmer {} },
    },
    Entry {
        slug: "skeleton",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_skeleton.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_skeleton::DemoSkeleton {} },
    },
    Entry {
        slug: "slider",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_slider.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_slider::DemoSlider {} },
    },
    Entry {
        slug: "spinner",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_spinner.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_spinner::DemoSpinner {} },
    },
    Entry {
        slug: "status",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_status.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_status::DemoStatus {} },
    },
    Entry {
        slug: "stepper",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_stepper.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_stepper::DemoStepper {} },
    },
    Entry {
        slug: "switch",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_switch.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_switch::DemoSwitch {} },
    },
    Entry {
        slug: "table",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_table.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_table::DemoTable {} },
    },
    Entry {
        slug: "tabs",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_tabs.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_tabs::DemoTabs {} },
    },
    Entry {
        slug: "textarea",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_textarea.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_textarea::DemoTextarea {} },
    },
    Entry {
        slug: "theme-toggle",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_theme_toggle.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_theme_toggle::DemoThemeToggle {} },
    },
    Entry {
        slug: "toast",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_toast.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_toast::DemoToast {} },
    },
    Entry {
        slug: "toggle-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_toggle_group.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_toggle_group::DemoToggleGroup {} },
    },
    Entry {
        slug: "toolbar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_toolbar.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_toolbar::DemoToolbar {} },
    },
    Entry {
        slug: "tooltip",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_tooltip.rs"),
        preview: || rsx! { conctr_ui_dioxus::demos::demo_tooltip::DemoTooltip {} },
    },
    Entry {
        slug: "workflow-1",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow01.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow01::Workflow01 {} },
    },
    Entry {
        slug: "workflow-2",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow02.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow02::Workflow02 {} },
    },
    Entry {
        slug: "workflow-3",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow03.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow03::Workflow03 {} },
    },
    Entry {
        slug: "workflow-4",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow04.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow04::Workflow04 {} },
    },
    Entry {
        slug: "workflow-5",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow05.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow05::Workflow05 {} },
    },
    Entry {
        slug: "workflow-6",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow06.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow06::Workflow06 {} },
    },
    Entry {
        slug: "workflow-7",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow07.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow07::Workflow07 {} },
    },
    Entry {
        slug: "workflow-8",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow08.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::workflows::workflow08::Workflow08 {} },
    },
];
