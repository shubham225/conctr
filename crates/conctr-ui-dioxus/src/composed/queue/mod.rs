//! Queue monitoring over caller-supplied request summaries and safe payload previews.
use crate::ui::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    card::Card,
    code_block::{CodeBlock, CodeLanguage},
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow, TableWrapper},
};
use dioxus::prelude::*;

/// Status reported by the queue's owner.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QueueStatus {
    Queued,
    Running,
    Done,
    Failed,
    NeedsAttention,
    Cancelled,
}

impl QueueStatus {
    const ALL: [Self; 6] = [
        Self::Queued,
        Self::Running,
        Self::Done,
        Self::Failed,
        Self::NeedsAttention,
        Self::Cancelled,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Running => "Running",
            Self::Done => "Done",
            Self::Failed => "Failed",
            Self::NeedsAttention => "Needs attention",
            Self::Cancelled => "Cancelled",
        }
    }

    fn variant(self) -> BadgeVariant {
        match self {
            Self::Queued | Self::Cancelled => BadgeVariant::Secondary,
            Self::Running => BadgeVariant::Info,
            Self::Done => BadgeVariant::Success,
            Self::Failed => BadgeVariant::Destructive,
            Self::NeedsAttention => BadgeVariant::Warning,
        }
    }
}

/// A captured exchange stage. Only pass payload text approved for display.
#[derive(Clone, PartialEq, Eq)]
pub struct QueueMessage {
    pub summary: String,
    pub body: Option<String>,
    pub language: CodeLanguage,
}

/// One history row and its source/target exchange. IDs are unique within a queue.
#[derive(Clone, PartialEq, Eq)]
pub struct QueueRequest {
    pub id: String,
    pub operation: String,
    pub received_at: String,
    pub status: QueueStatus,
    pub outcome: String,
    pub source_request: Option<QueueMessage>,
    pub source_response: Option<QueueMessage>,
    pub target_request: Option<QueueMessage>,
    pub target_response: Option<QueueMessage>,
}

/// Snapshot monitor. Counts cover the supplied records, independently of the history
/// filter. The caller applies pause/resume and provides updated snapshots.
#[component]
pub fn QueueMonitor(
    name: String,
    source: String,
    target: String,
    paused: bool,
    requests: Vec<QueueRequest>,
    on_pause_change: EventHandler<bool>,
) -> Element {
    let mut filter = use_signal(|| None::<QueueStatus>);
    let mut selected = use_signal(|| None::<String>);
    let current_filter = filter();
    let visible: Vec<_> = requests
        .iter()
        .filter(|request| current_filter.is_none_or(|status| status == request.status))
        .collect();
    let selected_id = selected.read();
    let current = visible
        .iter()
        .find(|request| selected_id.as_deref() == Some(request.id.as_str()))
        .copied()
        .or_else(|| visible.first().copied());
    let counts = QueueStatus::ALL.map(|status| {
        (
            status,
            requests
                .iter()
                .filter(|request| request.status == status)
                .count(),
        )
    });

    rsx! {
        section { class: "grid min-w-0 gap-4", "aria-label": "Queue monitor",
            Card { class: "flex flex-wrap items-center justify-between gap-4 p-4",
                div { class: "grid gap-2",
                    div { class: "flex flex-wrap items-center gap-2",
                        h2 { class: "text-lg font-semibold", "{name}" }
                        Badge { variant: if paused { BadgeVariant::Warning } else { BadgeVariant::Success },
                            if paused { "Paused" } else { "Active" }
                        }
                    }
                    p { class: "text-sm text-muted-foreground", "{source} → {target}" }
                    p { class: "text-xs text-muted-foreground", role: "status",
                        if paused { "New dispatch is paused. In-flight requests can finish." }
                        else { "Queued requests can be dispatched." }
                    }
                }
                Button {
                    variant: if paused { ButtonVariant::Default } else { ButtonVariant::Outline },
                    onclick: move |_| on_pause_change.call(!paused),
                    if paused { "Resume queue" } else { "Pause queue" }
                }
            }

            div { class: "grid grid-cols-2 gap-3 md:grid-cols-3 xl:grid-cols-6", "aria-label": "Queue totals",
                for (status, count) in counts {
                    Card { key: "{status.label()}", class: "grid gap-2 p-3",
                        span { class: "text-xs text-muted-foreground", "{status.label()}" }
                        strong { class: "text-2xl font-semibold tabular-nums", "{count}" }
                    }
                }
            }

            Card { class: "min-w-0",
                header { class: "flex flex-wrap items-center justify-between gap-3 p-4",
                    div {
                        h2 { class: "text-base font-semibold", "Request history" }
                        p { class: "text-xs text-muted-foreground", "{visible.len()} of {requests.len()} requests" }
                    }
                    label { class: "flex items-center gap-2 text-sm",
                        "Status"
                        select {
                            class: "h-9 rounded-md border border-input bg-background px-2 text-sm",
                            value: current_filter.map_or("All statuses", QueueStatus::label),
                            onchange: move |event| {
                                let label = event.value();
                                filter.set(QueueStatus::ALL.into_iter().find(|status| status.label() == label));
                                selected.set(None);
                            },
                            option { value: "All statuses", "All statuses" }
                            for status in QueueStatus::ALL {
                                option { value: status.label(), "{status.label()}" }
                            }
                        }
                    }
                }
                if visible.is_empty() {
                    p { class: "border-t border-border p-4 text-sm text-muted-foreground",
                        "No requests match this status."
                    }
                } else {
                    TableWrapper {
                        Table {
                            TableHeader {
                                TableRow {
                                    TableHead { "Request" }
                                    TableHead { "Received (UTC)" }
                                    TableHead { "Status" }
                                }
                            }
                            TableBody {
                                for request in visible {
                                    TableRow { key: "{request.id}",
                                        TableCell {
                                            Button {
                                                variant: if current.is_some_and(|row| row.id == request.id) {
                                                    ButtonVariant::Secondary
                                                } else { ButtonVariant::Ghost },
                                                size: ButtonSize::Sm,
                                                aria_label: format!("Inspect request {}", request.id),
                                                onclick: {
                                                    let id = request.id.clone();
                                                    move |_| selected.set(Some(id.clone()))
                                                },
                                                "{request.id}"
                                            }
                                            span { class: "ml-2 text-sm", "{request.operation}" }
                                        }
                                        TableCell { "{request.received_at}" }
                                        TableCell { Badge { variant: request.status.variant(), "{request.status.label()}" } }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(request) = current {
                section { class: "grid min-w-0 gap-3", "aria-label": "Request exchange",
                    header {
                        h2 { class: "text-base font-semibold", "Exchange · {request.id}" }
                        p { class: "text-sm text-muted-foreground", "{request.outcome}" }
                    }
                    div { class: "grid min-w-0 gap-3 lg:grid-cols-2",
                        {message_preview("Request from source", request.source_request.as_ref())}
                        {message_preview("Response to source", request.source_response.as_ref())}
                        {message_preview("Request to target", request.target_request.as_ref())}
                        {message_preview("Response from target", request.target_response.as_ref())}
                    }
                }
            }
        }
    }
}

fn message_preview(label: &str, message: Option<&QueueMessage>) -> Element {
    rsx! {
        Card { class: "grid min-w-0 gap-2 p-4",
            h3 { class: "text-sm font-semibold", "{label}" }
            if let Some(message) = message {
                p { class: "font-mono text-xs text-muted-foreground", "{message.summary}" }
                if let Some(body) = &message.body {
                    CodeBlock { code: body.clone(), language: message.language, label: Some(label.to_owned()) }
                } else {
                    p { class: "text-sm text-muted-foreground", "Payload unavailable." }
                }
            } else {
                p { class: "text-sm text-muted-foreground", "No message recorded." }
            }
        }
    }
}
