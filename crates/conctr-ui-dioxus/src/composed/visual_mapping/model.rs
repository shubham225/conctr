use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Path syntax belongs to the payload adapter, never to canvas geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayloadFormat {
    Json,
    Xml,
    Yaml,
    Toml,
    Text,
}
impl PayloadFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Xml => "XML",
            Self::Yaml => "YAML",
            Self::Toml => "TOML",
            Self::Text => "Text",
        }
    }
}
/// A normalized node (including containers). Use bracket JSONPath for JSON, XPath for XML (including namespaces).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaField {
    pub path: String,
    pub label: String,
    pub data_type: String,
    #[serde(default)]
    pub depth: usize,
    #[serde(default)]
    pub ignored: bool,
}
/// Common contract accepted regardless of the caller's original JSON/XML input.
/// Paths must be unique and nonempty. Samples are deliberately excluded from exports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingSchema {
    pub name: String,
    pub message: String,
    pub format: PayloadFormat,
    pub fields: Vec<SchemaField>,
    #[serde(default)]
    pub namespaces: BTreeMap<String, String>,
}
/// Optional transform configuration, not executable code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldTransform {
    Trim,
    Lowercase,
    Uppercase,
    ToNumber,
}
impl FieldTransform {
    pub const ALL: [Self; 4] = [Self::Trim, Self::Lowercase, Self::Uppercase, Self::ToNumber];
    pub fn id(self) -> &'static str {
        match self {
            Self::Trim => "trim",
            Self::Lowercase => "lowercase",
            Self::Uppercase => "uppercase",
            Self::ToNumber => "to_number",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Trim => "Trim whitespace",
            Self::Lowercase => "Lowercase",
            Self::Uppercase => "Uppercase",
            Self::ToNumber => "To number",
        }
    }
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.id() == id)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldLink {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub transform: Option<FieldTransform>,
    #[serde(default)]
    pub transforms: Vec<String>,
}

/// Infer a bounded field inventory from a JSON sample in the caller's import layer.
/// Arrays use [*]; all elements contribute fields. Conflicting leaf types become mixed.
/// This is sample inference, not JSON Schema validation. Empty containers remain fields.
pub fn schema_from_json(name: &str, message: &str, text: &str) -> Result<MappingSchema, String> {
    if text.len() > 1_048_576 {
        return Err("JSON sample exceeds 1 MiB.".into());
    }
    let value: Value = serde_json::from_str(text).map_err(|e| format!("Invalid JSON: {e}"))?;
    let mut fields = BTreeMap::new();
    fn visit(
        value: &Value,
        path: String,
        label: String,
        depth: usize,
        fields: &mut BTreeMap<String, SchemaField>,
    ) -> Result<(), String> {
        if depth > 32 || fields.len() >= 2048 {
            return Err("Sample exceeds 32 levels or 2048 fields.".into());
        }
        match value {
            Value::Object(map) if !map.is_empty() => {
                for (key, value) in map {
                    visit(
                        value,
                        format!("{path}[{}]", quote_path(key)),
                        key.clone(),
                        depth + 1,
                        fields,
                    )?;
                }
            }
            Value::Array(items) if !items.is_empty() => {
                for value in items {
                    visit(
                        value,
                        format!("{path}[*]"),
                        format!("{label}[]"),
                        depth + 1,
                        fields,
                    )?;
                }
            }
            _ => {
                let kind = match value {
                    Value::Null => "null",
                    Value::Bool(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Object(_) => "object",
                };
                fields
                    .entry(path.clone())
                    .and_modify(|field| {
                        if field.data_type != kind {
                            field.data_type = "mixed".into();
                        }
                    })
                    .or_insert(SchemaField {
                        path,
                        label,
                        data_type: kind.into(),
                        depth: depth.saturating_sub(1),
                        ignored: false,
                    });
            }
        }
        Ok(())
    }
    visit(&value, "$".into(), "body".into(), 0, &mut fields)?;
    Ok(MappingSchema {
        name: name.into(),
        message: message.into(),
        format: PayloadFormat::Json,
        fields: fields.into_values().collect(),
        namespaces: BTreeMap::new(),
    })
}
fn quote_path(value: &str) -> String {
    // JSON strings are also valid bracket member names.
    Value::String(value.into()).to_string()
}
/// Check supplied contracts before editing/export. One source may feed multiple targets.
pub fn validate_mapping(
    source: &MappingSchema,
    target: &MappingSchema,
    links: &[FieldLink],
) -> Result<(), String> {
    for schema in [source, target] {
        let mut paths = BTreeSet::new();
        for field in &schema.fields {
            if field.path.is_empty() || !paths.insert(&field.path) {
                return Err(format!(
                    "{} has empty or duplicate field paths.",
                    schema.name
                ));
            }
        }
    }
    let mut ids = BTreeSet::new();
    let mut targets = BTreeSet::new();
    for link in links {
        if link.id.is_empty() || !ids.insert(&link.id) {
            return Err("Mapping IDs must be unique and nonempty.".into());
        }
        if !source
            .fields
            .iter()
            .any(|f| f.path == link.source && !f.ignored)
            || !target
                .fields
                .iter()
                .any(|f| f.path == link.target && !f.ignored)
        {
            return Err(
                "A mapped field no longer exists. Remove the affected link or restore its schema."
                    .into(),
            );
        }
        if !targets.insert(&link.target) {
            return Err("A target field can have only one mapping.".into());
        }
    }
    Ok(())
}

/// Extensible searchable transformer catalog. IDs are persisted; labels are presentation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformerDefinition {
    pub id: String,
    pub label: String,
    pub description: String,
    pub category: String,
}
pub fn default_transformers() -> Vec<TransformerDefinition> {
    FieldTransform::ALL
        .into_iter()
        .map(|t| TransformerDefinition {
            id: t.id().into(),
            label: t.label().into(),
            description: format!("Apply {} to the mapped value.", t.label().to_lowercase()),
            category: if t == FieldTransform::ToNumber {
                "Conversion"
            } else {
                "Text"
            }
            .into(),
        })
        .collect()
}
impl FieldLink {
    /// Legacy single transforms remain first when opening older drafts.
    pub fn pipeline(&self) -> Vec<String> {
        self.transform
            .into_iter()
            .map(|t| t.id().into())
            .chain(self.transforms.iter().cloned())
            .collect()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingDraft {
    pub source: MappingSchema,
    pub target: MappingSchema,
    pub links: Vec<FieldLink>,
}
#[derive(Clone, PartialEq)]
pub enum FieldEdit {
    Add(SchemaField),
    Update {
        path: String,
        field: SchemaField,
    },
    Move {
        path: String,
        before: Option<String>,
    },
    Remove(String),
    ToggleIgnored(String),
}
impl MappingDraft {
    /// Atomically edit the inventory and prune links to removed or ignored nodes.
    pub fn edit_field(mut self, source: bool, edit: FieldEdit) -> Result<Self, String> {
        let schema = if source {
            &mut self.source
        } else {
            &mut self.target
        };
        match edit {
            FieldEdit::Add(field) => {
                if field.path.trim().is_empty()
                    || field.label.trim().is_empty()
                    || !NODE_TYPES.contains(&field.data_type.as_str())
                {
                    return Err("Name, path and type are required.".into());
                }
                if schema.fields.iter().any(|f| f.path == field.path) {
                    return Err("A field with this path already exists.".into());
                }
                schema.fields.push(field);
            }
            FieldEdit::Update { path, field } => {
                if field.path.trim().is_empty()
                    || field.label.trim().is_empty()
                    || !NODE_TYPES.contains(&field.data_type.as_str())
                {
                    return Err("Provide a name, path and supported datatype.".into());
                }
                if schema
                    .fields
                    .iter()
                    .any(|f| f.path == field.path && f.path != path)
                {
                    return Err("A field with this path already exists.".into());
                }
                let node = schema
                    .fields
                    .iter_mut()
                    .find(|f| f.path == path)
                    .ok_or("The selected node no longer exists.")?;
                *node = field.clone();
                // Renaming a path updates link endpoints without losing the pipeline.
                for link in &mut self.links {
                    let endpoint = if source {
                        &mut link.source
                    } else {
                        &mut link.target
                    };
                    if *endpoint == path {
                        *endpoint = field.path.clone();
                    }
                }
            }
            FieldEdit::Move { path, before } => {
                if before.as_ref() == Some(&path) {
                    return Ok(self);
                }
                let index = schema
                    .fields
                    .iter()
                    .position(|f| f.path == path)
                    .ok_or("The selected node no longer exists.")?;
                if let Some(ref anchor) = before
                    && !schema.fields.iter().any(|f| &f.path == anchor)
                {
                    return Err("Drop destination no longer exists.".into());
                }
                let field = schema.fields.remove(index);
                let at = before
                    .and_then(|p| schema.fields.iter().position(|f| f.path == p))
                    .unwrap_or(schema.fields.len());
                schema.fields.insert(at, field);
            }
            FieldEdit::Remove(path) => schema.fields.retain(|f| f.path != path),
            FieldEdit::ToggleIgnored(path) => {
                if let Some(field) = schema.fields.iter_mut().find(|f| f.path == path) {
                    field.ignored = !field.ignored;
                }
            }
        }
        self.links.retain(|l| {
            self.source
                .fields
                .iter()
                .any(|f| f.path == l.source && !f.ignored)
                && self
                    .target
                    .fields
                    .iter()
                    .any(|f| f.path == l.target && !f.ignored)
        });
        Ok(self)
    }
}

/// Portable type vocabulary; adapters preserve original path syntax and namespaces.
pub const NODE_TYPES: &[&str] = &[
    "string", "integer", "int", "number", "float", "decimal", "boolean", "null", "array", "object",
    "date", "time", "datetime", "binary", "mixed",
];
