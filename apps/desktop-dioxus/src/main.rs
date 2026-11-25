#[cfg(not(any(feature = "desktop", feature = "server")))]
use conctr_app_dioxus::App;
#[cfg(feature = "desktop")]
use dioxus::prelude::*;

fn main() -> Result<(), String> {
    #[cfg(feature = "server")]
    return conctr_app_dioxus::serve_workspace();
    #[cfg(not(feature = "server"))]
    {
        #[cfg(all(feature = "desktop", not(feature = "server")))]
        conctr_app_dioxus::configure_native_api()?;

        #[cfg(feature = "desktop")]
        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                dioxus::desktop::Config::new().with_menu(None).with_window(
                    dioxus::desktop::tao::window::WindowBuilder::new()
                        .with_title("Conctr")
                        .with_window_icon(Some(window_icon()?))
                        .with_decorations(false)
                        .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1200.0, 800.0))
                        .with_min_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(
                            480.0, 400.0,
                        )),
                ),
            )
            .launch(DesktopApp);

        #[cfg(not(any(feature = "desktop", feature = "server")))]
        dioxus::launch(App);
        Ok(())
    }
}

#[cfg(all(feature = "desktop", not(feature = "server")))]
fn window_icon() -> Result<dioxus::desktop::tao::window::Icon, String> {
    let png = include_bytes!("../../../crates/conctr-ui-dioxus/assets/logo/png/app-icon-1024.png");
    let rgba = image::load_from_memory_with_format(png, image::ImageFormat::Png)
        .map_err(|error| format!("Could not decode the Conctr window icon: {error}"))?
        .into_rgba8();
    let (width, height) = rgba.dimensions();
    dioxus::desktop::tao::window::Icon::from_rgba(rgba.into_raw(), width, height)
        .map_err(|error| format!("Could not create the Conctr window icon: {error}"))
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
    let mut drag_start = use_signal(|| None::<dioxus::html::geometry::ScreenPoint>);
    rsx! {
        div { class: "flex h-10 shrink-0 items-center border-b border-border bg-background",
            div {
                class: "flex h-full min-w-0 flex-1 select-none items-center px-4 text-muted-foreground",
                onmousedown: move |event| {
                    if event.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
                        drag_start.set(Some(event.screen_coordinates()));
                    }
                },
                onmousemove: move |event| {
                    if !event.held_buttons().contains(dioxus::html::input_data::MouseButton::Primary) {
                        drag_start.set(None);
                        return;
                    }
                    if let Some(start) = drag_start() {
                        let delta = event.screen_coordinates() - start;
                        if delta.square_length() >= 16.0 {
                            drag_start.set(None);
                            drag.drag();
                        }
                    }
                },
                onmouseup: move |_| drag_start.set(None),
                onmouseleave: move |_| drag_start.set(None),
                ondoubleclick: move |_| maximize.toggle_maximized(),
                p { class: "truncate text-xs text-muted-foreground",
                    span { class: "font-semibold tracking-wide text-foreground", "CONCTR" }
                    " - where systems meet"
                }
            }
            div { class: "flex shrink-0",
                button {
                    class: "group flex h-10 w-12 items-center justify-center hover:bg-muted focus-visible:outline-2 focus-visible:outline-ring cursor-pointer",
                    aria_label: "Minimize window",
                    title: "Minimize",
                    onclick: move |_| minimize.set_minimized(true),
                    icons::Minus { class: "size-4 motion-safe:transition-transform motion-safe:duration-120 motion-safe:ease-out motion-safe:group-hover:scale-110 motion-safe:group-active:scale-90" }
                }
                button {
                    class: "group flex h-10 w-12 items-center justify-center hover:bg-muted focus-visible:outline-2 focus-visible:outline-ring cursor-pointer",
                    aria_label: "Maximize or restore window",
                    title: "Maximize or restore",
                    onclick: move |_| toggle.toggle_maximized(),
                    icons::Maximize2 { class: "size-4 motion-safe:transition-transform motion-safe:duration-120 motion-safe:ease-out motion-safe:group-hover:scale-110 motion-safe:group-active:scale-90" }
                }
                button {
                    class: "group flex h-10 w-12 items-center justify-center hover:bg-destructive hover:text-destructive-foreground focus-visible:outline-2 focus-visible:outline-ring cursor-pointer",
                    aria_label: "Close window",
                    title: "Close",
                    onclick: move |_| window.close(),
                    icons::X { class: "size-4 motion-safe:transition-transform motion-safe:duration-120 motion-safe:ease-out motion-safe:group-hover:scale-110 motion-safe:group-active:scale-90" }
                }
            }
        }
    }
}
