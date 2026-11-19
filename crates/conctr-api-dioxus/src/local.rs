//! Development-only sign-in for an unconfigured, single-user launcher.
use super::{ServerFnError, response_error};
use dioxus_fullstack_core::FullstackContext;
use http::{HeaderValue, header};
use std::{io::Write, path::PathBuf, sync::LazyLock};
use subtle::ConstantTimeEq;
use uuid::Uuid;

const COOKIE: &str = "conctr_local_credentials";
struct Session {
    code: String,
    cookie: String,
    code_file: tempfile::NamedTempFile,
}
static SESSION: LazyLock<std::io::Result<Session>> = LazyLock::new(|| {
    let code = Uuid::new_v4().to_string();
    let mut code_file = tempfile::Builder::new()
        .prefix("conctr-signin-")
        .tempfile()?;
    writeln!(code_file, "{code}")?;
    Ok(Session {
        code,
        code_file,
        cookie: Uuid::new_v4().to_string(),
    })
});
pub(super) fn enabled() -> bool {
    [
        "CONCTR_CREDENTIAL_ADMIN_TOKEN",
        "CONCTR_CREDENTIAL_DIRECTORY",
    ]
    .iter()
    .all(|k| std::env::var_os(k).is_none())
}
fn session() -> Result<&'static Session, ServerFnError> {
    SESSION
        .as_ref()
        .map_err(|_| response_error(503, "Cannot initialize the local sign-in file."))
}
fn denied() -> ServerFnError {
    response_error(403, "Credential management access required.")
}
fn context() -> Result<FullstackContext, ServerFnError> {
    let ctx = FullstackContext::current().ok_or_else(denied)?;
    {
        let parts = ctx.parts_mut();
        let authority = parts
            .headers
            .get(header::HOST)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<http::uri::Authority>().ok())
            .ok_or_else(denied)?;
        if !matches!(authority.host(), "localhost" | "127.0.0.1" | "[::1]") {
            return Err(denied());
        }
        if let Some(origin) = parts.headers.get(header::ORIGIN) {
            let uri = origin
                .to_str()
                .ok()
                .and_then(|s| s.parse::<http::Uri>().ok())
                .ok_or_else(denied)?;
            // dx serve rewrites Host to its internal listener. Browsers supply
            // Fetch Metadata against the original URL before proxying.
            let same_origin = parts
                .headers
                .get("sec-fetch-site")
                .is_some_and(|value| value == "same-origin");
            if !matches!(uri.host(), Some("localhost" | "127.0.0.1" | "[::1]"))
                || (!same_origin && uri.authority() != Some(&authority))
                || !matches!(uri.scheme_str(), Some("http" | "https"))
            {
                return Err(denied());
            }
        }
    }
    Ok(ctx)
}
pub(super) fn sign_in(code: &str) -> Result<(), ServerFnError> {
    let ctx = context()?;
    let s = session()?;
    if !bool::from(s.code.as_bytes().ct_eq(code.trim().as_bytes())) {
        return Err(response_error(403, "The local sign-in code is incorrect."));
    }
    let value = format!(
        "{COOKIE}={}; HttpOnly; SameSite=Strict; Path=/api/credentials/",
        s.cookie
    );
    ctx.add_response_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&value).map_err(|_| denied())?,
    );
    Ok(())
}
pub(super) fn directory() -> Result<PathBuf, ServerFnError> {
    let ctx = context()?;
    let s = session()?;
    let authenticated = ctx
        .parts_mut()
        .headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .split(';')
        .filter_map(|kv| kv.trim().split_once('='))
        .any(|(k, v)| k == COOKIE && bool::from(v.as_bytes().ct_eq(s.cookie.as_bytes())));
    if !authenticated {
        return Err(response_error(
            401,
            &format!(
                "Local sign-in required. Read the code from {} on the machine running Conctr.",
                s.code_file.path().display()
            ),
        ));
    }
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|v| PathBuf::from(v).join(".local/state")))
        .ok_or_else(|| response_error(503, "Set XDG_STATE_HOME for the local workspace."))?;
    Ok(base.join("conctr/local-workspace"))
}
