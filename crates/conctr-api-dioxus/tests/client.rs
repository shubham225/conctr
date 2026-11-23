#![cfg(feature = "server")]
use conctr_api_dioxus::{current_session, logout, router, system_details};
use dioxus::prelude::ServerFnError;

#[tokio::test]
async fn client_reads_success_problem_and_empty_bodies() {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    dioxus::fullstack::set_server_url(Box::leak(origin.into_boxed_str()));
    let server = tokio::spawn(async {
        dioxus::server::axum::serve(listener, router())
            .await
            .unwrap()
    });
    assert!(current_session().await.unwrap().is_none());
    assert!(matches!(
        system_details().await,
        Err(ServerFnError::ServerError { code: 401, .. })
    ));
    logout().await.unwrap();
    server.abort();
}
