use super::*;

#[component]
pub(super) fn MappingRow(
    number: usize,
    link: MappingLink,
    source_fields: Rc<Vec<MappingField>>,
    target_fields: Rc<Vec<MappingField>>,
    transforms: Rc<Vec<MappingTransform>>,
    occupied: Vec<String>,
    read_only: bool,
    on_change: EventHandler<MappingLink>,
    on_remove: EventHandler<()>,
) -> Element {
    let transform = transforms
        .iter()
        .find(|item| Some(&item.id) == link.transform.as_ref());
    let source_link = link.clone();
    let target_link = link.clone();
    let transform_link = link.clone();
    rsx! {
        div { class: "cn-field-map-row {Styles::row.inner}", "data-mapping-id": link.id,
            FieldPicker {
                role: "Source",
                number,
                fields: source_fields,
                selected: link.source.clone(),
                read_only,
                on_change: move |value| {
                    let mut updated = source_link.clone();
                    updated.source = value;
                    on_change.call(updated);
                },
            }
            div {
                class: "cn-field-map-transform-lane {Styles::transform_lane.inner}",
                "data-transformed": link.transform.is_some().to_string(),
                svg {
                    class: "cn-field-map-wire {Styles::wire.inner}",
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
                    class: "cn-field-map-port cn-field-map-port-source {Styles::port.inner} {Styles::port_source.inner}",
                    "aria-hidden": "true",
                }
                div {
                    class: "cn-field-map-transform {Styles::transform.inner}",
                    title: transform
                        .map_or("Copy the field without a transform", |item| item.description.as_str()),
                    icons::WandSparkles { class: "size-4" }
                    select {
                        "aria-label": "Transform for mapping {number}",
                        value: link.transform.clone().unwrap_or_default(),
                        disabled: read_only,
                        onchange: move |event: Event<FormData>| {
                            let mut updated = transform_link.clone();
                            let value = event.value();
                            updated.transform = (!value.is_empty()).then_some(value);
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
                        for item in transforms.iter() {
                            option {
                                value: item.id.clone(),
                                selected: Some(&item.id) == link.transform.as_ref(),
                                "{item.label}"
                            }
                        }
                    }
                }
                span {
                    class: "cn-field-map-port cn-field-map-port-target {Styles::port.inner} {Styles::port_target.inner}",
                    "aria-hidden": "true",
                }
            }
            FieldPicker {
                role: "Target",
                number,
                fields: target_fields,
                selected: link.target.clone(),
                read_only,
                occupied,
                on_remove,
                on_change: move |value| {
                    let mut updated = target_link.clone();
                    updated.target = value;
                    on_change.call(updated);
                },
            }
        }
    }
}

#[component]
fn FieldPicker(
    role: &'static str,
    number: usize,
    fields: Rc<Vec<MappingField>>,
    selected: String,
    read_only: bool,
    #[props(default)] occupied: Vec<String>,
    on_change: EventHandler<String>,
    #[props(default)] on_remove: Option<EventHandler<()>>,
) -> Element {
    let current = fields.iter().find(|field| field.id == selected);
    let side = role.to_ascii_lowercase();
    let target_class = if on_remove.is_some() {
        "cn-field-map-target"
    } else {
        ""
    };
    rsx! {
        div {
            class: "cn-field-map-field {target_class} {Styles::field.inner}",
            "data-missing": current.is_none().to_string(),
            div { class: "cn-field-map-field-top {Styles::field_top.inner}",
                span { class: "cn-field-map-field-icon {Styles::field_icon.inner}",
                    icons::Braces { class: "size-4" }
                }
                select {
                    "aria-label": "{role} field for mapping {number}",
                    "aria-invalid": current.is_none().to_string(),
                    value: selected.clone(),
                    disabled: read_only,
                    onchange: move |event: Event<FormData>| {
                        on_change.call(event.value());
                    },
                    if current.is_none() {
                        option {
                            value: selected.clone(),
                            disabled: true,
                            selected: true,
                            "Missing {side} field"
                        }
                    }
                    for field in fields.iter() {
                        option {
                            value: field.id.clone(),
                            selected: field.id == selected,
                            disabled: occupied.contains(&field.id),
                            "{field.label}"
                        }
                    }
                }
                span { class: "cn-field-map-type {Styles::field_type.inner}",
                    {current.map_or("unknown", |field| field.data_type.as_str())}
                }
                if let Some(on_remove) = on_remove.filter(|_| !read_only) {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::IconXs,
                        aria_label: "Remove mapping {number}",
                        onclick: move |_| on_remove.call(()),
                        icons::X { class: "size-4" }
                    }
                }
            }
            code { {current.map_or_else(|| format!("Choose a {side} field"), |field| field.path.clone())} }
        }
    }
}
