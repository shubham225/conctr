use conctr_ui_dioxus::composed::{FieldMappingEditor, MappingField, MappingLink, MappingTransform};
use dioxus::prelude::*;

fn field(id: &str) -> MappingField {
    MappingField {
        id: id.into(),
        label: id.into(),
        path: format!("$.{id}"),
        data_type: "string".into(),
    }
}

#[component]
fn MappingConsumer(#[props(default)] missing: bool) -> Element {
    rsx! {
        FieldMappingEditor {
            source_label: "Source",
            target_label: "Target",
            source_fields: vec![field("first-source"), field("selected-source")],
            target_fields: vec![field("first-target"), field("selected-target")],
            transforms: vec![MappingTransform {
                id: "trim".into(),
                label: "Trim".into(),
                description: "Trim whitespace".into(),
            }],
            mappings: vec![MappingLink {
                id: "one".into(),
                source: if missing { "removed-source".into() } else { "selected-source".into() },
                target: if missing { "removed-target".into() } else { "selected-target".into() },
                transform: Some(if missing { "removed-transform".into() } else { "trim".into() }),
            }],
            read_only: true,
            on_change: |_| {},
        }
    }
}

fn render(missing: bool) -> String {
    let mut dom = VirtualDom::new_with_props(MappingConsumer, MappingConsumerProps { missing });
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn initial_html_selects_caller_values_before_hydration() {
    let html = render(false);
    let selected: Vec<_> = html
        .split("<option")
        .skip(1)
        .filter_map(|part| part.split_once('>').map(|(attributes, _)| attributes))
        .filter(|attributes| {
            attributes
                .split_ascii_whitespace()
                .any(|attribute| attribute == "selected" || attribute.starts_with("selected="))
        })
        .collect();
    assert_eq!(selected.len(), 3);
    for value in ["selected-source", "selected-target", "trim"] {
        assert!(
            selected
                .iter()
                .any(|option| option.contains(&format!("value=\"{value}\""))),
            "Missing initial selection for {value}"
        );
    }
    assert!(!selected.iter().any(|option| option.contains("first-")));
    assert!(!html.contains("Add mapping"));
    assert!(!html.contains("Remove mapping"));
    assert_eq!(html.matches("<select").count(), 3);
    assert_eq!(html.matches("disabled").count(), 3);
}

#[test]
fn removed_schema_references_render_explicit_missing_selections() {
    let html = render(true);
    for label in [
        "Missing source field",
        "Missing target field",
        "Missing transform",
    ] {
        assert!(html.contains(label));
    }
    assert!(html.contains("aria-invalid=\"true\""));
}
