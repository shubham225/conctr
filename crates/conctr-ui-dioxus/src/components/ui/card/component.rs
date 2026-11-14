use dioxus::prelude::*;

#[css_module("/src/components/ui/card/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/card/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div { class: "cn-ui-card cn-card {Styles::cn_card.inner} flex flex-col gap-4 text-card-foreground", "data-slot": "card", ..attributes, {children} }
    }
}

#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-card-header {Styles::cn_ui_card_header.inner} grid gap-2 items-start",
            "data-slot": "card-header",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div { class: "cn-ui-card-title {Styles::cn_ui_card_title.inner}", "data-slot": "card-title", ..attributes, {children} }
    }
}

#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-card-description {Styles::cn_ui_card_description.inner} text-muted-foreground",
            "data-slot": "card-description",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-card-action {Styles::cn_ui_card_action.inner}",
            "data-slot": "card-action",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-card-content min-w-0",
            "data-slot": "card-content",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-card-footer flex items-center",
            "data-slot": "card-footer",
            ..attributes,
            {children}
        }
    }
}
