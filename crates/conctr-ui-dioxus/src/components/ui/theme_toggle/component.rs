use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Moon, Sun};
use std::rc::Rc;

#[css_module("/src/components/ui/theme_toggle/style.css")]
struct Styles;

// Register for each document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/theme_toggle/style.css",
    manganis::AssetOptions::css_module()
);

/// Icon button that requests a light/dark theme change without owning the provider.
/// `dark` reflects the caller's current theme; `on_change` receives the next value.
/// Both themes expand from the button using view transitions.
/// Linux native WebKitGTK, reduced motion and unsupported WebViews use an immediate update.
/// Native button semantics support Enter/Space, focus and disabled state.
#[component]
pub fn ThemeToggle(
    dark: bool,
    on_change: EventHandler<bool>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut changing = use_signal(|| false);
    let label = if dark {
        "Switch to light theme"
    } else {
        "Switch to dark theme"
    };
    rsx! {
        document::Stylesheet { href: STYLES }
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconLg,
            disabled,
            aria_label: label,
            title: label,
            onclick: move |_| {
                if disabled || changing() { return; }
                // Linux desktop uses WebKitGTK. Avoid its native view-transition
                // crash; the sun/moon CSS animation still runs on theme changes.
                if cfg!(target_os = "linux") {
                    on_change.call(!dark);
                    return;
                }
                changing.set(true);
                let element = mounted.peek().clone();
                spawn(async move {
                    let rect = if let Some(element) = element {
                        element.get_client_rect().await.ok()
                    } else { None };
                    if let Some(rect) = rect {
                        let center = rect.center();
                        let mut eval = document::eval(include_str!("reveal.js"));
                        if eval.send((center.x, center.y, !dark)).is_ok() {
                            match eval.recv::<bool>().await {
                                Ok(true) => {
                                    on_change.call(!dark);
                                    // Keep the eval alive until its transition cleans up.
                                    let _ = eval.recv::<bool>().await;
                                }
                                Ok(false) => {} // Another toggle owns the document transition.
                                Err(_) => on_change.call(!dark),
                            }
                        } else {
                            on_change.call(!dark);
                        }
                    } else {
                        on_change.call(!dark);
                    }
                    changing.set(false);
                });
            },
            attributes,
            span {
                class: "cn-theme-toggle {Styles::toggle.inner} relative inline-grid h-5 w-5",
                "data-dark": dark.to_string(),
                aria_hidden: "true",
                onmounted: move |event| mounted.set(Some(event.data())),
                span { class: "{Styles::sun.inner} absolute inset-0", Sun { size: "20" } }
                span { class: "{Styles::moon.inner} absolute inset-0", Moon { size: "20" } }
            }
        }
    }
}
