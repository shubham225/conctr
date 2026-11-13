use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// The kit’s 4px spacing scale. Values map directly to CSS tokens.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Space {
    None,
    Xs,
    Sm,
    Md,
    #[default]
    Lg,
    Xl,
    Xxl,
    Section,
}
impl Space {
    pub fn token(self) -> &'static str {
        match self {
            Self::None => "var(--cn-space-0)",
            Self::Xs => "var(--cn-space-1)",
            Self::Sm => "var(--cn-space-2)",
            Self::Md => "var(--cn-space-3)",
            Self::Lg => "var(--cn-space-4)",
            Self::Xl => "var(--cn-space-6)",
            Self::Xxl => "var(--cn-space-8)",
            Self::Section => "var(--cn-space-12)",
        }
    }
}
/// Cross-axis item alignment.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Align {
    Start,
    Center,
    End,
    #[default]
    Stretch,
    Baseline,
}
impl Align {
    pub fn css(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Stretch => "stretch",
            Self::Baseline => "baseline",
        }
    }
}
/// Main-axis distribution.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    Between,
}
impl Justify {
    pub fn css(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "space-between",
        }
    }
}
/// Direction of a flex layout.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Direction {
    #[default]
    Row,
    Column,
}
impl Direction {
    pub fn css(self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::Column => "column",
        }
    }
}

/// A neutral container with token-based padding and native div attributes.
#[component]
pub fn LayoutBox(
    #[props(default = Space::None)] padding: Space,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-box",
        style: format!("padding: {};", padding.token())
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { div { ..merged, {children} } }
}

/// Flexible row/column layout. Use `Inline` for wrapping rows or `Stack` for columns.
#[component]
pub fn Flex(
    #[props(default)] direction: Direction,
    #[props(default)] gap: Space,
    #[props(default)] align: Align,
    #[props(default)] justify: Justify,
    #[props(default)] wrap: bool,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let style = format!(
        "display: flex; flex-direction: {}; gap: {}; align-items: {}; justify-content: {}; flex-wrap: {};",
        direction.css(),
        gap.token(),
        align.css(),
        justify.css(),
        if wrap { "wrap" } else { "nowrap" }
    );
    let base = attributes!(div {
        class: "cn-flex",
        style
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { div { ..merged, {children} } }
}

/// Responsive equal-width columns; `min_item_width` collapses columns on narrow containers.
#[component]
pub fn Grid(
    #[props(default = 3)] columns: u8,
    #[props(default = "12rem".to_string())] min_item_width: String,
    #[props(default)] gap: Space,
    #[props(default)] align: Align,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let columns = columns.clamp(1, 12);
    let style = format!(
        "--cn-grid-columns: {columns}; --cn-grid-min: {min_item_width}; --cn-grid-gap: {}; gap: var(--cn-grid-gap); align-items: {};",
        gap.token(),
        align.css()
    );
    let base = attributes!(div {
        class: "cn-grid",
        style
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { div { ..merged, {children} } }
}

/// Wrapping horizontal layout with centered items by default.
#[component]
pub fn Inline(
    #[props(default = Space::Md)] gap: Space,
    #[props(default = Align::Center)] align: Align,
    #[props(default)] justify: Justify,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-inline",
        style: format!(
            "gap: {}; align-items: {}; justify-content: {};",
            gap.token(),
            align.css(),
            justify.css()
        )
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { div { ..merged, {children} } }
}
