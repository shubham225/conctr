use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut layout = use_signal(|| vec![30.0, 70.0]);
    rsx! {
        Stack { gap: Space::Lg,
            ResizablePanelGroup {
                id: "workspace-panels",
                default_sizes: vec![30.0, 70.0],
                min_sizes: vec![20.0, 30.0],
                style: "height: 280px;",
                on_layout_change: move |sizes| layout.set(sizes),
                ResizablePanel { index: 0,
                    LayoutBox { padding: Space::Xl,
                        Heading { level: HeadingLevel::H4, "Connections" }
                        p { "Sources and targets" }
                    }
                }
                ResizableHandle {
                    index: 0,
                    with_handle: true,
                    label: "Resize connections and workspace",
                }
                ResizablePanel { index: 1,
                    ResizablePanelGroup {
                        id: "detail-panels",
                        orientation: ResizableOrientation::Vertical,
                        default_sizes: vec![65.0, 35.0],
                        min_sizes: vec![25.0, 20.0],
                        ResizablePanel { index: 0,
                            LayoutBox { padding: Space::Xl,
                                Heading { level: HeadingLevel::H4, "Workspace" }
                                p { "Inspect the selected connection." }
                            }
                        }
                        ResizableHandle {
                            index: 0,
                            with_handle: true,
                            label: "Resize workspace and run details",
                        }
                        ResizablePanel { index: 1,
                            LayoutBox { padding: Space::Lg, "Run details" }
                        }
                    }
                }
            }
            p { role: "status",
                "Panels: {layout()[0]:.0}% / {layout()[1]:.0}%. Use arrow keys on a divider to resize."
            }
        }
    }
}
