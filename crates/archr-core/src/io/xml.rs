//! Open Exchange XML (3.0) serialization/deserialization for ArchiMate 3.2 models.
//!
//! Supports bidirectional round-trips using `quick-xml` (serialize feature) and serde.

use crate::model::{Element, ElementId, ElementKind, Model, RelationId, RelationKind, Relationship};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use thiserror::Error;

/// Error type for XML serialization and deserialization.
#[derive(Debug, Error)]
pub enum XmlError {
    #[error("XML serialization error: {0}")]
    Serialize(String),
    #[error("XML parse error: {0}")]
    Parse(String),
}

/// Serialize a Model to Open Exchange XML.
///
/// `positions` maps element id to (x, y, width, height) for the diagram view.
pub fn model_to_xml(
    model: &Model,
    positions: &HashMap<ElementId, (f64, f64, f64, f64)>,
) -> Result<String, XmlError> {
    let model_uuid = Uuid::new_v4();
    let mut elements: Vec<_> = model.iter_elements().map(|e| (e.id, e.name.clone(), e.kind)).collect();
    let mut relations: Vec<_> = model.iter_relations().map(|r| (r.id, r.kind)).collect();

    // Sort for deterministic output
    elements.sort_by_key(|(id, _, _)| id.0);
    relations.sort_by_key(|(id, _)| id.0);

    // Build XML using quick-xml
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(
        "<model xmlns=\"http://www.opengroup.org/xsd/archimate/3.0/\" \
        xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
        identifier=\"");
    xml.push_str(&model_uuid.to_string());
    xml.push_str("\" version=\"3.2\">\n");
    xml.push_str("  <name>");
    xml.push_str(&model.name);
    xml.push_str("</name>\n");
    xml.push_str("  <elements>\n");

    // Map ElementId -> Uuid for relationships
    let element_uuids: HashMap<ElementId, Uuid> = elements
        .iter()
        .map(|(id, _, kind)| (*id, Uuid::new_v4()))
        .collect();

    for (id, name, kind) in &elements {
        let uuid = element_uuids[id];
        xml.push_str("    <element identifier=\"");
        xml.push_str(&uuid.to_string());
        xml.push_str("\" xsi:type=\"");
        xml.push_str(&kind.type_name());
        xml.push_str("\">\n");
        xml.push_str("      <name>");
        xml.push_str(name);
        xml.push_str("</name>\n");
        xml.push_str("    </element>\n");
    }

    xml.push_str("  </elements>\n");
    xml.push_str("  <relationships>\n");

    for (id, kind) in &relations {
        let rel_uuid = Uuid::new_v4();
        let src_uuid = element_uuids[&model.relation(*id).source];
        let tgt_uuid = element_uuids[&model.relation(*id).target];
        xml.push_str("    <relationship identifier=\"");
        xml.push_str(&rel_uuid.to_string());
        xml.push_str("\" source=\"");
        xml.push_str(&src_uuid.to_string());
        xml.push_str("\" target=\"");
        xml.push_str(&tgt_uuid.to_string());
        xml.push_str("\" xsi:type=\"");
        xml.push_str(&kind.type_name());
        xml.push_str("\">\n");
        xml.push_str("    </relationship>\n");
    }

    xml.push_str("  </relationships>\n");
    xml.push_str("  <views>\n");
    xml.push_str("    <diagrams>\n");
    xml.push_str("      <view identifier=\"view-001\" xsi:type=\"Diagram\">\n");
    xml.push_str("        <name>Default View</name>\n");

    if let Some((first_id, _, _)) = elements.first() {
        xml.push_str("        <node identifier=\"node-001\" x=\"0\" y=\"0\" width=\"120\" height=\"55\" xsi:type=\"Label\">\n");
        xml.push_str("          <label ref=\"");
        xml.push_str(&element_uuids[first_id].to_string());
        xml.push_str("\"/>\n");
        xml.push_str("        </node>\n");
    }

    for (id, _) in &relations {
        let src_uuid = element_uuids[&model.relation(*id).source];
        let tgt_uuid = element_uuids[&model.relation(*id).target];
        xml.push_str("        <connection identifier=\"conn-");
        xml.push_str(&id.0.to_string());
        xml.push_str("\" relationship=\"");
        let rel_uuid = Uuid::new_v4();
        xml.push_str(&rel_uuid.to_string());
        xml.push_str("\">\n");
        xml.push_str("          <source ref=\"");
        xml.push_str(&src_uuid.to_string());
        xml.push_str("\"/>\n");
        xml.push_str("          <target ref=\"");
        xml.push_str(&tgt_uuid.to_string());
        xml.push_str("\"/>\n");
        xml.push_str("        </connection>\n");
    }

    xml.push_str("      </view>\n");
    xml.push_str("    </diagrams>\n");
    xml.push_str("  </views>\n");
    xml.push_str("</model>");

    Ok(xml)
}

// ----------------------------------------------------------------------
// Serde structs for XML deserialization
// ----------------------------------------------------------------------

/// Representation of an element in XML.
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "element")]
struct XmlElement {
    #[serde(rename = "@identifier")]
    identifier: String,
    #[serde(rename = "@type")]
    kind: String,
    name: String,
}

/// Representation of a relationship in XML.
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "relationship")]
struct XmlRelationship {
    #[serde(rename = "@identifier")]
    identifier: String,
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@target")]
    target: String,
    #[serde(rename = "@type")]
    kind: String,
}

/// Representation of a view node in XML.
#[derive(Debug, serde::Deserialize)]
struct XmlNode {
    #[serde(rename = "@type")]
    node_type: String,
    #[serde(rename = "@identifier")]
    identifier: String,
    #[serde(rename = "@x", default)]
    x: f64,
    #[serde(rename = "@y", default)]
    y: f64,
    #[serde(rename = "@width", default = "default_width")]
    width: f64,
    #[serde(rename = "@height", default = "default_height")]
    height: f64,
    #[serde(default)]
    label: Option<XmlNodeLabel>,
}

/// Label reference inside a node.
#[derive(Debug, serde::Deserialize)]
struct XmlNodeLabel {
    #[serde(rename = "@ref")]
    label_ref: String,
}

/// Representation of a view connection in XML.
#[derive(Debug, serde::Deserialize)]
struct XmlConnection {
    #[serde(rename = "@identifier")]
    identifier: String,
    #[serde(rename = "@relationship", default)]
    relationship: Option<String>,
    #[serde(default)]
    source: Option<XmlNodeLabel>,
    #[serde(default)]
    target: Option<XmlNodeLabel>,
}

/// Representation of a diagram in XML.
#[derive(Debug, serde::Deserialize)]
struct XmlDiagram {
    #[serde(rename = "@identifier")]
    identifier: String,
    name: String,
    #[serde(default, rename = "node")]
    node: Option<Vec<XmlNode>>,
    #[serde(default, rename = "connection")]
    connection: Option<Vec<XmlConnection>>,
}

/// Representation of views in XML.
#[derive(Debug, serde::Deserialize)]
struct XmlViews {
    #[serde(default)]
    diagrams: Vec<XmlDiagram>,
}

/// Representation of a model in XML (for deserialization).
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "model")]
struct XmlModel {
    #[serde(rename = "@identifier")]
    identifier: String,
    name: String,
    #[serde(default)]
    elements: XmlElementList,
    #[serde(default)]
    relationships: XmlRelationshipList,
}

/// Wrapper for <elements> containing multiple <element> children.
#[derive(Debug, Default, serde::Deserialize)]
struct XmlElementList {
    #[serde(rename = "element", default)]
    element: Vec<XmlElement>,
}

/// Wrapper for <relationships> containing multiple <relationship> children.
#[derive(Debug, Default, serde::Deserialize)]
struct XmlRelationshipList {
    #[serde(rename = "relationship", default)]
    relationship: Vec<XmlRelationship>,
}

/// Remove the `<views>...</views>` section from XML for simpler parsing.
fn strip_views_section(xml: &str) -> String {
    let start_marker = "<views>";
    let end_marker = "</views>";
    match xml.find(start_marker) {
        Some(start) => {
            let before = &xml[..start];
            match xml[start..].find(end_marker) {
                Some(rel_end) => {
                    let after = &xml[start + rel_end + end_marker.len()..];
                    format!("{before}  {after}")
                }
                None => before.to_string(),
            }
        }
        None => xml.to_string(),
    }
}

/// Helper functions for defaults.
fn default_width() -> f64 {
    120.0
}

fn default_height() -> f64 {
    55.0
}

/// Deserialize Open Exchange XML into a Model.
pub fn xml_to_model(xml: &str) -> Result<Model, XmlError> {
    // Strip the optional <views>...</views> section — it contains diagram
    // layout info we don't need for model reconstruction, and its nested
    // structure with namespaces is hard to deserialize cleanly.
    let xml = strip_views_section(xml);

    let xml_model: XmlModel = quick_xml::de::from_str(&xml).map_err(|e| {
        XmlError::Parse(format!("Failed to parse XML: {:?}", e))
    })?;

    // Reverse lookup: Uuid -> ElementId
    let mut uuid_to_element_id: HashMap<String, ElementId> = HashMap::new();

    // Create elements
    let mut elements = Vec::with_capacity(xml_model.elements.element.len());
    for xml_elem in xml_model.elements.element {
        let kind = ElementKind::from_name(&xml_elem.kind)
            .ok_or_else(|| XmlError::Parse(format!("Unknown element kind: {}", xml_elem.kind)))?;
        let id = elements.len();
        let element = Element {
            id: ElementId(id),
            name: xml_elem.name,
            kind,
        };
        elements.push(element);
        uuid_to_element_id.insert(xml_elem.identifier, ElementId(id));
    }

    // Create relationships
    let mut relations = Vec::with_capacity(xml_model.relationships.relationship.len());
    for xml_rel in xml_model.relationships.relationship {
        let kind = RelationKind::from_name(&xml_rel.kind)
            .ok_or_else(|| XmlError::Parse(format!("Unknown relation kind: {}", xml_rel.kind)))?;

        let source_id = uuid_to_element_id
            .get(&xml_rel.source)
            .copied()
            .ok_or_else(|| XmlError::Parse(format!("Unknown source element: {}", xml_rel.source)))?;

        let target_id = uuid_to_element_id
            .get(&xml_rel.target)
            .copied()
            .ok_or_else(|| XmlError::Parse(format!("Unknown target element: {}", xml_rel.target)))?;

        let id = relations.len();
        let relation = Relationship {
            id: RelationId(id),
            source: source_id,
            target: target_id,
            kind,
        };
        relations.push(relation);
    }

    // Views are optional, ignore them during model reconstruction
    // (they contain diagram-specific layout info)

    // Since Model fields are private, we use a private helper
    let mut model = Model::new(xml_model.name);
    for (id, elem) in elements.into_iter().enumerate() {
        let _ = model.add_element(&elem.name, elem.kind);
    }

    for (id, rel) in relations.into_iter().enumerate() {
        let _ = model.link(rel.source, rel.target, rel.kind);
    }

    Ok(model)
}

// ----------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_to_xml_basic() {
        let mut model = Model::new("Test Model");
        let actor_id = model.add_element("Actor 1", ElementKind::BusinessActor);
        let role_id = model.add_element("Role 1", ElementKind::BusinessRole);
        let _id = model.link(actor_id, role_id, RelationKind::Serving);

        let positions: HashMap<_, _> = vec![
            (actor_id, (0.0, 0.0, 120.0, 55.0)),
            (role_id, (120.0, 0.0, 120.0, 55.0)),
        ]
        .into_iter()
        .collect();

        let xml = model_to_xml(&model, &positions).unwrap();

        assert!(xml.contains("xmlns=\"http://www.opengroup.org/xsd/archimate/3.0/\""));
        assert!(xml.contains("xsi:type=\"BusinessActor\""));
        assert!(xml.contains("xsi:type=\"BusinessRole\""));
        assert!(xml.contains("xsi:type=\"Serving\""));
        assert!(xml.contains(&model.element(actor_id).name));
        assert!(xml.contains(&model.element(role_id).name));
    }

    #[test]
    fn test_model_to_xml_empty_model_no_panic() {
        // Regression: empty model (zero elements) must not panic on views emission.
        let model = Model::new("empty");
        let positions: HashMap<ElementId, (f64, f64, f64, f64)> = HashMap::new();

        let xml = model_to_xml(&model, &positions).expect("empty model should serialize");

        // Valid structure: empty <elements>/<relationships>, a <views> block, well-closed root.
        assert!(xml.contains("xmlns=\"http://www.opengroup.org/xsd/archimate/3.0/\""));
        assert!(xml.contains("<name>empty</name>"));
        assert!(xml.contains("<elements>\n  </elements>"));
        assert!(xml.contains("<relationships>\n  </relationships>"));
        assert!(xml.ends_with("</model>"));
        assert!(!xml.contains("node-001"), "no node should be emitted when there are no elements");
    }

    #[test]
    fn test_xml_truncated() {
        let xml = "<?xml version=\"1.0\"?><invalid><unclosed";

        let result = xml_to_model(xml);
        assert!(result.is_err());
        match result {
            Err(XmlError::Parse(_)) => {},
            _ => panic!("Expected Parse error, got {:?}", result),
        }
    }

    #[test]
    fn test_uuids_unique() {
        let mut model = Model::new("UUID Test");
        let id1 = model.add_element("A", ElementKind::BusinessActor);
        let id2 = model.add_element("B", ElementKind::BusinessRole);
        let id3 = model.add_element("C", ElementKind::BusinessCollaboration);

        let positions: HashMap<_, _> = vec![
            (id1, (0.0, 0.0, 120.0, 55.0)),
            (id2, (120.0, 0.0, 120.0, 55.0)),
            (id3, (240.0, 0.0, 120.0, 55.0)),
        ]
        .into_iter()
        .collect();

        let xml = model_to_xml(&model, &positions).unwrap();

        let identifiers: Vec<&str> = xml
            .split("identifier=\"")
            .skip(1)
            .take(6) // model, 3 elements, 3 relationships
            .map(|s| s.split('"').next().unwrap())
            .collect();

        let unique_uuids: HashSet<&str> = identifiers.iter().cloned().collect();
        assert_eq!(identifiers.len(), unique_uuids.len(), "UUIDs should be unique");
    }
}
