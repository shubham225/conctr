//! Caller-side import example. JSON samples are inferred here; XML adapters pass
//! the same MappingSchema with XPath fields instead of raw XML to the component.
use crate::composed::visual_mapping::*;
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;
const SOURCE: &str = r#"{"customer":{"id":"C-1042","name":"Asha Kumar","email":"asha@example.test","address":{"city":"Pune"}}}"#;
const TARGET: &str = r#"{"account":{"number":"","displayName":"","email":"","city":""}}"#;
fn initial() -> (MappingSchema, MappingSchema, Vec<FieldLink>) {
    let source = schema_from_json("Customer portal", "Response body", SOURCE)
        .unwrap_or_else(|_| xml_source());
    let target =
        schema_from_json("ERP system", "Request body", TARGET).unwrap_or_else(|_| xml_source());
    let links = vec![FieldLink {
        id: "customer-id".into(),
        source: "$[\"customer\"][\"id\"]".into(),
        target: "$[\"account\"][\"number\"]".into(),
        transform: None,
        transforms: vec![],
    }];
    (source, target, links)
}
fn xml_source() -> MappingSchema {
    MappingSchema {
        name: "Order service".into(),
        message: "SOAP response · normalized XML".into(),
        format: PayloadFormat::Xml,
        namespaces: Default::default(),
        fields: vec![
            SchemaField {
                path: "/Order/Customer/Id/text()".into(),
                label: "Customer ID".into(),
                data_type: "string".into(),
                depth: 1,
                ignored: false,
            },
            SchemaField {
                path: "/Order/Customer/Name/text()".into(),
                label: "Customer name".into(),
                data_type: "string".into(),
                depth: 1,
                ignored: false,
            },
            SchemaField {
                path: "/Order/Customer/Email/text()".into(),
                label: "Email".into(),
                data_type: "string".into(),
                depth: 1,
                ignored: false,
            },
        ],
    }
}
#[component]
pub fn VisualMappingDemo() -> Element {
    let mut state = use_signal(initial);
    let mut source_text = use_signal(|| SOURCE.to_owned());
    let mut target_text = use_signal(|| TARGET.to_owned());
    let mut error = use_signal(|| None::<String>);
    let mut review = use_signal(|| false);
    let (source, target, links) = state();
    rsx! {
        div { class: "grid min-w-0 gap-4",
            details { class: "rounded-lg border border-border bg-card p-4",
                summary { class: "cursor-pointer text-sm font-medium", "Import payloads" }
                p { class: "my-4 text-sm text-muted-foreground",
                    "Paste JSON samples or load a normalized XML example. Importing replaces the current mapping draft."
                }
                div { class: "grid gap-4 md:grid-cols-2",
                    label { class: "grid gap-2 text-sm",
                        "Source response · JSON sample"
                        textarea {
                            class: "min-h-32 rounded-md border border-border bg-background p-3 font-mono text-xs",
                            value: source_text(),
                            oninput: move |e| source_text.set(e.value()),
                            "aria-label": "Source JSON sample",
                        }
                    }
                    label { class: "grid gap-2 text-sm",
                        "Target request · JSON sample"
                        textarea {
                            class: "min-h-32 rounded-md border border-border bg-background p-3 font-mono text-xs",
                            value: target_text(),
                            oninput: move |e| target_text.set(e.value()),
                            "aria-label": "Target JSON sample",
                        }
                    }
                }
                div { class: "mt-4 flex flex-wrap gap-3",
                    Button {
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            match (
                                schema_from_json("Customer portal", "Response body", &source_text()),
                                schema_from_json("ERP system", "Request body", &target_text()),
                            ) {
                                (Ok(source), Ok(target)) => {
                                    state.set((source, target, vec![]));
                                    error.set(None);
                                }
                                (Err(e), _) => error.set(Some(format!("Source: {e}"))),
                                (_, Err(e)) => error.set(Some(format!("Target: {e}"))),
                            }
                        },
                        "Load JSON samples"
                    }
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            let (_, target, _) = initial();
                            state.set((xml_source(), target, vec![]));
                            error.set(None);
                        },
                        "Load normalized XML example"
                    }
                }
                if let Some(error) = error() {
                    p { class: "mt-4 text-sm text-destructive", role: "alert", "{error}" }
                }
            }
            div { class: "flex flex-wrap items-center justify-between gap-4",
                p { class: "text-sm text-muted-foreground",
                    "Drag a dot to a target to connect, or activate a source dot and target field. Select a field or wire to open its tools."
                }
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| review.set(!review()),
                    if review() {
                        "Edit mapping"
                    } else {
                        "Review mapping"
                    }
                }
            }
            VisualFieldMapper {
                source,
                target,
                links,
                read_only: review(),
                on_change: move |links| state.write().2 = links,
                on_schema_change:move |draft:MappingDraft| state.set((draft.source,draft.target,draft.links)),
            }
            p { class: "text-xs text-muted-foreground",
                "Mapping configuration only. Transform execution and type validation belong to the calling application."
            }
        }
    }
}
#[component]
pub fn ReadOnlyMappingDemo() -> Element {
    let (source, target, links) = initial();
    rsx! {
        VisualFieldMapper {
            source,
            target,
            links,
            read_only: true,
            on_change: |_| {},
        }
    }
}
