use conctr_ui_dioxus::ui::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
    checkbox::Checkbox,
    data_grid::{Grid, GridCell, GridHeaderCell},
    data_table::DataTable,
    input::Input,
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow, TableWrapper},
};
use dioxus::prelude::*;

const CONNECTIONS: [(&str, &str, &str, &str, &str); 6] = [
    (
        "Orders to ERP",
        "Salesforce → SAP S/4HANA",
        "Healthy",
        "REST",
        "2 minutes ago",
    ),
    (
        "Customer directory",
        "HubSpot → PostgreSQL",
        "Healthy",
        "REST",
        "8 minutes ago",
    ),
    (
        "Inventory updates",
        "Warehouse → SAP S/4HANA",
        "Degraded",
        "SOAP",
        "12 minutes ago",
    ),
    (
        "Invoice delivery",
        "Billing → Finance",
        "Healthy",
        "OData",
        "24 minutes ago",
    ),
    (
        "Shipment events",
        "Fulfilment → Operations",
        "Draft",
        "Webhook",
        "Not run yet",
    ),
    (
        "Product catalog",
        "Commerce → Warehouse",
        "Failing",
        "GraphQL",
        "1 hour ago",
    ),
];

#[component]
fn ConnectionName(index: usize) -> Element {
    let (name, systems, _, _, _) = CONNECTIONS[index];
    rsx! {
        div { class: "cn-data-identity",
            span { class: "cn-data-symbol",
                icons::Workflow { class: "size-4" }
            }
            div {
                strong { "{name}" }
                span { "{systems}" }
            }
        }
    }
}

#[component]
fn ConnectionStatus(status: &'static str) -> Element {
    let variant = match status {
        "Healthy" => BadgeVariant::Success,
        "Degraded" => BadgeVariant::Warning,
        "Failing" => BadgeVariant::Destructive,
        _ => BadgeVariant::Secondary,
    };
    rsx! {
        Badge { variant,
            if status == "Healthy" {
                icons::Check { class: "size-3" }
            } else if status == "Draft" {
                icons::Minus { class: "size-3" }
            } else {
                icons::CircleAlert { class: "size-3" }
            }
            "{status}"
        }
    }
}

/// A read-only table with a clear primary column and contextual metadata.
#[component]
pub fn TableDemo() -> Element {
    rsx! {
        ConnectionTable { interactive: false }
    }
}

/// Filtering, sorting and selection are local demo state.
#[component]
pub fn DataTableDemo() -> Element {
    rsx! {
        ConnectionTable { interactive: true }
    }
}

#[component]
fn ConnectionTable(interactive: bool) -> Element {
    let mut query = use_signal(String::new);
    let mut healthy_only = use_signal(|| false);
    let mut descending = use_signal(|| false);
    let mut selected = use_signal(Vec::<usize>::new);
    let query_value = query().to_lowercase();
    let mut rows: Vec<_> = CONNECTIONS
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            (!healthy_only() || row.2 == "Healthy")
                && format!("{} {}", row.0, row.1)
                    .to_lowercase()
                    .contains(&query_value)
        })
        .map(|(index, _)| index)
        .collect();
    rows.sort_by_key(|index| CONNECTIONS[*index].0);
    if descending() {
        rows.reverse();
    }
    let count = rows.len();
    let all_selected = count > 0 && rows.iter().all(|index| selected.read().contains(index));
    let rows_for_selection = rows.clone();
    let content = rsx! {
        TableHeader {
            TableRow {
                if interactive {
                    TableHead { class: "cn-data-check",
                        Checkbox {
                            aria_label: "Select all visible connections",
                            checked: all_selected,
                            on_checked_change: move |checked| {
                                selected
                                    .with_mut(|items| {
                                        for index in &rows_for_selection {
                                            if checked && !items.contains(index) {
                                                items.push(*index);
                                            } else if !checked {
                                                items.retain(|item| item != index);
                                            }
                                        }
                                    })
                            },
                        }
                    }
                }
                TableHead {
                    if interactive {
                        button {
                            class: "cn-data-sort",
                            onclick: move |_| descending.set(!descending()),
                            "Connection"
                            if descending() {
                                icons::ArrowDown { class: "size-3" }
                            } else {
                                icons::ArrowUp { class: "size-3" }
                            }
                        }
                    } else {
                        "Connection"
                    }
                }
                TableHead { "Status" }
                TableHead { "Protocol" }
                TableHead { class: "cn-data-last", "Last activity" }
            }
        }
        TableBody {
            for index in rows {
                TableRow {
                    key: "{index}",
                    data_state: if selected.read().contains(&index) { "selected" } else { "idle" },
                    if interactive {
                        TableCell { class: "cn-data-check",
                            Checkbox {
                                aria_label: format!("Select {}", CONNECTIONS[index].0),
                                checked: selected.read().contains(&index),
                                on_checked_change: move |checked| {
                                    selected
                                        .with_mut(|items| {
                                            if checked && !items.contains(&index) {
                                                items.push(index);
                                            } else if !checked {
                                                items.retain(|item| *item != index);
                                            }
                                        })
                                },
                            }
                        }
                    }
                    TableCell {
                        ConnectionName { index }
                    }
                    TableCell {
                        ConnectionStatus { status: CONNECTIONS[index].2 }
                    }
                    TableCell {
                        span { class: "cn-data-protocol", "{CONNECTIONS[index].3}" }
                    }
                    TableCell { class: "cn-data-last", "{CONNECTIONS[index].4}" }
                }
            }
        }
    };
    rsx! {
        section { class: "cn-data-panel", "aria-label": "Example connections",
            header { class: "cn-data-heading",
                div {
                    h2 {
                        "Connections"
                        span { class: "cn-data-count", "6" }
                    }
                    p { "A shared view of your connected systems." }
                }
                span { class: "cn-data-demo-label", "Demo data" }
            }
            if interactive {
                div { class: "cn-data-toolbar",
                    div { class: "cn-data-search",
                        icons::Search { class: "size-4" }
                        Input {
                            id: "connection-search",
                            placeholder: "Find a connection…",
                            value: query(),
                            oninput: move |event: FormEvent| query.set(event.value()),
                        }
                        label { r#for: "connection-search", class: "sr-only", "Find a connection" }
                    }
                    button {
                        class: "cn-data-filter",
                        "aria-pressed": healthy_only().to_string(),
                        onclick: move |_| healthy_only.set(!healthy_only()),
                        icons::ListFilter { class: "size-4" }
                        "Healthy"
                        if healthy_only() {
                            span { "3" }
                        }
                    }
                }
            }
            TableWrapper {
                if interactive {
                    DataTable { {content} }
                } else {
                    Table { {content} }
                }
            }
            if count == 0 {
                div { class: "cn-data-empty",
                    icons::Search { class: "size-5" }
                    strong { "No connections found" }
                    p { "Try another name or clear the Healthy filter." }
                }
            }
            footer { class: "cn-data-footer",
                span {
                    if interactive && !selected.read().is_empty() {
                        "{selected.read().len()} selected · "
                    }
                    "{count} of 6 connections"
                }
                span {
                    icons::CircleHelp { class: "size-4" }
                    "Example workspace"
                }
            }
        }
    }
}

const MAPPINGS: [(&str, &str, &str, bool); 5] = [
    ("Customer ID", "customer.id", "account.external_id", true),
    ("Email address", "customer.email", "account.email", true),
    (
        "Display name",
        "customer.name",
        "account.display_name",
        true,
    ),
    ("Company", "customer.company", "account.company", false),
    ("Country", "address.country", "account.country_code", false),
];

/// An editable grid using the facade's Grid, GridHeaderCell and GridCell primitives.
#[component]
pub fn DataGridDemo() -> Element {
    let rowcount = use_signal(|| 6_i32);
    let mut targets = use_signal(|| {
        MAPPINGS
            .iter()
            .map(|row| row.2.to_string())
            .collect::<Vec<_>>()
    });
    let changed = targets
        .read()
        .iter()
        .zip(MAPPINGS)
        .filter(|(target, row)| target.as_str() != row.2)
        .count();
    rsx! {
        section { class: "cn-data-panel", "aria-label": "Example field mappings",
            header { class: "cn-data-heading",
                div {
                    h2 {
                        "Field mappings"
                        span { class: "cn-data-count", "5" }
                    }
                    p { "Match source fields to their destination." }
                }
                span { class: "cn-data-demo-label", "Demo data" }
            }
            div { class: "cn-data-toolbar",
                span { class: "cn-data-grid-hint",
                    icons::MousePointer2 { class: "size-4" }
                    "Select a target path to edit"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    disabled: changed == 0,
                    onclick: move |_| {
                        targets.set(MAPPINGS.iter().map(|row| row.2.to_string()).collect());
                    },
                    icons::RotateCcw { class: "size-4" }
                    "Reset changes"
                }
            }
            Grid {
                rowcount,
                colcount: 4,
                style: "--header-field-size:220;--header-source-size:230;--header-target-size:270;--header-required-size:128;--col-field-size:220;--col-source-size:230;--col-target-size:270;--col-required-size:128",
                div { role: "rowgroup",
                    div {
                        role: "row",
                        class: "cn-mapping-row",
                        "aria-rowindex": "1",
                        for (index, column, label) in [
                            (1, "field", "Field"),
                            (2, "source", "Source path"),
                            (3, "target", "Target path"),
                            (4, "required", "Requirement"),
                        ]
                        {
                            GridHeaderCell { colindex: index, column, "{label}" }
                        }
                    }
                }
                div { role: "rowgroup",
                    for (index, (field, source, _, required)) in MAPPINGS.iter().copied().enumerate() {
                        div {
                            key: "{index}",
                            role: "row",
                            class: "cn-mapping-row",
                            "data-name": "GridRow",
                            "aria-rowindex": (index + 2).to_string(),
                            GridCell { colindex: 1, column: "field",
                                div { class: "cn-mapping-field",
                                    span { "{index + 1:02}" }
                                    strong { "{field}" }
                                }
                            }
                            GridCell { colindex: 2, column: "source",
                                code { class: "cn-mapping-path", "{source}" }
                            }
                            MappingTarget { key: "{index}", index, targets }
                            GridCell { colindex: 4, column: "required",
                                span { class: "cn-mapping-requirement",
                                    if required {
                                        "Required"
                                    } else {
                                        "Optional"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            footer { class: "cn-data-footer",
                span { "5 mappings · 3 required fields" }
                span {
                    if changed == 0 {
                        icons::Check { class: "size-4" }
                        "No changes"
                    } else {
                        "{changed} edited"
                    }
                }
            }
        }
    }
}

#[component]
fn MappingTarget(index: usize, mut targets: Signal<Vec<String>>) -> Element {
    let mut editing = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let field = MAPPINGS[index].0;
    let mut commit = move || {
        if editing() {
            targets.with_mut(|items| items[index] = draft());
            editing.set(false);
        }
    };
    rsx! {
        GridCell { colindex: 3, column: "target", active: editing(),
            if editing() {
                input {
                    class: "cn-mapping-edit",
                    "aria-label": format!("Target path for {field}"),
                    value: draft(),
                    onmounted: move |event| {
                        spawn(async move {
                            let _ = event.set_focus(true).await;
                        });
                    },
                    oninput: move |event| draft.set(event.value()),
                    onfocusout: move |_| commit(),
                    onkeydown: move |event| match event.key() {
                        Key::Enter => commit(),
                        Key::Escape => editing.set(false),
                        _ => {}
                    },
                }
            } else {
                button {
                    class: "cn-mapping-target",
                    "aria-label": format!("Edit target path for {field}"),
                    onclick: move |_| {
                        draft.set(targets.read()[index].clone());
                        editing.set(true);
                    },
                    code { "{targets.read()[index]}" }
                    icons::Pencil { class: "size-3" }
                }
            }
        }
    }
}
