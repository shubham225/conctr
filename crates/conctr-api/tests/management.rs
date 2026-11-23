#![cfg(feature = "server")]
use axum::{
    body::{Body, to_bytes},
    http::{HeaderMap, Request},
};
use conctr_api::http::router;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn call(path: &str, body: String, cookie: &str) -> (u16, HeaderMap, Value) {
    let request = Request::builder()
        .method(if path == "system/details" {
            "GET"
        } else {
            "POST"
        })
        .uri(format!("/api/v1/{path}"))
        .header("x-request-client", "conctr")
        .header("x-request-id", "contract-test")
        .header("content-type", "application/json")
        .header("cookie", cookie)
        .body(Body::from(body))
        .unwrap();
    let response = router().oneshot(request).await.unwrap();
    let (parts, body) = response.into_parts();
    assert_eq!(parts.headers["x-request-id"], "contract-test");
    let bytes = to_bytes(body, 1024 * 1024).await.unwrap();
    let json = if parts.status == 204 {
        assert!(bytes.is_empty());
        Value::Null
    } else {
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        if parts.status.is_success() {
            assert_eq!(parts.headers["content-type"], "application/json");
        } else {
            assert_eq!(parts.headers["content-type"], "application/problem+json");
            assert_eq!(value["status"], parts.status.as_u16());
        }
        value
    };
    (parts.status.as_u16(), parts.headers, json)
}

#[tokio::test]
async fn management_contract() {
    // Process-local settings and signing keys must stay isolated from the operator's workspace.
    if std::env::var_os("CONCTR_CONTRACT_TEST_CHILD").is_none() {
        let dir = tempfile::tempdir().unwrap();
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "management_contract"])
            .current_dir(dir.path())
            .env("CONCTR_CONTRACT_TEST_CHILD", "1")
            .env("CONCTR_ACCOUNTS_FILE", dir.path().join("accounts.json"))
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    conctr_api::http::initialize().unwrap();
    let secret = std::fs::read_to_string("initial-admin-password").unwrap();
    let password = "contract test permanent password";
    let (status, _, body) = call("auth/session", "{}".into(), "").await;
    assert_eq!((status, body), (200, json!({"data":null})));
    assert_eq!(call("system/details", "".into(), "").await.0, 401);
    let (status, _, body) = call("auth/login", "{private input".into(), "").await;
    assert_eq!(status, 400);
    assert!(!body.to_string().contains("private input"));
    let (_, _, body) = call(
        "auth/login",
        json!({"username":"admin","password":secret}).to_string(),
        "",
    )
    .await;
    assert_eq!(body, json!({"data":"SetPassword"}));
    let (status, headers, _) = call(
        "auth/first-password",
        json!({"temporary":secret,"password":password}).to_string(),
        "",
    )
    .await;
    assert_eq!(status, 204);
    let cookie = headers["set-cookie"]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap();
    let (status, _, body) = call("system/details", "".into(), cookie).await;
    assert_eq!(status, 200);
    assert_eq!(body["data"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(body["data"]["os"], std::env::consts::OS);
    assert_eq!(body["data"]["architecture"], std::env::consts::ARCH);
    assert_eq!(call("auth/logout", "{}".into(), cookie).await.0, 204);
    assert_eq!(call("system/details", "{}".into(), cookie).await.0, 401);
}

#[tokio::test]
async fn cross_site_requests_are_rejected_and_receive_request_ids() {
    for path in ["/api/v1/auth/login", "/api/v1/auth/logout"] {
        let request = Request::post(path)
            .header("x-request-client", "dioxus")
            .header("sec-fetch-site", "cross-site")
            .body(Body::empty())
            .unwrap();
        let response = router().oneshot(request).await.unwrap();
        assert_eq!(response.status(), 403);
        assert!(
            uuid::Uuid::parse_str(response.headers()["x-request-id"].to_str().unwrap()).is_ok()
        );
    }
}

#[tokio::test]
async fn accepted_execution_can_use_the_shared_success_contract() {
    use axum::{Json, http::StatusCode, response::IntoResponse};
    use conctr_api::response::ApiResponse;
    let queued = json!({"id":"test-run","status":"queued"});
    let response = (StatusCode::ACCEPTED, Json(ApiResponse::new(&queued))).into_response();
    assert_eq!(response.status(), 202);
    assert_eq!(response.headers()["content-type"], "application/json");
    let bytes = to_bytes(response.into_body(), 1024).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&bytes).unwrap(),
        json!({"data":queued})
    );
}
