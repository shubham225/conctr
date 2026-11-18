//! Pipeline authoring page. Drafts and configuration are local presentation state.
mod state;
use crate::Route;
use conctr_ui_dioxus::{
    composed::pipeline_workspace::PipelineWorkspace,
    ui::{
        button::{Button, ButtonSize, ButtonVariant},
        input::{Input, InputType},
        workflow::*,
    },
};
use dioxus::prelude::*;
use state::StepKind;
use std::collections::BTreeMap;

#[component]
pub fn PipelineStudio() -> Element {
    let (nodes, edges) = state::sample();
    let mut canvas = use_workflow(nodes, edges);
    let mut kinds = use_signal(|| {
        BTreeMap::from([
            ("step-0".into(), StepKind::Trigger),
            ("step-1".into(), StepKind::Verify),
            ("step-2".into(), StepKind::Mapping),
            ("step-3".into(), StepKind::Condition),
            ("step-4".into(), StepKind::Call),
            ("step-5".into(), StepKind::Call),
            ("step-6".into(), StepKind::Respond),
            ("step-7".into(), StepKind::Error),
        ])
    });
    let mut sequence = use_signal(|| 100_u64);
    let mut search = use_signal(String::new);
    let mut show_checks = use_signal(|| false);
    let nodes = canvas.nodes.read().clone();
    let edges = canvas.edges.read().clone();
    let selected = canvas
        .selected_idx()
        .and_then(|i| nodes.get(i).cloned().map(|n| (i, n)));
    let selected_edge = canvas
        .selected_edge_idx()
        .and_then(|i| edges.get(i).cloned().map(|e| (i, e)));
    let issues = state::draft_issues(&nodes, &edges);
    rsx! {
        document::Title {"Pipeline Studio · Conctr"}
        section {class:"flex min-w-0 flex-col gap-4", "data-pipeline-studio":"",
            div {class:"flex flex-wrap items-center justify-between gap-4",
                div {p {class:"mb-1 text-xs text-muted-foreground","Pipelines / Order intake"} h1 {class:"text-2xl font-semibold tracking-tight","Pipeline Studio"}}
                span {class:"rounded-full border border-border bg-muted px-3 py-1 text-xs text-muted-foreground","Local draft"}
            }
            div {class:"flex flex-wrap items-center gap-3 rounded-lg border border-border bg-card p-3",role:"toolbar","aria-label":"Pipeline actions",
                div {class:"mr-auto", strong {class:"text-sm","Order intake"} p {class:"text-xs text-muted-foreground","{nodes.len()} steps · {edges.len()} connections"}}
                Button {variant:ButtonVariant::Outline,size:ButtonSize::Sm,onclick:move |_|canvas.zoom_step(1.0/1.2),aria_label:"Zoom out pipeline", "−"}
                span {class:"text-xs tabular-nums text-muted-foreground","{(canvas.zoom_value()*100.0).round()}%"}
                Button {variant:ButtonVariant::Outline,size:ButtonSize::Sm,onclick:move |_|canvas.zoom_step(1.2),aria_label:"Zoom in pipeline", "+"}
                Button {variant:ButtonVariant::Outline,size:ButtonSize::Sm,onclick:move |_|canvas.zoom_reset(),"Reset view"}
                Button {size:ButtonSize::Sm,onclick:move |_|show_checks.set(!show_checks()),"Check draft"}
            }
            if show_checks() {
                div {class:"rounded-lg border border-border bg-muted p-4",role:"status",
                    strong {class:"text-sm",if issues.is_empty(){"Draft connections look complete"}else{"Review draft connections"}}
                    p {class:"mt-1 text-xs text-muted-foreground","Authoring checks only. Execution, credentials and deployment have not been validated."}
                    if !issues.is_empty(){ul {class:"mt-2 list-inside list-disc text-sm",for issue in &issues {li {"{issue}"}}}}
                }
            }
            PipelineWorkspace {state:canvas,
                library:rsx! {
                    div {class:"mb-4", h2 {class:"mb-1 text-sm font-semibold","Add a step"} p {class:"text-xs text-muted-foreground","Build your integration"}}
                    label {class:"grid gap-2 text-xs text-muted-foreground", "Find steps" Input {r#type:InputType::Search,placeholder:"Find steps…",value:search(),oninput:move |ev:FormEvent|search.set(ev.value())}}
                    for group in ["Triggers","Logic","Data","Output"] {
                        div {class:"mt-4 grid gap-1",h3 {class:"mb-1 text-xs font-medium text-muted-foreground","{group}"}
                            for kind in StepKind::ALL.into_iter().filter(|kind|kind.group()==group&&kind.label().to_lowercase().contains(&search().to_lowercase())) {
                                Button {variant:ButtonVariant::Ghost,size:ButtonSize::Sm,class:"justify-start",onclick:move |_| {
                                    let mut id=format!("step-{}",sequence());
                                    while canvas.nodes.peek().iter().any(|n|n.id==id) {sequence+=1;id=format!("step-{}",sequence());}
                                    sequence+=1;
                                    let (px,py)=*canvas.pan.peek();let zoom=canvas.zoom_value();
                                    let node=kind.node(id.clone(),(-px/zoom+100.).max(0.),(-py/zoom+100.).max(0.));
                                    let idx=canvas.nodes.peek().len();
                                    canvas.positions.write().push((node.initial_x,node.initial_y));
                                    canvas.nodes.write().push(node);kinds.write().insert(id,kind);canvas.select_node(idx);
                                }, StepIcon {kind} "{kind.label()}"}
                            }
                        }
                    }
                    p {class:"mt-6 text-xs text-muted-foreground","Click a step to add it to the visible canvas."}
                },
                inspector:rsx! {
                    if let Some((idx,node))=selected {
                        StepInspector {key:"{node.id}",canvas,idx,node:node.clone(),kind:kinds.read().get(&node.id).copied(),
                            on_duplicate:move |_|{
                                let original=canvas.nodes.peek().get(idx).cloned();
                                if let Some(original)=original {
                                    let kind=kinds.peek().get(&original.id).copied();
                                    canvas.duplicate_node(idx);
                                    if let Some(new)=canvas.nodes.peek().last() && let Some(kind)=kind {kinds.write().insert(new.id.clone(),kind);}
                                }
                            }
                        }
                    } else if let Some((idx,edge))=selected_edge {
                        div {class:"grid gap-4",h3 {class:"text-base font-semibold","Connection"}
                            p {class:"break-all font-mono text-xs text-muted-foreground","{edge.from} → {edge.to}"}
                            label {class:"grid gap-2 text-sm","Branch label"
                                Input {value:edge.label.clone().unwrap_or_default(),placeholder:"e.g. Yes, No, On error",oninput:move |ev:FormEvent|{if let Some(edge)=canvas.edges.write().get_mut(idx){edge.label=if ev.value().trim().is_empty(){None}else{Some(ev.value())};}}}
                            }
                            Button {variant:ButtonVariant::Destructive,size:ButtonSize::Sm,onclick:move |_|canvas.delete_edge(idx),"Delete connection"}
                        }
                    } else {
                        div {class:"grid gap-4",h3 {class:"text-base font-semibold","Your integration, step by step"}p {class:"text-sm text-muted-foreground","Select a step to configure it, or select a connection to name a branch."}
                            div {class:"rounded-md border border-border bg-muted p-3 text-xs text-muted-foreground","Draft changes stay in this page session. Testing, credential binding and deployment will be added separately."}
                        }
                    }
                },
                for (idx,node) in nodes.iter().enumerate() {
                    WorkflowNodeWrapper {key:"{node.id}",state:canvas,idx,
                        WfNode {class:"h-[72px]",target:node.has_target,source:node.has_source,
                            div {class:"min-w-0", "data-pipeline-node":node.id.clone(),
                                tabindex:"0",role:"button","aria-label":format!("Select {}",node.label),
                                onfocus:move |_|canvas.select_node(idx),
                                onkeydown:move |event:KeyboardEvent| {if event.key()==Key::Enter || event.key()==Key::Character(" ".into()) {event.prevent_default();event.stop_propagation();canvas.select_node(idx);}},
                                WfNodeHeader {
                                    if let Some(kind)=kinds.read().get(&node.id).copied(){StepIcon {kind}}else{icons::Workflow {class:"size-4"}}
                                    WfNodeTitle {class:"truncate","{node.label}"}
                                }
                                p {class:"truncate px-3 py-2 text-xs text-muted-foreground", title:node.description.clone(), "{node.description}"}
                            }
                        }
                    }
                }
            }
            p {class:"text-xs text-muted-foreground","Visual authoring preview · No requests are sent · Changes are kept until you leave this page"}
        }
    }
}

#[component]
fn StepIcon(kind: StepKind) -> Element {
    rsx! {span {"aria-hidden":"true", match kind {
        StepKind::Trigger=>rsx!{icons::Zap {class:"size-4"}},StepKind::Verify=>rsx!{icons::ShieldCheck {class:"size-4"}},StepKind::Mapping=>rsx!{icons::ArrowLeftRight {class:"size-4"}},StepKind::Condition=>rsx!{icons::GitBranch {class:"size-4"}},StepKind::Call=>rsx!{icons::Plug {class:"size-4"}},StepKind::Respond=>rsx!{icons::Send {class:"size-4"}},StepKind::Error=>rsx!{icons::TriangleAlert {class:"size-4"}},StepKind::Delay=>rsx!{icons::Clock {class:"size-4"}},
    }}}
}

#[component]
fn StepInspector(
    mut canvas: WorkflowState,
    idx: usize,
    node: WorkflowNode,
    kind: Option<StepKind>,
    on_duplicate: EventHandler<()>,
) -> Element {
    let caption = kind.map_or("Step", StepKind::label);
    let detail = match kind {
        Some(StepKind::Call) => "Request",
        Some(StepKind::Condition) => "Expression",
        Some(StepKind::Trigger) => "Webhook route",
        Some(StepKind::Delay) => "Delay",
        Some(StepKind::Respond) => "Response",
        Some(StepKind::Error) => "Failure action",
        Some(StepKind::Mapping) => "Mapping reference",
        _ => "Configuration",
    };
    rsx! {
        div {class:"grid gap-5",
            div {class:"flex items-center gap-3",span {class:"flex size-10 items-center justify-center rounded-md bg-primary/10 text-primary",if let Some(kind)=kind {StepIcon {kind}}}div {p {class:"text-xs text-muted-foreground","{caption}"}h3 {class:"text-base font-semibold","{node.label}"}}}
            label {class:"grid gap-2 text-sm","Step name",Input {value:node.label,oninput:move |ev:FormEvent|{if let Some(node)=canvas.nodes.write().get_mut(idx){node.label=ev.value();}}}}
            label {class:"grid gap-2 text-sm","{detail}",Input {value:node.description,oninput:move |ev:FormEvent|{if let Some(node)=canvas.nodes.write().get_mut(idx){node.description=ev.value();}}}}
            if kind==Some(StepKind::Mapping) {
                Link {to:Route::MappingStudio {},class:"text-sm text-primary underline underline-offset-4","Open mapping editor →"}
            }
            if kind==Some(StepKind::Call) {
                p {class:"rounded-md border border-border bg-muted p-3 text-xs text-muted-foreground","Request details are a draft. Credentials and operation binding are not configured yet."}
            }
            if kind==Some(StepKind::Condition) {p {class:"text-xs text-muted-foreground","Select the outgoing connections to label your Yes and No branches. Expressions are not evaluated in this preview."}}
            if kind==Some(StepKind::Error) {p {class:"text-xs text-muted-foreground","Failure handling requires an explicit runtime policy. This preview does not retry external writes."}}
            div {class:"grid grid-cols-2 gap-2",Button {variant:ButtonVariant::Outline,size:ButtonSize::Sm,onclick:move |_|on_duplicate.call(()),"Duplicate"}Button {variant:ButtonVariant::Destructive,size:ButtonSize::Sm,onclick:move |_|{canvas.select_node(idx);canvas.delete_selected();},"Delete step"}}
            p {class:"text-xs text-muted-foreground","Drag to reposition. Right-click for canvas actions."}
        }
    }
}
