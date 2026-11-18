//! Read-only, syntax-highlighted text rendered with escaped Dioxus nodes.
mod syntax;

use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CodeLanguage {
    #[default]
    PlainText,
    Json,
    Yaml,
    Toml,
    Xml,
}

/// Highlight a display-ready payload or configuration snippet.
#[component]
pub fn CodeBlock(
    code: String,
    #[props(default)] language: CodeLanguage,
    #[props(default)] label: Option<String>,
) -> Element {
    let tokens = use_memo(use_reactive((&code, &language), |(code, language)| {
        syntax::highlight(&code, language)
    }));
    rsx! {
        pre {
            class: "m-0 max-h-60 min-w-0 overflow-auto rounded-md bg-muted p-3 font-mono text-xs",
            tabindex: "0",
            "aria-label": label.unwrap_or_else(|| "Code".into()),
            code {
                for (class, value) in tokens.read().iter() {
                    if class.is_empty() {
                        "{value}"
                    } else {
                        span { class: *class, "{value}" }
                    }
                }
            }
        }
    }
}
