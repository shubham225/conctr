//! Framework-independent credential reference directory.
use serde::{Deserialize, Serialize};

/// Intended use of the referenced credential.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Purpose {
    #[default]
    InboundApiKey,
    OutboundOAuth,
    WebhookSigning,
}
/// Editable metadata. The alias names a credential in an external store.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialDraft {
    pub name: String,
    pub purpose: Purpose,
    pub alias: String,
}
/// Directory record with an optimistic concurrency token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credential {
    pub id: u64,
    pub revision: u64,
    pub metadata: CredentialDraft,
}
#[cfg(feature = "storage")]
pub mod directory;
