use std::{cell::RefCell, rc::Rc};

use conctr_ui_dioxus::prelude::*;
use dioxus::{
    document::{Document, Eval, LinkProps, NoOpDocument},
    prelude::*,
};

#[derive(Default)]
struct RecordingDocument {
    links: RefCell<Vec<String>>,
}

impl Document for RecordingDocument {
    fn eval(&self, script: String) -> Eval {
        NoOpDocument.eval(script)
    }

    fn create_link(&self, props: LinkProps) {
        if props.rel.as_deref() == Some("stylesheet")
            && let Some(href) = props.href
        {
            self.links.borrow_mut().push(href);
        }
    }
}

fn styles(app: fn() -> Element) -> Vec<String> {
    let document = Rc::new(RecordingDocument::default());
    let mut dom = VirtualDom::new(app);
    dom.provide_root_context(document.clone() as Rc<dyn Document>);
    dom.rebuild_in_place();
    document.links.borrow().clone()
}

#[test]
fn separate_documents_register_only_used_styles_and_deduplicate_instances() {
    let button = asset!(
        "/src/components/ui/button/style.css",
        manganis::AssetOptions::css_module()
    )
    .to_string();
    let calendar = asset!(
        "/src/components/ui/calendar/style.css",
        manganis::AssetOptions::css_module()
    )
    .to_string();
    let spinner = asset!(
        "/src/components/ui/spinner/style.css",
        manganis::AssetOptions::css_module()
    )
    .to_string();
    // Dioxus/Manganis 0.7.10 uses a process-wide CSS-module registration flag.
    // Keep per-document registration until the resolved version includes #5707:
    // https://github.com/DioxusLabs/dioxus/pull/5707
    for _ in 0..2 {
        let links = styles(|| {
            rsx! {
                ThemeProvider {
                    Button { "Save" }
                    Button { "Cancel" }
                }
            }
        });
        assert_eq!(links.iter().filter(|href| **href == button).count(), 1);
        assert!(!links.contains(&calendar));
        assert!(!links.contains(&spinner));
        for asset in [
            conctr_ui_dioxus::assets::styles::TOKENS_CSS,
            conctr_ui_dioxus::assets::styles::MAIN_CSS,
            conctr_ui_dioxus::assets::styles::TAILWIND_CSS,
        ] {
            assert_eq!(
                links
                    .iter()
                    .filter(|href| **href == asset.to_string())
                    .count(),
                1
            );
        }
    }
}

#[test]
fn loading_buttons_and_protocol_badges_load_their_visual_dependencies() {
    let links = styles(|| {
        rsx! {
            Button { loading: true, "Saving" }
            ProtocolBadge { protocol: Protocol::Rest }
        }
    });
    for asset in [
        asset!(
            "/src/components/ui/button/style.css",
            manganis::AssetOptions::css_module()
        ),
        asset!(
            "/src/components/ui/spinner/style.css",
            manganis::AssetOptions::css_module()
        ),
        asset!(
            "/src/components/ui/badge/style.css",
            manganis::AssetOptions::css_module()
        ),
        asset!(
            "/src/components/ui/shape/style.css",
            manganis::AssetOptions::css_module()
        ),
    ] {
        assert!(
            links.contains(&asset.to_string()),
            "Missing stylesheet {asset}"
        );
    }
}

#[test]
fn new_components_register_their_own_modules_in_each_ssr_document() {
    for _ in 0..2 {
        let links = styles(|| {
            rsx! {
                FileUploader { id: "files" }
                KbdGroup { Kbd { "Ctrl" } Kbd { "K" } }
                Command { id: "commands", CommandInput {} CommandList { CommandEmpty { "Empty" } } }
                ResizablePanelGroup { id: "panels",
                    ResizablePanel { index: 0, "One" }
                    ResizableHandle { index: 0 }
                    ResizablePanel { index: 1, "Two" }
                }
            }
        });
        for asset in [
            asset!(
                "/src/components/ui/file_uploader/style.css",
                manganis::AssetOptions::css_module()
            ),
            asset!(
                "/src/components/ui/kbd/style.css",
                manganis::AssetOptions::css_module()
            ),
            asset!(
                "/src/components/ui/command/style.css",
                manganis::AssetOptions::css_module()
            ),
            asset!(
                "/src/components/ui/resizable/style.css",
                manganis::AssetOptions::css_module()
            ),
        ] {
            assert_eq!(
                links
                    .iter()
                    .filter(|link| **link == asset.to_string())
                    .count(),
                1
            );
        }
        assert_eq!(links.len(), 4, "Unused component styles must stay absent");
    }
}

#[test]
fn module_source_markers_follow_their_paths() {
    use std::{fs, path::Path};

    fn visit(directory: &Path, root: &Path) -> usize {
        let mut checked = 0;
        for entry in fs::read_dir(directory).expect("component directory") {
            let path = entry.expect("component source").path();
            if path.is_dir() {
                checked += visit(&path, root);
            } else if path.file_name().is_some_and(|name| name == "style.css") {
                let relative = path.strip_prefix(root).expect("crate-local stylesheet");
                let marker = format!("/* CSS module source: /{} */", relative.to_string_lossy());
                assert!(
                    fs::read_to_string(&path)
                        .expect("module source")
                        .starts_with(&marker),
                    "Update the source marker after moving {} so the asset cache tracks its identity",
                    path.display()
                );
                checked += 1;
            }
        }
        checked
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(visit(&root.join("src/components"), root) > 0);
}

#[test]
fn theme_toggle_registers_its_animation_styles_in_every_document() {
    fn app() -> Element {
        rsx! { ThemeToggle { dark: false, on_change: move |_| {} } }
    }
    let sheet = asset!(
        "/src/components/ui/theme_toggle/style.css",
        manganis::AssetOptions::css_module()
    )
    .to_string();
    for _ in 0..2 {
        let links = styles(app);
        assert_eq!(links.iter().filter(|href| **href == sheet).count(), 1);
    }
}
