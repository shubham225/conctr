#[cfg(not(feature = "desktop"))]
use conctr_app_dioxus::App;
#[cfg(feature = "desktop")]
use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new().with_menu(None).with_window(
                dioxus::desktop::tao::window::WindowBuilder::new()
                    .with_title("Conctr")
                    .with_decorations(false)
                    .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1200.0, 800.0))
                    .with_min_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(480.0, 400.0)),
            ),
        )
        .launch(DesktopApp);

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[cfg(feature = "desktop")]
#[component]
fn DesktopApp() -> Element {
    rsx! { conctr_app_dioxus::AppWithTitlebar { window_titlebar: rsx! { WindowTitlebar {} } } }
}

/// Actual OS operations remain in the native adapter.
#[cfg(feature = "desktop")]
#[component]
fn WindowTitlebar() -> Element {
    let window = dioxus::desktop::use_window();
    let drag = window.clone();
    let maximize = window.clone();
    let minimize = window.clone();
    let toggle = window.clone();
    rsx! {
        div { class: "flex h-10 shrink-0 items-center border-b border-border bg-background",
            div {
                class: "flex h-full min-w-0 flex-1 select-none items-center px-4 text-xs text-muted-foreground",
                onmousedown: move |event| {
                    if event.data().trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) { drag.drag(); }
                },
                ondoubleclick: move |_| maximize.toggle_maximized(),
                "Conctr"
            }
            button { class: "h-10 w-12 hover:bg-accent focus-visible:outline-2 focus-visible:outline-ring", aria_label: "Minimize window", title: "Minimize", onclick: move |_| minimize.set_minimized(true), "−" }
            button { class: "h-10 w-12 hover:bg-accent focus-visible:outline-2 focus-visible:outline-ring", aria_label: "Maximize or restore window", title: "Maximize or restore", onclick: move |_| toggle.toggle_maximized(), "□" }
            button { class: "h-10 w-12 hover:bg-destructive hover:text-destructive-foreground focus-visible:outline-2 focus-visible:outline-ring", aria_label: "Close window", title: "Close", onclick: move |_| window.close(), "×" }
        }
    }
}
