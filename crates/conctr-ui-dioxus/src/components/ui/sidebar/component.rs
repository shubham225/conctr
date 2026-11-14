use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::separator::Separator;
use crate::components::ui::sheet::{
    Sheet, SheetContentClose, SheetDescription, SheetHeader, SheetSide, SheetTitle,
};
use crate::components::ui::skeleton::Skeleton;
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use dioxus::core::use_drop;
use dioxus::prelude::*;
use dioxus_icons::lucide::PanelLeft;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::use_controlled;

#[css_module("/src/components/ui/sidebar/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/sidebar/style.css",
    manganis::AssetOptions::css_module()
);

// constants
const SIDEBAR_WIDTH: &str = "240px";
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";
const MOBILE_BREAKPOINT: u32 = 1024;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
}

impl SidebarState {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarState::Expanded => "expanded",
            SidebarState::Collapsed => "collapsed",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

impl SidebarSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

impl SidebarCollapsible {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct SidebarCtx {
    pub state: Memo<SidebarState>,
    pub side: Signal<SidebarSide>,
    pub is_mobile: Signal<bool>,
    // From use_controlled:
    open: Memo<bool>,
    set_open: Callback<bool>,
    // Mobile state:
    open_mobile: Signal<bool>,
}

impl SidebarCtx {
    /// Toggle the sidebar open/closed state
    pub fn toggle(&self) {
        if (self.is_mobile)() {
            let current = (self.open_mobile)();
            let mut open_mobile = self.open_mobile;
            open_mobile.set(!current);
        } else {
            self.set_open.call(!self.open());
        }
    }

    /// Set the mobile sidebar open state
    pub fn set_open_mobile(&self, value: bool) {
        let mut open_mobile = self.open_mobile;
        open_mobile.set(value);
    }

    /// Get the current open state (desktop)
    pub fn open(&self) -> bool {
        self.open.cloned()
    }
}

pub fn use_sidebar() -> SidebarCtx {
    use_context::<SidebarCtx>()
}

pub fn use_is_mobile() -> Signal<bool> {
    let mut is_mobile = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            let js_code = format!(
                r#"
                function checkMobile() {{
                    return window.innerWidth < {MOBILE_BREAKPOINT};
                }}
                function handleResize() {{
                    dioxus.send(checkMobile());
                }}
                window.__sidebarResizeHandler = handleResize;
                window.addEventListener('resize', window.__sidebarResizeHandler);
                dioxus.send(checkMobile());
                "#
            );
            let mut eval = document::eval(&js_code);

            while let Ok(result) = eval.recv::<bool>().await {
                is_mobile.set(result);
            }
        });
    });

    use_drop(|| {
        _ = document::eval(
            r#"
            window.removeEventListener('resize', window.__sidebarResizeHandler);
            delete window.__sidebarResizeHandler;
            "#,
        );
    });

    is_mobile
}

#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let is_mobile = use_is_mobile();
    let side = use_signal(|| SidebarSide::Left);
    let open_mobile = use_signal(|| false);

    let (open, set_open) = use_controlled(open, default_open, on_open_change);

    let state = use_memo(move || {
        if open() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    });

    let ctx = SidebarCtx {
        state,
        side,
        is_mobile,
        open,
        set_open,
        open_mobile,
    };

    use_context_provider(|| ctx);

    use_effect(move || {
        spawn(async move {
            let js_code = format!(
                r#"
                function sidebarKeyHandler(event) {{
                    if (event.key === '{SIDEBAR_KEYBOARD_SHORTCUT}' && (event.metaKey || event.ctrlKey)) {{
                        event.preventDefault();
                        dioxus.send(true);
                    }}
                }}
                window.__sidebarKeyHandler = sidebarKeyHandler;
                window.addEventListener('keydown', window.__sidebarKeyHandler);
                "#
            );
            let mut eval = document::eval(&js_code);

            while let Ok(true) = eval.recv::<bool>().await {
                ctx.toggle();
            }
        });
    });

    use_drop(|| {
        _ = document::eval(
            r#"
            window.removeEventListener('keydown', window.__sidebarKeyHandler);
            delete window.__sidebarKeyHandler;
            "#,
        );
    });

    let sidebar_style = format!(
        r#"--dx-sidebar-width: {SIDEBAR_WIDTH}; --dx-sidebar-width-mobile: {SIDEBAR_WIDTH_MOBILE}; --dx-sidebar-width-icon: {SIDEBAR_WIDTH_ICON}"#
    );

    let base = attributes!(div {
        class: "cn-ui-sidebar-wrapper {Styles::cn_ui_sidebar_wrapper.inner} flex w-full overflow-hidden h-svh min-h-svh",
        "data-slot": "sidebar-wrapper",
        style: sidebar_style,
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn Sidebar(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapsible,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_sidebar();
    let mut ctx_side = ctx.side;
    if *ctx_side.peek() != side {
        ctx_side.set(side);
    }

    let is_mobile = ctx.is_mobile;
    let state = ctx.state;
    let open_mobile = ctx.open_mobile;

    if collapsible == SidebarCollapsible::None {
        let base = attributes!(div {
            class: "cn-ui-sidebar-static flex flex-col h-full bg-sidebar text-sidebar-foreground w-(--dx-sidebar-width)",
            "data-slot": "sidebar",
        });
        let merged = merge_attributes(vec![base, attributes]);

        return rsx! {
            document::Stylesheet { href: STYLES }

            div { ..merged,{children} }
        };
    }

    if is_mobile() {
        let sheet_side = match side {
            SidebarSide::Left => SheetSide::Left,
            SidebarSide::Right => SheetSide::Right,
        };

        return rsx! {
            document::Stylesheet { href: STYLES }

            Sheet {
                open: open_mobile(),
                on_open_change: move |v| ctx.set_open_mobile(v),
                "data-side": sheet_side.as_str(),
                class: format!("cn-ui-sidebar-sheet {} bg-sidebar", Styles::cn_ui_sidebar_sheet.inner),
                "data-sidebar": "sidebar",
                "data-slot": "sidebar",
                "data-mobile": "true",
                SheetContentClose { class: "cn-ui-sidebar-sheet-close {Styles::cn_ui_sidebar_sheet_close.inner}" }
                SheetHeader { class: "sr-only",
                    SheetTitle { "Sidebar" }
                    SheetDescription { "Displays the mobile sidebar." }
                }
                div { class: "cn-ui-sidebar-mobile-inner flex flex-col w-full h-full", {children} }
            }
        };
    }

    let collapsible_str = if state() == SidebarState::Collapsed {
        collapsible.as_str()
    } else {
        ""
    };

    let container_base = attributes!(div {
        class: "cn-ui-sidebar-container {Styles::cn_ui_sidebar_container.inner} box-border top-0 fixed bottom-0 h-svh",
        "data-slot": "sidebar-container",
    });
    let container_attrs = merge_attributes(vec![container_base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-sidebar-desktop {Styles::cn_ui_sidebar_desktop.inner} text-sidebar-foreground",
            "data-state": state().as_str(),
            "data-collapsible": collapsible_str,
            "data-variant": variant.as_str(),
            "data-side": side.as_str(),
            "data-slot": "sidebar",
            div { class: "cn-ui-sidebar-gap {Styles::cn_ui_sidebar_gap.inner} relative bg-transparent", "data-slot": "sidebar-gap" }
            div {..container_attrs,
                div {
                    class: "cn-ui-sidebar-inner {Styles::cn_ui_sidebar_inner.inner} flex flex-col w-full box-border h-full bg-sidebar",
                    "data-sidebar": "sidebar",
                    "data-slot": "sidebar-inner",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn SidebarTrigger(
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_sidebar();

    let base = attributes!(button {
        class: "cn-ui-sidebar-trigger {Styles::cn_ui_sidebar_trigger.inner} items-center inline-flex justify-center w-7 h-7 w-(--cn-control-sm) h-(--cn-control-sm)",
        "data-sidebar": "trigger",
        "data-slot": "sidebar-trigger",
        aria_expanded: if (ctx.is_mobile)() { (ctx.open_mobile)() } else { ctx.open() },
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconSm,
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
                ctx.toggle();
            },
            attributes: merged,
            PanelLeft { class: "cn-ui-sidebar-trigger-icon w-4 h-4", size: "1rem" }
            span { class: "sr-only", "Toggle sidebar" }
        }
    }
}

#[component]
pub fn SidebarRail(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let ctx = use_sidebar();

    let base = attributes!(button {
        class: "cn-ui-sidebar-rail {Styles::cn_ui_sidebar_rail.inner} border-0 p-0",
        "data-sidebar": "rail",
        "data-slot": "sidebar-rail",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        button {
            aria_label: "Toggle sidebar",
            tabindex: -1,
            onclick: move |_| ctx.toggle(),
            title: "Toggle sidebar",
            ..merged,
        }
    }
}

#[component]
pub fn SidebarInset(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(main {
        class: "cn-ui-sidebar-inset {Styles::cn_ui_sidebar_inset.inner} flex relative flex-col w-full flex-1 bg-background min-w-0",
        "data-slot": "sidebar-inset",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        main { ..merged,{children} }
    }
}

#[component]
pub fn SidebarHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-header {Styles::cn_ui_sidebar_header.inner} flex gap-2 flex-col shrink-0 justify-center",
        "data-slot": "sidebar-header",
        "data-sidebar": "header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-content {Styles::cn_ui_sidebar_content.inner} flex gap-2 flex-col min-h-0 flex-1",
        "data-slot": "sidebar-content",
        "data-sidebar": "content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-footer {Styles::cn_ui_sidebar_footer.inner} flex gap-2 flex-col",
        "data-slot": "sidebar-footer",
        "data-sidebar": "footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarSeparator(
    #[props(default = true)] horizontal: bool,
    #[props(default = true)] decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-separator {Styles::cn_ui_sidebar_separator.inner}",
        "data-slot": "sidebar-separator",
        "data-sidebar": "separator",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        Separator { horizontal, decorative, attributes: merged }
    }
}

#[component]
pub fn SidebarGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-group min-w-0 flex relative flex-col p-2 gap-2",
        "data-slot": "sidebar-group",
        "data-sidebar": "group",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarGroupLabel(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-group-label {Styles::cn_ui_sidebar_group_label.inner} flex items-center rounded-sm text-sidebar-foreground text-subtle-foreground",
        "data-slot": "sidebar-group-label",
        "data-sidebar": "group-label",
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            document::Stylesheet { href: STYLES }

            div { ..merged,{children} }
        }
    }
}

#[component]
pub fn SidebarGroupAction(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: "cn-ui-sidebar-group-action {Styles::cn_ui_sidebar_group_action.inner} items-center border-0 p-0 cursor-pointer justify-center aspect-square right-3 rounded-sm top-3.5",
        "data-slot": "sidebar-group-action",
        "data-sidebar": "group-action",
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            document::Stylesheet { href: STYLES }

            button { ..merged,{children} }
        }
    }
}

#[component]
pub fn SidebarGroupContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-group-content text-sm w-full",
        "data-slot": "sidebar-group-content",
        "data-sidebar": "group-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarMenu(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ul {
        class: "cn-ui-sidebar-menu {Styles::cn_ui_sidebar_menu.inner} min-w-0 gap-1 flex-col p-0 m-0 list-none",
        "data-slot": "sidebar-menu",
        "data-sidebar": "menu",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        ul { ..merged,{children} }
    }
}

#[component]
pub fn SidebarMenuItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(li {
        class: "cn-ui-sidebar-menu-item {Styles::cn_ui_sidebar_menu_item.inner}",
        "data-slot": "sidebar-menu-item",
        "data-sidebar": "menu-item",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        li { ..merged,{children} }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

impl SidebarMenuButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuButtonVariant::Default => "default",
            SidebarMenuButtonVariant::Outline => "outline",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SidebarMenuButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuButtonSize::Default => "default",
            SidebarMenuButtonSize::Sm => "sm",
            SidebarMenuButtonSize::Lg => "lg",
        }
    }
}

#[component]
pub fn SidebarMenuButton(
    #[props(default = false)] is_active: bool,
    #[props(default)] variant: SidebarMenuButtonVariant,
    #[props(default)] size: SidebarMenuButtonSize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] tooltip: Option<Element>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx = use_sidebar();
    let is_mobile = ctx.is_mobile;
    let state = ctx.state;

    let base = attributes!(button {
        class: "cn-ui-sidebar-menu-button {Styles::cn_ui_sidebar_menu_button.inner}",
        "data-slot": "sidebar-menu-button",
        "data-sidebar": "menu-button",
        "data-size": size.as_str(),
        "data-variant": variant.as_str(),
        "data-active": if is_active { "true" } else { "false" },
        aria_current: is_active.then_some("page"),
    });
    let merged = merge_attributes(vec![base, attributes]);

    let Some(tooltip_content) = tooltip else {
        return if let Some(dynamic) = r#as {
            dynamic.call(merged)
        } else {
            rsx! {
                document::Stylesheet { href: STYLES }
                button { ..merged,{children} }
            }
        };
    };

    let hidden = state() != SidebarState::Collapsed || is_mobile();
    let sidebar_side = ctx.side;

    rsx! {
        document::Stylesheet { href: STYLES }

        Tooltip { class: "cn-ui-sidebar-tooltip {Styles::cn_ui_sidebar_tooltip.inner} w-full! block!", disabled: hidden,
            TooltipTrigger {
                r#as: move |tooltip_attrs: Vec<Attribute>| {
                    let final_attrs = merge_attributes(vec![tooltip_attrs, merged.clone()]);
                    let children = children.clone();
                    if let Some(dynamic) = &r#as {
                        dynamic.call(final_attrs)
                    } else {
                        rsx! {
                            document::Stylesheet { href: STYLES }
                            button { ..final_attrs,{children} }
                        }
                    }
                },
            }
            TooltipContent {
                side: match sidebar_side() {
                    SidebarSide::Left => dioxus_primitives::ContentSide::Right,
                    SidebarSide::Right => dioxus_primitives::ContentSide::Left,
                },
                {tooltip_content}
            }
        }
    }
}

#[component]
pub fn SidebarMenuAction(
    #[props(default = false)] show_on_hover: bool,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: "cn-ui-sidebar-menu-action {Styles::cn_ui_sidebar_menu_action.inner}",
        "data-slot": "sidebar-menu-action",
        "data-sidebar": "menu-action",
        "data-show-on-hover": if show_on_hover { "true" } else { "false" },
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            document::Stylesheet { href: STYLES }

            button { ..merged,{children} }
        }
    }
}

#[component]
pub fn SidebarMenuBadge(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-menu-badge {Styles::cn_ui_sidebar_menu_badge.inner} flex items-center absolute text-xs justify-center font-medium h-5 rounded-sm select-none right-1 min-w-5 tabular-nums",
        "data-slot": "sidebar-menu-badge",
        "data-sidebar": "menu-badge",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarMenuSkeleton(
    #[props(default = false)] show_icon: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-sidebar-menu-skeleton {Styles::cn_ui_sidebar_menu_skeleton.inner} flex items-center gap-2 h-8 rounded-sm",
        "data-slot": "sidebar-menu-skeleton",
        "data-sidebar": "menu-skeleton",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div {..merged,
            if show_icon {
                Skeleton { class: "cn-ui-sidebar-menu-skeleton-icon w-4 h-4 rounded-sm" }
            }
            Skeleton { class: "cn-ui-sidebar-menu-skeleton-text h-4 flex-1", width: "70%" }
        }
    }
}

#[component]
pub fn SidebarMenuSub(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ul {
        class: "cn-ui-sidebar-menu-sub {Styles::cn_ui_sidebar_menu_sub.inner} flex gap-1 flex-col list-none",
        "data-slot": "sidebar-menu-sub",
        "data-sidebar": "menu-sub",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        ul { ..merged,{children} }
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(li {
        class: "cn-ui-sidebar-menu-sub-item relative",
        "data-slot": "sidebar-menu-sub-item",
        "data-sidebar": "menu-sub-item",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        li { ..merged,{children} }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
}

impl SidebarMenuSubButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuSubButtonSize::Sm => "sm",
            SidebarMenuSubButtonSize::Md => "md",
        }
    }
}

#[component]
pub fn SidebarMenuSubButton(
    #[props(default = false)] is_active: bool,
    #[props(default)] size: SidebarMenuSubButtonSize,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(a {
        class: "cn-ui-sidebar-menu-sub-button {Styles::cn_ui_sidebar_menu_sub_button.inner} min-w-0 flex items-center gap-2 box-border cursor-pointer no-underline",
        "data-slot": "sidebar-menu-sub-button",
        "data-sidebar": "menu-sub-button",
        "data-size": size.as_str(),
        "data-active": if is_active { "true" } else { "false" },
        aria_current: is_active.then_some("page"),
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            document::Stylesheet { href: STYLES }

            a { ..merged,{children} }
        }
    }
}
