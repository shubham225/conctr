use conctr_ui_dioxus::assets::icons;
use conctr_ui_dioxus::{assets::images, composed::WorkflowBuilder, ui::workflow::*};
use dioxus::prelude::*;

#[component]
pub fn ConctrWorkflow() -> Element {
    let nodes = vec![
        WorkflowNode {
            id: "source".into(),
            initial_x: 32.0,
            initial_y: 160.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "REST source".into(),
            description: "Illustrative source".into(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "mapping".into(),
            initial_x: 304.0,
            initial_y: 160.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Conctr mapping".into(),
            description: "Illustrative mapping".into(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "target".into(),
            initial_x: 576.0,
            initial_y: 160.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "SOAP target".into(),
            description: "Illustrative target".into(),
            kind: WorkflowNodeKind::Output,
        },
    ];
    let edges = vec![
        WorkflowEdge {
            from: "source".into(),
            to: "mapping".into(),
            ..Default::default()
        },
        WorkflowEdge {
            from: "mapping".into(),
            to: "target".into(),
            ..Default::default()
        },
    ];
    let state = use_workflow(nodes, edges);
    rsx! {
        div { class: "grid gap-4",
            p { class: "text-sm text-muted-foreground",
                "Drag nodes, connect handles, pan, zoom, and undo canvas edits. This example does not run a connection."
            }
            WorkflowBuilder { state,
                for (idx, node) in state.nodes.read().iter().cloned().enumerate() {
                    WorkflowNodeWrapper { key: "{node.id}", state, idx,
                        WfNode { target: node.has_target, source: node.has_source,
                            WfNodeHeader {
                                img {
                                    src: match node.id.as_str() {
                                        "source" => images::REST,
                                        "target" => images::SOAP,
                                        _ => images::HUB,
                                    },
                                    alt: "",
                                    width: 24,
                                    height: 24,
                                }
                                WfNodeTitle { "{node.label}" }
                            }
                            p { class: "p-3 text-sm text-muted-foreground", "{node.description}" }
                        }
                    }
                }
            }
            IconPalette {}
        }
    }
}

#[component]
fn IconPalette() -> Element {
    let mut monochrome = use_signal(|| false);
    let (brand, patterns) = if monochrome() {
        (icons::mono::BRAND_GROUPS, icons::mono::PATTERN_GROUPS)
    } else {
        (icons::color::BRAND_GROUPS, icons::color::PATTERN_GROUPS)
    };
    let image_class = if monochrome() { "dark:invert" } else { "" };
    rsx! {
        section { "aria-label": "Icon library",
            h2 { class: "text-lg font-medium", "Icon library" }
            label { class: "inline-flex items-center gap-2",
                input { r#type: "checkbox", checked: monochrome(), onchange: move |event| monochrome.set(event.checked()) }
                "Monochrome"
            }
            for &(group, glyphs) in brand.iter().chain(patterns) {
                div { key: "{group}",
                    h3 { class: "text-sm font-medium", "{group}" }
                    ul { class: "flex flex-wrap gap-4 py-2",
                        for &(label, icon) in glyphs {
                            li { key: "{label}", class: "flex items-center gap-2 text-sm",
                                img { src: icon, alt: "", width: 48, height: 48, class: image_class }
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
