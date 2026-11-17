use super::{Styles, TransformerDefinition};
use dioxus::prelude::*;
/// Searchable catalog and ordered removable/reorderable pipeline, without a large select.
#[component]
pub(super) fn TransformPicker(
    steps: Vec<String>,
    catalog: Vec<TransformerDefinition>,
    read_only: bool,
    on_change: EventHandler<Vec<String>>,
) -> Element {
    let mut query = use_signal(String::new);
    let mut open = use_signal(|| false);
    let search = query().to_lowercase();
    // Search presentation text, but persist stable catalog IDs in pipeline order.
    let matches: Vec<_> = catalog
        .iter()
        .filter(|t| {
            format!("{} {} {}", t.label, t.description, t.category)
                .to_lowercase()
                .contains(&search)
        })
        .collect();
    rsx! {
        div {
            class: Styles::transform_picker.inner,
            "data-transform-picker": "",
            strong { "Transform pipeline" }
            if steps.is_empty() {
                p { "Direct copy · no transforms" }
            }
            ol { class: Styles::pipeline.inner,
                for (index, id) in steps.iter().enumerate() {
                    li {
                        key: "{index}-{id}",
                        "data-transform-step": id.clone(),
                        span { "{index+1}" }
                        strong {
                            {catalog.iter().find(|t| t.id == *id).map(|t| t.label.as_str()).unwrap_or(id)}
                        }
                        if !read_only {
                            button {
                                r#type: "button",
                                "aria-label": format!("Move transform {} up", index + 1),
                                disabled: index == 0,
                                onclick: {
                                    let steps = steps.clone();
                                    move |_| {
                                        let mut next = steps.clone();
                                        next.swap(index, index - 1);
                                        on_change.call(next);
                                    }
                                },
                                icons::ArrowUp { class: "size-4" }
                            }
                            button {
                                r#type: "button",
                                "aria-label": format!("Remove transform {}", index + 1),
                                onclick: {
                                    let steps = steps.clone();
                                    move |_| {
                                        let mut next = steps.clone();
                                        next.remove(index);
                                        on_change.call(next);
                                    }
                                },
                                icons::X { class: "size-4" }
                            }
                        }
                    }
                }
            }
            if !read_only {
                button {
                    r#type: "button",
                    class: Styles::picker_trigger.inner,
                    "aria-expanded": open().to_string(),
                    onclick: move |_| open.set(!open()),
                    icons::Plus { class: "size-4" }
                    "Add transformer"
                }
                if open() {
                    div { class: Styles::picker_panel.inner,
                        label { class: Styles::search.inner,
                            icons::Search { class: "size-4" }
                            input {
                                r#type: "search",
                                "aria-label": "Search transformers",
                                placeholder: "Search name, category or description…",
                                value: query(),
                                oninput: move |event| query.set(event.value()),
                                onkeydown: move |event| {
                                    if event.key() == Key::Escape {
                                        open.set(false);
                                    }
                                },
                            }
                        }
                        div { class: Styles::picker_results.inner,
                            if matches.is_empty() {
                                p { role: "status", "No matching transformers" }
                            }
                            for item in matches {
                                button {
                                    key: "{item.id}",
                                    r#type: "button",
                                    "data-transform-option": item.id.clone(),
                                    onclick: {
                                        let steps = steps.clone();
                                        let id = item.id.clone();
                                        move |_| {
                                            let mut next = steps.clone();
                                            next.push(id.clone());
                                            on_change.call(next);
                                            open.set(false);
                                            query.set(String::new());
                                        }
                                    },
                                    span { "{item.category}" }
                                    strong { "{item.label}" }
                                    small { "{item.description}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
