use crate::Shell;
use dioxus::prelude::*;

/// Workspace destinations shared by all launchers.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
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
            Self::FieldMappings {} => "Field Mappings",
            Self::Transformers {} => "Transformers",
            Self::ChannelsQueues {} => "Channels/Queues",
            Self::WorkflowPipeline {} => "Workflow Pipeline",
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
fn Dashboard() -> Element {
    rsx! { h1 { class: "text-2xl", "Dashboard" } }
}

#[component]
fn Credentials() -> Element {
    rsx! { h1 { class: "text-2xl", "Credentials" } }
}

#[component]
fn Connections() -> Element {
    rsx! { h1 { class: "text-2xl", "Connections" } }
}

#[component]
fn Endpoints() -> Element {
    rsx! { h1 { class: "text-2xl", "Endpoints" } }
}

#[component]
fn FieldMappings() -> Element {
    rsx! { h1 { class: "text-2xl", "Field Mappings" } }
}

#[component]
fn Transformers() -> Element {
    rsx! { h1 { class: "text-2xl", "Transformers" } }
}

#[component]
fn ChannelsQueues() -> Element {
    rsx! { h1 { class: "text-2xl", "Channels/Queues" } }
}

#[component]
fn WorkflowPipeline() -> Element {
    rsx! { h1 { class: "text-2xl", "Workflow Pipeline" } }
}

#[component]
fn History() -> Element {
    rsx! { h1 { class: "text-2xl", "History" } }
}

#[component]
fn Settings() -> Element {
    rsx! { h1 { class: "text-2xl", "Settings" } }
}

#[component]
fn Profile() -> Element {
    rsx! { h1 { class: "text-2xl", "Profile" } }
}

#[component]
fn Notifications() -> Element {
    rsx! { h1 { class: "text-2xl", "Notifications" } }
}

#[component]
fn Help() -> Element {
    rsx! { h1 { class: "text-2xl", "Help" } }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! { h1 { "Page not found" } Link { to: Route::Dashboard {}, "Back to dashboard" } }
}
