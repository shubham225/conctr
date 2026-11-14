//! Catalog entries are the single source for navigation, examples and source snippets.
use dioxus::prelude::*;
pub struct Entry {
    pub slug: &'static str,
    pub name: &'static str,
    pub group: &'static str,
    pub description: &'static str,
    pub module: &'static str,
    pub source: &'static str,
    pub preview: fn() -> Element,
}
pub const GROUPS: &[&str] = &[
    "Primitives",
    "Core · Actions",
    "Core · Forms",
    "Core · Layout",
    "Core · Navigation",
    "Core · Data display",
    "Core · Feedback",
    "Core · Overlays",
    "Composed",
];
pub static ENTRIES: &[Entry] = &[
    Entry {
        slug: "theme-toggle",
        name: "Theme toggle",
        group: "Core · Actions",
        description: "Both themes expand outward from the button. Includes animated icons, keyboard support and reduced-motion handling.",
        module: "ui::theme_toggle",
        source: include_str!("examples/theme_toggle.rs"),
        preview: || rsx! { crate::examples::theme_toggle::Demo {} },
    },
    Entry {
        slug: "file-uploader",
        name: "File uploader",
        group: "Core · Forms",
        description: "Drag-and-drop file selection with type and size validation, keyboard browsing and removable files.",
        module: "ui::file_uploader",
        source: include_str!("examples/file_uploader.rs"),
        preview: || rsx! { crate::examples::file_uploader::Demo {} },
    },
    Entry {
        slug: "resizable",
        name: "Resizable",
        group: "Core · Layout",
        description: "Nested panel layouts with pointer capture, keyboard resizing and minimum sizes.",
        module: "ui::resizable",
        source: include_str!("examples/resizable.rs"),
        preview: || rsx! { crate::examples::resizable::Demo {} },
    },
    Entry {
        slug: "kbd",
        name: "Kbd",
        group: "Core · Data display",
        description: "Keyboard keys and grouped shortcut hints using the Conctr type and border scales.",
        module: "ui::kbd",
        source: include_str!("examples/kbd.rs"),
        preview: || rsx! { crate::examples::kbd::Demo {} },
    },
    Entry {
        slug: "command",
        name: "Command",
        group: "Core · Overlays",
        description: "Searchable grouped commands with keyboard navigation, empty results and a dialog palette.",
        module: "ui::command",
        source: include_str!("examples/command.rs"),
        preview: || rsx! { crate::examples::command::Demo {} },
    },
    Entry {
        slug: "box",
        name: "Box",
        group: "Primitives",
        description: "A neutral container with token-based padding.",
        module: "primitives",
        source: include_str!("examples/box.rs"),
        preview: || rsx! { crate::examples::r#box::Demo {} },
    },
    Entry {
        slug: "flex",
        name: "Flex",
        group: "Primitives",
        description: "Rows and columns with alignment, distribution and optional wrapping.",
        module: "primitives",
        source: include_str!("examples/flex.rs"),
        preview: || rsx! { crate::examples::flex::Demo {} },
    },
    Entry {
        slug: "grid",
        name: "Grid",
        group: "Primitives",
        description: "Responsive columns with token-based gaps; adapts to the available container width.",
        module: "primitives",
        source: include_str!("examples/grid.rs"),
        preview: || rsx! { crate::examples::grid::Demo {} },
    },
    Entry {
        slug: "stack",
        name: "Stack",
        group: "Primitives",
        description: "Vertical layout with token-based gaps and alignment.",
        module: "primitives",
        source: include_str!("examples/stack.rs"),
        preview: || rsx! { crate::examples::stack::Demo {} },
    },
    Entry {
        slug: "inline",
        name: "Inline",
        group: "Primitives",
        description: "Wrapping rows for controls, badges and other related content.",
        module: "primitives",
        source: include_str!("examples/inline.rs"),
        preview: || rsx! { crate::examples::inline::Demo {} },
    },
    Entry {
        slug: "button",
        name: "Button",
        group: "Core · Actions",
        description: "Actions with primary, secondary, ghost and danger variants, sizes and loading states.",
        module: "ui::button",
        source: include_str!("examples/button.rs"),
        preview: || rsx! { crate::examples::button::Demo {} },
    },
    Entry {
        slug: "toggle",
        name: "Toggle",
        group: "Core · Actions",
        description: "A persistent pressed state using the primary-soft selected surface.",
        module: "ui::toggle",
        source: include_str!("examples/toggle.rs"),
        preview: || rsx! { crate::examples::toggle::Demo {} },
    },
    Entry {
        slug: "toggle-group",
        name: "Toggle group",
        group: "Core · Actions",
        description: "Related toggle controls with keyboard navigation and multiple selection.",
        module: "ui::toggle_group",
        source: include_str!("examples/toggle_group.rs"),
        preview: || rsx! { crate::examples::toggle_group::Demo {} },
    },
    Entry {
        slug: "toolbar",
        name: "Toolbar",
        group: "Core · Actions",
        description: "Toolbar composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::toolbar",
        source: include_str!("examples/toolbar.rs"),
        preview: || rsx! { crate::examples::toolbar::Demo {} },
    },
    Entry {
        slug: "input",
        name: "Input",
        group: "Core · Forms",
        description: "Text entry with a visible label, hint, validation error and disabled state.",
        module: "ui::input",
        source: include_str!("examples/input.rs"),
        preview: || rsx! { crate::examples::input::Demo {} },
    },
    Entry {
        slug: "textarea",
        name: "Textarea",
        group: "Core · Forms",
        description: "Textarea composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::textarea",
        source: include_str!("examples/textarea.rs"),
        preview: || rsx! { crate::examples::textarea::Demo {} },
    },
    Entry {
        slug: "label",
        name: "Label",
        group: "Core · Forms",
        description: "Label composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::label",
        source: include_str!("examples/label.rs"),
        preview: || rsx! { crate::examples::label::Demo {} },
    },
    Entry {
        slug: "form",
        name: "Form",
        group: "Core · Forms",
        description: "Form composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::form",
        source: include_str!("examples/form.rs"),
        preview: || rsx! { crate::examples::form::Demo {} },
    },
    Entry {
        slug: "checkbox",
        name: "Checkbox",
        group: "Core · Forms",
        description: "Checkbox composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::checkbox",
        source: include_str!("examples/checkbox.rs"),
        preview: || rsx! { crate::examples::checkbox::Demo {} },
    },
    Entry {
        slug: "switch",
        name: "Switch",
        group: "Core · Forms",
        description: "Immediate on/off settings with a 40 × 24 track and a centered 18px thumb.",
        module: "ui::switch",
        source: include_str!("examples/switch.rs"),
        preview: || rsx! { crate::examples::switch::Demo {} },
    },
    Entry {
        slug: "radio-group",
        name: "Radio group",
        group: "Core · Forms",
        description: "Radio group composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::radio_group",
        source: include_str!("examples/radio_group.rs"),
        preview: || rsx! { crate::examples::radio_group::Demo {} },
    },
    Entry {
        slug: "select",
        name: "Select",
        group: "Core · Forms",
        description: "Select composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::select",
        source: include_str!("examples/select.rs"),
        preview: || rsx! { crate::examples::select::Demo {} },
    },
    Entry {
        slug: "combobox",
        name: "Combobox",
        group: "Core · Forms",
        description: "Combobox composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::combobox",
        source: include_str!("examples/combobox.rs"),
        preview: || rsx! { crate::examples::combobox::Demo {} },
    },
    Entry {
        slug: "slider",
        name: "Slider",
        group: "Core · Forms",
        description: "Slider composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::slider",
        source: include_str!("examples/slider.rs"),
        preview: || rsx! { crate::examples::slider::Demo {} },
    },
    Entry {
        slug: "calendar",
        name: "Calendar",
        group: "Core · Forms",
        description: "Calendar composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::calendar",
        source: include_str!("examples/calendar.rs"),
        preview: || rsx! { crate::examples::calendar::Demo {} },
    },
    Entry {
        slug: "date-picker",
        name: "Date picker",
        group: "Core · Forms",
        description: "Date picker composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::date_picker",
        source: include_str!("examples/date_picker.rs"),
        preview: || rsx! { crate::examples::date_picker::Demo {} },
    },
    Entry {
        slug: "color-picker",
        name: "Color picker",
        group: "Core · Forms",
        description: "Color picker composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::color_picker",
        source: include_str!("examples/color_picker.rs"),
        preview: || rsx! { crate::examples::color_picker::Demo {} },
    },
    Entry {
        slug: "accordion",
        name: "Accordion",
        group: "Core · Layout",
        description: "Accordion composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::accordion",
        source: include_str!("examples/accordion.rs"),
        preview: || rsx! { crate::examples::accordion::Demo {} },
    },
    Entry {
        slug: "collapsible",
        name: "Collapsible",
        group: "Core · Layout",
        description: "Collapsible composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::collapsible",
        source: include_str!("examples/collapsible.rs"),
        preview: || rsx! { crate::examples::collapsible::Demo {} },
    },
    Entry {
        slug: "card",
        name: "Card",
        group: "Core · Layout",
        description: "Grouped content with the kit’s surface, 14px radius and optional elevation.",
        module: "ui::card",
        source: include_str!("examples/card.rs"),
        preview: || rsx! { crate::examples::card::Demo {} },
    },
    Entry {
        slug: "aspect-ratio",
        name: "Aspect ratio",
        group: "Core · Layout",
        description: "Aspect ratio composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::aspect_ratio",
        source: include_str!("examples/aspect_ratio.rs"),
        preview: || rsx! { crate::examples::aspect_ratio::Demo {} },
    },
    Entry {
        slug: "scroll-area",
        name: "Scroll area",
        group: "Core · Layout",
        description: "Scroll area composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::scroll_area",
        source: include_str!("examples/scroll_area.rs"),
        preview: || rsx! { crate::examples::scroll_area::Demo {} },
    },
    Entry {
        slug: "separator",
        name: "Separator",
        group: "Core · Layout",
        description: "Separator composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::separator",
        source: include_str!("examples/separator.rs"),
        preview: || rsx! { crate::examples::separator::Demo {} },
    },
    Entry {
        slug: "layout",
        name: "Layout composition",
        group: "Primitives",
        description: "Layout composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "primitives",
        source: include_str!("examples/layout.rs"),
        preview: || rsx! { crate::examples::layout::Demo {} },
    },
    Entry {
        slug: "tabs",
        name: "Tabs",
        group: "Core · Navigation",
        description: "Related views with keyboard navigation and an indigo underline.",
        module: "ui::tabs",
        source: include_str!("examples/tabs.rs"),
        preview: || rsx! { crate::examples::tabs::Demo {} },
    },
    Entry {
        slug: "breadcrumb",
        name: "Breadcrumb",
        group: "Core · Navigation",
        description: "Breadcrumb composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::breadcrumb",
        source: include_str!("examples/breadcrumb.rs"),
        preview: || rsx! { crate::examples::breadcrumb::Demo {} },
    },
    Entry {
        slug: "navigation",
        name: "Navigation",
        group: "Core · Navigation",
        description: "Navigation composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::navigation",
        source: include_str!("examples/navigation.rs"),
        preview: || rsx! { crate::examples::navigation::Demo {} },
    },
    Entry {
        slug: "navbar",
        name: "Navbar",
        group: "Core · Navigation",
        description: "Navbar composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::navbar",
        source: include_str!("examples/navbar.rs"),
        preview: || rsx! { crate::examples::navbar::Demo {} },
    },
    Entry {
        slug: "sidebar",
        name: "Sidebar",
        group: "Core · Navigation",
        description: "Sidebar composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::sidebar",
        source: include_str!("examples/sidebar.rs"),
        preview: || rsx! { crate::examples::sidebar::Demo {} },
    },
    Entry {
        slug: "pagination",
        name: "Pagination",
        group: "Core · Navigation",
        description: "Pagination composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::pagination",
        source: include_str!("examples/pagination.rs"),
        preview: || rsx! { crate::examples::pagination::Demo {} },
    },
    Entry {
        slug: "menubar",
        name: "Menubar",
        group: "Core · Navigation",
        description: "Menubar composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::menubar",
        source: include_str!("examples/menubar.rs"),
        preview: || rsx! { crate::examples::menubar::Demo {} },
    },
    Entry {
        slug: "badge",
        name: "Badge",
        group: "Core · Data display",
        description: "Protocol shapes, fixed brand colors and readable status labels.",
        module: "ui::badge",
        source: include_str!("examples/badge.rs"),
        preview: || rsx! { crate::examples::badge::Demo {} },
    },
    Entry {
        slug: "avatar",
        name: "Avatar",
        group: "Core · Data display",
        description: "Avatar composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::avatar",
        source: include_str!("examples/avatar.rs"),
        preview: || rsx! { crate::examples::avatar::Demo {} },
    },
    Entry {
        slug: "table",
        name: "Table",
        group: "Core · Data display",
        description: "Semantic tabular data with protocol and status badges.",
        module: "ui::table",
        source: include_str!("examples/table.rs"),
        preview: || rsx! { crate::examples::table::Demo {} },
    },
    Entry {
        slug: "stat",
        name: "Stat",
        group: "Composed",
        description: "Stat composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "composed::stat",
        source: include_str!("examples/stat.rs"),
        preview: || rsx! { crate::examples::stat::Demo {} },
    },
    Entry {
        slug: "code-block",
        name: "Code block",
        group: "Core · Data display",
        description: "Code block composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::code_block",
        source: include_str!("examples/code_block.rs"),
        preview: || rsx! { crate::examples::code_block::Demo {} },
    },
    Entry {
        slug: "item",
        name: "Item",
        group: "Core · Data display",
        description: "Item composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::item",
        source: include_str!("examples/item.rs"),
        preview: || rsx! { crate::examples::item::Demo {} },
    },
    Entry {
        slug: "tag-group",
        name: "Tag group",
        group: "Core · Data display",
        description: "Tag group composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::tag_group",
        source: include_str!("examples/tag_group.rs"),
        preview: || rsx! { crate::examples::tag_group::Demo {} },
    },
    Entry {
        slug: "virtual-list",
        name: "Virtual list",
        group: "Core · Data display",
        description: "Virtual list composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::virtual_list",
        source: include_str!("examples/virtual_list.rs"),
        preview: || rsx! { crate::examples::virtual_list::Demo {} },
    },
    Entry {
        slug: "drag-and-drop-list",
        name: "Drag and drop list",
        group: "Core · Data display",
        description: "Drag and drop list composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::drag_and_drop_list",
        source: include_str!("examples/drag_and_drop_list.rs"),
        preview: || rsx! { crate::examples::drag_and_drop_list::Demo {} },
    },
    Entry {
        slug: "connection-flow",
        name: "Connection flow",
        group: "Composed",
        description: "Source, Conctr hub and target in healthy, paused and failing states.",
        module: "composed::connection_flow",
        source: include_str!("examples/connection_flow.rs"),
        preview: || rsx! { crate::examples::connection_flow::Demo {} },
    },
    Entry {
        slug: "empty-state",
        name: "Empty state",
        group: "Composed",
        description: "An explanation and a clear action when there is no content.",
        module: "composed::empty_state",
        source: include_str!("examples/empty_state.rs"),
        preview: || rsx! { crate::examples::empty_state::Demo {} },
    },
    Entry {
        slug: "alerts",
        name: "Alerts",
        group: "Core · Feedback",
        description: "Alerts composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::alert",
        source: include_str!("examples/alerts.rs"),
        preview: || rsx! { crate::examples::alerts::Demo {} },
    },
    Entry {
        slug: "toast",
        name: "Toast",
        group: "Core · Feedback",
        description: "Five-second confirmations with hover and focus pausing.",
        module: "ui::toast",
        source: include_str!("examples/toast.rs"),
        preview: || rsx! { crate::examples::toast::Demo {} },
    },
    Entry {
        slug: "progress",
        name: "Progress",
        group: "Core · Feedback",
        description: "Progress composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::progress",
        source: include_str!("examples/progress.rs"),
        preview: || rsx! { crate::examples::progress::Demo {} },
    },
    Entry {
        slug: "spinner",
        name: "Spinner",
        group: "Core · Feedback",
        description: "Spinner composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::spinner",
        source: include_str!("examples/spinner.rs"),
        preview: || rsx! { crate::examples::spinner::Demo {} },
    },
    Entry {
        slug: "skeleton",
        name: "Skeleton",
        group: "Core · Feedback",
        description: "Skeleton composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::skeleton",
        source: include_str!("examples/skeleton.rs"),
        preview: || rsx! { crate::examples::skeleton::Demo {} },
    },
    Entry {
        slug: "dialog",
        name: "Dialog",
        group: "Core · Overlays",
        description: "Focus-managed modal content with a title and description.",
        module: "ui::dialog",
        source: include_str!("examples/dialog.rs"),
        preview: || rsx! { crate::examples::dialog::Demo {} },
    },
    Entry {
        slug: "alert-dialog",
        name: "Alert dialog",
        group: "Core · Overlays",
        description: "Alert dialog composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::alert_dialog",
        source: include_str!("examples/alert_dialog.rs"),
        preview: || rsx! { crate::examples::alert_dialog::Demo {} },
    },
    Entry {
        slug: "modal",
        name: "Modal",
        group: "Composed",
        description: "Conctr’s composed dialog with a title, body and action footer.",
        module: "composed::modal",
        source: include_str!("examples/modal.rs"),
        preview: || rsx! { crate::examples::modal::Demo {} },
    },
    Entry {
        slug: "sheet",
        name: "Sheet",
        group: "Core · Overlays",
        description: "Sheet composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::sheet",
        source: include_str!("examples/sheet.rs"),
        preview: || rsx! { crate::examples::sheet::Demo {} },
    },
    Entry {
        slug: "popover",
        name: "Popover",
        group: "Core · Overlays",
        description: "Popover composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::popover",
        source: include_str!("examples/popover.rs"),
        preview: || rsx! { crate::examples::popover::Demo {} },
    },
    Entry {
        slug: "hover-card",
        name: "Hover card",
        group: "Core · Overlays",
        description: "Hover card composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::hover_card",
        source: include_str!("examples/hover_card.rs"),
        preview: || rsx! { crate::examples::hover_card::Demo {} },
    },
    Entry {
        slug: "dropdown-menu",
        name: "Dropdown menu",
        group: "Core · Overlays",
        description: "Dropdown menu composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::dropdown_menu",
        source: include_str!("examples/dropdown_menu.rs"),
        preview: || rsx! { crate::examples::dropdown_menu::Demo {} },
    },
    Entry {
        slug: "context-menu",
        name: "Context menu",
        group: "Core · Overlays",
        description: "Context menu composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::context_menu",
        source: include_str!("examples/context_menu.rs"),
        preview: || rsx! { crate::examples::context_menu::Demo {} },
    },
    Entry {
        slug: "tooltip",
        name: "Tooltip",
        group: "Core · Overlays",
        description: "Tooltip composed from the shared Conctr component library. Inspect the live example and copy its source.",
        module: "ui::tooltip",
        source: include_str!("examples/tooltip.rs"),
        preview: || rsx! { crate::examples::tooltip::Demo {} },
    },
    Entry {
        slug: "menu",
        name: "Menu",
        group: "Core · Overlays",
        description: "A labeled actions trigger with keyboard navigation and closed-by-default content.",
        module: "ui::menu",
        source: include_str!("examples/menu.rs"),
        preview: || rsx! { crate::examples::menu::Demo {} },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_covers_every_registry_family_with_unique_live_examples() {
        let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/conctr-ui-dioxus/src/components/ui");
        let modules: HashSet<_> = ENTRIES.iter().map(|entry| entry.module).collect();
        let slugs: HashSet<_> = ENTRIES.iter().map(|entry| entry.slug).collect();
        assert_eq!(slugs.len(), ENTRIES.len(), "Duplicate catalog URLs");
        // Every primitive-backed styled family needs a live example. Native kit
        // helpers can be demonstrated together in an existing family example.
        for family in std::fs::read_dir(ui).expect("UI families") {
            let family = family.expect("UI family").path();
            let source = std::fs::read_to_string(family.join("component.rs")).unwrap_or_default();
            if !family.join("style.css").is_file() || !source.contains("dioxus_primitives::") {
                continue;
            }
            let name = family.file_name().expect("family name").to_string_lossy();
            // Native helpers import attribute-merging utilities, not primitives.
            if !source.contains(&format!("dioxus_primitives::{name}")) {
                continue;
            }
            assert!(
                modules.contains(format!("ui::{name}").as_str()),
                "Missing live example for {name}"
            );
        }
        for entry in ENTRIES {
            assert!(
                GROUPS.contains(&entry.group),
                "Unlisted group for {}",
                entry.slug
            );
            assert!(
                entry.source.contains("pub fn Demo()"),
                "Missing usage source for {}",
                entry.slug
            );
            assert!(entry.source.contains("conctr_ui_dioxus::"));
        }
    }

    #[test]
    fn bundled_reference_and_palette_match_the_design_kit() {
        assert_eq!(
            include_str!("../assets/reference/ui-kit/index.html"),
            include_str!("../../../design/ui-kit/index.html")
        );
        assert_eq!(
            include_str!("../assets/reference/ui-kit/conctr-ui.css"),
            include_str!("../../../design/ui-kit/conctr-ui.css")
        );
        // The library scopes the same kit values to nested providers.
        fn tokens(css: &str) -> std::collections::BTreeSet<(&str, &str)> {
            css.split([';', '{', '}'])
                .filter_map(|part| part.trim().split_once(':'))
                .filter(|(name, _)| name.starts_with("--cn-"))
                .map(|(name, value)| (name.trim(), value.trim()))
                .collect()
        }
        let canonical = tokens(include_str!("../../../design/colors/tokens.css"));
        let bundled = tokens(include_str!(
            "../../../crates/conctr-ui-dioxus/assets/colors/tokens.css"
        ));
        assert!(!canonical.is_empty());
        assert!(
            canonical.is_subset(&bundled),
            "Keep canonical kit token values"
        );
    }
}
