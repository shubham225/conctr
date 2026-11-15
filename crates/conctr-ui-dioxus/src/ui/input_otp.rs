//! Native editing adapter for Rust UI slots. The upstream root references an
//! unavailable `/components/otp.js`; Conctr keeps that file unchanged and uses
//! a native input here so consumers do not need a fixed public script path.
pub use crate::rust_ui::ui::input_otp::*;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct OtpContext {
    value: Signal<String>,
    focused: Signal<bool>,
}

/// One-time-code input with native keyboard, paste and autofill support.
/// `value` initializes or externally updates the field; `on_change` reports edits.
#[component]
pub fn InputOTP(
    children: Element,
    max_length: u32,
    #[props(default)] disabled: bool,
    #[props(default)] value: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] on_change: Option<EventHandler<String>>,
) -> Element {
    let mut current = use_signal(|| value.clone().unwrap_or_default());
    let mut focused = use_signal(|| false);
    use_context_provider(|| OtpContext {
        value: current,
        focused,
    });
    use_effect(use_reactive(&value, move |value| {
        if let Some(value) = value {
            current.set(value);
        }
    }));
    let class = tw_merge::tw_merge!(
        "relative flex w-fit items-center gap-2 has-[:disabled]:opacity-45",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { "data-slot": "input-otp", class,
            {children}
            input {
                "aria-label": "One-time code", autocomplete: "one-time-code",
                inputmode: "numeric", pattern: "[0-9]*", maxlength: max_length as i64,
                disabled, value: current(),
                class: "absolute inset-0 h-full w-full cursor-text opacity-0",
                onfocus: move |_| focused.set(true), onblur: move |_| focused.set(false),
                oninput: move |event| {
                    let next: String = event.value().chars().filter(char::is_ascii_digit).take(max_length as usize).collect();
                    current.set(next.clone());
                    if let Some(handler) = on_change { handler.call(next); }
                },
            }
        }
    }
}

/// Rust UI's visual slot with a native-input value overlay.
#[allow(non_snake_case)]
pub fn InputOTPSlot(mut props: InputOTPSlotProps) -> Element {
    let Some(context) = try_consume_context::<OtpContext>() else {
        return crate::rust_ui::ui::input_otp::InputOTPSlot(props);
    };
    let value = context.value.read();
    let character = value.chars().nth(props.index as usize).unwrap_or(' ');
    let active = (context.focused)() && props.index as usize == value.len();
    props.class = Some(tw_merge::tw_merge!(
        "h-10 w-10",
        if active { "ring-2 ring-ring" } else { "" },
        props.class.as_deref().unwrap_or("")
    ));
    rsx! {
        div { class: "relative", "aria-hidden": "true",
            {crate::rust_ui::ui::input_otp::InputOTPSlot(props)}
            span { class: "pointer-events-none absolute inset-0 flex items-center justify-center font-mono", "{character}" }
        }
    }
}
