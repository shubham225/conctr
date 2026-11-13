//! Explicit stylesheet registration keeps every SSR response styled.
//! The CSS-module macro's process-wide OnceLock alone cannot register links
//! in each separate server-rendered document.
//!
//! Manganis scopes selectors using the CSS source path, but the processed asset
//! cache can reuse identical source contents after a move. Each CSS module starts
//! with its source path so moving it also changes its content/cache identity.
use dioxus::prelude::*;

const REGISTRY_STYLES: &[Asset] = &[
    asset!(
        "/src/components/ui/accordion/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/alert_dialog/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/aspect_ratio/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/avatar/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/badge/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/button/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/calendar/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/card/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/checkbox/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/collapsible/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/color_picker/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/combobox/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/context_menu/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/date_picker/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/dialog/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/drag_and_drop_list/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/dropdown_menu/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/form/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/hover_card/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/input/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/item/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/label/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/menubar/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/navbar/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/pagination/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/popover/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/progress/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/radio_group/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/select/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/separator/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/sheet/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/sidebar/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/skeleton/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/slider/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/switch/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/tabs/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/tag_group/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/textarea/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/toast/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/toggle/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/toggle_group/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/toolbar/style.css",
        manganis::AssetOptions::css_module()
    ),
    asset!(
        "/src/components/ui/tooltip/style.css",
        manganis::AssetOptions::css_module()
    ),
];

#[component]
pub fn RegistryStyles() -> Element {
    rsx! { for stylesheet in REGISTRY_STYLES { document::Stylesheet { href: *stylesheet } } }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    #[test]
    fn css_module_content_identity_tracks_its_source_path() {
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let ui = crate_root.join("src/components/ui");
        let mut modules = 0;
        for family in fs::read_dir(ui).expect("UI component directory") {
            let file = family.expect("component family").path().join("style.css");
            if !file.is_file() {
                continue;
            }
            modules += 1;
            let relative = file
                .strip_prefix(crate_root)
                .expect("crate-local stylesheet");
            let marker = format!("/* CSS module source: /{} */", relative.to_string_lossy());
            let source = fs::read_to_string(&file).expect("stylesheet source");
            assert!(
                source.starts_with(&marker),
                "Update the CSS module source marker after moving {}: cached selectors depend on its path",
                file.display()
            );
        }
        assert!(modules > 0, "No CSS modules checked");
    }
}
