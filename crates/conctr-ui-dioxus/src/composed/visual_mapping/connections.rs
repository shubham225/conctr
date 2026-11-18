use super::{FieldLink, MappingSchema, Styles};
use dioxus::prelude::*;

/// Canvas wires are a projection of paths; selection is owned by the editor.
#[component]
pub(super) fn MappingConnections(
    source: MappingSchema,
    target: MappingSchema,
    links: Vec<FieldLink>,
    selected: Option<String>,
    height: usize,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
                        svg {
                            class: Styles::wires.inner,
                            view_box: format!("0 0 1000 {height}"),
                            preserve_aspect_ratio: "none",
                            "aria-label": "Mapping connections",
                            for link in &links {
                                if let (Some(si), Some(ti)) = (
                                    source.fields.iter().position(|f| f.path == link.source),
                                    target.fields.iter().position(|f| f.path == link.target),
                                )
                                {
                                    g {
                                        key: "{link.id}",
                                        "data-link-id": link.id.clone(),
                                        "data-active": (Some(&link.id) == selected.as_ref()).to_string(),
                                        path {
                                            d: format!(
                                                "M380 {} C480 {},520 {},620 {}",
                                                si * 40 + 20,
                                                si * 40 + 20,
                                                ti * 40 + 20,
                                                ti * 40 + 20,
                                            ),
                                            class: Styles::wire.inner,
                                        }
                                        path {
                                            d: format!(
                                                "M380 {} C480 {},520 {},620 {}",
                                                si * 40 + 20,
                                                si * 40 + 20,
                                                ti * 40 + 20,
                                                ti * 40 + 20,
                                            ),
                                            class: Styles::wire_hit.inner,
                                            onclick: {
                                                let id = link.id.clone();
                                                move |_| {
                                                    on_select.call(id.clone());
                                                }
                                            },
                                        }
                                        if !link.pipeline().is_empty() {
                                            circle {
                                                cx: "500",
                                                cy: ((si + ti) * 20 + 20).to_string(),
                                                r: "12",
                                                class: Styles::transform_dot.inner,
                                            }
                                            text {
                                                x: "500",
                                                y: ((si + ti) * 20 + 25).to_string(),
                                                text_anchor: "middle",
                                                class: Styles::transform_text.inner,
                                                "ƒ"
                                            }
                                        }
                                    }
                                }
                            }
                        }
    }
}
