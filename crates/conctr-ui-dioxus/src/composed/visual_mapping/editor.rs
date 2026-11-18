use super::{
    CSS, FieldEdit, FieldLink, MappingDraft, MappingSchema, Styles, TransformerDefinition,
    default_transformers, fields::FieldPanel, mapping_yaml, validate_mapping,
};
use crate::composed::resizable::{PanelControls, PanelMode, ResizableHandle, ResizablePanelGroup};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

/// Controlled mapping workspace with compact schema rows and a persistent inspector.
/// Drag dot ports or activate a source port then a target field to connect. Existing target links are replaced;
/// source fan-out is supported. Read-only mode preserves inspection and download.
#[component]
pub fn VisualFieldMapper(
    source: MappingSchema,
    target: MappingSchema,
    links: Vec<FieldLink>,
    #[props(default)] read_only: bool,
    /// Workspace layout with a persistent right inspector and no export controls.
    #[props(default)]
    studio: bool,
    /// Caller-owned direction and history controls.
    #[props(default)]
    toolbar: Option<Element>,
    /// Caller-supplied source and target preview panes.
    #[props(default)]
    previews: Option<Element>,
    on_change: EventHandler<Vec<FieldLink>>,
    #[props(default)] on_schema_change: Option<EventHandler<MappingDraft>>,
    #[props(default)] validation_messages: Vec<String>,
    #[props(default)] inspector: Option<Element>,
    #[props(default = default_transformers())] transformers: Vec<TransformerDefinition>,
) -> Element {
    // Interaction state stays local; schemas and links remain controlled by the caller.
    let mut sidebar_size = use_signal(|| 320.0);
    let mut preview_size = use_signal(|| 260.0);
    let mut sidebar_mode = use_signal(PanelMode::default);
    let mut preview_mode = use_signal(PanelMode::default);
    let mut query = use_signal(String::new);
    let mut unmapped = use_signal(|| false);
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
    let source_for_click = source.clone();
    let changes = links.clone();
    let source_selection = links.clone();
    let source_select_fields = source.fields.clone();
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
    let filter = |schema: &MappingSchema, source_side: bool| {
        let mut visible = schema.clone();
        visible.fields.retain(|field| {
            (query().is_empty()
                || format!("{} {}", field.label, field.path)
                    .to_lowercase()
                    .contains(&query().to_lowercase()))
                && (!unmapped()
                    || !links.iter().any(|l| {
                        if source_side {
                            l.source == field.path
                        } else {
                            l.target == field.path
                        }
                    }))
        });
        visible
    };
    let visible_source = filter(&source, true);
    let visible_target = filter(&target, false);
    let height = visible_source
        .fields
        .len()
        .max(visible_target.fields.len())
        .max(8)
        * 40;
    let auto_source = source.clone();
    let auto_target = target.clone();
    let auto_links = links.clone();
    let yaml_text = yaml.clone().unwrap_or_else(|error| format!("# {error}"));
    rsx! {
        document::Stylesheet { href: CSS }
        ResizablePanelGroup {
        section {
            class: Styles::root.inner,
            style:format!("--mapping-sidebar:{}px;--mapping-preview:{}px",sidebar_size(),preview_size()),
            "data-visual-mapper": "",
            "data-studio": studio.to_string(),
            "data-sidebar-mode":sidebar_mode().as_str(),
            "data-preview-mode":preview_mode().as_str(),
            "data-panel-group":"",
            "data-read-only": read_only.to_string(),
            onkeydown: move |event| {if event.key()==Key::Escape {pending.set(None);linking.set(false);notice.set("Connection cancelled.".into());}},
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
                div { class: Styles::title.inner, icons::GitBranch { class:"size-5" } h2 { "Mapping Studio" } }
                {toolbar}
                label { class:Styles::filter.inner,
                    icons::Search {class:"size-4"}
                    input { r#type:"search", "aria-label":"Filter fields", placeholder:"Filter fields", value:query(), oninput:move |e|query.set(e.value()) }
                }
                button { r#type:"button", class:Styles::filter_toggle.inner, "aria-pressed":unmapped().to_string(), onclick:move |_|unmapped.set(!unmapped()), "Unmapped only" }
                if !read_only {
                    Button {variant:ButtonVariant::Outline, size:ButtonSize::Sm,
                        onclick:move |_| {
                            let mut next=auto_links.clone(); let mut count=0;
                            for target in auto_target.fields.iter().filter(|f|!f.ignored) {
                                if next.iter().any(|l|l.target==target.path) {continue;}
                                let normalize=|v:&str|v.chars().filter(|c|c.is_alphanumeric()).flat_map(char::to_lowercase).collect::<String>();
                                if let Some(source)=auto_source.fields.iter().find(|f|!f.ignored && f.data_type==target.data_type && normalize(&f.label)==normalize(&target.label)) {
                                    let mut n=1;while next.iter().any(|l|l.id==format!("link-{n}")){n+=1;}
                                    next.push(FieldLink{id:format!("link-{n}"), source:source.path.clone(),target:target.path.clone(),transform:None,transforms:vec![]});count+=1;
                                }
                            }
                            on_change.call(next);notice.set(format!("Auto-mapped {count} fields by matching name and type."));
                        }, "Auto-map" }
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
                        super::connections::MappingConnections {
                            source: visible_source.clone(), target: visible_target.clone(), links: links.clone(),
                            selected: selected(), height,
                            on_select: move |id| { node.set(None); selected.set(Some(id)); pending.set(None); },
                        }
                        FieldPanel {
                            schema: visible_source.clone(),
                            editable:on_schema_change.is_some(),
                            on_edit:move |field|dialog.set(Some((true,Some(field)))),
                            on_field_change:edit_source,
                            source: true,
                            read_only,
                            selected: node().filter(|(side,_)|*side).map(|(_,p)|p).or_else(&*pending),
                            on_select:move |path:String| {
                                if linking() && source_select_fields.iter().any(|f|f.path==path && !f.ignored) {
                                    pending.set(Some(path)); node.set(None);
                                } else {
                                    selected.set(source_selection.iter().find(|l|l.source==path).map(|l|l.id.clone()));
                                    node.set(Some((true,path)));
                                }
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
                            schema: visible_target.clone(),
                            editable:on_schema_change.is_some(),
                            on_edit:move |field|dialog.set(Some((false,Some(field)))),
                            on_field_change:edit_target,
                            source: false,
                            read_only,
                            selected: node().filter(|(side,_)|!*side).map(|(_,p)|p),
                            on_select:move |path:String| {
                                if pending().is_some() {connect_target.call(path);}
                                else {connect_target.call(path.clone());node.set(Some((false,path)));}
                            },
                            mapped: links.iter().map(|l| l.target.clone()).collect(),
                            on_connect: connect_target,
                        }
                    }
                }
            }
            details {class:Styles::connections_drawer.inner, summary {"Connections · {links.len()}"}
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
            }
            if sidebar_mode()==PanelMode::Normal {
                ResizableHandle {label:"Resize mapping tools",vertical:false,size:sidebar_size(),on_change:move |size|sidebar_size.set(size),minimum:240.0,maximum_fraction:0.65,class:Styles::sidebar_resize.inner}
            }
            aside { class: Styles::inspector_panel.inner, "aria-label": "Connection inspector",
                div {class:Styles::inspector_heading.inner, icons::SlidersHorizontal {class:"size-4"} strong { "Mapping tools" }
                    PanelControls {label:"mapping tools",mode:sidebar_mode(),on_change:move |mode| {sidebar_mode.set(mode);if mode==PanelMode::Maximized {preview_mode.set(PanelMode::Normal);}}}
                }
            div { class:Styles::ribbon.inner, role:"toolbar", "aria-label":"Mapping editor actions",
                if !read_only {
                    Button { variant:ButtonVariant::Outline, size:ButtonSize::Sm, disabled:on_schema_change.is_none(), onclick:move |_|dialog.set(Some((true,None))), icons::Plus {class:"size-4"} "New source" }
                    Button { variant:ButtonVariant::Outline, size:ButtonSize::Sm, disabled:on_schema_change.is_none(), onclick:move |_|dialog.set(Some((false,None))), icons::Plus {class:"size-4"} "New target" }

                }
                span { class:Styles::selection_label.inner,
                    if let Some((side,field))=&node_value { "{side_label(*side)}: {field.label}" }
                    else if active.is_some() { "Connection selected" }
                    else { "Select a field or wire" }
                }
            }

            if active.is_none() {
                div { class: Styles::inspector.inner,
                    if let Some((side,field))=&node_value {
                        strong {"{side_label(*side)} field · {field.label}"}
                        p {"{field.path}"}
                        p {"Type: {field.data_type}"}
                    } else {strong { "Mapping summary" }}
                    div {class:Styles::metrics.inner,
                        div {span {"Mapped"} b {"{links.len()}"}}
                        div {span {"Unfilled targets"} b {"{target.fields.iter().filter(|f|!f.ignored && !links.iter().any(|l|l.target==f.path)).count()}"}}
                    }
                    p { "Select a wire or connection below the canvas to inspect its fields and compose an ordered transform pipeline." }
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
            }
            if preview_mode()==PanelMode::Normal {
                ResizableHandle {label:"Resize payload panels",vertical:true,size:preview_size(),on_change:move |size|preview_size.set(size),minimum:120.0,maximum_fraction:0.7,class:Styles::preview_resize.inner}
            }
            section {class:Styles::preview_panel.inner, "aria-label":"Payload panels",
                header {class:Styles::panel_heading.inner, strong {"Payload panels"}
                    PanelControls {label:"payload panels",mode:preview_mode(),on_change:move |mode| {preview_mode.set(mode);if mode==PanelMode::Maximized {sidebar_mode.set(PanelMode::Normal);}}}
                }
            div {class:Styles::previews.inner,
                if let Some(content)=previews { {content} }
                else {
                    div {h3 {"Source fields"} pre {for field in &source.fields {"{field.path} : {field.data_type}\n"}}}
                    div {h3 {"Target mapping"} pre {for link in &links {"{link.target} ← {link.source}\n"}}}
                }
            }
            }
            footer { class: Styles::footer.inner,
                span {
                    "{links.len()} connections · {target.fields.iter().filter(|f|!f.ignored).count().saturating_sub(links.len())} unmapped targets"
                }
                if !studio {
                    if let Ok(text)=&yaml {a { href:super::export::download_url(text), download:"conctr-mapping.yaml", "Download YAML" }}
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
    }
}

fn side_label(source: bool) -> &'static str {
    if source { "Source" } else { "Target" }
}
