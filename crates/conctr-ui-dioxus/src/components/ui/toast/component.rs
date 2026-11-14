use dioxus::prelude::*;
use dioxus_primitives::toast::{
    self, Toast, ToastCloseButtonProps, ToastContentProps, ToastDescriptionProps, ToastProps,
    ToastTitleProps,
};
use std::time::Duration;

#[css_module("/src/components/ui/toast/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/toast/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
fn StyledToast(props: ToastProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            duration: props.duration,
            class: "cn-ui-toast {Styles::cn_ui_toast.inner} flex border border-solid border-border rounded-md gap-3 w-full p-4 items-start h-auto",
            attributes: props.attributes,
            ToastContent {
                ToastTitle {}
                ToastDescription {}
            }
            ToastCloseButton {}
        }
    }
}

#[component]
fn ToastContent(props: ToastContentProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        toast::ToastContent { class: "cn-ui-toast-content {Styles::cn_ui_toast_content.inner} min-w-0 flex-1", attributes: props.attributes, {props.children} }
    }
}

#[component]
fn ToastTitle(props: ToastTitleProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        toast::ToastTitle {
            class: "cn-ui-toast-title {Styles::cn_ui_toast_title.inner}",
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastDescription(props: ToastDescriptionProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        toast::ToastDescription {
            class: "cn-ui-toast-description {Styles::cn_ui_toast_description.inner}",
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastCloseButton(props: ToastCloseButtonProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        toast::ToastCloseButton {
            class: "cn-ui-toast-close {Styles::cn_ui_toast_close.inner} grid place-items-center border-0 p-0 cursor-pointer w-(--cn-control-sm) h-(--cn-control-sm) shrink-0 text-inherit rounded-sm",
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
pub fn ToastProvider(
    #[props(default = ReadSignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadSignal<Option<Duration>>,
    #[props(default = ReadSignal::new(Signal::new(10)))] max_toasts: ReadSignal<usize>,
    #[props(default)] render_toast: Option<Callback<toast::ToastPropsWithOwner, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let render_toast = render_toast.unwrap_or_else(|| {
        Callback::new(|p: toast::ToastPropsWithOwner| {
            rsx! {
                StyledToast { ..p }
            }
        })
    });

    rsx! {
        document::Stylesheet { href: STYLES }

        toast::ToastProvider {
            class: "cn-ui-toast-container {Styles::cn_ui_toast_container.inner} fixed",
            default_duration,
            max_toasts,
            render_toast,
            attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_primitives::toast::{ToastOptions, use_toast};

    #[component]
    fn TriggerToast() -> Element {
        let toast_api = use_toast();
        use_hook(move || {
            toast_api.success(
                "Saved".to_string(),
                ToastOptions::new()
                    .description("Everything synced")
                    .permanent(true),
            );
        });

        rsx! {}
    }

    #[test]
    fn styled_toast_preserves_primitive_fallback_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ToastProvider { TriggerToast {} }
            }
        });
        dom.rebuild_in_place();
        dom.mark_all_dirty();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Saved"));
        assert!(html.contains("Everything synced"));
        assert!(html.contains('\u{00d7}') || html.contains("&#215;") || html.contains("&times;"));
    }
}
