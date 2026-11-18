//! Native modal drawers with scoped open/close listeners and no global key handlers.
pub use crate::rust_ui::ui::drawer::*;
use dioxus::prelude::*;

/// Owns one native drawer; listeners are attached to its DOM nodes and collected on unmount.
#[component]
pub fn Drawer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: tw_merge::tw_merge!("cn-drawer-root", class.as_deref().unwrap_or("")),
            "data-name": "Drawer",
            onmounted: move |_| {
                document::eval(
                    r#"
                    document.querySelectorAll('.cn-drawer-root:not([data-ready])').forEach(root => {
                        const dialog = root.querySelector('dialog');
                        const trigger = root.querySelector('[data-name="DrawerTrigger"]');
                        if (!dialog || !trigger) return;
                        root.dataset.ready='true';
                        trigger.addEventListener('click', () => dialog.showModal());
                        dialog.addEventListener('click', event => {
                            if (event.target.closest('[data-name="DrawerClose"]')) dialog.close();
                            if (event.target===dialog && dialog.dataset.dismissible==='true') {
                                const r=dialog.getBoundingClientRect();
                                if(event.clientX<r.left||event.clientX>r.right||event.clientY<r.top||event.clientY>r.bottom)dialog.close();
                            }
                        });
                        dialog.addEventListener('cancel', event => { if(dialog.dataset.dismissible!=='true')event.preventDefault(); });
                    });
                "#,
                );
            },
            {children}
        }
    }
}
#[component]
pub fn DrawerTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            "data-name": "DrawerTrigger",
            "aria-haspopup": "dialog",
            class: tw_merge::tw_merge!("cn-drawer-trigger", class.as_deref().unwrap_or("")),
            {children}
        }
    }
}
/// Slides content from a viewport edge. Native dialogs supply focus containment and Escape.
#[component]
pub fn DrawerContent(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] style: Option<String>,
    #[props(default = true)] dismissible: bool,
    #[props(default)] position: DrawerPosition,
    #[props(default)] variant: DrawerVariant,
    children: Element,
) -> Element {
    let side = match position {
        DrawerPosition::Bottom => "bottom",
        DrawerPosition::Top => "top",
        DrawerPosition::Left => "left",
        DrawerPosition::Right => "right",
    };
    rsx! {
        dialog {
            class: tw_merge::tw_merge!("cn-drawer-surface", class.as_deref().unwrap_or("")),
            "data-name": "DrawerContent",
            "aria-label": "Drawer",
            "data-position": side,
            "data-floating": (variant == DrawerVariant::Floating).to_string(),
            "data-dismissible": dismissible.to_string(),
            style: style.unwrap_or_default(),
            {children}
        }
    }
}
#[component]
pub fn DrawerClose(
    #[props(into, optional)] class: Option<String>,
    #[props(default = crate::ui::button::ButtonVariant::Outline)]
    variant: crate::ui::button::ButtonVariant,
    #[props(default)] size: crate::ui::button::ButtonSize,
    #[props(default = "Close drawer".into())] aria_label: String,
    children: Element,
) -> Element {
    let tone = match variant {
        crate::ui::button::ButtonVariant::Destructive => "danger",
        crate::ui::button::ButtonVariant::Ghost => "ghost",
        crate::ui::button::ButtonVariant::Default => "primary",
        _ => "secondary",
    };
    let dimensions = match size {
        crate::ui::button::ButtonSize::Sm | crate::ui::button::ButtonSize::IconSm => "small",
        crate::ui::button::ButtonSize::IconXs => "extra-small",
        crate::ui::button::ButtonSize::Lg => "large",
        _ => "default",
    };
    rsx! {
        button {
            r#type: "button",
            class: tw_merge::tw_merge!("cn-drawer-trigger", class.as_deref().unwrap_or("")),
            "data-name": "DrawerClose", "data-variant": tone, "data-size": dimensions,
            "aria-label": aria_label,
            {children}
        }
    }
}
