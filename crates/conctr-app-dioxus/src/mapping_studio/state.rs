use conctr_ui_dioxus::composed::visual_mapping::{
    MappingDraft, MappingSchema, PayloadFormat, SchemaField,
};
use std::collections::BTreeMap;

/// One presentation document contains both directions; history restores them atomically.
#[derive(Clone, PartialEq)]
pub(super) struct Document {
    pub request: MappingDraft,
    pub response: MappingDraft,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            request: MappingDraft {
                source: schema(
                    "Order API",
                    "Request",
                    &[
                        "order_id",
                        "customer_id",
                        "email",
                        "name",
                        "currency",
                        "total",
                        "status",
                        "city",
                        "country",
                        "notes",
                    ],
                ),
                target: schema(
                    "Sales order API",
                    "Request",
                    &[
                        "external_id",
                        "contact_email",
                        "display_name",
                        "currency",
                        "net_amount",
                        "status",
                        "city",
                        "country",
                        "notes",
                    ],
                ),
                links: vec![],
            },
            response: MappingDraft {
                source: schema("Target API", "Response", &["id", "status", "created_at"]),
                target: schema(
                    "Source API",
                    "Response",
                    &["record_id", "state", "timestamp"],
                ),
                links: vec![],
            },
        }
    }
}
fn schema(name: &str, message: &str, fields: &[&str]) -> MappingSchema {
    MappingSchema {
        name: name.into(),
        message: message.into(),
        format: PayloadFormat::Json,
        namespaces: BTreeMap::new(),
        fields: fields
            .iter()
            .map(|name| SchemaField {
                path: format!("$['{name}']"),
                label: (*name).into(),
                data_type: "string".into(),
                depth: 0,
                ignored: false,
            })
            .collect(),
    }
}
#[derive(Clone, Default)]
pub(super) struct History {
    pub current: Document,
    pub past: Vec<Document>,
    pub future: Vec<Document>,
}
impl History {
    pub fn update(&mut self, response: bool, draft: MappingDraft) {
        let mut next = self.current.clone();
        if response {
            next.response = draft;
        } else {
            next.request = draft;
        }
        if next == self.current {
            return;
        }
        self.past.push(std::mem::replace(&mut self.current, next));
        if self.past.len() > 100 {
            self.past.remove(0);
        }
        self.future.clear();
    }
    pub fn undo(&mut self) {
        if let Some(previous) = self.past.pop() {
            self.future
                .push(std::mem::replace(&mut self.current, previous));
        }
    }
    pub fn redo(&mut self) {
        if let Some(next) = self.future.pop() {
            self.past.push(std::mem::replace(&mut self.current, next));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn history_keeps_directions_independent_and_discards_redo_after_edit() {
        let mut history = History::default();
        let original = history.current.clone();
        let mut request = original.request.clone();
        request.source.name = "Changed source".into();
        history.update(false, request.clone());
        assert_eq!(history.current.response, original.response);
        history.undo();
        assert!(history.current == original);
        history.redo();
        assert_eq!(history.current.request, request);
        history.undo();
        let mut response = original.response.clone();
        response.target.name = "Changed response".into();
        history.update(true, response.clone());
        assert!(history.future.is_empty());
        assert_eq!(history.current.request, original.request);
        assert_eq!(history.current.response, response);
    }
}
