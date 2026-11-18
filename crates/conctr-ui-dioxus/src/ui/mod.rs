//! App-facing component families. Conctr owns this facade; upstream stays untouched.

// Families that inherit upstream behavior and shared theme defaults.
pub use crate::rust_ui::ui::{
    accordion, alert_dialog, animate, aspect_ratio, attachment, auto_form, avatar, bottom_nav,
    breadcrumb, bubble, button_action, button_group, callout, card_carousel, carousel, charts,
    collapsible, context_menu, data_table, date_picker_dual_state, date_picker_state, dialog,
    direction_provider, drag_and_drop, dropdown_menu, dropzone, empty, field, footer, form, header,
    hover_card, image, input_group, input_prompt, item, kbd, label, link, marker, marquee, mask,
    message, multi_select, navigation_menu, pagination, popover, pressable, progress, radio_button,
    radio_button_group, scroll_area, select, separator, sheet, sidenav, skeleton, slider, spinner,
    status, stepper, switch, toggle_group, toolbar, tooltip,
};

// Families with Conctr behavior or component-owned styles.
pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chips;
pub mod code_block;
pub mod command;
pub mod data_grid;
pub mod date_picker;
pub mod drawer;
pub mod input;
pub mod input_otp;
pub mod input_phone;
pub mod menubar;
pub mod shimmer;
pub mod sidebar;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod theme;
pub mod theme_toggle;
pub mod toast;
pub mod workflow;

pub mod feedback_message;
