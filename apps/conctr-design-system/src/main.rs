mod catalog;
mod examples;
use conctr_ui_dioxus::{assets::images, prelude::*};
use dioxus::prelude::*;
#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/components/:slug")]
    ComponentPage { slug: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let dark = use_signal(|| false);
    use_context_provider(|| dark);
    rsx! {
     document::Title { "Conctr design system · Rust UI" }
     document::Link { rel: "icon", href: images::FAVICON }
     ThemeProvider { theme: if dark() { Theme::Dark } else { Theme::Light }, Router::<Route> {} }
    }
}
#[component]
fn Shell() -> Element {
    let mut dark = use_context::<Signal<bool>>();
    rsx! {
     div { class: "min-h-screen",
      header { class: "sticky top-0 z-50 flex flex-wrap items-center justify-between gap-4 border-b border-border bg-background p-4 md:px-8",
       Link { to: Route::Home {}, img { src: if dark() { images::HORIZONTAL_COLOR_DARK_SVG } else { images::HORIZONTAL_COLOR_SVG }, alt: "Conctr", width: 140, height: 36 } }
       span { class: "text-sm text-muted-foreground", "Design system / Rust UI" }
       BrandThemeToggle { dark: dark(), on_change: move |value| dark.set(value) }
      }
      div { class: "mx-auto grid max-w-[1440px] gap-8 p-4 md:grid-cols-[240px_minmax(0,1fr)] md:p-8",
       nav { class: "max-h-[70vh] overflow-y-auto", "aria-label": "Component catalog",
        Link { to: Route::Home {}, class: "block rounded-md p-2 font-medium text-primary", "Brand foundations" }
        h2 { class:"px-2 py-3 text-sm font-semibold", "Composed" }
        for entry in catalog::COMPOSED { Link { to: Route::ComponentPage {slug:entry.slug.into()}, class:"block rounded-md p-2 text-sm hover:bg-accent", {entry.slug.replace('-', " ")} } }
        h2 { class:"px-2 py-3 text-sm font-semibold", "Components" }
        for entry in catalog::ENTRIES {
         Link { to: Route::ComponentPage { slug: entry.slug.to_string() }, class: "block rounded-md p-2 text-sm hover:bg-accent focus-visible:ring-2", {entry.slug.replace('-', " ")} }
        }
       }
       main { class: "min-w-0", Outlet::<Route> {} }
      }
     }
    }
}
#[component]
fn Home() -> Element {
    rsx! {
     div { class: "grid gap-8",
      div { class: "grid gap-4", h1 { "Where systems meet." } p { class: "text-muted-foreground", "Conctr’s brand kit, presented through Rust UI components." } }
      section { class: "grid gap-4", h2 { "Brand foundations" }
       div { class: "grid gap-4 sm:grid-cols-3",
        for (name, color) in [("Indigo", "var(--cn-primary)"), ("Teal", "var(--cn-teal)"), ("Coral", "var(--cn-coral)"), ("Amber", "var(--cn-amber)"), ("Surface", "var(--cn-surface)"), ("Text", "var(--cn-text)")] {
         div { class: "overflow-hidden rounded-lg border border-border", div { class: "h-16", style: "background:{color}" } p { class: "p-3 text-sm", "{name}" } }
        }
       }
       p { "Sora headings · Inter interface · JetBrains Mono code" }
      }
      section { class: "grid gap-4", h2 { "Controls" }
       div { class: "flex flex-wrap gap-4",
        Button { "Test connection" }
        Button { variant: ButtonVariant::Outline, "View source" }
        Button { disabled: true, "Unavailable" }
       }
       conctr_ui_dioxus::ui::input::Input { placeholder: "Connection name" }
      }
      section { class: "grid gap-4", h2 { "Workflow canvas" }
       p { class: "text-sm text-muted-foreground", "Interactive upstream demonstration. Canvas state is presentation only; no flow execution or persistence is connected." }
       Link { to: Route::ComponentPage { slug: "conctr-workflow".into() }, class: "text-primary underline", "Open workflow canvas" }
      }
      p { class: "text-sm text-muted-foreground", "Rust UI is experimental. AutoForm is an upstream shell, not automatic form generation. Examples illustrate controls, not available Conctr integrations." }
     }
    }
}
#[component]
fn ComponentPage(slug: String) -> Element {
    let entry = catalog::COMPOSED
        .iter()
        .chain(catalog::ENTRIES.iter())
        .find(|entry| entry.slug == slug);
    rsx! {
     if let Some(entry) = entry {
      div { class: "grid gap-6", key: "{slug}",
       h1 { {slug.replace('-', " ")} }
       div { class: "min-w-0 rounded-lg border border-border bg-card p-6",
        style: if ["date-picker", "composed-date-picker", "multi-select", "input-phone", "popover", "dropdown-menu", "select", "combobox"].contains(&slug.as_str()) { "min-height:480px;overflow:visible" } else { "overflow:auto" },
        {(entry.preview)()} }
       details { summary { class: "cursor-pointer text-primary", "View example source" }
        p { class: "py-4 text-sm text-muted-foreground", "Upstream example imports resolve to the Conctr facade. In apps use conctr_ui_dioxus::ui." }
        pre { class: "overflow-auto rounded-md bg-muted p-4 text-xs", code { "{entry.source}" } }
       }
      }
     } else { EmptyState { title: "Example not found", description: "Choose a component from the catalog." } }
    }
}
#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! { EmptyState { title: "Page not found", description: "Choose a component from the catalog." } }
}
