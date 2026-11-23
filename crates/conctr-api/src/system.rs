use serde::{Deserialize, Serialize};

/// Build details of the server serving this workspace.
#[derive(Clone, Deserialize, Serialize)]
pub struct SystemDetails {
    pub version: String,
    pub os: String,
    pub architecture: String,
}
