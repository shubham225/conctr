use conctr_ui_dioxus::composed::visual_mapping::*;
fn schema(text: &str) -> MappingSchema {
    schema_from_json("System: \"A\"\nline two", "request", text).expect("valid fixture")
}
#[test]
fn infers_nested_union_without_sample_values() {
    let s = schema(r#"{"rows":[{"id":1},{"name":"private sample","id":"2"}],"a.b":true}"#);
    assert_eq!(s.fields.len(), 3);
    assert!(
        s.fields
            .iter()
            .any(|f| f.path == "$[\"rows\"][*][\"id\"]" && f.data_type == "mixed")
    );
    assert!(s.fields.iter().any(|f| f.path == "$[\"a.b\"]"));
    assert!(
        !serde_json::to_string(&s)
            .unwrap()
            .contains("private sample")
    );
    assert_eq!(
        serde_json::from_str::<MappingSchema>(&serde_json::to_string(&s).unwrap()).unwrap(),
        s
    );
}
#[test]
fn rejects_bad_and_oversized_json() {
    assert!(schema_from_json("a", "b", "{").is_err());
    assert!(schema_from_json("a", "b", &" ".repeat(1_048_577)).is_err());
}
#[test]
fn exports_quoted_paths_and_transform_configuration() {
    let s = schema(r#"{"id":1}"#);
    let t = schema(r#"{"number":1}"#);
    let links = vec![FieldLink {
        id: "one".into(),
        source: s.fields[0].path.clone(),
        target: t.fields[0].path.clone(),
        transform: Some(FieldTransform::ToNumber),
        transforms: vec![],
    }];
    let yaml = mapping_yaml(&s, &t, &links).unwrap();
    assert!(yaml.contains("apiVersion: conctr.dev/v1alpha1"));
    assert!(yaml.contains("type: \"to_number\""));
    assert!(yaml.contains("\\nline two"));
    assert!(yaml.contains("pathLanguage: jsonpath"));
    assert!(mapping_yaml(&s, &t, &[]).unwrap().contains("mappings: []"));
    let mut duplicate = links.clone();
    duplicate.push(FieldLink {
        id: "two".into(),
        ..links[0].clone()
    });
    assert!(mapping_yaml(&s, &t, &duplicate).is_err());
    let mut missing = links;
    missing[0].source = "missing".into();
    assert!(mapping_yaml(&s, &t, &missing).is_err());
}
#[test]
fn xml_adapter_contract_keeps_namespaces_and_paths() {
    let mut s = schema(r#"{"id":1}"#);
    s.format = PayloadFormat::Xml;
    s.fields[0].path = "/c:Customer/@id".into();
    s.namespaces.insert("c".into(), "urn:customer".into());
    let yaml = mapping_yaml(&s, &s, &[]).unwrap();
    assert!(yaml.contains("pathLanguage: xpath"));
    assert!(yaml.contains("\"c\": \"urn:customer\""));
}
#[test]
fn rejects_duplicate_schema_paths() {
    let mut s = schema(r#"{"id":1}"#);
    s.fields.push(s.fields[0].clone());
    assert!(validate_mapping(&s, &s, &[]).is_err());
}

use dioxus::document::{Document, Eval, LinkProps, NoOpDocument};
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};
#[derive(Default)]
struct RecordingDocument(RefCell<Vec<String>>);
impl Document for RecordingDocument {
    fn eval(&self, script: String) -> Eval {
        NoOpDocument.eval(script)
    }
    fn create_link(&self, props: LinkProps) {
        if props.rel.as_deref() == Some("stylesheet")
            && let Some(href) = props.href
        {
            self.0.borrow_mut().push(href);
        }
    }
}
#[component]
fn ReadOnlyConsumer() -> Element {
    rsx! {VisualFieldMapper {source:schema(r#"{"id":1}"#),target:schema(r#"{"id":1}"#),links:vec![],read_only:true,on_change: |_|{}}}
}
#[test]
fn css_registers_in_each_ssr_document_and_read_only_has_no_edit_controls() {
    for _ in 0..2 {
        let document = Rc::new(RecordingDocument::default());
        let mut dom = VirtualDom::new(ReadOnlyConsumer);
        dom.provide_root_context(document.clone() as Rc<dyn Document>);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert_eq!(document.0.borrow().len(), 1);
        assert!(html.contains("Download YAML"));
        assert!(!html.contains("New link"));
        assert!(!html.contains("Remove link"));
        assert!(html.contains("disabled"));
    }
}

fn draft() -> MappingDraft {
    let source = schema(r#"{"id":1,"name":""}"#);
    let target = schema(r#"{"id":1}"#);
    MappingDraft {
        links: vec![FieldLink {
            id: "one".into(),
            source: source.fields[0].path.clone(),
            target: target.fields[0].path.clone(),
            transform: Some(FieldTransform::Trim),
            transforms: vec!["custom:format".into(), "uppercase".into()],
        }],
        source,
        target,
    }
}
#[test]
fn ignores_and_removes_nodes_atomically_without_restoring_links() {
    let d = draft();
    let path = d.source.fields[0].path.clone();
    let ignored = d
        .edit_field(true, FieldEdit::ToggleIgnored(path.clone()))
        .unwrap();
    assert!(ignored.source.fields[0].ignored);
    assert!(ignored.links.is_empty());
    let yaml = mapping_yaml(&ignored.source, &ignored.target, &ignored.links).unwrap();
    assert!(yaml.contains("ignored:"));
    let restored = ignored
        .edit_field(true, FieldEdit::ToggleIgnored(path.clone()))
        .unwrap();
    assert!(!restored.source.fields[0].ignored);
    assert!(restored.links.is_empty());
    let removed = draft()
        .edit_field(
            false,
            FieldEdit::Remove(draft().target.fields[0].path.clone()),
        )
        .unwrap();
    assert!(removed.target.fields.is_empty());
    assert!(removed.links.is_empty());
}
#[test]
fn rejects_empty_or_duplicate_nodes_and_keeps_custom_transform_order() {
    let d = draft();
    assert!(
        d.clone()
            .edit_field(true, FieldEdit::Add(d.source.fields[0].clone()))
            .is_err()
    );
    assert!(
        d.clone()
            .edit_field(
                true,
                FieldEdit::Add(SchemaField {
                    path: "".into(),
                    label: "".into(),
                    data_type: "string".into(),
                    depth: 0,
                    ignored: false
                })
            )
            .is_err()
    );
    let pipeline = d.links[0].pipeline();
    assert_eq!(pipeline, vec!["trim", "custom:format", "uppercase"]);
    let yaml = mapping_yaml(&d.source, &d.target, &d.links).unwrap();
    assert!(yaml.find("\"trim\"").unwrap() < yaml.find("\"custom:format\"").unwrap());
    let decoded: FieldLink =
        serde_json::from_str(r#"{"id":"old","source":"a","target":"b","transform":"trim"}"#)
            .unwrap();
    assert_eq!(decoded.pipeline(), vec!["trim"]);
}

#[test]
fn editor_updates_paths_and_reorders_without_changing_link_semantics() {
    let source = schema(r#"{"a":"", "b":1}"#);
    let target = schema(r#"{"x":""}"#);
    let mut node = source.fields[0].clone();
    let old = node.path.clone();
    let destination = target.fields[0].path.clone();
    let draft = MappingDraft {
        source,
        target,
        links: vec![FieldLink {
            id: "stable".into(),
            source: old.clone(),
            target: destination,
            transform: None,
            transforms: vec!["custom".into(), "trim".into()],
        }],
    };
    node.path = "$['renamed']".into();
    node.label = "Renamed".into();
    node.data_type = "object".into();
    let draft = draft
        .edit_field(
            true,
            FieldEdit::Update {
                path: old,
                field: node.clone(),
            },
        )
        .unwrap();
    assert_eq!(draft.links[0].source, node.path);
    let draft = draft
        .edit_field(
            true,
            FieldEdit::Move {
                path: node.path.clone(),
                before: None,
            },
        )
        .unwrap();
    assert_eq!(draft.source.fields.last().unwrap().path, node.path);
    assert_eq!(draft.links[0].pipeline(), ["custom", "trim"]);
    let encoded = serde_json::to_string(&draft).unwrap();
    assert_eq!(
        serde_json::from_str::<MappingDraft>(&encoded).unwrap(),
        draft
    );
    assert!(
        draft
            .clone()
            .edit_field(
                true,
                FieldEdit::Move {
                    path: node.path.clone(),
                    before: Some("missing".into())
                }
            )
            .is_err()
    );
    node.data_type = "unsupported".into();
    assert!(
        draft
            .edit_field(
                true,
                FieldEdit::Update {
                    path: node.path.clone(),
                    field: node
                }
            )
            .is_err()
    );
}
