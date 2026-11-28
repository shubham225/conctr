//! Controlled visual mapping drafts. This module edits presentation data; it does
//! not evaluate transforms or define the platform's executable mapping semantics.
mod row;
use row::MappingRow;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;
use std::rc::Rc;

#[css_module("/src/composed/field_mapping/style.css")]
struct Styles;
const CSS: Asset = asset!(
    "/src/composed/field_mapping/style.css",
    AssetOptions::css_module()
);

mod model;
pub use model::{MappingField, MappingLink, MappingTransform};

/// Visual source → optional transform → target editor with caller-owned drafts.
/// New rows use the first source and next unused target. A source may feed several
/// targets; each target is offered once. Existing unknown references stay visible
/// as missing selections. Callers own validation, persistence and execution.
#[component]
pub fn FieldMappingEditor(
    source_label: String,
    target_label: String,
    source_fields: Vec<MappingField>,
    target_fields: Vec<MappingField>,
    mappings: Vec<MappingLink>,
    #[props(default)] transforms: Vec<MappingTransform>,
    #[props(default = "Response fields".into())] source_caption: String,
    #[props(default = "Request fields".into())] target_caption: String,
    #[props(default)] read_only: bool,
    on_change: EventHandler<Vec<MappingLink>>,
) -> Element {
    let available_target = target_fields
        .iter()
        .find(|field| !mappings.iter().any(|link| link.target == field.id));
    let next_link = source_fields
        .first()
        .zip(available_target)
        .map(|(source, target)| {
            let mut number = 1;
            while mappings
                .iter()
                .any(|link| link.id == format!("mapping-{number}"))
            {
                number += 1;
            }
            MappingLink {
                id: format!("mapping-{number}"),
                source: source.id.clone(),
                target: target.id.clone(),
                transform: None,
            }
        });
    // Rows and event handlers share the render's inputs. Each edit emits an
    // owned draft for the caller.
    let source_fields = Rc::new(source_fields);
    let target_fields = Rc::new(target_fields);
    let transforms = Rc::new(transforms);
    let mappings = Rc::new(mappings);
    let additions = Rc::clone(&mappings);
    let count = mappings.len();
    rsx! {
        document::Stylesheet { href: CSS }
        section { class: "cn-field-mapping {Styles::field_mapping.inner}", "aria-label": "Field mapping editor",
            header { class: "cn-field-map-toolbar {Styles::toolbar.inner}",
                div {
                    h2 { "Field mapping" }
                    p { "Connect fields and shape the data between systems." }
                }
                if !read_only {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        disabled: next_link.is_none(),
                        onclick: move |_| {
                            if let Some(link) = next_link.clone() {
                                let mut updated = additions.as_ref().clone();
                                updated.push(link);
                                on_change.call(updated);
                            }
                        },
                        icons::Plus { class: "size-4" }
                        "Add mapping"
                    }
                }
            }
            div {
                class: "cn-field-map-scroll {Styles::scroll.inner}",
                tabindex: "0",
                "aria-label": "Mapping connections",
                div { class: "cn-field-map-board {Styles::board.inner}",
                    div { class: "cn-field-map-columns {Styles::columns.inner}",
                        div { class: "cn-field-map-system {Styles::system.inner}",
                            icons::Braces { class: "size-5" }
                            div {
                                strong { "{source_label}" }
                                span { "{source_caption} · {source_fields.len()} fields" }
                            }
                        }
                        div { class: "cn-field-map-middle-heading {Styles::middle_heading.inner}",
                            icons::Workflow { class: "size-4" }
                            "Transform"
                        }
                        div { class: "cn-field-map-system {Styles::system.inner}",
                            icons::Braces { class: "size-5" }
                            div {
                                strong { "{target_label}" }
                                span { "{target_caption} · {target_fields.len()} fields" }
                            }
                        }
                    }
                    if mappings.is_empty() {
                        div { class: "cn-field-map-empty {Styles::empty.inner}",
                            icons::GitBranch { class: "size-8" }
                            strong { "No fields connected yet" }
                            p {
                                if source_fields.is_empty() || target_fields.is_empty() {
                                    "Provide source and target fields to begin."
                                } else {
                                    "Add a mapping, choose its fields, then optionally select a transform."
                                }
                            }
                        }
                    }
                    for (index, link) in mappings.iter().enumerate() {
                        MappingRow {
                            key: "{link.id}",
                            number: index + 1,
                            link: link.clone(),
                            source_fields: Rc::clone(&source_fields),
                            target_fields: Rc::clone(&target_fields),
                            transforms: Rc::clone(&transforms),
                            occupied: mappings
                                .iter()
                                .filter(|other| other.id != link.id)
                                .map(|other| other.target.clone())
                                .collect(),
                            read_only,
                            on_change: {
                                let snapshot = Rc::clone(&mappings);
                                move |updated| {
                                    let mut next = snapshot.as_ref().clone();
                                    next[index] = updated;
                                    on_change.call(next);
                                }
                            },
                            on_remove: {
                                let snapshot = Rc::clone(&mappings);
                                move |_| {
                                    let mut next = snapshot.as_ref().clone();
                                    next.remove(index);
                                    on_change.call(next);
                                }
                            },
                        }
                    }
                }
            }
            footer { class: "cn-field-map-footer {Styles::footer.inner}", "aria-live": "polite",
                span {
                    icons::GitBranch { class: "size-4" }
                    "{count} mappings"
                }
                span {
                    if read_only {
                        "Read only"
                    } else {
                        "Changes update your mapping draft"
                    }
                }
            }
        }
    }
}
