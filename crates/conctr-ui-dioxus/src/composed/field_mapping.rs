//! Controlled visual mapping drafts. This module edits presentation data; it does
//! not evaluate transforms or define the platform's executable mapping semantics.
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

/// A request or response field. IDs must be unique within each system's schema.
#[derive(Clone, PartialEq, Eq)]
pub struct MappingField {
    pub id: String,
    pub label: String,
    pub path: String,
    pub data_type: String,
}

/// An available transform supplied by the caller. IDs must be unique and nonempty.
#[derive(Clone, PartialEq, Eq)]
pub struct MappingTransform {
    pub id: String,
    pub label: String,
    pub description: String,
}

/// One editable link. Source/target refer to field IDs; transform refers to a
/// caller-supplied transform ID. Link IDs must be unique and remain stable.
#[derive(Clone, PartialEq, Eq)]
pub struct MappingLink {
    pub id: String,
    pub source: String,
    pub target: String,
    pub transform: Option<String>,
}

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
    let additions = mappings.clone();
    let count = mappings.len();
    rsx! {
        section { class: "cn-field-mapping", "aria-label": "Field mapping editor",
            header { class: "cn-field-map-toolbar",
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
                                let mut updated = additions.clone();
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
                class: "cn-field-map-scroll",
                tabindex: "0",
                "aria-label": "Mapping connections",
                div { class: "cn-field-map-board",
                    div { class: "cn-field-map-columns",
                        div { class: "cn-field-map-system",
                            icons::Braces { class: "size-5" }
                            div {
                                strong { "{source_label}" }
                                span { "{source_caption} · {source_fields.len()} fields" }
                            }
                        }
                        div { class: "cn-field-map-middle-heading",
                            icons::Workflow { class: "size-4" }
                            "Transform"
                        }
                        div { class: "cn-field-map-system",
                            icons::Braces { class: "size-5" }
                            div {
                                strong { "{target_label}" }
                                span { "{target_caption} · {target_fields.len()} fields" }
                            }
                        }
                    }
                    if mappings.is_empty() {
                        div { class: "cn-field-map-empty",
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
                            source_fields: source_fields.clone(),
                            target_fields: target_fields.clone(),
                            transforms: transforms.clone(),
                            occupied: mappings
                                .iter()
                                .filter(|other| other.id != link.id)
                                .map(|other| other.target.clone())
                                .collect(),
                            read_only,
                            on_change: {
                                let snapshot = mappings.clone();
                                move |updated| {
                                    let mut next = snapshot.clone();
                                    next[index] = updated;
                                    on_change.call(next);
                                }
                            },
                            on_remove: {
                                let snapshot = mappings.clone();
                                move |_| {
                                    let mut next = snapshot.clone();
                                    next.remove(index);
                                    on_change.call(next);
                                }
                            },
                        }
                    }
                }
            }
            footer { class: "cn-field-map-footer", "aria-live": "polite",
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

#[component]
fn MappingRow(
    number: usize,
    link: MappingLink,
    source_fields: Vec<MappingField>,
    target_fields: Vec<MappingField>,
    transforms: Vec<MappingTransform>,
    occupied: Vec<String>,
    read_only: bool,
    on_change: EventHandler<MappingLink>,
    on_remove: EventHandler<()>,
) -> Element {
    let source = source_fields.iter().find(|field| field.id == link.source);
    let target = target_fields.iter().find(|field| field.id == link.target);
    let transform = transforms
        .iter()
        .find(|item| Some(&item.id) == link.transform.as_ref());
    let source_link = link.clone();
    let target_link = link.clone();
    let transform_link = link.clone();
    rsx! {
        div { class: "cn-field-map-row", "data-mapping-id": link.id,
            div {
                class: "cn-field-map-field",
                "data-missing": source.is_none().to_string(),
                div { class: "cn-field-map-field-top",
                    span { class: "cn-field-map-field-icon",
                        icons::Braces { class: "size-4" }
                    }
                    select {
                        "aria-label": "Source field for mapping {number}",
                        "aria-invalid": source.is_none().to_string(),
                        value: link.source.clone(),
                        disabled: read_only,
                        onchange: move |event: Event<FormData>| {
                            let mut updated = source_link.clone();
                            updated.source = event.value();
                            on_change.call(updated);
                        },
                        if source.is_none() {
                            option {
                                value: link.source.clone(),
                                disabled: true,
                                selected: true,
                                "Missing source field"
                            }
                        }
                        for field in &source_fields {
                            option {
                                value: field.id.clone(),
                                selected: field.id == link.source,
                                "{field.label}"
                            }
                        }
                    }
                    span { class: "cn-field-map-type",
                        {source.map_or("unknown", |field| field.data_type.as_str())}
                    }
                }
                code { {source.map_or("Choose a source field", |field| field.path.as_str())} }
            }
            div {
                class: "cn-field-map-transform-lane",
                "data-transformed": link.transform.is_some().to_string(),
                svg {
                    class: "cn-field-map-wire",
                    view_box: "0 0 100 64",
                    preserve_aspect_ratio: "none",
                    "aria-hidden": "true",
                    path {
                        d: "M0 32 C20 32 12 24 30 24 H70 C88 24 80 32 100 32",
                        fill: "none",
                        vector_effect: "non-scaling-stroke",
                    }
                }
                span {
                    class: "cn-field-map-port cn-field-map-port-source",
                    "aria-hidden": "true",
                }
                div {
                    class: "cn-field-map-transform",
                    title: transform
                        .map_or("Copy the field without a transform", |item| item.description.as_str()),
                    icons::WandSparkles { class: "size-4" }
                    select {
                        "aria-label": "Transform for mapping {number}",
                        value: link.transform.clone().unwrap_or_default(),
                        disabled: read_only,
                        onchange: move |event: Event<FormData>| {
                            let mut updated = transform_link.clone();
                            updated.transform = if event.value().is_empty() {
                                None
                            } else {
                                Some(event.value())
                            };
                            on_change.call(updated);
                        },
                        option { value: "", selected: link.transform.is_none(), "Direct copy" }
                        if link.transform.is_some() && transform.is_none() {
                            option {
                                value: link.transform.clone().unwrap_or_default(),
                                selected: true,
                                disabled: true,
                                "Missing transform"
                            }
                        }
                        for item in &transforms {
                            option {
                                value: item.id.clone(),
                                selected: Some(&item.id) == link.transform.as_ref(),
                                "{item.label}"
                            }
                        }
                    }
                }
                span {
                    class: "cn-field-map-port cn-field-map-port-target",
                    "aria-hidden": "true",
                }
            }
            div {
                class: "cn-field-map-field cn-field-map-target",
                "data-missing": target.is_none().to_string(),
                div { class: "cn-field-map-field-top",
                    span { class: "cn-field-map-field-icon",
                        icons::Braces { class: "size-4" }
                    }
                    select {
                        "aria-label": "Target field for mapping {number}",
                        "aria-invalid": target.is_none().to_string(),
                        value: link.target.clone(),
                        disabled: read_only,
                        onchange: move |event: Event<FormData>| {
                            let mut updated = target_link.clone();
                            updated.target = event.value();
                            on_change.call(updated);
                        },
                        if target.is_none() {
                            option {
                                value: link.target.clone(),
                                disabled: true,
                                selected: true,
                                "Missing target field"
                            }
                        }
                        for field in &target_fields {
                            option {
                                value: field.id.clone(),
                                selected: field.id == link.target,
                                disabled: occupied.contains(&field.id),
                                "{field.label}"
                            }
                        }
                    }
                    span { class: "cn-field-map-type",
                        {target.map_or("unknown", |field| field.data_type.as_str())}
                    }
                    if !read_only {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::IconXs,
                            aria_label: "Remove mapping {number}",
                            onclick: move |_| on_remove.call(()),
                            icons::X { class: "size-4" }
                        }
                    }
                }
                code { {target.map_or("Choose a target field", |field| field.path.as_str())} }
            }
        }
    }
}
