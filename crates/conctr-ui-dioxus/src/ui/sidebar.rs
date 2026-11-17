//! Responsive sidebar with a shadcn-style icon rail and a native mobile drawer.
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct SidebarState {
    collapsed: Signal<bool>,
}

/// Shares sidebar state with a trigger placed in the adjacent main content.
/// Wrap the Sidebar and main content together; providers are independent.
#[component]
pub fn SidebarProvider(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let collapsed = use_signal(|| false);
    use_context_provider(|| SidebarState { collapsed });
    rsx! {
        div {
            class: tw_merge::tw_merge!("cn-sidebar-provider", class.as_deref().unwrap_or("")),
            onmounted: move |_| {
                document::eval(
                    r#"
                            document.querySelectorAll('.cn-sidebar-provider:not([data-ready])').forEach(root => {
                                const trigger = root.querySelector('[data-sidebar-trigger]');
                                const drawer = root.querySelector('.cn-sidebar-drawer');
                                if (!trigger || !drawer) return;
                                root.dataset.ready = 'true';
                                trigger.addEventListener('click', () => { drawer.showModal(); trigger.setAttribute('aria-expanded', 'true'); });
                                drawer.addEventListener('close', () => trigger.setAttribute('aria-expanded', 'false'));
                                drawer.addEventListener('click', event => {
                                    if (event.target.closest('[data-sidebar-close], a[href]')) drawer.close();
                                    if (event.target === drawer) { const r = drawer.getBoundingClientRect(); if (event.clientX < r.left || event.clientX > r.right || event.clientY < r.top || event.clientY > r.bottom) drawer.close(); }
                                });
                            });
                        "#,
                );
            },
            {children}
        }
    }
}

/// Place in the main-content header. Toggles the desktop rail or opens the mobile drawer.
#[component]
pub fn SidebarTrigger() -> Element {
    let SidebarState { mut collapsed } = use_context::<SidebarState>();
    rsx! {
        button {
            r#type: "button",
            class: "cn-sidebar-toggle cn-sidebar-desktop-trigger",
            title: if collapsed() { "Expand sidebar" } else { "Collapse sidebar" },
            "aria-label": "Toggle sidebar",
            "aria-expanded": (!collapsed()).to_string(),
            onclick: move |_| collapsed.set(!collapsed()),
            icons::PanelLeft { class: "size-5" }
        }
        button {
            r#type: "button",
            class: "cn-sidebar-mobile-trigger",
            "data-sidebar-trigger": "",
            "aria-haspopup": "dialog",
            "aria-expanded": "false",
            "aria-label": "Open navigation",
            icons::PanelLeft { class: "size-5" }
        }
    }
}

/// Collapses to a 56 px icon rail on desktop, with a native mobile drawer.
/// Use inside SidebarProvider alongside a main-content SidebarTrigger.
#[component]
pub fn Sidebar(
    #[props(default = "Main navigation".into())] label: String,
    #[props(default = VNode::empty())] header: Element,
    #[props(default = VNode::empty())] footer: Element,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let SidebarState { collapsed } = use_context::<SidebarState>();
    rsx! {
        aside {
            class: tw_merge::tw_merge!("cn-sidebar", class.as_deref().unwrap_or("")),
            "data-collapsed": collapsed().to_string(),
            div { class: "cn-sidebar-desktop",
                div { class: "cn-sidebar-head",
                    div { class: "cn-sidebar-brand", {header.clone()} }
                }
                nav { class: "cn-sidebar-content", "aria-label": label.clone(), {children.clone()} }
                div { class: "cn-sidebar-footer", {footer.clone()} }
            }
            dialog { class: "cn-sidebar-drawer", "aria-label": label,
                div { class: "cn-sidebar-mobile-content",
                    div { class: "cn-sidebar-head",
                        {header}
                        button {
                            r#type: "button",
                            class: "cn-sidebar-toggle",
                            "data-sidebar-close": "",
                            "aria-label": "Close navigation",
                            icons::X { class: "size-4" }
                        }
                    }
                    nav {
                        class: "cn-sidebar-content",
                        "aria-label": "Mobile navigation",
                        {children}
                    }
                    div { class: "cn-sidebar-footer", {footer} }
                }
            }
        }
    }
}

/// An independently collapsible navigation group.
#[component]
pub fn SidebarGroup(title: String, children: Element) -> Element {
    rsx! {
        details { class: "cn-sidebar-group", open: true,
            summary { class: "cn-sidebar-group-label",
                span { "{title}" }
                icons::ChevronRight { class: "size-3" }
            }
            div { class: "cn-sidebar-menu", {children} }
        }
    }
}

/// Caller-selected navigation item. Titles label the icon rail without relying on hover.
#[component]
pub fn SidebarItem(
    label: String,
    href: String,
    #[props(default)] active: bool,
    #[props(default)] disabled: bool,
    #[props(default = VNode::empty())] icon: Element,
) -> Element {
    rsx! {
        if disabled {
            span {
                class: "cn-sidebar-item",
                "aria-disabled": "true",
                title: label.clone(),
                {icon}
                span { "{label}" }
            }
        } else {
            a {
                class: "cn-sidebar-item",
                href,
                title: label.clone(),
                "aria-label": label.clone(),
                "aria-current": if active { Some("page") } else { None },
                {icon}
                span { "{label}" }
            }
        }
    }
}
