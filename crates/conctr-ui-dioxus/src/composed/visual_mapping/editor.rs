use super::{
    CSS, FieldEdit, FieldLink, MappingDraft, MappingSchema, Styles, TransformerDefinition,
    default_transformers, export::download_url, fields::FieldPanel, mapping_yaml, validate_mapping,
};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

/// Controlled two-column mapping editor. Drag cards to reorder and + handles to
/// connect, or choose New link for keyboard linking. Existing target links are replaced;
/// source fan-out is supported. Read-only mode preserves inspection and download.
#[component]
pub fn VisualFieldMapper(
    source: MappingSchema,
    target: MappingSchema,
    links: Vec<FieldLink>,
    #[props(default)] read_only: bool,
    on_change: EventHandler<Vec<FieldLink>>,
    #[props(default)] on_schema_change: Option<EventHandler<MappingDraft>>,
    #[props(default)] validation_messages: Vec<String>,
    #[props(default)] inspector: Option<Element>,
    #[props(default = default_transformers())] transformers: Vec<TransformerDefinition>,
) -> Element {
    // Interaction state stays local; schemas and links remain controlled by the caller.
    let mut linking = use_signal(|| false);
    let mut node = use_signal(|| None::<(bool, String)>);
    let mut dialog = use_signal(|| None::<(bool, Option<super::SchemaField>)>);
    let mut pending = use_signal(|| None::<String>);
    let mut selected = use_signal(|| None::<String>);
    let mut notice =
        use_signal(|| String::from("Choose a source field, then a target to connect them."));
    let problem = validate_mapping(&source, &target, &links).err();
    let yaml = mapping_yaml(&source, &target, &links);
    let active = links
        .iter()
        .find(|link| Some(&link.id) == selected.read().as_ref());
    let height = source.fields.len().max(target.fields.len()).max(1) * 64 + 32;
    let source_for_click = source.clone();
    let changes = links.clone();
    let targets = links.clone();
    let new_links = links.clone();
    // Inventory edits must publish schemas and pruned links together.
    let snapshot = MappingDraft {
        source: source.clone(),
        target: target.clone(),
        links: links.clone(),
    };
    let source_snapshot = snapshot.clone();
    let edit_source = EventHandler::new(move |edit: FieldEdit| {
        if let Some(change) = on_schema_change {
            match source_snapshot.clone().edit_field(true, edit) {
                Ok(draft) => {
                    pending.set(None);
                    selected.set(None);
                    change.call(draft);
                    notice.set("Source fields updated.".into());
                }
                Err(error) => notice.set(error),
            }
        }
    });
    let edit_target = EventHandler::new(move |edit: FieldEdit| {
        if let Some(change) = on_schema_change {
            match snapshot.clone().edit_field(false, edit) {
                Ok(draft) => {
                    pending.set(None);
                    selected.set(None);
                    change.call(draft);
                    notice.set("Target fields updated.".into());
                }
                Err(error) => notice.set(error),
            }
        }
    });
    let node_value = node().and_then(|(side, path)| {
        let schema = if side { &source } else { &target };
        schema
            .fields
            .iter()
            .find(|f| f.path == path)
            .cloned()
            .map(|f| (side, f))
    });
    let drag_source = source.clone();
    let drag_target = target.clone();
    let drag_links = links.clone();
    let toolbar_links = links.clone();
    let target_for_click = target.clone();
    let connect_target = EventHandler::new(move |path: String| {
        if !target_for_click
            .fields
            .iter()
            .any(|f| f.path == path && !f.ignored)
        {
            return;
        }
        node.set(None);
        if let Some(from) = pending() {
            if !source_for_click
                .fields
                .iter()
                .any(|f| f.path == from && !f.ignored)
            {
                pending.set(None);
                notice.set("Source schema changed. Choose a source field again.".into());
                return;
            }
            let mut next = changes.clone();
            if let Some(existing) = next.iter_mut().find(|l| l.target == path) {
                existing.source = from;
                selected.set(Some(existing.id.clone()));
            } else {
                let mut n = 1;
                while next.iter().any(|l| l.id == format!("link-{n}")) {
                    n += 1;
                }
                let id = format!("link-{n}");
                next.push(FieldLink {
                    id: id.clone(),
                    source: from,
                    target: path,
                    transform: None,
                    transforms: vec![],
                });
                selected.set(Some(id));
            }
            on_change.call(next);
            pending.set(None);
            linking.set(false);
            notice.set("Connection updated. Choose another source to continue.".into());
        } else if let Some(link) = targets.iter().find(|l| l.target == path) {
            selected.set(Some(link.id.clone()));
        } else {
            notice.set("Choose a source first to link this target.".into());
        }
    });
    let yaml_text = yaml.clone().unwrap_or_else(|error| format!("# {error}"));
    rsx! {
        document::Stylesheet { href: CSS }
        section {
            class: Styles::root.inner,
            "data-visual-mapper": "",
            "data-read-only": read_only.to_string(),
            onmounted: move |_| { document::eval(super::drag::SCRIPT); },
            input { r#type:"hidden", "data-editor-events":"",
                oninput: move |event| {
                    if read_only { return; }
                    // The browser only reports gestures; Rust owns every document mutation.
                    let Ok(value) = serde_json::from_str::<serde_json::Value>(&event.value()) else { return; };
                    let path = value["path"].as_str().unwrap_or_default().to_string();
                    if value["kind"] == "move" {
                        let side = value["side"] == "source";
                        let edit = FieldEdit::Move { path:path.clone(), before:value["before"].as_str().map(str::to_owned) };
                        if side { edit_source.call(edit); } else { edit_target.call(edit); }
                        node.set(Some((side,path)));
                    } else if value["kind"] == "link" {
                        let to = value["target"].as_str().unwrap_or_default().to_string();
                        if !drag_source.fields.iter().any(|f| f.path==path && !f.ignored)
                            || !drag_target.fields.iter().any(|f| f.path==to && !f.ignored) { return; }
                        let mut next = drag_links.clone();
                        if let Some(link) = next.iter_mut().find(|l| l.target==to) {
                            link.source=path;
                            selected.set(Some(link.id.clone()));
                        } else {
                            let mut index=1;
                            while next.iter().any(|l|l.id==format!("link-{index}")) { index+=1; }
                            let id=format!("link-{index}");
                            next.push(FieldLink {id:id.clone(),source:path,target:to,transform:None,transforms:vec![]});
                            selected.set(Some(id));
                        }
                        node.set(None); pending.set(None); on_change.call(next);
                        notice.set("Link created. Select it to edit its transform pipeline.".into());
                    }
                }
            }
            header { class: Styles::toolbar.inner,
                div {
                    span { class: Styles::eyebrow.inner, "Data studio" }
                    h2 { "Mapping editor" }
                    p { "Shape nodes, connect systems and compose transforms." }
                }
                div { class: Styles::actions.inner,
                    span { class: Styles::status.inner,
                        if read_only {
                            "Read only"
                        } else {
                            "Draft"
                        }
                    }
                    if !read_only {
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            disabled: source.fields.is_empty() || target.fields.is_empty(),
                            onclick: move |_| {
                                linking.set(true);
                                node.set(None);
                                pending.set(None);
                                selected.set(None);
                                notice.set("New link: choose a source field, then a target.".into());
                            },
                            icons::Plus { class: "size-4" }
                            "New link"
                        }
                    }
                    if let Ok(text) = &yaml {
                        a {
                            class: Styles::download.inner,
                            href: download_url(text),
                            download: "conctr-mapping.yaml",
                            icons::Download { class: "size-4" }
                            "Download YAML"
                        }
                    }
                }
            }
            div { class:Styles::ribbon.inner, role:"toolbar", "aria-label":"Mapping editor actions",
                if !read_only {
                    Button { variant:ButtonVariant::Outline, size:ButtonSize::Sm, disabled:on_schema_change.is_none(), onclick:move |_|dialog.set(Some((true,None))), icons::Plus {class:"size-4"} "New source" }
                    Button { variant:ButtonVariant::Outline, size:ButtonSize::Sm, disabled:on_schema_change.is_none(), onclick:move |_|dialog.set(Some((false,None))), icons::Plus {class:"size-4"} "New target" }
                    Button { variant:ButtonVariant::Ghost, size:ButtonSize::Sm, disabled:node_value.is_none() || on_schema_change.is_none(),
                        onclick:{ let value=node_value.clone(); move |_| { if let Some((side,field))=value.clone() {dialog.set(Some((side,Some(field))));} } }, "Edit node" }
                    Button { variant:ButtonVariant::Ghost, size:ButtonSize::Sm, disabled:node_value.is_none() || on_schema_change.is_none(),
                        onclick:{ let value=node_value.clone(); move |_| {if let Some((side,field))=&value {let edit=FieldEdit::ToggleIgnored(field.path.clone());if *side {edit_source.call(edit)} else {edit_target.call(edit)}}} }, "Ignore / restore" }
                    Button { variant:ButtonVariant::Ghost, size:ButtonSize::Sm, disabled:node_value.is_none() || on_schema_change.is_none(),
                        onclick:{ let value=node_value.clone(); move |_| {if let Some((side,field))=&value {let edit=FieldEdit::Remove(field.path.clone());if *side {edit_source.call(edit)} else {edit_target.call(edit)}node.set(None);}} }, "Remove node" }
                    Button { variant:ButtonVariant::Ghost, size:ButtonSize::Sm, disabled:node_value.is_none() || on_schema_change.is_none(),
                        onclick:{ let value=node_value.clone(); let source=source.clone();let target=target.clone(); move |_| {
                            if let Some((side,field))=&value {
                                let fields=if *side {&source.fields} else {&target.fields};
                                if let Some(index)=fields.iter().position(|f|f.path==field.path) {
                                    let destination = index.checked_sub(1).map(|i|Some(fields[i].path.clone()));
                                    if let Some(before)=destination {
                                        let edit=FieldEdit::Move {path:field.path.clone(),before};
                                        if *side {edit_source.call(edit)} else {edit_target.call(edit)}
                                    }
                                }
                            }
                        } }, "Move up" }
                    Button { variant:ButtonVariant::Ghost, size:ButtonSize::Sm, disabled:node_value.is_none() || on_schema_change.is_none(),
                        onclick:{ let value=node_value.clone(); let source=source.clone();let target=target.clone(); move |_| {
                            if let Some((side,field))=&value {
                                let fields=if *side {&source.fields} else {&target.fields};
                                if let Some(index)=fields.iter().position(|f|f.path==field.path) {
                                    let destination = if index+1<fields.len() {Some(fields.get(index+2).map(|f|f.path.clone()))} else {None};
                                    if let Some(before)=destination {
                                        let edit=FieldEdit::Move {path:field.path.clone(),before};
                                        if *side {edit_source.call(edit)} else {edit_target.call(edit)}
                                    }
                                }
                            }
                        } }, "Move down" }
                    Button { variant:ButtonVariant::Ghost, size:ButtonSize::Sm, disabled:active.is_none(),
                        onclick:move |_| {if let Some(id)=selected() {on_change.call(toolbar_links.iter().filter(|l|l.id!=id).cloned().collect());selected.set(None);}}, "Remove link" }
                }
                span { class:Styles::selection_label.inner,
                    if let Some((side,field))=&node_value { "{side_label(*side)}: {field.label}" }
                    else if active.is_some() { "Link selected · edit pipeline below" }
                    else { "Select a node or link" }
                }
            }
            if let Some((side,field))=dialog() {
                super::node_manager::NodeDialog { source:side, field,
                    paths:if side {source.fields.iter().map(|f|f.path.clone()).collect()} else {target.fields.iter().map(|f|f.path.clone()).collect()},
                    on_save:move |edit| {if side {edit_source.call(edit)} else {edit_target.call(edit)}},
                    on_close:move |_|dialog.set(None),
                }
            }
            div { class: Styles::hint.inner,
                icons::MousePointer2 { class: "size-4" }
                span { "aria-live": "polite",
                    if read_only {
                        "Inspect connections and download the mapping document."
                    } else {
                        "{notice}"
                    }
                }
            }
            for message in &validation_messages {
                p { class:Styles::error.inner, role:"status", "{message}" }
            }
            if let Some(problem) = problem {
                p { class: Styles::error.inner, role: "alert", "{problem}" }
            }
            div {
                class: Styles::scroll.inner,
                tabindex: "0",
                "aria-label": "Side by side field mapping",
                div { class: Styles::canvas.inner,
                    div { class: Styles::schema_heads.inner,
                        div {
                            strong { "{source.name}" }
                            span {
                                "{source.message} · {source.format.label()} · {source.fields.len()} fields"
                            }
                        }
                        span { class: Styles::bridge_label.inner,
                            icons::GitBranch { class: "size-4" }
                            "{links.len()} links"
                        }
                        div {
                            strong { "{target.name}" }
                            span {
                                "{target.message} · {target.format.label()} · {target.fields.len()} fields"
                            }
                        }
                    }
                    div {
                        class: Styles::board.inner,
                        style: "height:{height}px",
                        svg {
                            class: Styles::wires.inner,
                            view_box: format!("0 0 1000 {height}"),
                            preserve_aspect_ratio: "none",
                            "aria-label": "Mapping connections",
                            for link in &links {
                                if let (Some(si), Some(ti)) = (
                                    source.fields.iter().position(|f| f.path == link.source),
                                    target.fields.iter().position(|f| f.path == link.target),
                                )
                                {
                                    g {
                                        key: "{link.id}",
                                        "data-link-id": link.id.clone(),
                                        "data-active": (Some(&link.id) == selected.read().as_ref()).to_string(),
                                        path {
                                            d: format!(
                                                "M364 {} C480 {},520 {},636 {}",
                                                si * 64 + 44,
                                                si * 64 + 44,
                                                ti * 64 + 44,
                                                ti * 64 + 44,
                                            ),
                                            class: Styles::wire.inner,
                                        }
                                        path {
                                            d: format!(
                                                "M364 {} C480 {},520 {},636 {}",
                                                si * 64 + 44,
                                                si * 64 + 44,
                                                ti * 64 + 44,
                                                ti * 64 + 44,
                                            ),
                                            class: Styles::wire_hit.inner,
                                            onclick: {
                                                let id = link.id.clone();
                                                move |_| {
                                                    node.set(None);
                                                    selected.set(Some(id.clone()));
                                                    pending.set(None);
                                                }
                                            },
                                        }
                                        if !link.pipeline().is_empty() {
                                            circle {
                                                cx: "500",
                                                cy: ((si + ti) * 32 + 44).to_string(),
                                                r: "12",
                                                class: Styles::transform_dot.inner,
                                            }
                                            text {
                                                x: "500",
                                                y: ((si + ti) * 32 + 49).to_string(),
                                                text_anchor: "middle",
                                                class: Styles::transform_text.inner,
                                                "ƒ"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FieldPanel {
                            schema: source.clone(),
                            source: true,
                            read_only,
                            selected: node().filter(|(side,_)|*side).map(|(_,p)|p),
                            on_select:move |path:String| {
                                if linking() && source.fields.iter().any(|f|f.path==path && !f.ignored) {
                                    pending.set(Some(path)); node.set(None);
                                } else { node.set(Some((true,path))); }
                                selected.set(None);
                            },
                            mapped: links.iter().map(|l| l.source.clone()).collect(),
                            on_connect: move |path: String| {
                                node.set(None);
                                pending.set(Some(path));
                                selected.set(None);
                                notice
                                    .set(
                                        "Now choose a target field. Existing target links will be replaced."
                                            .into(),
                                    );
                            },
                        }
                        div { class: Styles::gutter.inner,
                            if links.is_empty() {
                                div { class: Styles::empty.inner,
                                    icons::GitBranch { class: "size-8" }
                                    p { "Your connections appear here" }
                                }
                            }
                        }
                        FieldPanel {
                            schema: target.clone(),
                            source: false,
                            read_only,
                            selected: node().filter(|(side,_)|!*side).map(|(_,p)|p),
                            on_select:move |path:String| {
                                if linking() && pending().is_some() {connect_target.call(path);}
                                else {node.set(Some((false,path)));selected.set(None);}
                            },
                            mapped: links.iter().map(|l| l.target.clone()).collect(),
                            on_connect: connect_target,
                        }
                    }
                }
            }
            div {
                class: Styles::connection_list.inner,
                "aria-label": "Connections",
                for link in &links {
                    button {
                        key: "{link.id}",
                        r#type: "button",
                        class: Styles::link_chip.inner,
                        "aria-pressed": (Some(&link.id) == selected.read().as_ref()).to_string(),
                        "data-connection": link.id.clone(),
                        onclick: {
                            let id = link.id.clone();
                            move |_| {
                                node.set(None);
                                selected.set(Some(id.clone()));
                                pending.set(None);
                            }
                        },
                        code { "{link.source}" }
                        icons::ArrowRight { class: "size-4" }
                        code { "{link.target}" }
                        for id in link.pipeline() {
                            span {
                                {
                                    transformers.iter().find(|t| t.id == id).map(|t| t.label.as_str()).unwrap_or(&id)
                                }
                            }
                        }
                    }
                }
            }
            if let Some(link) = active {
                div { class: Styles::inspector.inner,
                    div {
                        strong { "Selected connection" }
                        p { "{link.source} → {link.target}" }
                    }
                    super::transform_picker::TransformPicker {
                        key: "{link.id}",
                        steps: link.pipeline(),
                        catalog: transformers.clone(),
                        read_only,
                        on_change: {
                            let id = link.id.clone();
                            move |steps| {
                                let mut next = new_links.clone();
                                if let Some(link) = next.iter_mut().find(|l| l.id == id) {
                                    link.transform = None;
                                    link.transforms = steps;
                                    on_change.call(next);
                                }
                            }
                        },
                    }
                    if !read_only {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: {
                                let id = link.id.clone();
                                let snapshot = links.clone();
                                move |_| {
                                    on_change.call(snapshot.iter().filter(|l| l.id != id).cloned().collect());
                                    selected.set(None);
                                    notice.set("Connection removed.".into());
                                }
                            },
                            icons::Trash2 { class: "size-4" }
                            "Remove link"
                        }
                    }
                }
            }
            {inspector}
            footer { class: Styles::footer.inner,
                span {
                    "{links.len()} connections · {target.fields.iter().filter(|f|!f.ignored).count().saturating_sub(links.len())} unmapped targets"
                }
                details {
                    summary { "Mapping YAML · read only" }
                    p { "Conctr mapping format · not an executable Camel route." }
                    pre {
                        code { "{yaml_text}" }
                    }
                }
            }
        }
    }
}

fn side_label(source: bool) -> &'static str {
    if source { "Source" } else { "Target" }
}
