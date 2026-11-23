//! Local account authentication and revocable Conctr API sessions.
//! HTTP cookies and Dioxus types belong in the transport adapter.
mod account;
mod attempts;
mod authority;
pub mod local_accounts;

pub use account::{Account, Principal, Role};
pub use attempts::LoginAttempts;
pub use authority::{AuthError, Authority, SESSION_SECONDS};

mod recovery;
mod workspace;
pub use workspace::{PasswordLogin, WorkspaceAuth};
