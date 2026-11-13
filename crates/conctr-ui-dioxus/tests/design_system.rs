use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dom.mark_all_dirty();
    dom.render_immediate_to_vec();
    dioxus_ssr::render(&dom)
}

#[test]
fn button_loading_disables_submission_and_preserves_custom_attributes() {
    let html = render(|| {
        rsx! {
            Button { loading: true, class: "consumer-action", aria_label: "Create connection", "Create connection" }
        }
    });
    assert!(html.contains("disabled"));
    assert!(html.contains("aria-busy=true"));
    assert!(html.contains("consumer-action"));
    assert!(html.contains("cn-spinner"));
    assert!(html.contains("type=\"button\""));
}

#[test]
fn protocols_keep_brand_shapes_and_colors() {
    let html = render(|| {
        rsx! {
            for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] {
                ProtocolBadge { protocol }
            }
        }
    });
    for shape in ["circle", "square", "triangle", "diamond", "hex", "ring"] {
        assert!(html.contains(&format!("data-s=\"{shape}\"")));
    }
    for token in ["--cn-teal", "--cn-coral", "--cn-amber", "--cn-primary"] {
        assert!(html.contains(token));
    }
}

#[test]
fn tabs_allow_explicit_ssr_panel_associations() {
    let html = render(|| {
        rsx! {
            Tabs { value: None, default_value: "overview".to_string(),
                TabList { TabTrigger { id: "overview-tab", aria_controls: "overview-panel", value: "overview".to_string(), index: 0usize, "Overview" } }
                TabContent { id: Some("overview-panel".to_string()), aria_labelledby: "overview-tab", value: "overview".to_string(), index: 0usize, "Connection overview" }
            }
        }
    });
    assert!(html.contains("role=\"tablist\""));
    assert!(html.contains("role=\"tabpanel\""));
    assert!(html.contains("aria-selected=true"));
    assert!(html.contains("aria-controls="));
}
