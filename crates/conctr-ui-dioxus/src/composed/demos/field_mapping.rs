use crate::composed::{FieldMappingEditor, MappingField, MappingLink, MappingTransform};
use dioxus::prelude::*;

#[component]
pub fn FieldMappingDemo() -> Element {
    let mut mappings = use_signal(|| {
        vec![
            link("customer-id", "customerId", "CustomerNumber", None),
            link("customer-name", "name", "CustomerName", Some("trim")),
            link("customer-email", "email", "EmailAddress", Some("lowercase")),
        ]
    });
    rsx! {
        FieldMappingEditor {
            source_label: "Customer portal",
            source_caption: "Response body",
            target_label: "ERP system",
            target_caption: "Create customer · request body",
            source_fields: vec![
                field("customerId", "Customer ID", "$.customer.id", "string"),
                field("name", "Full name", "$.customer.name", "string"),
                field("email", "Email", "$.customer.email", "string"),
                field("creditLimit", "Credit limit", "$.customer.creditLimit", "number"),
            ],
            target_fields: vec![
                field("CustomerNumber", "Customer number", "$.CustomerNumber", "string"),
                field("CustomerName", "Customer name", "$.CustomerName", "string"),
                field("EmailAddress", "Email address", "$.EmailAddress", "string"),
                field("CreditLimit", "Credit limit", "$.CreditLimit", "number"),
            ],
            transforms: transforms(),
            mappings: mappings(),
            on_change: move |updated| mappings.set(updated),
        }
        p { class: "mt-4 text-sm text-muted-foreground",
            "Local mapping draft. Transform choices are configuration only; this demo does not execute them."
        }
    }
}

/// A second direction uses the same editor and caller-owned mapping model.
#[component]
pub fn ResponseMappingDemo() -> Element {
    let mut mappings = use_signal(|| {
        vec![
            link("response-id", "CustomerNumber", "id", None),
            link("response-name", "CustomerName", "displayName", Some("trim")),
        ]
    });
    rsx! {
        FieldMappingEditor {
            source_label: "ERP system",
            source_caption: "Create customer · response body",
            target_label: "Customer portal",
            target_caption: "Response body",
            source_fields: vec![
                field("CustomerNumber", "Customer number", "$.CustomerNumber", "string"),
                field("CustomerName", "Customer name", "$.CustomerName", "string"),
            ],
            target_fields: vec![
                field("id", "Customer ID", "$.customer.id", "string"),
                field("displayName", "Display name", "$.customer.displayName", "string"),
            ],
            transforms: transforms(),
            mappings: mappings(),
            on_change: move |updated| mappings.set(updated),
        }
    }
}

fn field(id: &str, label: &str, path: &str, data_type: &str) -> MappingField {
    MappingField {
        id: id.into(),
        label: label.into(),
        path: path.into(),
        data_type: data_type.into(),
    }
}
fn link(id: &str, source: &str, target: &str, transform: Option<&str>) -> MappingLink {
    MappingLink {
        id: id.into(),
        source: source.into(),
        target: target.into(),
        transform: transform.map(str::to_owned),
    }
}
fn transforms() -> Vec<MappingTransform> {
    vec![
        MappingTransform {
            id: "trim".into(),
            label: "Trim whitespace".into(),
            description: "Remove leading and trailing whitespace.".into(),
        },
        MappingTransform {
            id: "lowercase".into(),
            label: "Lowercase".into(),
            description: "Convert text to lowercase.".into(),
        },
        MappingTransform {
            id: "number".into(),
            label: "To number".into(),
            description: "Configure a numeric conversion; validate in the owning application."
                .into(),
        },
    ]
}
