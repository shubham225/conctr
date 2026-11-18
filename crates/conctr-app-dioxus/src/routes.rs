use crate::Shell;
use crate::mapping_studio::MappingStudio;
use crate::pages::{Connections, NotFound, Runs, Settings};
use crate::pipeline_studio::PipelineStudio;
use crate::queues::Queues;
use dioxus::prelude::*;

/// Product routes shared by web, desktop and mobile.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Connections {},
    #[route("/mappings")]
    MappingStudio {},
    #[route("/pipelines")]
    PipelineStudio {},
    #[route("/queues")]
    Queues {},
    #[route("/runs")]
    Runs {},
    #[route("/settings")]
    Settings {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
