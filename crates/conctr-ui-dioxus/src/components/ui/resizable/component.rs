use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus_icons::lucide::GripVertical;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::rc::Rc;

#[css_module("/src/components/ui/resizable/style.css")]
struct Styles;
const STYLES: Asset = asset!(
    "/src/components/ui/resizable/style.css",
    manganis::AssetOptions::css_module()
);

/// Direction in which the panel sizes change.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum ResizableOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl ResizableOrientation {
    fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Layout {
    sizes: Vec<f64>,
    minimums: Vec<f64>,
}
impl Layout {
    fn new(mut sizes: Vec<f64>, minimums: &[f64]) -> Self {
        if sizes.len() < 2 {
            sizes = vec![50.0, 50.0];
        }
        for size in &mut sizes {
            if !size.is_finite() || *size <= 0.0 {
                *size = 1.0;
            }
        }
        let scale = sizes.iter().copied().fold(1.0_f64, f64::max);
        for size in &mut sizes {
            *size /= scale;
        }
        let total: f64 = sizes.iter().sum();
        for size in &mut sizes {
            *size = *size / total * 100.0;
        }
        let mut minimums: Vec<_> = (0..sizes.len())
            .map(|i| {
                let value = minimums.get(i).copied().unwrap_or(10.0);
                if value.is_finite() {
                    value.clamp(0.0, 100.0)
                } else {
                    10.0
                }
            })
            .collect();
        let minimum_total: f64 = minimums.iter().sum();
        if minimum_total > 100.0 {
            for min in &mut minimums {
                *min = *min / minimum_total * 100.0;
            }
        }
        let deficit: f64 = sizes
            .iter()
            .zip(&minimums)
            .map(|(size, min)| (min - size).max(0.0))
            .sum();
        let slack: f64 = sizes
            .iter()
            .zip(&minimums)
            .map(|(size, min)| (size - min).max(0.0))
            .sum();
        for (size, min) in sizes.iter_mut().zip(&minimums) {
            if *size < *min {
                *size = *min;
            } else if slack > 0.0 {
                *size -= deficit * (*size - *min) / slack;
            }
        }
        Self { sizes, minimums }
    }
    fn bounds(&self, index: usize) -> Option<(f64, f64)> {
        let left = *self.sizes.get(index)?;
        let right = *self.sizes.get(index + 1)?;
        Some((
            self.minimums[index],
            left + right - self.minimums[index + 1],
        ))
    }
    fn resize(&mut self, index: usize, value: f64) -> bool {
        let Some((min, max)) = self.bounds(index) else {
            return false;
        };
        if !value.is_finite() {
            return false;
        }
        let total = self.sizes[index] + self.sizes[index + 1];
        let value = value.clamp(min, max.max(min));
        if (value - self.sizes[index]).abs() < 0.0001 {
            return false;
        }
        self.sizes[index] = value;
        self.sizes[index + 1] = total - value;
        true
    }
}

#[derive(Clone, Copy)]
struct Drag {
    index: usize,
    pointer: i32,
    coordinate: f64,
    extent: f64,
    size: f64,
}
#[derive(Clone)]
struct ResizeContext {
    id: String,
    orientation: ReadSignal<ResizableOrientation>,
    rtl: ReadSignal<bool>,
    layout: Signal<Layout>,
    mounted: Signal<Option<Rc<MountedData>>>,
    drag: Signal<Option<Drag>>,
    on_layout_change: Option<EventHandler<Vec<f64>>>,
}
impl ResizeContext {
    fn update(&mut self, index: usize, value: f64) {
        let mut next = self.layout.peek().clone();
        if next.resize(index, value) {
            let sizes = next.sizes.clone();
            self.layout.set(next);
            if let Some(callback) = self.on_layout_change {
                callback.call(sizes);
            }
        }
    }
}

/// Uncontrolled panel layout. Percentages and minimums are supplied in panel order.
/// `id` is required for stable SSR/ARIA IDs. Use Panel indices 0..n and Handle
/// indices 0..n-1; nested groups each own an independent context.
#[component]
pub fn ResizablePanelGroup(
    id: String,
    #[props(default = ReadSignal::new(Signal::new(ResizableOrientation::Horizontal)))] orientation: ReadSignal<ResizableOrientation>,
    #[props(default = ReadSignal::new(Signal::new(false)))] rtl: ReadSignal<bool>,
    #[props(default = vec![50.0, 50.0])] default_sizes: Vec<f64>,
    #[props(default = vec![10.0, 10.0])] min_sizes: Vec<f64>,
    #[props(default)] on_layout_change: Option<EventHandler<Vec<f64>>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let layout = use_signal(|| Layout::new(default_sizes, &min_sizes));
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let drag = use_signal(|| None::<Drag>);
    use_context_provider(|| ResizeContext {
        id: id.clone(),
        orientation,
        rtl,
        layout,
        mounted,
        drag,
        on_layout_change,
    });
    let base = attributes!(div { class: "cn-resizable {Styles::group.inner} flex min-w-0 min-h-0 overflow-hidden", "data-orientation": orientation().name(), "data-dragging": drag().is_some() });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            id,
            dir: if rtl() { "rtl" } else { "ltr" },
            onmounted: move |event| mounted.set(Some(event.data())),
            ..merged,
            {children}
        }
    }
}

/// A panel consumes the corresponding percentage of available group space.
#[component]
pub fn ResizablePanel(
    index: usize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ResizeContext>();
    let Some(size) = (context.layout)().sizes.get(index).copied() else {
        return VNode::empty();
    };
    let base = attributes!(div {
        class: "cn-resizable-panel {Styles::panel.inner} min-w-0 min-h-0 overflow-auto",
        style: format!("flex: {size} 1 0%;")
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            id: "{context.id}-panel-{index}",
            "data-panel-index": index,
            ..merged,
            {children}
        }
    }
}

/// Focusable splitter. Arrows resize by 1%; Shift by 10%; Home/End use bounds.
/// Pointer capture keeps dragging active beyond the group's bounds.
#[component]
pub fn ResizableHandle(
    index: usize,
    #[props(default)] with_handle: bool,
    #[props(default)] disabled: bool,
    #[props(default = "Resize panels".to_string())] label: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let context = use_context::<ResizeContext>();
    let layout = (context.layout)();
    let Some((min, max)) = layout.bounds(index) else {
        return VNode::empty();
    };
    let size = layout.sizes[index];
    let id = format!("{}-handle-{index}", context.id);
    let controls = format!("{}-panel-{index}", context.id);
    let vertical = (context.orientation)() == ResizableOrientation::Vertical;
    let mut keys = context.clone();
    let down = context.clone();
    let mut moving = context.clone();
    let mut release = context.clone();
    let mut cancel = context.clone();
    let mut lost_capture = context;
    let base = attributes!(div { class: "cn-resizable-handle {Styles::handle.inner} relative flex items-center justify-center shrink-0", "data-disabled": disabled });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            id: id.clone(),
            role: "separator",
            tabindex: if disabled { -1 } else { 0 },
            aria_label: label,
            aria_controls: controls,
            aria_disabled: disabled,
            aria_orientation: if vertical { "horizontal" } else { "vertical" },
            aria_valuemin: min,
            aria_valuemax: max,
            aria_valuenow: (size * 10.0).round() / 10.0,
            aria_valuetext: format!("{size:.0}%"),
            onkeydown: move |event: KeyboardEvent| {
                if disabled {
                    return;
                }
                let step = if event.modifiers().shift() { 10.0 } else { 1.0 };
                let current = keys.layout.peek().sizes[index];
                let direction = if (keys.rtl)() && !vertical { -1.0 } else { 1.0 };
                let next = match event.key() {
                    Key::ArrowLeft if !vertical => current - step * direction,
                    Key::ArrowRight if !vertical => current + step * direction,
                    Key::ArrowUp if vertical => current - step,
                    Key::ArrowDown if vertical => current + step,
                    Key::Home => min,
                    Key::End => max,
                    _ => return,
                };
                event.prevent_default();
                event.stop_propagation();
                keys.update(index, next);
            },
            onpointerdown: move |event: PointerEvent| {
                if !disabled && event.trigger_button() == Some(MouseButton::Primary) {
                    event.prevent_default();
                    event.stop_propagation();
                }
                let mut context = down.clone();
                let id = id.clone();
                async move {
                    if disabled || event.trigger_button() != Some(MouseButton::Primary) {
                        return;
                    }
                    event.prevent_default();
                    event.stop_propagation();
                    let Some(element) = context.mounted.peek().clone() else {
                        return;
                    };
                    let pointer = event.pointer_id();
                    let coordinate = if vertical {
                        event.client_coordinates().y
                    } else {
                        event.client_coordinates().x
                    };
                    let size = context.layout.peek().sizes[index];
                    context
                        .drag
                        .set(
                            Some(Drag {
                                index,
                                pointer,
                                coordinate,
                                extent: 0.0,
                                size,
                            }),
                        );
                    // Awaited evals need a JSON result; undefined would end the drag as an error.
                    let capture = document::eval(
                        "const [id, pointer] = await dioxus.recv(); document.getElementById(id)?.setPointerCapture(pointer); return null;",
                    );
                    if capture.send((&id, pointer)).is_err() || capture.await.is_err() {
                        context.drag.set(None);
                        return;
                    }
                    if let Ok(rect) = element.get_client_rect().await {
                        let extent = if vertical { rect.size.height } else { rect.size.width };
                        let still_pressed = context
                            .drag
                            .peek()
                            .is_some_and(|drag| drag.pointer == pointer);
                        if still_pressed && extent > 0.0 {
                            context
                                .drag
                                .set(
                                    Some(Drag {
                                        index,
                                        pointer,
                                        coordinate,
                                        extent,
                                        size,
                                    }),
                                );
                        }
                    }
                }
            },
            onpointermove: move |event: PointerEvent| {
                if disabled {
                    return;
                }
                let drag = *moving.drag.peek();
                if let Some(drag) = drag && drag.index == index
                    && drag.pointer == event.pointer_id() && drag.extent > 0.0
                {
                    let coordinate = if vertical {
                        event.client_coordinates().y
                    } else {
                        event.client_coordinates().x
                    };
                    let direction = if (moving.rtl)() && !vertical { -1.0 } else { 1.0 };
                    moving
                        .update(
                            index,
                            drag.size
                                + (coordinate - drag.coordinate) / drag.extent * 100.0
                                    * direction,
                        );
                }
            },
            onpointerup: move |_| release.drag.set(None),
            onpointercancel: move |_| cancel.drag.set(None),
            onlostpointercapture: move |_| lost_capture.drag.set(None),
            ..merged,
            if with_handle {
                span {
                    class: "cn-resizable-grip {Styles::grip.inner} flex items-center justify-center",
                    aria_hidden: "true",
                    GripVertical { size: "12" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resizing_preserves_pair_total_and_other_panels_and_clamps_bounds() {
        let mut layout = Layout::new(vec![30.0, 40.0, 30.0], &[20.0, 15.0, 10.0]);
        assert!(layout.resize(0, 90.0));
        assert_eq!(layout.sizes, vec![55.0, 15.0, 30.0]);
        assert!(layout.resize(0, -10.0));
        assert_eq!(layout.sizes, vec![20.0, 50.0, 30.0]);
        assert!(!layout.resize(4, 50.0));
        assert!(!layout.resize(0, f64::NAN));
    }
    #[test]
    fn initial_layout_sanitizes_values_and_honors_minimums() {
        let layout = Layout::new(vec![1.0, 99.0], &[30.0, 20.0]);
        assert!((layout.sizes[0] - 30.0).abs() < 0.001);
        assert!((layout.sizes.iter().sum::<f64>() - 100.0).abs() < 0.001);
        for sizes in [
            vec![],
            vec![f64::NAN, -20.0],
            vec![1.0, 1.0],
            vec![f64::MAX, f64::MAX],
        ] {
            let layout = Layout::new(sizes, &[80.0, 80.0]);
            assert!(
                layout
                    .sizes
                    .iter()
                    .all(|value| value.is_finite() && *value >= 0.0)
            );
            assert!((layout.sizes.iter().sum::<f64>() - 100.0).abs() < 0.001);
        }
    }
}
