//! YAML I/O for ArchiMate models.
//!
//! Handles (de)serialization to/from YAML, with schema-level validation.
use std::collections::HashMap;

use crate::model::{ElementId, ElementKind, Model, RelationId, RelationKind};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Internal DTO for YAML unmarshaling.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlModel {
    model: YamlModelInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlModelInner {
    name: String,
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
    #[serde(default)]
    viewpoints: Vec<YamlViewpointDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YamlViewpointKind {
    None,
    Business,
    Application,
    Implementation,
    Motivation,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlViewpointDefinition {
    pub id: String,
    pub name: String,
    pub kind: YamlViewpointKind,
    #[serde(default)]
    pub elements: Vec<YamlElement>,
    #[serde(default)]
    pub relationships: Vec<YamlRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlElement {
    pub id: String,
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlRelationship {
    pub id: String,
    pub source: String,
    pub target: String,
    pub kind: String,
}
/// Schema validation errors returned during parse.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SchemaError {
    MalformedYaml(String),
    InvalidId,
    DuplicateId,
    UnknownKind,
    UndefinedId,
}
/// Returns a human-readable message for a `SchemaError`.
///
/// Structured-data variants (`UnknownKind`, `UndefinedId`, `DuplicateId`, `InvalidId`)
/// collapse to their variant name; `MalformedYaml` surfaces the carried serde message.
pub fn schema_error_message(error: &SchemaError) -> String {
    match error {
        SchemaError::MalformedYaml(msg) => msg.clone(),
        other => format!("{other:?}"),
    }
}

/// Result of YAML parsing with accumulated schema errors.
pub type ParseResult<T> = Result<T, Vec<SchemaError>>;

pub type YamlParseResult = ParseResult<(
    Model,
    HashMap<String, ElementId>,
    HashMap<String, RelationId>,
    Vec<YamlViewpointDefinition>,
    HashMap<String, ElementId>,
    HashMap<String, RelationId>,
)>;
/// Parsed Model with original string ID mappings preserved for round-trip fidelity.
/// Parses YAML input into an ArchiMate Model.
///
/// Returns errors if:
/// - Any element `kind` does not match an ElementKind variant
/// - Any relationship `kind` does not match a RelationKind variant
/// - Any relationship references a non-existent element
/// - Any element `id` is duplicated
/// - Any `id` is empty or contains spaces
pub fn parse_yaml(input: &str) -> ParseResult<Model> {
    Ok(parse_yaml_with_ids(input)?.0)
}

pub fn parse_yaml_with_viewpoint_ids(input: &str) -> YamlParseResult {
    let (model, elem_ids, rel_ids, viewpoints, vp_elem_ids, vp_rel_ids) =
        parse_yaml_with_ids(input)?;
    Ok((
        model,
        elem_ids,
        rel_ids,
        viewpoints,
        vp_elem_ids,
        vp_rel_ids,
    ))
}
pub fn parse_yaml_with_ids(input: &str) -> YamlParseResult {
    let yaml_model: YamlModel =
        serde_yaml::from_str(input).map_err(|e| vec![SchemaError::MalformedYaml(e.to_string())])?;

    let YamlModelInner {
        name,
        elements,
        relationships,
        viewpoints,
    } = yaml_model.model;

    let mut errors = Vec::new();

    // Validate global element IDs (empty / spaces / duplicates) and element kinds.
    let mut seen_ids: HashSet<String> = HashSet::new();
    for elem in &elements {
        if elem.id.is_empty() || elem.id.contains(' ') {
            errors.push(SchemaError::InvalidId);
        } else if !seen_ids.insert(elem.id.clone()) {
            errors.push(SchemaError::DuplicateId);
        }
        if ElementKind::from_name(&elem.kind).is_none() {
            errors.push(SchemaError::UnknownKind);
        }
    }

    // Build id -> index map for global elements (only valid ids resolve; undefined refs flagged below).
    let mut id_to_index = std::collections::HashMap::new();
    for (idx, elem) in elements.iter().enumerate() {
        id_to_index.insert(elem.id.clone(), idx);
    }

    // Validate global relationships: reference resolution + relation kind.
    for rel in &relationships {
        if rel.id.is_empty() || rel.id.contains(' ') {
            errors.push(SchemaError::InvalidId);
        }
        if !id_to_index.contains_key(&rel.source) {
            errors.push(SchemaError::UndefinedId);
        }
        if !id_to_index.contains_key(&rel.target) {
            errors.push(SchemaError::UndefinedId);
        }
        if RelationKind::from_name(&rel.kind).is_none() {
            errors.push(SchemaError::UnknownKind);
        }
    }

    // Validate viewpoint elements and relationships (after global elements are created)
    for vp_def in viewpoints.iter() {
        // Validate viewpoint element IDs
        let mut vp_seen_ids: HashSet<String> = HashSet::new();
        for elem in &vp_def.elements {
            if elem.id.is_empty() || elem.id.contains(' ') {
                errors.push(SchemaError::InvalidId);
            } else if !vp_seen_ids.insert(elem.id.clone()) {
                errors.push(SchemaError::DuplicateId);
            }
            if ElementKind::from_name(&elem.kind).is_none() {
                errors.push(SchemaError::UnknownKind);
            }
        }

        // Build viewpoint element ID map (only includes elements from this viewpoint)
        let mut vp_id_to_index = std::collections::HashMap::new();
        for (idx, elem) in vp_def.elements.iter().enumerate() {
            vp_id_to_index.insert(elem.id.clone(), idx);
        }

        // Validate viewpoint relationships (can reference elements from this viewpoint only)
        for rel in &vp_def.relationships {
            if rel.id.is_empty() || rel.id.contains(' ') {
                errors.push(SchemaError::InvalidId);
            }
            // Check if source or target exists in this viewpoint's elements
            if !vp_id_to_index.contains_key(&rel.source) {
                errors.push(SchemaError::UndefinedId);
            }
            if !vp_id_to_index.contains_key(&rel.target) {
                errors.push(SchemaError::UndefinedId);
            }
            if RelationKind::from_name(&rel.kind).is_none() {
                errors.push(SchemaError::UnknownKind);
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    // Build the Model with global elements (created first for viewpoints to reference)
    let mut model = Model::new(name);
    let mut elem_ids: Vec<ElementId> = Vec::with_capacity(elements.len());
    for elem in &elements {
        let kind = ElementKind::from_name(&elem.kind).expect("validated above");
        let id = model.add_element(&elem.name, kind);
        elem_ids.push(id);
    }

    // Map original string ids to internal ElementId handles.
    let mut str_to_elem: HashMap<String, ElementId> = HashMap::new();
    for (elem, id) in elements.iter().zip(elem_ids.iter()) {
        str_to_elem.insert(elem.id.clone(), *id);
    }

    let mut str_to_rel: HashMap<String, RelationId> = HashMap::new();
    for rel in &relationships {
        let kind = RelationKind::from_name(&rel.kind).expect("validated above");
        let source = str_to_elem[&rel.source];
        let target = str_to_elem[&rel.target];
        let rel_id = model.link(source, target, kind);
        str_to_rel.insert(rel.id.clone(), rel_id);
    }

    // Map viewpoint element IDs to global element IDs
    let mut vp_str_to_elem: HashMap<String, ElementId> = HashMap::new();
    for vp_def in viewpoints.iter() {
        for elem in &vp_def.elements {
            if let Some(elem_id) = str_to_elem.get(&elem.id) {
                vp_str_to_elem.insert(elem.id.clone(), *elem_id);
            }
        }
    }

    // Map viewpoint relationship IDs to global relationship IDs
    let mut vp_str_to_rel: HashMap<String, RelationId> = HashMap::new();
    for vp_def in viewpoints.iter() {
        for rel in &vp_def.relationships {
            if let Some(rel_id) = str_to_rel.get(&rel.id) {
                vp_str_to_rel.insert(rel.id.clone(), *rel_id);
            }
        }
    }

    Ok((
        model,
        str_to_elem,
        str_to_rel,
        viewpoints,
        vp_str_to_elem,
        vp_str_to_rel,
    ))
}

/// Serializes an ArchiMate Model back to YAML.
pub fn model_to_yaml(model: &Model) -> String {
    let inner = YamlModelInner {
        name: model.name.clone(),
        elements: model
            .iter_elements()
            .map(|elem| YamlElement {
                id: format!("e_{}", elem.id.0),
                name: elem.name.clone(),
                kind: elem.kind.to_string(),
            })
            .collect(),
        relationships: model
            .iter_relations()
            .map(|rel| YamlRelationship {
                id: format!("r_{}", rel.id.0),
                source: format!("e_{}", rel.source.0),
                target: format!("e_{}", rel.target.0),
                kind: rel.kind.to_string(),
            })
            .collect(),
        viewpoints: Vec::new(), // No viewpoint support in current model structure
    };

    let yaml_model = YamlModel { model: inner };
    serde_yaml::to_string(&yaml_model).unwrap_or_else(|_| "Error".to_string())
}
/// Serialize an ArchiMate Model to YAML, optionally preserving original XML IDs.
///
/// When ID mappings are provided, original string identifiers are used instead
/// of synthetic `e_N`/`r_N` indices, enabling faithful round-trip conversion.
pub fn model_to_yaml_with_ids(
    model: &Model,
    elem_ids: Option<&HashMap<String, ElementId>>,
    rel_ids: Option<&HashMap<String, RelationId>>,
) -> String {
    // Reverse maps for O(1) lookup by internal ID.
    let elem_map: HashMap<&ElementId, &str> = elem_ids
        .map(|m| m.iter().map(|(k, v)| (v, k.as_str())).collect())
        .unwrap_or_default();
    let rel_map: HashMap<&RelationId, &str> = rel_ids
        .map(|m| m.iter().map(|(k, v)| (v, k.as_str())).collect())
        .unwrap_or_default();

    let elem_id = |id: &ElementId| -> String {
        elem_map
            .get(id)
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("e_{}", id.0))
    };

    let inner = YamlModelInner {
        name: model.name.clone(),
        elements: model
            .iter_elements()
            .map(|elem| YamlElement {
                id: elem_id(&elem.id),
                name: elem.name.clone(),
                kind: elem.kind.to_string(),
            })
            .collect(),
        relationships: model
            .iter_relations()
            .map(|rel| YamlRelationship {
                id: rel_map
                    .get(&rel.id)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("r_{}", rel.id.0)),
                source: elem_id(&rel.source),
                target: elem_id(&rel.target),
                kind: rel.kind.to_string(),
            })
            .collect(),
        viewpoints: Vec::new(), // No viewpoint support in current model structure
    };

    let yaml_model = YamlModel { model: inner };
    serde_yaml::to_string(&yaml_model).unwrap_or_else(|_| "Error".to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_yaml() {
        let yaml = r#"
model:
  name: Test
  elements:
    - id: e1
      name: Actor
      kind: BusinessActor
    - id: e2
      name: App
      kind: ApplicationComponent
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Serving
"#;

        let model = parse_yaml(yaml).unwrap();
        assert_eq!(model.name, "Test");
        assert_eq!(model.element_count(), 2);
        assert_eq!(model.relation_count(), 1);
        let rel = model.iter_relations().next().unwrap();
        assert_eq!(rel.kind, RelationKind::Serving);
    }

    #[test]
    fn test_unknown_kind() {
        let yaml = r#"
model:
  name: Test
  elements:
    - id: e1
      name: Foo
      kind: FooBar
"#;

        let result = parse_yaml(yaml);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors.contains(&SchemaError::UnknownKind),
            "expected UnknownKind in {:?}",
            errors
        )
    }

    #[test]
    fn test_undefined_id() {
        let yaml = r#"
model:
  name: Test
  elements:
    - id: e1
      name: Actor
      kind: BusinessActor
  relationships:
    - id: r1
      source: e1
      target: nonexistent
      kind: Serving
"#;

        let result = parse_yaml(yaml);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors.contains(&SchemaError::UndefinedId),
            "expected UndefinedId in {:?}",
            errors
        )
    }

    #[test]
    fn test_duplicate_id() {
        let yaml = r#"
model:
  name: Test
  elements:
    - id: e1
      name: Actor
      kind: BusinessActor
    - id: e1
      name: Another
      kind: BusinessRole
"#;

        let result = parse_yaml(yaml);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors.contains(&SchemaError::DuplicateId),
            "expected DuplicateId in {:?}",
            errors
        )
    }

    #[test]
    fn test_invalid_id() {
        let yaml = r#"
model:
  name: Test
  elements:
    - id: "my id"
      name: Actor
      kind: BusinessActor
"#;

        let result = parse_yaml(yaml);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors.contains(&SchemaError::InvalidId),
            "expected InvalidId in {:?}",
            errors
        )
    }

    #[test]
    fn test_round_trip() {
        let original = r#"
model:
  name: Test
  elements:
    - id: e1
      name: Actor
      kind: BusinessActor
    - id: e2
      name: App
      kind: ApplicationComponent
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Serving
"#;

        let model = parse_yaml(original).unwrap();
        let serialized = model_to_yaml(&model);
        let reparsed = parse_yaml(&serialized).unwrap();

        assert_eq!(model.name, reparsed.name);
        assert_eq!(model.element_count(), reparsed.element_count());
        assert_eq!(model.relation_count(), reparsed.relation_count());
    }

    #[test]
    fn test_empty_yaml() {
        let yaml = r#"
model:
  name: Empty
"#;

        let model = parse_yaml(yaml).unwrap();
        assert_eq!(model.name, "Empty");
        assert_eq!(model.element_count(), 0);
        assert_eq!(model.relation_count(), 0);
    }

    #[test]
    fn test_self_loop() {
        let yaml = r#"
model:
  name: Test
  elements:
    - id: e1
      name: Self
      kind: BusinessActor
  relationships:
    - id: r1
      source: e1
      target: e1
      kind: Association
"#;

        let model = parse_yaml(yaml).unwrap();
        assert_eq!(model.element_count(), 1);
        assert_eq!(model.relation_count(), 1);
        let rel = model.iter_relations().next().unwrap();
        assert_eq!(rel.kind, RelationKind::Association);
    }
}

#[test]
fn test_malformed_yaml_not_invalid_id() {
    // Stray indentation under `name:` — structurally broken YAML, not an id bug.
    let yaml = r#"
model:
  name: Malformed Model
  elements:
    - id: actor_001
      name: Broken
        kind: BusinessActor
"#;

    let result = parse_yaml(yaml);
    assert!(result.is_err(), "expected parse failure");
    let errors = result.unwrap_err();
    // The fix: a malformed file must NOT surface as InvalidId.
    assert!(
        !errors.contains(&SchemaError::InvalidId),
        "malformed YAML must not be reported as InvalidId: {:?}",
        errors
    );

    assert!(
        errors
            .iter()
            .any(|e| matches!(e, SchemaError::MalformedYaml(_))),
        "expected a MalformedYaml error in {:?}",
        errors
    );

    let malformed = errors
        .iter()
        .find_map(|e| match e {
            SchemaError::MalformedYaml(msg) => Some(msg),
            _ => None,
        })
        .expect("MalformedYaml present");
    assert!(
        !malformed.is_empty(),
        "MalformedYaml must carry the serde message, got empty"
    )
}
