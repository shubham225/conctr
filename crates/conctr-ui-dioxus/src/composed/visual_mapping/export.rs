use super::{FieldLink, MappingSchema, PayloadFormat, validate_mapping};
use serde_json::Value;
fn scalar(value: &str) -> String {
    Value::String(value.into()).to_string()
}
/// Versioned Conctr mapping YAML, not an executable Apache Camel route.
/// JSON-quoted YAML scalars keep paths, Unicode and multiline names unambiguous.
pub fn mapping_yaml(
    source: &MappingSchema,
    target: &MappingSchema,
    links: &[FieldLink],
) -> Result<String, String> {
    validate_mapping(source, target, links)?;
    let mut yaml = String::from(
        "# Conctr mapping document; not an executable Camel route.\napiVersion: conctr.dev/v1alpha1\nkind: FieldMapping\n",
    );
    for (side, schema) in [("source", source), ("target", target)] {
        let language = match schema.format {
            PayloadFormat::Json => "jsonpath",
            PayloadFormat::Xml => "xpath",
            PayloadFormat::Yaml | PayloadFormat::Toml | PayloadFormat::Text => "adapter-defined",
        };
        yaml.push_str(&format!(
            "{side}:\n  name: {}\n  message: {}\n  format: {}\n  pathLanguage: {language}\n",
            scalar(&schema.name),
            scalar(&schema.message),
            schema.format.label().to_lowercase()
        ));
        let ignored: Vec<_> = schema.fields.iter().filter(|f| f.ignored).collect();
        if !ignored.is_empty() {
            yaml.push_str("  ignored:\n");
            for field in ignored {
                yaml.push_str(&format!("    - {}\n", scalar(&field.path)));
            }
        }
        if !schema.namespaces.is_empty() {
            yaml.push_str("  namespaces:\n");
            for (prefix, uri) in &schema.namespaces {
                yaml.push_str(&format!("    {}: {}\n", scalar(prefix), scalar(uri)));
            }
        }
    }
    if links.is_empty() {
        yaml.push_str("mappings: []\n");
    } else {
        yaml.push_str("mappings:\n");
        for link in links {
            yaml.push_str(&format!(
                "  - id: {}\n    from: {}\n    to: {}\n",
                scalar(&link.id),
                scalar(&link.source),
                scalar(&link.target)
            ));
            let pipeline = link.pipeline();
            if !pipeline.is_empty() {
                yaml.push_str("    transforms:\n");
                for id in pipeline {
                    yaml.push_str(&format!("      - type: {}\n", scalar(&id)));
                }
            }
        }
    }
    Ok(yaml)
}
/// Percent-encoded data URL avoids browser-only APIs during SSR.
pub(super) fn download_url(text: &str) -> String {
    let mut url = String::from("data:application/yaml;charset=utf-8,");
    use std::fmt::Write;
    for byte in text.bytes() {
        let _ = write!(url, "%{byte:02X}");
    }
    url
}
