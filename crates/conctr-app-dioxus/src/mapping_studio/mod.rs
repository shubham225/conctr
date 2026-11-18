//! Product-owned mapping document and workspace. No execution or persistence yet.
mod state;
use conctr_ui_dioxus::composed::{
    mapping_workspace::MappingWorkspaceToolbar, visual_mapping::VisualFieldMapper,
};
use conctr_ui_dioxus::ui::code_block::{CodeBlock, CodeLanguage};
use dioxus::prelude::*;
use state::History;

/// Request and response authoring in one local, reversible draft.
#[component]
pub fn MappingStudio() -> Element {
    let mut history = use_signal(History::default);
    let mut response = use_signal(|| false);
    let direction = response();
    let draft = if direction {
        history.read().current.response.clone()
    } else {
        history.read().current.request.clone()
    };
    let links_snapshot = draft.clone();
    let source_sample = if direction {
        r#"{
  "id": "SO-10042",
  "status": "created",
  "created_at": "2026-11-17T10:30:00Z"
}"#
    } else {
        r#"{
  "order_id": "ORD-10042",
  "customer_id": "CUS-208",
  "email": "alex@example.test",
  "name": "Alex Morgan",
  "currency": "USD",
  "total": "149.00",
  "status": "paid",
  "city": "Pune",
  "country": "IN",
  "notes": "Leave at reception"
}"#
    };
    rsx! {
            document::Title { "Mapping Studio · Conctr" }
            section { class: "flex min-w-0 flex-col gap-4",
                p {class:"text-xs text-muted-foreground", "Mappings / Order to sales order · Local draft"}
                VisualFieldMapper {
                    key: "{direction}", studio: true,
                    toolbar: rsx! {            MappingWorkspaceToolbar {
                    response: direction, can_undo: !history.read().past.is_empty(), can_redo: !history.read().future.is_empty(),
                    on_direction: move |value| response.set(value),
                    on_undo: move |_| history.write().undo(), on_redo: move |_| history.write().redo(),
                }
    },
                    previews: rsx! {
                        div {h3 {span {"Source sample"} span {if direction {"Response · JSON"} else {"Request · JSON"}}}
                            CodeBlock { code: source_sample.to_owned(), language: CodeLanguage::Json }
                        }
                        div {h3 {span {"Target mapping"} span {"Draft · not executed"}}
                            pre {if draft.links.is_empty() {"Connect fields to build the target mapping."}
                                for link in &draft.links {"{link.target} ← {link.source}\n"}
                            }
                        }
                    },
                    source: draft.source.clone(), target: draft.target.clone(), links: draft.links.clone(),
                    on_change: move |links| { let mut next = links_snapshot.clone(); next.links = links; history.write().update(direction, next); },
                    on_schema_change: move |draft| history.write().update(direction, draft),
                }
                p { class: "text-xs text-muted-foreground", "Sample fields are editable. Drafts stay in this page session; transforms are configured but do not execute." }
            }
        }
}
