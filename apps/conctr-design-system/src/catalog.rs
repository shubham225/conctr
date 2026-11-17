use dioxus::prelude::*;
pub struct Entry {
    pub slug: &'static str,
    pub source: &'static str,
    pub preview: fn() -> Element,
}
pub static COMPOSED: &[Entry] = &[
    Entry {
        slug: "visual-field-mapping",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/composed/demos/visual_mapping.rs"
        ),
        preview: || rsx! { conctr_ui_dioxus::composed::demos::visual_mapping::VisualMappingDemo {} },
    },
    Entry {
        slug: "field-mapping",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/composed/demos/field_mapping.rs"
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::field_mapping::FieldMappingDemo {}
            }
        },
    },
    Entry {
        slug: "calendar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/calendar.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::calendar::CalendarDemo {}
            }
        },
    },
    Entry {
        slug: "composed-date-picker",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/date_picker.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::date_picker::DatePickerDemo {}
            }
        },
    },
    Entry {
        slug: "date-time-picker",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/composed/demos/date_time_picker.rs"
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::date_time_picker::DateTimePickerDemo {}
            }
        },
    },
    Entry {
        slug: "time-picker",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/time_picker.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::time_picker::TimePickerDemo {}
            }
        },
    },
    Entry {
        slug: "tags",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/tags.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::tags::TagsDemo {}
            }
        },
    },
    Entry {
        slug: "tag-input",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/tag_input.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::tag_input::TagInputDemo {}
            }
        },
    },
    Entry {
        slug: "composed-input-group",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/input_group.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::input_group::InputGroupDemo {}
            }
        },
    },
    Entry {
        slug: "badges-and-shapes",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/composed/demos/badges_and_shapes.rs"
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::badges_and_shapes::BadgesAndShapesDemo {}
            }
        },
    },
    Entry {
        slug: "feedback",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/feedback.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::feedback::FeedbackDemo {}
            }
        },
    },
    Entry {
        slug: "empty-state",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/composed/demos/empty_state.rs"),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::empty_state::EmptyStateDemo {}
            }
        },
    },
    Entry {
        slug: "connection-flow",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/composed/demos/connection_flow.rs"
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::composed::demos::connection_flow::ConnectionFlowDemo {}
            }
        },
    },
];
pub static ENTRIES: &[Entry] = &[
    Entry {
        slug: "sidebar",
        source: include_str!("examples/sidebar.rs"),
        preview: || {
            rsx! {
                crate::examples::sidebar::SidebarDemo {}
            }
        },
    },
    Entry {
        slug: "conctr-workflow",
        source: include_str!("examples/workflow.rs"),
        preview: || {
            rsx! {
                crate::examples::workflow::ConctrWorkflow {}
            }
        },
    },
    Entry {
        slug: "accordion",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_accordion.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_accordion::DemoAccordion {}
            }
        },
    },
    Entry {
        slug: "alert",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_alert.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_alert::DemoAlert {}
            }
        },
    },
    Entry {
        slug: "alert-dialog",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_alert_dialog.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_alert_dialog::DemoAlertDialog {}
            }
        },
    },
    Entry {
        slug: "animate",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_animate.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_animate::DemoAnimate {}
            }
        },
    },
    Entry {
        slug: "aspect-ratio",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_aspect_ratio.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_aspect_ratio::DemoAspectRatio {}
            }
        },
    },
    Entry {
        slug: "attachment",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_attachment.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_attachment::DemoAttachment {}
            }
        },
    },
    Entry {
        slug: "auto-form",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_auto_form.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_auto_form::DemoAutoForm {}
            }
        },
    },
    Entry {
        slug: "avatar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_avatar.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_avatar::DemoAvatar {}
            }
        },
    },
    Entry {
        slug: "badge",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_badge.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_badge::DemoBadge {}
            }
        },
    },
    Entry {
        slug: "bottom-nav",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_bottom_nav.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_bottom_nav::DemoBottomNav {}
            }
        },
    },
    Entry {
        slug: "breadcrumb",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_breadcrumb.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_breadcrumb::DemoBreadcrumb {}
            }
        },
    },
    Entry {
        slug: "bubble",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_bubble.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_bubble::DemoBubble {}
            }
        },
    },
    Entry {
        slug: "button",
        source: include_str!("demo/button.rs"),
        preview: || {
            rsx! {
                crate::demo::button::ButtonDemo {}
            }
        },
    },
    Entry {
        slug: "button-action",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_button_action.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_button_action::DemoButtonAction {}
            }
        },
    },
    Entry {
        slug: "button-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_button_group.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_button_group::DemoButtonGroup {}
            }
        },
    },
    Entry {
        slug: "callout",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_callout.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_callout::DemoCallout {}
            }
        },
    },
    Entry {
        slug: "card",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_card.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_card::DemoCard {}
            }
        },
    },
    Entry {
        slug: "card-carousel",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_card_carousel.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_card_carousel::DemoCardCarousel {}
            }
        },
    },
    Entry {
        slug: "carousel",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_carousel.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_carousel::DemoCarousel {}
            }
        },
    },
    Entry {
        slug: "checkbox",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_checkbox.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_checkbox::DemoCheckbox {}
            }
        },
    },
    Entry {
        slug: "chips",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_chips.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_chips::DemoChips {}
            }
        },
    },
    Entry {
        slug: "collapsible",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_collapsible.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_collapsible::DemoCollapsible {}
            }
        },
    },
    Entry {
        slug: "command",
        source: include_str!("demo/command.rs"),
        preview: || {
            rsx! {
                crate::demo::command::CommandDemo {}
            }
        },
    },
    Entry {
        slug: "context-menu",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_context_menu.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_context_menu::DemoContextMenu {}
            }
        },
    },
    Entry {
        slug: "data-grid",
        source: include_str!("demo/data.rs"),
        preview: || {
            rsx! {
                crate::demo::data::DataGridDemo {}
            }
        },
    },
    Entry {
        slug: "data-table",
        source: include_str!("demo/data.rs"),
        preview: || {
            rsx! {
                crate::demo::data::DataTableDemo {}
            }
        },
    },
    Entry {
        slug: "date-picker",
        source: include_str!("demo/date_picker.rs"),
        preview: || {
            rsx! {
                crate::demo::date_picker::DatePickerDemo {}
            }
        },
    },
    Entry {
        slug: "dialog",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_dialog.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_dialog::DemoDialog {}
            }
        },
    },
    Entry {
        slug: "direction-provider",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_direction_provider.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_direction_provider::DemoDirectionProvider {}
            }
        },
    },
    Entry {
        slug: "drag-and-drop",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_drag_and_drop.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_drag_and_drop::DemoDragAndDrop {}
            }
        },
    },
    Entry {
        slug: "drawer",
        source: include_str!("demo/drawer.rs"),
        preview: || {
            rsx! {
                crate::demo::drawer::DrawerDemo {}
            }
        },
    },
    Entry {
        slug: "dropdown-menu",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_dropdown_menu.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_dropdown_menu::DemoDropdownMenu {}
            }
        },
    },
    Entry {
        slug: "dropzone",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_dropzone.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_dropzone::DemoDropzone {}
            }
        },
    },
    Entry {
        slug: "empty",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_empty.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_empty::DemoEmpty {}
            }
        },
    },
    Entry {
        slug: "field",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_field.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_field::DemoField {}
            }
        },
    },
    Entry {
        slug: "form",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_form.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_form::DemoForm {}
            }
        },
    },
    Entry {
        slug: "hover-card",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_hover_card.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_hover_card::DemoHoverCard {}
            }
        },
    },
    Entry {
        slug: "image",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_image.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_image::DemoImage {}
            }
        },
    },
    Entry {
        slug: "input",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_input::DemoInput {}
            }
        },
    },
    Entry {
        slug: "input-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_group.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_input_group::DemoInputGroup {}
            }
        },
    },
    Entry {
        slug: "input-otp",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_otp.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_input_otp::DemoInputOtp {}
            }
        },
    },
    Entry {
        slug: "input-phone",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_phone.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_input_phone::DemoInputPhone {}
            }
        },
    },
    Entry {
        slug: "input-prompt",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_input_prompt.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_input_prompt::DemoInputPrompt {}
            }
        },
    },
    Entry {
        slug: "item",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_item.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_item::DemoItem {}
            }
        },
    },
    Entry {
        slug: "kbd",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_kbd.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_kbd::DemoKbd {}
            }
        },
    },
    Entry {
        slug: "label",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_label.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_label::DemoLabel {}
            }
        },
    },
    Entry {
        slug: "marker",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_marker.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_marker::DemoMarker {}
            }
        },
    },
    Entry {
        slug: "marquee",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_marquee.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_marquee::DemoMarquee {}
            }
        },
    },
    Entry {
        slug: "mask",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_mask.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_mask::DemoMask {}
            }
        },
    },
    Entry {
        slug: "menubar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_menubar.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_menubar::DemoMenubar {}
            }
        },
    },
    Entry {
        slug: "message",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_message.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_message::DemoMessage {}
            }
        },
    },
    Entry {
        slug: "multi-select",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_multi_select.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_multi_select::DemoMultiSelect {}
            }
        },
    },
    Entry {
        slug: "navigation-menu",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_navigation_menu.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_navigation_menu::DemoNavigationMenu {}
            }
        },
    },
    Entry {
        slug: "pagination",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_pagination.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_pagination::DemoPagination {}
            }
        },
    },
    Entry {
        slug: "popover",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_popover.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_popover::DemoPopover {}
            }
        },
    },
    Entry {
        slug: "pressable",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_pressable.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_pressable::DemoPressable {}
            }
        },
    },
    Entry {
        slug: "progress",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_progress.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_progress::DemoProgress {}
            }
        },
    },
    Entry {
        slug: "radio-button",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_radio_button.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_radio_button::DemoRadioButton {}
            }
        },
    },
    Entry {
        slug: "radio-button-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_radio_button_group.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_radio_button_group::DemoRadioButtonGroup {}
            }
        },
    },
    Entry {
        slug: "scroll-area",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_scroll_area.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_scroll_area::DemoScrollArea {}
            }
        },
    },
    Entry {
        slug: "select",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_select.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_select::DemoSelect {}
            }
        },
    },
    Entry {
        slug: "separator",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_separator.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_separator::DemoSeparator {}
            }
        },
    },
    Entry {
        slug: "sheet",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_sheet.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_sheet::DemoSheet {}
            }
        },
    },
    Entry {
        slug: "shimmer",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_shimmer.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_shimmer::DemoShimmer {}
            }
        },
    },
    Entry {
        slug: "skeleton",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_skeleton.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_skeleton::DemoSkeleton {}
            }
        },
    },
    Entry {
        slug: "slider",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_slider.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_slider::DemoSlider {}
            }
        },
    },
    Entry {
        slug: "spinner",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_spinner.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_spinner::DemoSpinner {}
            }
        },
    },
    Entry {
        slug: "status",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_status.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_status::DemoStatus {}
            }
        },
    },
    Entry {
        slug: "stepper",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_stepper.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_stepper::DemoStepper {}
            }
        },
    },
    Entry {
        slug: "switch",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_switch.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_switch::DemoSwitch {}
            }
        },
    },
    Entry {
        slug: "table",
        source: include_str!("demo/data.rs"),
        preview: || {
            rsx! {
                crate::demo::data::TableDemo {}
            }
        },
    },
    Entry {
        slug: "tabs",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_tabs.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_tabs::DemoTabs {}
            }
        },
    },
    Entry {
        slug: "textarea",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_textarea.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_textarea::DemoTextarea {}
            }
        },
    },
    Entry {
        slug: "theme-toggle",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_theme_toggle.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_theme_toggle::DemoThemeToggle {}
            }
        },
    },
    Entry {
        slug: "toast",
        source: include_str!("demo/toast.rs"),
        preview: || {
            rsx! {
                crate::demo::toast::ToastDemo {}
            }
        },
    },
    Entry {
        slug: "toggle-group",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_toggle_group.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_toggle_group::DemoToggleGroup {}
            }
        },
    },
    Entry {
        slug: "toolbar",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_toolbar.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_toolbar::DemoToolbar {}
            }
        },
    },
    Entry {
        slug: "tooltip",
        source: include_str!("../../../crates/conctr-ui-dioxus/src/rust-ui/demos/demo_tooltip.rs",),
        preview: || {
            rsx! {
                conctr_ui_dioxus::demos::demo_tooltip::DemoTooltip {}
            }
        },
    },
    Entry {
        slug: "workflow-1",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow01.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow01::Workflow01 {}
            }
        },
    },
    Entry {
        slug: "workflow-2",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow02.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow02::Workflow02 {}
            }
        },
    },
    Entry {
        slug: "workflow-3",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow03.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow03::Workflow03 {}
            }
        },
    },
    Entry {
        slug: "workflow-4",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow04.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow04::Workflow04 {}
            }
        },
    },
    Entry {
        slug: "workflow-5",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow05.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow05::Workflow05 {}
            }
        },
    },
    Entry {
        slug: "workflow-6",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow06.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow06::Workflow06 {}
            }
        },
    },
    Entry {
        slug: "workflow-7",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow07.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow07::Workflow07 {}
            }
        },
    },
    Entry {
        slug: "workflow-8",
        source: include_str!(
            "../../../crates/conctr-ui-dioxus/src/rust-ui/workflows/workflow08.rs",
        ),
        preview: || {
            rsx! {
                conctr_ui_dioxus::workflows::workflow08::Workflow08 {}
            }
        },
    },
];
// Keep the preview and displayed source pinned to the same example.
macro_rules! variant {
    ($module:ident, $component:ident, $label:literal) => {
        Entry {
            slug: $label,
            source: include_str!(concat!(
                "../../../crates/conctr-ui-dioxus/src/rust-ui/demos/",
                stringify!($module),
                ".rs"
            )),
            preview: || rsx! { conctr_ui_dioxus::demos::$module::$component {} },
        }
    };
}
/// Up to two additional examples for families with distinct supported variants.
pub fn variants(slug: &str) -> Vec<Entry> {
    match slug {
        "visual-field-mapping" => vec![Entry {
            slug: "read-only-mapping",
            source: include_str!(
                "../../../crates/conctr-ui-dioxus/src/composed/demos/visual_mapping.rs"
            ),
            preview: || rsx! { conctr_ui_dioxus::composed::demos::visual_mapping::ReadOnlyMappingDemo {} },
        }],
        "field-mapping" => vec![Entry {
            slug: "response-mapping",
            source: include_str!(
                "../../../crates/conctr-ui-dioxus/src/composed/demos/field_mapping.rs"
            ),
            preview: || rsx! { conctr_ui_dioxus::composed::demos::field_mapping::ResponseMappingDemo {} },
        }],
        "data-grid" => vec![variant!(demo_data_grid, DemoDataGrid, "advanced-grid")],
        "accordion" => vec![
            variant!(demo_accordion_bordered, DemoAccordionBordered, "bordered"),
            variant!(demo_accordion_card, DemoAccordionCard, "card"),
        ],
        "alert" => vec![
            variant!(demo_alert_colors, DemoAlertColors, "colors"),
            variant!(demo_alert_dialog, DemoAlertDialog, "dialog"),
        ],
        "alert-dialog" => vec![
            variant!(demo_alert_dialog_media, DemoAlertDialogMedia, "media"),
            variant!(demo_alert_dialog_rtl, DemoAlertDialogRtl, "rtl"),
        ],
        "animate" => vec![variant!(demo_animate_group, DemoAnimateGroup, "group")],
        "aspect-ratio" => vec![
            variant!(
                demo_aspect_ratio_portrait,
                DemoAspectRatioPortrait,
                "portrait"
            ),
            variant!(demo_aspect_ratio_square, DemoAspectRatioSquare, "square"),
        ],
        "attachment" => vec![
            variant!(demo_attachment_group, DemoAttachmentGroup, "group"),
            variant!(demo_attachment_image, DemoAttachmentImage, "image"),
        ],
        "avatar" => vec![
            variant!(demo_avatar_badge, DemoAvatarBadge, "badge"),
            variant!(demo_avatar_badge_icon, DemoAvatarBadgeIcon, "badge-icon"),
        ],
        "badge" => vec![
            variant!(demo_badge_colors, DemoBadgeColors, "colors"),
            variant!(demo_badge_custom, DemoBadgeCustom, "custom"),
        ],
        "breadcrumb" => vec![
            variant!(demo_breadcrumb_dropdown, DemoBreadcrumbDropdown, "dropdown"),
            variant!(demo_breadcrumb_rtl, DemoBreadcrumbRtl, "rtl"),
        ],
        "bubble" => vec![
            variant!(demo_bubble_alignment, DemoBubbleAlignment, "alignment"),
            variant!(
                demo_bubble_collapsible,
                DemoBubbleCollapsible,
                "collapsible"
            ),
        ],
        "button" => vec![variant!(
            demo_button_disabled,
            DemoButtonDisabled,
            "disabled"
        )],
        "button-group" => vec![
            variant!(demo_button_group_icon, DemoButtonGroupIcon, "icon"),
            variant!(demo_button_group_input, DemoButtonGroupInput, "input"),
        ],
        "callout" => vec![
            variant!(demo_callout_info, DemoCalloutInfo, "info"),
            variant!(demo_callout_warning, DemoCalloutWarning, "warning"),
        ],
        "card" => vec![
            variant!(demo_card_action, DemoCardAction, "action"),
            variant!(demo_card_carousel, DemoCardCarousel, "carousel"),
        ],
        "carousel" => vec![
            variant!(demo_carousel_api, DemoCarouselApi, "api"),
            variant!(
                demo_carousel_orientation,
                DemoCarouselOrientation,
                "orientation"
            ),
        ],
        "checkbox" => vec![variant!(demo_checkbox_rtl, DemoCheckboxRtl, "rtl")],
        "collapsible" => vec![
            variant!(demo_collapsible_basic, DemoCollapsibleBasic, "basic"),
            variant!(
                demo_collapsible_file_tree,
                DemoCollapsibleFileTree,
                "file-tree"
            ),
        ],
        "command" => vec![variant!(demo_command_dialog, DemoCommandDialog, "dialog")],
        "context-menu" => vec![
            variant!(demo_context_menu_action, DemoContextMenuAction, "action"),
            variant!(demo_context_menu_rtl, DemoContextMenuRtl, "rtl"),
        ],
        "data-table" => vec![variant!(
            demo_data_table_filters,
            DemoDataTableFilters,
            "filters"
        )],
        "date-picker" => vec![
            variant!(demo_date_picker_booked, DemoDatePickerBooked, "booked"),
            variant!(
                demo_date_picker_dropdown,
                DemoDatePickerDropdown,
                "dropdown"
            ),
        ],
        "dialog" => vec![
            variant!(demo_dialog_rtl, DemoDialogRtl, "rtl"),
            variant!(demo_dialog_scrollable, DemoDialogScrollable, "scrollable"),
        ],
        "direction-provider" => vec![
            variant!(
                demo_direction_provider_default,
                DemoDirectionProviderDefault,
                "default"
            ),
            variant!(demo_direction_provider_rtl, DemoDirectionProviderRtl, "rtl"),
        ],
        "drawer" => vec![Entry {
            slug: "side-drawer",
            source: include_str!("demo/drawer.rs"),
            preview: || rsx! { crate::demo::drawer::SideDrawerDemo {} },
        }],
        "dropdown-menu" => vec![
            variant!(
                demo_dropdown_menu_destructive,
                DemoDropdownMenuDestructive,
                "destructive"
            ),
            variant!(demo_dropdown_menu_end, DemoDropdownMenuEnd, "end"),
        ],
        "dropzone" => vec![
            variant!(demo_dropzone_grid, DemoDropzoneGrid, "grid"),
            variant!(demo_dropzone_toggle, DemoDropzoneToggle, "toggle"),
        ],
        "empty" => vec![
            variant!(
                demo_empty_avatar_group,
                DemoEmptyAvatarGroup,
                "avatar-group"
            ),
            variant!(demo_empty_background, DemoEmptyBackground, "background"),
        ],
        "field" => vec![
            variant!(demo_field_checkbox, DemoFieldCheckbox, "checkbox"),
            variant!(demo_field_fieldset, DemoFieldFieldset, "fieldset"),
        ],
        "form" => vec![
            variant!(demo_form_error, DemoFormError, "error"),
            variant!(demo_form_fieldset, DemoFormFieldset, "fieldset"),
        ],
        "hover-card" => vec![variant!(demo_hover_card_rtl, DemoHoverCardRtl, "rtl")],
        "input" => vec![
            variant!(demo_input_copy, DemoInputCopy, "copy"),
            variant!(demo_input_group, DemoInputGroup, "group"),
        ],
        "input-group" => vec![
            variant!(demo_input_group_block, DemoInputGroupBlock, "block"),
            variant!(demo_input_group_custom, DemoInputGroupCustom, "custom"),
        ],
        "input-otp" => vec![
            variant!(demo_input_otp_disabled, DemoInputOtpDisabled, "disabled"),
            variant!(demo_input_otp_rtl, DemoInputOtpRtl, "rtl"),
        ],
        "input-phone" => vec![variant!(
            demo_input_phone_disabled,
            DemoInputPhoneDisabled,
            "disabled"
        )],
        "input-prompt" => vec![variant!(
            demo_input_prompt_with_tools,
            DemoInputPromptWithTools,
            "with-tools"
        )],
        "item" => vec![
            variant!(
                demo_item_dropdown_menu,
                DemoItemDropdownMenu,
                "dropdown-menu"
            ),
            variant!(demo_item_file_upload, DemoItemFileUpload, "file-upload"),
        ],
        "kbd" => vec![
            variant!(demo_kbd_button, DemoKbdButton, "button"),
            variant!(demo_kbd_input_group, DemoKbdInputGroup, "input-group"),
        ],
        "label" => vec![variant!(demo_label_rtl, DemoLabelRtl, "rtl")],
        "marker" => vec![
            variant!(demo_marker_border, DemoMarkerBorder, "border"),
            variant!(demo_marker_icon, DemoMarkerIcon, "icon"),
        ],
        "menubar" => vec![variant!(demo_menubar_rtl, DemoMenubarRtl, "rtl")],
        "message" => vec![
            variant!(demo_message_actions, DemoMessageActions, "actions"),
            variant!(demo_message_attachment, DemoMessageAttachment, "attachment"),
        ],
        "multi-select" => vec![
            variant!(demo_multi_select_align, DemoMultiSelectAlign, "align"),
            variant!(
                demo_multi_select_scrollable,
                DemoMultiSelectScrollable,
                "scrollable"
            ),
        ],
        "navigation-menu" => vec![
            variant!(
                demo_navigation_menu_complex,
                DemoNavigationMenuComplex,
                "complex"
            ),
            variant!(demo_navigation_menu_rtl, DemoNavigationMenuRtl, "rtl"),
        ],
        "pagination" => vec![variant!(demo_pagination_rtl, DemoPaginationRtl, "rtl")],
        "popover" => vec![
            variant!(demo_popover_end, DemoPopoverEnd, "end"),
            variant!(demo_popover_end_outer, DemoPopoverEndOuter, "end-outer"),
        ],
        "progress" => vec![
            variant!(
                demo_progress_controlled,
                DemoProgressControlled,
                "controlled"
            ),
            variant!(demo_progress_label, DemoProgressLabel, "label"),
        ],
        "radio-button" => vec![
            variant!(demo_radio_button_custom, DemoRadioButtonCustom, "custom"),
            variant!(demo_radio_button_group, DemoRadioButtonGroup, "group"),
        ],
        "radio-button-group" => vec![variant!(
            demo_radio_button_group_rtl,
            DemoRadioButtonGroupRtl,
            "rtl"
        )],
        "scroll-area" => vec![
            variant!(
                demo_scroll_area_horizontal,
                DemoScrollAreaHorizontal,
                "horizontal"
            ),
            variant!(demo_scroll_area_rtl, DemoScrollAreaRtl, "rtl"),
        ],
        "select" => vec![
            variant!(demo_select_rtl, DemoSelectRtl, "rtl"),
            variant!(demo_select_scrollable, DemoSelectScrollable, "scrollable"),
        ],
        "separator" => vec![variant!(demo_separator_rtl, DemoSeparatorRtl, "rtl")],
        "sheet" => vec![
            variant!(demo_sheet_directions, DemoSheetDirections, "directions"),
            variant!(
                demo_sheet_experimental,
                DemoSheetExperimental,
                "experimental"
            ),
        ],
        "shimmer" => vec![variant!(demo_shimmer_marker, DemoShimmerMarker, "marker")],
        "skeleton" => vec![
            variant!(demo_skeleton_avatar, DemoSkeletonAvatar, "avatar"),
            variant!(demo_skeleton_form, DemoSkeletonForm, "form"),
        ],
        "slider" => vec![
            variant!(demo_slider_controlled, DemoSliderControlled, "controlled"),
            variant!(demo_slider_flat, DemoSliderFlat, "flat"),
        ],
        "spinner" => vec![
            variant!(demo_spinner_button, DemoSpinnerButton, "button"),
            variant!(demo_spinner_rtl, DemoSpinnerRtl, "rtl"),
        ],
        "status" => vec![variant!(
            demo_status_variants,
            DemoStatusVariants,
            "variants"
        )],
        "stepper" => vec![
            variant!(demo_stepper_controlled, DemoStepperControlled, "controlled"),
            variant!(demo_stepper_vertical, DemoStepperVertical, "vertical"),
        ],
        "switch" => vec![
            variant!(demo_switch_choice_card, DemoSwitchChoiceCard, "choice-card"),
            variant!(demo_switch_rtl, DemoSwitchRtl, "rtl"),
        ],
        "table" => vec![variant!(demo_table_rtl, DemoTableRtl, "rtl")],
        "tabs" => vec![
            variant!(demo_tabs_line, DemoTabsLine, "line"),
            variant!(demo_tabs_rtl, DemoTabsRtl, "rtl"),
        ],
        "textarea" => vec![variant!(demo_textarea_rtl, DemoTextareaRtl, "rtl")],
        "toggle-group" => vec![
            variant!(
                demo_toggle_group_font_weight,
                DemoToggleGroupFontWeight,
                "font-weight"
            ),
            variant!(demo_toggle_group_outline, DemoToggleGroupOutline, "outline"),
        ],
        "tooltip" => vec![variant!(demo_tooltip_rtl, DemoTooltipRtl, "rtl")],
        _ => vec![],
    }
}
