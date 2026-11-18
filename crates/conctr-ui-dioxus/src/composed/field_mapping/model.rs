/// A request or response field. IDs must be unique within each system's schema.
#[derive(Clone, PartialEq, Eq)]
pub struct MappingField {
    pub id: String,
    pub label: String,
    pub path: String,
    pub data_type: String,
}

/// An available transform supplied by the caller. IDs must be unique and nonempty.
#[derive(Clone, PartialEq, Eq)]
pub struct MappingTransform {
    pub id: String,
    pub label: String,
    pub description: String,
}

/// One editable link. Source/target refer to field IDs; transform refers to a
/// caller-supplied transform ID. Link IDs must be unique and remain stable.
#[derive(Clone, PartialEq, Eq)]
pub struct MappingLink {
    pub id: String,
    pub source: String,
    pub target: String,
    pub transform: Option<String>,
}
