use dioxus::document::{Document, Eval, LinkProps, NoOpDocument};
use std::{cell::RefCell, rc::Rc};
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
use conctr_ui_dioxus::{prelude::*, ui::input::Input};
use dioxus::prelude::*;

#[component]
fn Consumer() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::Dark,
            Button { class: "h-12", disabled: true, "Test connection" }
            Input { placeholder: "Connection name" }
        }
    }
}

#[test]
fn standalone_facade_preserves_props_and_styles_in_each_document() {
    for _ in 0..2 {
        let document = Rc::new(RecordingDocument::default());
        let mut dom = VirtualDom::new(Consumer);
        dom.provide_root_context(document.clone() as Rc<dyn Document>);
        dom.rebuild_in_place();
        let markup = dioxus_ssr::render(&dom);
        assert!(markup.contains("data-theme=\"dark\""));
        assert!(markup.contains("h-12"));
        assert!(!markup.contains("h-10 h-12"));
        assert!(markup.contains("disabled"));
        assert!(markup.contains("placeholder=\"Connection name\""));
        let links = document.links.borrow();
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
