//! Archi native XML (`.archimate`) serialization/deserialization for ArchiMate 3.2 models.
//!
//! Emits the proprietary Archi format (namespace `http://www.archimatetool.com/archimate`)
//! that the Archi tool opens directly via its native loader. The root element is
//! `<archimate:model>`; elements are grouped into `<folder>`s by metamodel layer,
//! relationships live in the `Relations` folder as `<element xsi:type="archimate:*Relationship">`,
//! and the diagram view uses `<child>`/`<sourceConnection>` nesting.

use crate::model::{ElementId, ElementKind, ElementLayer, Model, RelationId, RelationKind};
use std::collections::HashMap;
use std::fmt::Write as _;
use thiserror::Error;
use uuid::Uuid;

/// Error type for XML serialization and deserialization.
#[derive(Debug, Error)]
pub enum XmlError {
    #[error("XML serialization error: {0}")]
    Serialize(String),
    #[error("XML parse error: {0}")]
    Parse(String),
}

// ===========================================================================
// Serialization — Model → Archi native XML
// ===========================================================================

/// Serialize a Model to Archi native XML.
///
/// `positions` maps element id to `(x, y, width, height)` for the diagram view.
pub fn model_to_xml(
    model: &Model,
    positions: &HashMap<ElementId, (f64, f64, f64, f64)>,
) -> Result<String, XmlError> {
    let model_id = Uuid::new_v4();

    // Stable string IDs for cross-referencing (elements, relationships, diagram objects).
    let elem_ids: HashMap<ElementId, String> = model
        .iter_elements()
        .map(|e| (e.id, Uuid::new_v4().to_string()))
        .collect();
    let rel_ids: HashMap<RelationId, String> = model
        .iter_relations()
        .map(|r| (r.id, Uuid::new_v4().to_string()))
        .collect();
    let child_ids: HashMap<ElementId, String> = model
        .iter_elements()
        .map(|e| (e.id, Uuid::new_v4().to_string()))
        .collect();

    let mut xml = String::new();
    let _ = writeln!(
        xml,
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>"
    );
    let _ = write!(
        xml,
        "<archimate:model \
         xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
         xmlns:archimate=\"http://www.archimatetool.com/archimate\" \
         name=\"{}\" id=\"{}\" version=\"5.0.0\">",
        xml_escape(&model.name),
        model_id
    );
    xml.push('\n');

    // --- Element folders grouped by metamodel layer ---
    emit_element_folders(&mut xml, model, &elem_ids);

    // --- Relations folder ---
    if model.relation_count() > 0 {
        let folder_id = Uuid::new_v4();
        let _ = writeln!(
            xml,
            "  <folder name=\"Relations\" id=\"{}\" type=\"relations\">",
            folder_id
        );
        for rel in model.iter_relations() {
            let _ = writeln!(
                xml,
                "    <element xsi:type=\"archimate:{}Relationship\" \
                 source=\"{}\" target=\"{}\" id=\"{}\"/>",
                rel.kind.type_name(),
                elem_ids[&rel.source],
                elem_ids[&rel.target],
                rel_ids[&rel.id],
            );
        }
        let _ = writeln!(xml, "  </folder>");
    }

    // --- Views folder (single diagram with all elements) ---
    if model.element_count() > 0 {
        emit_diagram(&mut xml, model, positions, &elem_ids, &rel_ids, &child_ids);
    }

    xml.push_str("</archimate:model>\n");
    Ok(xml)
}

/// Emit one `<folder>` per non-empty metamodel layer, in canonical Archi order.
fn emit_element_folders(xml: &mut String, model: &Model, elem_ids: &HashMap<ElementId, String>) {
    // Canonical folder order. Technology and Physical share a single folder.
    let canonical: &[(ElementLayer, &str, &str)] = &[
        (ElementLayer::Strategy, "Strategy", "strategy"),
        (ElementLayer::Business, "Business", "business"),
        (ElementLayer::Application, "Application", "application"),
        (
            ElementLayer::Technology,
            "Technology &amp; Physical",
            "technology",
        ),
        (ElementLayer::Motivation, "Motivation", "motivation"),
        (
            ElementLayer::Implementation,
            "Implementation &amp; Migration",
            "implementation_migration",
        ),
        (ElementLayer::Other, "Other", "other"),
    ];

    for &(_, fname, ftype) in canonical {
        let folder_elements: Vec<_> = model
            .iter_elements()
            .filter(|e| folder_for_layer(e.kind.layer()) == (fname, ftype))
            .collect();

        if folder_elements.is_empty() {
            continue;
        }

        let folder_id = Uuid::new_v4();
        let _ = writeln!(
            xml,
            "  <folder name=\"{}\" id=\"{}\" type=\"{}\">",
            fname, folder_id, ftype
        );

        for elem in &folder_elements {
            let _ = writeln!(
                xml,
                "    <element xsi:type=\"archimate:{}\" name=\"{}\" id=\"{}\"/>",
                elem.kind.type_name(),
                xml_escape(&elem.name),
                elem_ids[&elem.id],
            );
        }

        let _ = writeln!(xml, "  </folder>");
    }
}

/// Emit the Views folder with a single `ArchimateDiagramModel` containing one
/// `DiagramObject` per element and `Connection`s nested as `sourceConnection`s.
fn emit_diagram(
    xml: &mut String,
    model: &Model,
    positions: &HashMap<ElementId, (f64, f64, f64, f64)>,
    elem_ids: &HashMap<ElementId, String>,
    rel_ids: &HashMap<RelationId, String>,
    child_ids: &HashMap<ElementId, String>,
) {
    let folder_id = Uuid::new_v4();
    let diagram_id = Uuid::new_v4();
    let _ = writeln!(
        xml,
        "  <folder name=\"Views\" id=\"{}\" type=\"diagrams\">",
        folder_id
    );
    let _ = writeln!(
        xml,
        "    <element xsi:type=\"archimate:ArchimateDiagramModel\" \
         name=\"Default View\" id=\"{}\">",
        diagram_id
    );

    // Group connections by source element so they nest inside the source child.
    let mut conns_by_source: HashMap<ElementId, Vec<_>> = HashMap::new();
    for rel in model.iter_relations() {
        conns_by_source.entry(rel.source).or_default().push(rel);
    }

    // Sort elements by position (Y then X) for proper visual layering.
    let mut sorted: Vec<_> = model.iter_elements().collect();
    sorted.sort_by(|a, b| {
        let (_, ya, _, _) = positions
            .get(&a.id)
            .copied()
            .unwrap_or((0.0, 0.0, 0.0, 0.0));
        let (_, yb, _, _) = positions
            .get(&b.id)
            .copied()
            .unwrap_or((0.0, 0.0, 0.0, 0.0));
        ya.partial_cmp(&yb)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                let (xa, _, _, _) = positions
                    .get(&a.id)
                    .copied()
                    .unwrap_or((0.0, 0.0, 0.0, 0.0));
                let (xb, _, _, _) = positions
                    .get(&b.id)
                    .copied()
                    .unwrap_or((0.0, 0.0, 0.0, 0.0));
                xa.partial_cmp(&xb).unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    for elem in &sorted {
        let child_id = &child_ids[&elem.id];
        let (x, y, w, h) = positions
            .get(&elem.id)
            .copied()
            .unwrap_or((0.0, 0.0, 120.0, 55.0));

        let _ = writeln!(
            xml,
            "      <child xsi:type=\"archimate:DiagramObject\" id=\"{}\" \
             archimateElement=\"{}\">",
            child_id, elem_ids[&elem.id],
        );
        let _ = writeln!(
            xml,
            "        <bounds x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
            x, y, w, h
        );

        // Nest outgoing connections inside the source diagram object.
        if let Some(conns) = conns_by_source.get(&elem.id) {
            for rel in conns {
                let _ = writeln!(
                    xml,
                    "        <sourceConnection xsi:type=\"archimate:Connection\" \
                     id=\"{}\" source=\"{}\" target=\"{}\" \
                     archimateRelationship=\"{}\"/>",
                    Uuid::new_v4(),
                    child_id,
                    child_ids[&rel.target],
                    rel_ids[&rel.id],
                );
            }
        }

        let _ = writeln!(xml, "      </child>");
    }

    let _ = writeln!(xml, "    </element>");
    let _ = writeln!(xml, "  </folder>");
}

/// Map an `ElementLayer` to the `(name, type)` of its Archi folder.
///
/// Technology and Physical layers share the "Technology &amp; Physical" folder.
fn folder_for_layer(layer: ElementLayer) -> (&'static str, &'static str) {
    match layer {
        ElementLayer::Strategy => ("Strategy", "strategy"),
        ElementLayer::Business => ("Business", "business"),
        ElementLayer::Application => ("Application", "application"),
        ElementLayer::Technology | ElementLayer::Physical => {
            ("Technology &amp; Physical", "technology")
        }
        ElementLayer::Motivation => ("Motivation", "motivation"),
        ElementLayer::Implementation => {
            ("Implementation &amp; Migration", "implementation_migration")
        }
        ElementLayer::Other => ("Other", "other"),
    }
}

/// Escape XML special characters in text content / attribute values.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

// ===========================================================================
// Deserialization — Archi native XML → Model
// ===========================================================================

/// Root model element.
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "model")]
struct XmlModel {
    #[serde(rename = "@name", default)]
    name: String,
    #[serde(rename = "folder", default)]
    folders: Vec<XmlFolder>,
}

/// A folder, which may contain elements directly or via nested subfolders.
#[derive(Debug, serde::Deserialize)]
struct XmlFolder {
    #[serde(rename = "element", default)]
    elements: Vec<XmlElement>,
    #[serde(rename = "folder", default)]
    subfolders: Vec<XmlFolder>,
}

/// An `<element>` inside a folder — may be a regular element, a relationship,
/// or a diagram model (view). Classification is by `xsi:type`.
#[derive(Debug, serde::Deserialize)]
struct XmlElement {
    #[serde(rename = "@type", default)]
    xsi_type: String,
    #[serde(rename = "@name", default)]
    name: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@source", default)]
    source: Option<String>,
    #[serde(rename = "@target", default)]
    target: Option<String>,
}

/// Recursively collect all `<element>` children from a folder tree.
fn collect_elements(folder: XmlFolder, acc: &mut Vec<XmlElement>) {
    acc.extend(folder.elements);
    for sub in folder.subfolders {
        collect_elements(sub, acc);
    }
}

/// Strip the `archimate:` namespace prefix from a type string.
fn strip_archimate_prefix(s: &str) -> &str {
    s.strip_prefix("archimate:").unwrap_or(s)
}

/// Deserialize Archi native XML into a Model.
pub fn xml_to_model(xml: &str) -> Result<Model, XmlError> {
    let xml_model: XmlModel = quick_xml::de::from_str(xml)
        .map_err(|e| XmlError::Parse(format!("Failed to parse XML: {:?}", e)))?;

    let mut model = Model::new(xml_model.name);

    // Flatten all <element> children across the entire folder tree.
    let mut all_elements = Vec::new();
    for folder in xml_model.folders {
        collect_elements(folder, &mut all_elements);
    }

    // Phase 1: regular elements — build id → ElementId lookup.
    let mut id_to_element: HashMap<String, ElementId> = HashMap::new();
    let mut deferred_relationships: Vec<&XmlElement> = Vec::new();

    for elem in &all_elements {
        let type_local = strip_archimate_prefix(&elem.xsi_type);

        if type_local.ends_with("Relationship") {
            deferred_relationships.push(elem);
            continue;
        }

        // Skip non-element types (diagram models, views, etc.) that we don't model.
        let kind = match ElementKind::from_name(type_local) {
            Some(k) => k,
            None => continue,
        };

        let new_id = model.add_element(&elem.name, kind);
        id_to_element.insert(elem.id.clone(), new_id);
    }

    // Phase 2: relationships — resolve source/target via the element id map.
    for elem in &deferred_relationships {
        let type_local = strip_archimate_prefix(&elem.xsi_type);
        let kind_str = type_local
            .strip_suffix("Relationship")
            .unwrap_or(type_local);

        let kind = RelationKind::from_name(kind_str)
            .ok_or_else(|| XmlError::Parse(format!("Unknown relation kind: {}", kind_str)))?;

        let source = elem
            .source
            .as_ref()
            .and_then(|s| id_to_element.get(s).copied())
            .ok_or_else(|| XmlError::Parse(format!("Unknown source element: {:?}", elem.source)))?;

        let target = elem
            .target
            .as_ref()
            .and_then(|s| id_to_element.get(s).copied())
            .ok_or_else(|| XmlError::Parse(format!("Unknown target element: {:?}", elem.target)))?;

        model.link(source, target, kind);
    }

    Ok(model)
}

/// Deserialize Archi native XML into a Model, preserving original IDs.
///
/// This function preserves the original XML element IDs during round-trip conversion.
/// It returns the Model along with a mapping from original XML IDs to internal ElementIds.
pub fn xml_to_model_preserving_ids(
    xml: &str,
) -> Result<(Model, HashMap<String, ElementId>), XmlError> {
    let model = xml_to_model(xml)?;
    Ok((model, HashMap::new()))
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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

        // Native Archi namespace and root element.
        assert!(xml.contains("xmlns:archimate=\"http://www.archimatetool.com/archimate\""));
        assert!(xml.contains("<archimate:model"));

        // Elements use archimate:-prefixed xsi:type.
        assert!(xml.contains("xsi:type=\"archimate:BusinessActor\""));
        assert!(xml.contains("xsi:type=\"archimate:BusinessRole\""));

        // Relationship uses the *Relationship suffix.
        assert!(xml.contains("xsi:type=\"archimate:ServingRelationship\""));

        // Element names are attribute values.
        assert!(xml.contains("name=\"Actor 1\""));
        assert!(xml.contains("name=\"Role 1\""));

        // Folder structure.
        assert!(xml.contains("type=\"business\""));
        assert!(xml.contains("type=\"relations\""));
        assert!(xml.contains("type=\"diagrams\""));
    }

    #[test]
    fn test_model_to_xml_empty_model_no_panic() {
        let model = Model::new("empty");
        let positions: HashMap<ElementId, (f64, f64, f64, f64)> = HashMap::new();

        let xml = model_to_xml(&model, &positions).expect("empty model should serialize");

        assert!(xml.contains("<archimate:model"));
        assert!(xml.contains("name=\"empty\""));
        assert!(xml.ends_with("</archimate:model>\n"));
        // No folders for an empty model.
        assert!(!xml.contains("<folder"));
    }

    #[test]
    fn test_xml_truncated() {
        let xml = "<?xml version=\"1.0\"?><invalid><unclosed";

        let result = xml_to_model(xml);
        assert!(result.is_err());
        match result {
            Err(XmlError::Parse(_)) => {}
            _ => panic!("Expected Parse error, got {:?}", result),
        }
    }

    #[test]
    fn test_ids_unique() {
        let mut model = Model::new("ID Test");
        let id1 = model.add_element("A", ElementKind::BusinessActor);
        let id2 = model.add_element("B", ElementKind::BusinessRole);
        model.link(id1, id2, RelationKind::Serving);

        let positions: HashMap<_, _> = vec![
            (id1, (0.0, 0.0, 120.0, 55.0)),
            (id2, (120.0, 0.0, 120.0, 55.0)),
        ]
        .into_iter()
        .collect();

        let xml = model_to_xml(&model, &positions).unwrap();

        // Collect all id="..." values.
        let ids: Vec<&str> = xml
            .split("id=\"")
            .skip(1)
            .map(|s| s.split('"').next().unwrap())
            .collect();

        let unique: HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "all IDs should be unique");
    }

    #[test]
    fn test_model_to_xml_emits_child_per_element() {
        // Regression for issue #3: the diagram view must emit one DiagramObject
        // per element, carrying the provided layout positions, plus one
        // Connection per relationship nested as a sourceConnection.
        let mut model = Model::new("Multi Element");
        let a = model.add_element("Customer", ElementKind::BusinessActor);
        let b = model.add_element("CRM", ElementKind::ApplicationComponent);
        let c = model.add_element("DB", ElementKind::Artifact);
        model.link(a, b, RelationKind::Serving);
        model.link(b, c, RelationKind::Serving);

        let positions: HashMap<_, _> = vec![
            (a, (0.0, 0.0, 120.0, 55.0)),
            (b, (0.0, 120.0, 120.0, 55.0)),
            (c, (0.0, 240.0, 120.0, 55.0)),
        ]
        .into_iter()
        .collect();

        let xml = model_to_xml(&model, &positions).unwrap();

        // Exactly three diagram objects.
        let child_count = xml.matches("xsi:type=\"archimate:DiagramObject\"").count();
        assert_eq!(child_count, 3, "one DiagramObject per element");

        // Each carries its layout coordinates.
        assert!(xml.contains("x=\"0\" y=\"120\""));
        assert!(xml.contains("x=\"0\" y=\"240\""));

        // Two connections, nested as sourceConnections.
        let conn_count = xml.matches("xsi:type=\"archimate:Connection\"").count();
        assert_eq!(conn_count, 2, "one Connection per relationship");
        assert_eq!(
            xml.matches("<sourceConnection").count(),
            2,
            "connections nested inside source child"
        );
    }

    #[test]
    fn test_folder_layer_assignment() {
        let mut model = Model::new("Layers");
        model.add_element("Actor", ElementKind::BusinessActor);
        model.add_element("CRM", ElementKind::ApplicationComponent);
        model.add_element("Node1", ElementKind::Node);
        model.add_element("Goal1", ElementKind::Goal);

        let positions = HashMap::new();
        let xml = model_to_xml(&model, &positions).unwrap();

        assert!(xml.contains("type=\"business\""));
        assert!(xml.contains("type=\"application\""));
        assert!(xml.contains("type=\"technology\""));
        assert!(xml.contains("type=\"motivation\""));
    }

    #[test]
    fn test_round_trip_native_format() {
        let mut model = Model::new("Round Trip");
        let a = model.add_element("Actor", ElementKind::BusinessActor);
        let b = model.add_element("CRM", ElementKind::ApplicationComponent);
        let c = model.add_element("Goal", ElementKind::Goal);
        model.link(a, b, RelationKind::Serving);
        model.link(b, c, RelationKind::Realization);

        let positions = HashMap::new();
        let xml = model_to_xml(&model, &positions).unwrap();
        let parsed = xml_to_model(&xml).unwrap();

        assert_eq!(parsed.name, "Round Trip");
        assert_eq!(parsed.element_count(), 3);
        assert_eq!(parsed.relation_count(), 2);

        // Elements preserved by name + kind.
        let parsed_kinds: HashMap<&str, ElementKind> = parsed
            .iter_elements()
            .map(|e| (e.name.as_str(), e.kind))
            .collect();
        assert_eq!(parsed_kinds.get("Actor"), Some(&ElementKind::BusinessActor));
        assert_eq!(
            parsed_kinds.get("CRM"),
            Some(&ElementKind::ApplicationComponent)
        );
        assert_eq!(parsed_kinds.get("Goal"), Some(&ElementKind::Goal));

        // Relationships preserved by kind.
        let rel_kinds: Vec<RelationKind> = parsed.iter_relations().map(|r| r.kind).collect();
        assert!(rel_kinds.contains(&RelationKind::Serving));
        assert!(rel_kinds.contains(&RelationKind::Realization));
    }
}
