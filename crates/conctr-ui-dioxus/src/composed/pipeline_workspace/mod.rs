//! Generic workflow workspace framing. Product step definitions belong to callers.
use crate::composed::resizable::*;
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::workflow::{WorkflowCanvas, WorkflowMinimap, WorkflowState};
use dioxus::prelude::*;
use std::rc::Rc;

#[css_module("/src/composed/pipeline_workspace/style.css")]
struct Styles;
const CSS: Asset = asset!(
    "/src/composed/pipeline_workspace/style.css",
    AssetOptions::css_module()
);

/// Step library, interactive workflow canvas and resizable inspector in one workspace.
#[component]
pub fn PipelineWorkspace(
    state: WorkflowState,
    library: Element,
    inspector: Element,
    children: Element,
) -> Element {
    let mut state = state;
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut width = use_signal(|| 304.0);
    let mut mode = use_signal(PanelMode::default);
    rsx! {
        document::Stylesheet {href:CSS}
        ResizablePanelGroup {
            section {class:Styles::workspace.inner, style:"--pipeline-inspector:{width()}px", "data-panel-mode":mode().as_str(), "aria-label":"Pipeline workspace",
                aside {class:Styles::library.inner, "aria-label":"Step library", {library}}
                div {class:Styles::canvas.inner,
                    onmounted:move |event|{mounted.set(Some(event.data()));state.zoom.set(0.85);state.pan.set((16.,100.));},
                    WorkflowCanvas {state, overlay:rsx! {
                        div {class:"absolute left-3 top-3",Button {variant:ButtonVariant::Outline,size:ButtonSize::Sm,onclick:move |_|{let element=mounted.peek().clone();spawn(async move {if let Some(element)=element && let Ok(rect)=element.get_client_rect().await {state.fit_to_view(rect.size.width,(rect.size.height-48.).max(240.),72.);}});},"Fit canvas"}}
                        div {class:Styles::minimap.inner,WorkflowMinimap {state}}
                    }, {children}}
                    p {class:"px-4 py-3 text-xs text-muted-foreground", "Drag steps to arrange · Connect the right port to a left port · Scroll to zoom · Right-click for actions"}
                }
                if mode()==PanelMode::Normal {
                    ResizableHandle {label:"Resize step inspector",vertical:false,size:width(),minimum:240.0,maximum_fraction:0.55,class:Styles::resize.inner,on_change:move |value|width.set(value)}
                }
                aside {class: Styles::inspector.inner,"aria-label":"Step inspector",
                    header { class:"flex items-center justify-between gap-2 border-b border-border p-3",
                        if mode()!=PanelMode::Minimized {h2 {class:"text-sm font-medium", "Step inspector"}}
                        PanelControls {label:"step inspector",mode:mode(),on_change:move |value|mode.set(value)}
                    }
                    if mode()!=PanelMode::Minimized {div {class:"p-4",{inspector}}}
                }
            }
        }
    }
}
