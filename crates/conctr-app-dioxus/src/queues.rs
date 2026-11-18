//! Sample queue snapshots for the shared product preview.
use conctr_ui_dioxus::composed::queue::{QueueMessage, QueueMonitor, QueueRequest, QueueStatus};
use conctr_ui_dioxus::ui::code_block::CodeLanguage;
use dioxus::prelude::*;

#[component]
pub fn Queues() -> Element {
    let mut paused = use_signal(|| false);
    let requests = use_signal(sample_requests);
    rsx! {
        document::Title { "Queues · Conctr" }
        div { class: "mx-auto grid w-full min-w-0 max-w-6xl gap-4",
            header { class: "grid gap-2",
                h1 { class: "text-2xl font-semibold", "Queues" }
                p { class: "text-sm text-muted-foreground",
                    "Sample traffic · Queue controls apply to this preview."
                }
            }
            QueueMonitor {
                name: "Order intake",
                source: "Store webhook",
                target: "Order service",
                paused: paused(),
                requests: requests.read().clone(),
                on_pause_change: move |value| paused.set(value),
            }
        }
    }
}

fn message(summary: &str, body: &str) -> QueueMessage {
    QueueMessage {
        summary: summary.into(),
        body: Some(body.into()),
        language: CodeLanguage::Json,
    }
}

fn sample_requests() -> Vec<QueueRequest> {
    use QueueStatus::*;
    [
        (
            "1046",
            "10:42:18",
            Queued,
            "Waiting for an available worker.",
        ),
        (
            "1045",
            "10:41:52",
            Running,
            "The target request is awaiting a response.",
        ),
        ("1044", "10:40:33", Done, "The target accepted the order."),
        (
            "1043",
            "10:39:26",
            Failed,
            "The target rejected the order because the currency was missing.",
        ),
        (
            "1042",
            "10:38:04",
            NeedsAttention,
            "The target response was lost. The outcome needs reconciliation.",
        ),
        (
            "1041",
            "10:37:12",
            Cancelled,
            "Cancelled before target dispatch.",
        ),
    ]
    .into_iter()
    .map(|(number, time, status, outcome)| {
        let id = format!("req-{number}");
        let currency = if status == Failed {
            ""
        } else {
            ",\n  \"currency\": \"EUR\""
        };
        let target_request =
            matches!(status, Running | Done | Failed | NeedsAttention).then(|| {
                message(
                    "POST /orders",
                    &format!("{{\n  \"externalId\": \"{number}\",\n  \"total\": 420{currency}\n}}"),
                )
            });
        let target_response = match status {
            Done => Some(message(
                "201 Created",
                r#"{"orderId":"SO-1044","status":"created"}"#,
            )),
            Failed => Some(message(
                "422 Unprocessable Content",
                r#"{"error":"currency_required"}"#,
            )),
            _ => None,
        };
        QueueRequest {
            operation: "Create order".into(),
            received_at: format!("2026-11-18 {time}"),
            status,
            outcome: outcome.into(),
            source_request: Some(message(
                "POST /hooks/orders",
                &format!("{{\n  \"order_id\": \"{number}\",\n  \"amount\": 420{currency}\n}}"),
            )),
            source_response: Some(message(
                "202 Accepted",
                &format!("{{\"request_id\":\"{id}\"}}"),
            )),
            id,
            target_request,
            target_response,
        }
    })
    .collect()
}
