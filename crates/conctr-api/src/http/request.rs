use axum::http::{self, HeaderMap, HeaderValue, header};
const COOKIE: &str = "__Host-conctr-session";
// A configured public origin always takes precedence. For an unconfigured
// local launcher, allow loopback origins only. dx may proxy to a different
// loopback port, so browsers' same-origin Fetch Metadata matters here.
fn origin_allowed(
    headers: &http::HeaderMap,
    origin: &HeaderValue,
    configured: Option<&std::ffi::OsStr>,
) -> bool {
    if let Some(configured) = configured.filter(|value| !value.is_empty()) {
        let Some(configured) = configured.to_str() else {
            return false;
        };
        // A pasted site URL can end in a slash. Browser Origin headers
        // contain only scheme, host and port.
        let configured = configured.strip_suffix('/').unwrap_or(configured);
        return !configured.is_empty()
            && configured != "null"
            && configured.as_bytes() == origin.as_bytes();
    }
    let Some(uri) = origin
        .to_str()
        .ok()
        .and_then(|s| s.parse::<http::Uri>().ok())
    else {
        return false;
    };
    let Some(host) = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<http::uri::Authority>().ok())
    else {
        return false;
    };
    let loopback = |s: &str| matches!(s, "localhost" | "127.0.0.1" | "[::1]");
    matches!(uri.scheme_str(), Some("http" | "https"))
        && uri.host().is_some_and(loopback)
        && loopback(host.host())
        && (uri.authority() == Some(&host)
            || headers
                .get("sec-fetch-site")
                .is_some_and(|v| v == "same-origin"))
}
pub(super) fn allowed(headers: &HeaderMap) -> bool {
    headers
        .get("x-request-client")
        .is_some_and(|v| v == "dioxus" || v == "conctr")
        && !headers
            .get("sec-fetch-site")
            .is_some_and(|v| v != "same-origin")
        && headers.get(header::ORIGIN).is_none_or(|origin| {
            origin_allowed(
                headers,
                origin,
                std::env::var_os("CONCTR_PUBLIC_ORIGIN").as_deref(),
            )
        })
}
pub(super) fn token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|h| h.to_str().ok())
        .flat_map(|s| s.split(';'))
        .filter_map(|s| s.trim().split_once('='))
        .find(|(name, _)| *name == COOKIE)
        .map(|(_, value)| value)
}
pub(super) fn cookie(value: &str, seconds: u64) -> String {
    format!("{COOKIE}={value}; Path=/; Secure; HttpOnly; SameSite=Strict; Max-Age={seconds}")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_origin_accepts_a_pasted_root_url() {
        use std::ffi::OsStr;
        let headers = http::HeaderMap::new();
        let origin = HeaderValue::from_static("http://127.0.0.1:8080");
        for configured in ["http://127.0.0.1:8080", "http://127.0.0.1:8080/"] {
            assert!(origin_allowed(
                &headers,
                &origin,
                Some(OsStr::new(configured))
            ));
        }
        for configured in [
            "http://127.0.0.1:8081/",
            "http://127.0.0.1:8080/login",
            "https://127.0.0.1:8080/",
        ] {
            assert!(!origin_allowed(
                &headers,
                &origin,
                Some(OsStr::new(configured))
            ));
        }
    }
}
