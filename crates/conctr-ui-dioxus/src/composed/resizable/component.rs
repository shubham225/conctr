use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use std::rc::Rc;

#[css_module("/src/composed/resizable/style.css")]
struct Styles;
const CSS: Asset = asset!(
    "/src/composed/resizable/style.css",
    AssetOptions::css_module()
);

/// Presentation mode; restoring retains the caller's last resized dimensions.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PanelMode {
    #[default]
    Normal,
    Minimized,
    Maximized,
}
impl PanelMode {
    /// Stable value for panel layout attributes.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Minimized => "minimized",
            Self::Maximized => "maximized",
        }
    }
}

#[derive(Clone, Copy)]
struct ResizeContext {
    mounted: Signal<Option<Rc<MountedData>>>,
}

/// Measurement scope for a controlled resizable layout. Nested groups are isolated.
/// Callers own panel sizes and apply them through normal Rust-rendered styles.
#[component]
pub fn ResizablePanelGroup(children: Element) -> Element {
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_context_provider(|| ResizeContext { mounted });
    rsx! {
        document::Stylesheet { href: CSS }
        div { class:Styles::group.inner, "data-resizable-group":"",
            onmounted:move |event|mounted.set(Some(event.data())),
            {children}
        }
    }
}

/// Accessible minimize, maximize and restore controls.
#[component]
pub fn PanelControls(
    label: String,
    mode: PanelMode,
    on_change: EventHandler<PanelMode>,
) -> Element {
    rsx! {
        document::Stylesheet {href:CSS}
        div { class:Styles::controls.inner, "data-panel-controls":"",
            button {r#type:"button", "aria-label":format!("{} {label}",if mode==PanelMode::Minimized {"Restore"} else {"Minimize"}),
                onclick:move |_|on_change.call(if mode==PanelMode::Minimized {PanelMode::Normal} else {PanelMode::Minimized}),
                if mode==PanelMode::Minimized {icons::PanelRightOpen {class:"size-4"}} else {icons::Minus {class:"size-4"}}
            }
            button {r#type:"button", "aria-label":format!("{} {label}",if mode==PanelMode::Maximized {"Restore"} else {"Maximize"}),
                onclick:move |_|on_change.call(if mode==PanelMode::Maximized {PanelMode::Normal} else {PanelMode::Maximized}),
                if mode==PanelMode::Maximized {icons::Minimize2 {class:"size-4"}} else {icons::Maximize2 {class:"size-4"}}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Drag {
    pointer: i32,
    coordinate: f64,
    size: f64,
}

fn bounded(value: f64, minimum: f64, maximum: f64) -> f64 {
    if value.is_finite() {
        value.clamp(minimum, maximum.max(minimum)).round()
    } else {
        minimum
    }
}

/// Controlled trailing-panel splitter. All size changes are emitted from Rust.
/// Arrow keys resize by 16px, Shift by 48px; Home/End select the bounds.
/// A drag shield keeps events in the handle while crossing adjacent panels.
#[component]
pub fn ResizableHandle(
    label: String,
    vertical: bool,
    size: f64,
    on_change: EventHandler<f64>,
    #[props(default = 160.0)] minimum: f64,
    #[props(default = 0.7)] maximum_fraction: f64,
    #[props(default)] class: String,
) -> Element {
    let context = use_context::<ResizeContext>();
    let mut drag = use_signal(|| None::<Drag>);
    let mut maximum = use_signal(|| minimum.max(size));
    let mut ready = use_signal(|| false);
    let mut handle = use_signal(|| None::<Rc<MountedData>>);
    let fraction = if maximum_fraction.is_finite() {
        maximum_fraction.clamp(0.0, 1.0)
    } else {
        0.7
    };
    let minimum = if minimum.is_finite() {
        minimum.max(0.0)
    } else {
        0.0
    };
    use_effect(move || {
        let element = context.mounted.read().clone();
        if let Some(element) = element {
            spawn(async move {
                if let Ok(rect) = element.get_client_rect().await {
                    let extent = if vertical {
                        rect.size.height
                    } else {
                        rect.size.width
                    };
                    maximum.set((extent * fraction).max(minimum));
                }
            });
        }
    });
    rsx! {
        document::Stylesheet {href:CSS}
        div {class:format!("{} {class}",Styles::handle.inner),role:"separator",tabindex:"0", "aria-label":label,
            "aria-orientation":if vertical {"horizontal"} else {"vertical"},
            "aria-valuemin":minimum,"aria-valuemax":maximum(),"aria-valuenow":size,
            "data-orientation":if vertical {"vertical"} else {"horizontal"},
            "data-resize-handle":"", "data-resize-ready":if ready() {Some("true")} else {None},
            onmounted:move |event|{handle.set(Some(event.data()));ready.set(true);},
            onpointerdown:move |event:PointerEvent| {
                if event.trigger_button()!=Some(MouseButton::Primary) {return;}
                event.prevent_default();event.stop_propagation();
                let pointer=event.pointer_id();
                drag.set(Some(Drag {pointer,coordinate:if vertical {event.client_coordinates().y} else {event.client_coordinates().x},size}));
                let element=context.mounted.peek().clone();
                let focus=handle.peek().clone();
                spawn(async move {
                    if let Some(handle)=focus {let _=handle.set_focus(true).await;}
                    if let Some(element)=element
                        && let Ok(rect)=element.get_client_rect().await
                        && drag.peek().is_some_and(|d|d.pointer==pointer)
                    {
                        let extent=if vertical {rect.size.height} else {rect.size.width};
                        maximum.set((extent*fraction).max(minimum));
                    }
                });
            },
            onpointermove:move |event:PointerEvent| {
                if let Some(start)=*drag.peek() {
                    if start.pointer!=event.pointer_id() {return;}
                    event.prevent_default();event.stop_propagation();
                    let coordinate=if vertical {event.client_coordinates().y} else {event.client_coordinates().x};
                    on_change.call(bounded(start.size+start.coordinate-coordinate,minimum,maximum()));
                }
            },
            onpointerup:move |_|drag.set(None),
            onpointercancel:move |_|drag.set(None),
            onpointerleave:move |_|drag.set(None),
            onkeydown:move |event:KeyboardEvent| {
                let step=if event.modifiers().shift() {48.0} else {16.0};
                let next=match event.key() {
                    Key::ArrowLeft if !vertical=>size+step,
                    Key::ArrowRight if !vertical=>size-step,
                    Key::ArrowUp if vertical=>size+step,
                    Key::ArrowDown if vertical=>size-step,
                    Key::Home=>minimum,
                    Key::End=>maximum(),
                    Key::Escape=>{drag.set(None);return;},
                    _=>return,
                };
                event.prevent_default();event.stop_propagation();
                on_change.call(bounded(next,minimum,maximum()));
            },
            span {class:Styles::grip.inner,"aria-hidden":"true", "⋮"}
            if drag().is_some() {div {class:Styles::shield.inner,"data-resize-overlay":"","aria-hidden":"true"}}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bounded;
    #[test]
    fn size_bounds_reject_non_finite_values_and_clamp_drag_overshoot() {
        assert_eq!(bounded(900.0, 240.0, 600.0), 600.0);
        assert_eq!(bounded(-10.0, 240.0, 600.0), 240.0);
        assert_eq!(bounded(f64::NAN, 240.0, 600.0), 240.0);
        assert_eq!(bounded(350.6, 240.0, 600.0), 351.0);
        assert_eq!(bounded(350.0, 240.0, 100.0), 240.0);
    }
}
