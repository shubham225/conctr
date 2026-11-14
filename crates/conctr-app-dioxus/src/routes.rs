use crate::Shell;
use crate::pages::{Connections, NotFound, Runs, Settings};
use dioxus::prelude::*;

/// Product routes shared by web, desktop and mobile.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Connections {},
    #[route("/runs")]
    Runs {},
    #[route("/settings")]
    Settings {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
