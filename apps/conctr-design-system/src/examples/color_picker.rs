use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut color =
        use_signal(|| palette::Hsv::<palette::encoding::Srgb, f64>::new(244.0, 0.69, 0.90));
    rsx! {
    Stack {
        ColorPicker { label: "Select example color", color: color(), on_color_change: move |next| color.set(next) }
        Hint { "The spectrum represents editable color data; the controls use Conctr tokens." }
    }
        }
}
