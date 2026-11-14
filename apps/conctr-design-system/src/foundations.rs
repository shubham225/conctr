use conctr_ui_dioxus::{
    assets::{icons, images},
    prelude::*,
};
use dioxus::prelude::*;

#[component]
pub fn Typography() -> Element {
    rsx! {
        document::Title { "Typography · Conctr design system" }
        div { class: "ds-page-head", span { class: "ds-eyebrow", "Foundations" } h1 { "Typography" } p { "Sora for headings, Inter for interface and body text, JetBrains Mono for code. Fonts are bundled locally." } }
        div { class: "ds-type-list",
            for (class, name, spec) in [("cn-display", "Display", "Sora · 48 / 56 · 600"), ("cn-h1", "Heading 1", "Sora · 36 / 44 · 600"), ("cn-h2", "Heading 2", "Sora · 28 / 36 · 600"), ("cn-h3", "Heading 3", "Sora · 22 / 30 · 600"), ("cn-h4", "Heading 4", "Sora · 18 / 26 · 600"), ("cn-body-lg", "Body large", "Inter · 18 / 28 · 400"), ("cn-body-text", "Body", "Inter · 16 / 26 · 400"), ("cn-small", "Small", "Inter · 14 / 22 · 400"), ("cn-caption", "Caption", "Inter · 12 / 18 · 400"), ("cn-mono", "Code", "JetBrains Mono · 13 / 20 · 400")] {
                div { class: "ds-type-row", div { strong { "{name}" } span { class: "cn-caption", "{spec}" } code { "{class}" } } div { class, "Where systems meet." } }
            }
        }
        p { class: "ds-guidance", "Display, h1 and h2 step down at the kit’s 640px breakpoint. The rows above use the actual shared classes." }
        Cluster { KeyboardKey { "⌘" } KeyboardKey { "K" } span { class: "cn-muted", "Muted text" } span { class: "cn-subtle", "Subtle text" } }
    }
}

#[component]
pub fn Colors() -> Element {
    rsx! {
        document::Title { "Color palette · Conctr design system" }
        div { class: "ds-page-head", span { class: "ds-eyebrow", "Foundations" } h1 { "Color palette" } p { "Switch the theme to inspect the semantic token values. Protocols are identified by both shape and label." } }
        h2 { "Surfaces & text" }
        div { class: "ds-token-grid", for token in ["background", "foreground", "card", "card-foreground", "popover", "popover-foreground", "muted", "muted-foreground", "border", "input"] { ColorToken { token: token.to_string() } } }
        h2 { "Brand & feedback" }
        div { class: "ds-token-grid", for token in ["primary", "primary-foreground", "secondary", "secondary-foreground", "accent", "accent-foreground", "destructive", "ring", "sidebar", "sidebar-foreground", "sidebar-accent", "sidebar-border", "success", "warning", "cn-teal-text", "cn-coral-text", "cn-amber-text"] { ColorToken { token: token.to_string() } } }
        h2 { "Protocol language" }
        Cluster { class: "ds-foundation-stage", for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] { ProtocolBadge { protocol } } }
        p { class: "ds-guidance", "Use semantic tokens for UI. The -text accent tokens provide readable colored text on light backgrounds." }
    }
}

#[component]
fn ColorToken(token: String) -> Element {
    rsx! { div { class: "ds-token", div { class: "ds-color-swatch", style: "background: var(--{token});" } code { "--{token}" } } }
}

#[component]
pub fn Spacing() -> Element {
    rsx! {
        document::Title { "Spacing & elevation · Conctr design system" }
        div { class: "ds-page-head", span { class: "ds-eyebrow", "Foundations" } h1 { "Spacing & elevation" } p { "The 4px spacing grid, shared control heights, nested radii and elevation tokens from the design kit." } }
        h2 { "Spacing scale" }
        div { class: "ds-space-list", for unit in [1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24] { div { class: "ds-space-row", code { "--cn-space-{unit}" } span { class: "cn-caption", "{unit * 4}px" } i { style: "width: var(--cn-space-{unit});", aria_hidden: "true" } } } }
        h2 { "Corner radius" }
        div { class: "ds-token-grid", for (name, value) in [("xs", "4px"), ("sm", "6px"), ("md", "10px"), ("lg", "14px"), ("xl", "20px"), ("pill", "999px")] { div { class: "ds-token", div { class: "ds-radius-swatch", style: "border-radius: var(--cn-radius-{name});" } code { "--cn-radius-{name}" } span { class: "cn-caption", "{value}" } } } }
        h2 { "Elevation" }
        div { class: "ds-token-grid", for level in 1..=4 { div { class: "ds-token", div { class: "ds-shadow-swatch", style: "box-shadow: var(--cn-shadow-{level});" } code { "--cn-shadow-{level}" } } } }
        h2 { "Control sizes" }
        Cluster { Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary, "Small · 32px" } Button { variant: ButtonVariant::Secondary, "Default · 40px" } Button { size: ButtonSize::Lg, variant: ButtonVariant::Secondary, "Large · 48px" } }
    }
}

#[component]
pub fn Branding() -> Element {
    rsx! {
        document::Title { "Logo & icons · Conctr design system" }
        div { class: "ds-page-head", span { class: "ds-eyebrow", "Foundations" } h1 { "Logo & icons" } p { "Original supplied artwork, with themed logo variants. Protocol shapes retain their fixed meanings." } }
        div { class: "ds-brand-stage", Logo {} }
        h2 { "Application marks" }
        Cluster { class: "ds-foundation-stage", img { src: images::APP_ICON_SVG, alt: "Conctr application icon", class: "ds-brand-icon" } img { src: images::ICON_COLOR_SVG, alt: "Conctr mark on a light surface", class: "ds-brand-icon" } }
        h2 { "Interface icons" }
        div { class: "ds-token-grid ds-icon-grid",
            for (name, mono, color) in [("API key", icons::API_KEY_MONO, icons::API_KEY_COLOR), ("Basic auth", icons::BASIC_MONO, icons::BASIC_COLOR), ("OAuth 2", icons::OAUTH2_MONO, icons::OAUTH2_COLOR), ("JWT", icons::JWT_MONO, icons::JWT_COLOR), ("SAML", icons::SAML_MONO, icons::SAML_COLOR), ("Mutual TLS", icons::MTLS_MONO, icons::MTLS_COLOR)] {
                div { class: "ds-icon-tile", Cluster { Icon { src: mono, monochrome: true, alt: "{name}, monochrome" } Icon { src: color, alt: "{name}, color" } } span { class: "cn-small", "{name}" } }
            }
        }
        p { class: "cn-small", "These are the supplied authentication icons. General interface actions currently use Lucide; a branded interface icon set is not included in the kit." }
        h2 { "Interface actions" }
        div { class: "ds-token-grid ds-icon-grid",
            for (name, icon) in [("Connections", IconName::Connections), ("Runs", IconName::Runs), ("Settings", IconName::Settings), ("Search", IconName::Search), ("Add", IconName::Add), ("Panel", IconName::Panel), ("Folder", IconName::Folder), ("More", IconName::More), ("Close", IconName::Close)] {
                div { class: "ds-icon-tile", UiIcon { name: icon } span { class: "cn-small", "{name}" } }
            }
        }
        h2 { "Protocol shapes" }
        Cluster { class: "ds-foundation-stage", for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] { ProtocolBadge { protocol } } }
        h2 { "Protocol icons" }
        div { class: "ds-token-grid ds-icon-grid",
            for (name, mono, color) in [("REST", icons::REST_MONO, icons::REST_COLOR), ("SOAP", icons::SOAP_MONO, icons::SOAP_COLOR), ("OData", icons::ODATA_MONO, icons::ODATA_COLOR), ("GraphQL", icons::GRAPHQL_MONO, icons::GRAPHQL_COLOR), ("gRPC", icons::GRPC_MONO, icons::GRPC_COLOR), ("Webhook", icons::WEBHOOK_MONO, icons::WEBHOOK_COLOR)] {
                div { class: "ds-icon-tile", Cluster { Icon { src: mono, monochrome: true, alt: "{name}, monochrome" } Icon { src: color, alt: "{name}, color" } } span { class: "cn-small", "{name}" } }
            }
        }
        CodeBlock { "use conctr_ui_dioxus::prelude::*;\nuse conctr_ui_dioxus::assets::{{images, icons}};" }
    }
}
