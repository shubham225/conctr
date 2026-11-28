use crate::{Shell, dashboard::Dashboard};
use dioxus::prelude::*;

/// Workspace destinations shared by all launchers.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[redirect("/login", || Route::Dashboard {})]
    #[layout(Shell)]
    #[route("/")]
    Dashboard {},
    #[route("/credentials")]
    Credentials {},
    #[route("/connections")]
    Connections {},
    #[route("/endpoints")]
    Endpoints {},
    #[route("/mappings")]
    FieldMappings {},
    #[route("/transformers")]
    Transformers {},
    #[route("/queues")]
    ChannelsQueues {},
    #[route("/pipelines")]
    WorkflowPipeline {},
    #[route("/runs")]
    History {},
    #[route("/settings")]
    Settings {},
    #[route("/profile")]
    Profile {},
    #[route("/notifications")]
    Notifications {},
    #[route("/help")]
    Help {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Dashboard {} => "Dashboard",
            Self::Credentials {} => "Credentials",
            Self::Connections {} => "Connections",
            Self::Endpoints {} => "Endpoints",
            Self::FieldMappings {} => "Field mappings",
            Self::Transformers {} => "Transformers",
            Self::ChannelsQueues {} => "Channels and queues",
            Self::WorkflowPipeline {} => "Workflow pipeline",
            Self::History {} => "History",
            Self::Settings {} => "Settings",
            Self::Profile {} => "Profile",
            Self::Notifications {} => "Notifications",
            Self::Help {} => "Help",
            Self::NotFound { .. } => "Page not found",
        }
    }
}

#[component]
fn Credentials() -> Element {
    rsx! { UnavailablePage { route: Route::Credentials {} } }
}

#[component]
fn Connections() -> Element {
    rsx! { UnavailablePage { route: Route::Connections {} } }
}

#[component]
fn Endpoints() -> Element {
    rsx! { UnavailablePage { route: Route::Endpoints {} } }
}

#[component]
fn FieldMappings() -> Element {
    rsx! { UnavailablePage { route: Route::FieldMappings {} } }
}

#[component]
fn Transformers() -> Element {
    rsx! { UnavailablePage { route: Route::Transformers {} } }
}

#[component]
fn ChannelsQueues() -> Element {
    rsx! { UnavailablePage { route: Route::ChannelsQueues {} } }
}

#[component]
fn WorkflowPipeline() -> Element {
    rsx! { UnavailablePage { route: Route::WorkflowPipeline {} } }
}

#[component]
fn History() -> Element {
    rsx! { UnavailablePage { route: Route::History {} } }
}

#[component]
fn Settings() -> Element {
    rsx! { UnavailablePage { route: Route::Settings {} } }
}

#[component]
fn Profile() -> Element {
    rsx! { UnavailablePage { route: Route::Profile {} } }
}

#[component]
fn Notifications() -> Element {
    rsx! { UnavailablePage { route: Route::Notifications {} } }
}

#[component]
fn Help() -> Element {
    rsx! { UnavailablePage { route: Route::Help {} } }
}

#[component]
fn UnavailablePage(route: Route) -> Element {
    rsx! {
        div { class: "grid gap-2",
            h1 { class: "text-2xl font-semibold", "{route.label()}" }
            p { class: "text-sm text-muted-foreground", "This page is under development." }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "grid gap-2",
            h1 { class: "text-2xl font-semibold", "Page not found" }
            p { class: "text-sm text-muted-foreground", "Check the address or return to Dashboard." }
            Link {
                to: Route::Dashboard {},
                class: "w-fit rounded-sm text-sm font-medium text-primary underline",
                "Go to Dashboard"
            }
        }
    }
}
