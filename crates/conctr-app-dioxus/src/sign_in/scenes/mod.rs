//! The illustration is chosen once for each mounted sign-in screen. The initial
//! placeholder is identical in server HTML and on the hydrating client.
mod catalog;

use super::Styles;
use catalog::{SCENES, Scene};
use conctr_ui_dioxus::{
    assets::images,
    ui::{
        button::{Button, ButtonSize, ButtonVariant},
        theme_toggle::BrandThemeToggle,
    },
};
use dioxus::prelude::*;

#[component]
pub(super) fn ScenePanel(dark: bool, on_theme_change: EventHandler<bool>) -> Element {
    let mut selection = use_signal(|| None::<usize>);
    use_effect(move || {
        spawn(async move {
            let script = format!("return Math.floor(Math.random() * {});", SCENES.len());
            let index = document::eval(&script).join::<usize>().await.unwrap_or(0);
            selection.set(Some(index));
        });
    });
    let scene = selection().and_then(|index| SCENES.get(index));
    rsx! {
        aside {
            class: Styles::poster.inner,
            "data-paper": scene.map_or("dots", |s| s.texture),
            style: format!("--ink: var(--cn-{});", scene.map_or("primary", |s| s.accent)),
            div { class: Styles::identity.inner,
                img { src: if dark { images::HORIZONTAL_COLOR_DARK_SVG } else { images::HORIZONTAL_COLOR_SVG }, alt: "Conctr", width: 130, height: 36 }
                span { "Where systems meet." }
                // BrandThemeToggle { compact: true, dark, on_change: on_theme_change }
            }
            if let Some(scene) = scene {
                div { class: Styles::intro.inner,
                    p { class: Styles::eyebrow.inner, "{scene.pattern}" }
                    h2 { "{scene.title}" }
                    p { "{scene.description}" }
                }
                Illustration { scene }
            }
        }
    }
}

#[component]
fn Illustration(scene: &'static Scene) -> Element {
    let mut playback = use_signal(|| None::<document::Eval>);
    let mut paused = use_signal(|| false);
    use_drop(move || {
        if let Some(controller) = *playback.peek() {
            let _ = controller.send("dispose");
        }
    });
    rsx! {
        // Only checked-in SVG illustrations from the catalog enter this node.
        div { id: "sign-in-artwork", class: Styles::art.inner,
            dangerous_inner_html: scene.artwork,
            onmounted: move |_| playback.set(Some(document::eval(include_str!("playback.js")))),
        }
        div { class: Styles::legend.inner,
            span { "{scene.legend}" }
            div { class: Styles::motion_control.inner,
                Button {
                    variant: ButtonVariant::Ghost, size: ButtonSize::Sm,
                    onclick: move |_| {
                        let next = !paused();
                        if let Some(controller) = playback()
                            && controller.send(if next { "pause" } else { "resume" }).is_ok()
                        {
                            paused.set(next);
                        }
                    },
                    if paused() { "Resume motion" } else { "Pause motion" }
                }
            }
        }
    }
}
