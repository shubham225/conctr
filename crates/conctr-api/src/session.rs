/// Public session information. The signed token travels only in the HTTP cookie.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionInfo {
    pub username: String,
    /// Calculated by the server so client clock skew cannot prevent sign-in.
    pub remaining_seconds: u64,
}

/// The first-run invitation unlocks password setup. Only a completed login
/// creates a session that can access workspace APIs.
#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LoginOutcome {
    SetPassword,
    SignedIn,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FirstPassword {
    #[serde(default)]
    pub username: Option<String>,
    pub temporary: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RecoveryRequest {
    pub username: String,
}
