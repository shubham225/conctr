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
fn themes_preserve_scope_and_consumer_attributes_when_nested() {
    let html = render(|| {
        rsx! {
            ThemeProvider { theme: Theme::Dark, class: "consumer-theme", "data-brand": "ocean",
                Button { class: "consumer-action", "Save" }
                ThemeProvider { theme: Theme::Light, id: "light-preview",
                    Input { aria_label: "Endpoint", class: "consumer-field" }
                }
            }
        }
    });
    for attribute in [
        "cn-body",
        "consumer-theme",
        "data-brand=\"ocean\"",
        "data-theme=\"dark\"",
        "data-theme=\"light\"",
        "id=\"light-preview\"",
        "cn-btn",
        "cn-input",
        "consumer-action",
        "consumer-field",
    ] {
        assert!(
            html.contains(attribute),
            "Missing theme scope or recipe: {attribute}"
        );
    }
    assert_eq!(html.matches("class=\"cn-body").count(), 2);
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

#[test]
fn semantic_badges_and_compatibility_adapter_share_the_same_markup() {
    for (variant, status) in [
        (BadgeVariant::Success, Status::Success),
        (BadgeVariant::Warning, Status::Warning),
        (BadgeVariant::Error, Status::Danger),
        (BadgeVariant::Info, Status::Info),
    ] {
        let mut dom = VirtualDom::new_with_props(
            |(variant, status): (BadgeVariant, Status)| {
                rsx! {
                    Badge { variant, "Connection status" }
                    StatusBadge { status, "Connection status" }
                }
            },
            (variant, status),
        );
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert_eq!(
            html.matches(&format!("data-style=\"{}\"", variant.class()))
                .count(),
            2
        );
        assert_eq!(html.matches("Connection status").count(), 2);
    }
}

#[test]
fn layout_primitives_compose_and_preserve_consumer_attributes() {
    use conctr_ui_dioxus::components::primitives;
    let html = render(|| {
        rsx! {
            primitives::Box { padding: Space::Xl, id: "workspace", class: "consumer-layout", aria_label: "Workspace",
                Flex { direction: Direction::Column, wrap: true, justify: Justify::Between,
                    Grid { columns: 0, gap: Space::Sm,
                        Stack { gap: Space::Md, Inline { Badge { "Draft" } } }
                    }
                }
            }
        }
    });
    assert!(html.contains("consumer-layout"));
    assert!(html.contains("cn-box"));
    assert!(html.contains("id=\"workspace\""));
    assert!(html.contains("aria-label=\"Workspace\""));
    assert!(html.contains("padding: var(--cn-space-6)"));
    assert!(html.contains("flex-direction: column"));
    assert!(html.contains("flex-wrap: wrap"));
    assert!(html.contains("--cn-grid-columns: 1"));
    assert!(html.contains("--cn-grid-gap: var(--cn-space-2)"));
    assert!(html.contains("gap: var(--cn-space-3)"));
}

#[test]
fn layered_imports_and_legacy_imports_share_component_implementations() {
    use conctr_ui_dioxus::components::{composed, primitives, ui};
    let html = render(|| {
        rsx! {
            primitives::stack::Stack {
                ui::button::Button { "Layered button" }
                conctr_ui_dioxus::components::button::Button { "Existing button" }
                composed::stat::Stat { label: "Runs today", value: "0" }
                ui::pagination::PaginationRoot {
                    ui::pagination::PaginationContent { "Layered pagination parts" }
                }
                conctr_ui_dioxus::components::dioxus::pagination::Pagination {
                    "Existing registry pagination"
                }
                conctr_ui_dioxus::components::pagination::Pagination {
                    page: 1, pages: 2, onchange: |_| {},
                }
            }
        }
    });
    for label in [
        "Layered button",
        "Existing button",
        "Runs today",
        "Layered pagination parts",
        "Existing registry pagination",
    ] {
        assert!(
            html.contains(label),
            "Missing component after module reorganization: {label}"
        );
    }
    assert!(html.contains("aria-label=\"Page 2\""));
}
