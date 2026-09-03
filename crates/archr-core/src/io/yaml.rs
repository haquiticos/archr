//! YAML I/O for ArchiMate models.
//!
//! Handles (de)serialization to/from YAML, with schema-level validation.
use std::collections::HashMap;

use crate::model::{ElementId, ElementKind, Model, RelationId, RelationKind, ViewpointDefinition};
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
    viewpoints: Vec<ViewpointDefinition>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlElement {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
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

/// Parses YAML input into an ArchiMate Model.
///
/// Returns errors if:
/// - Any element `kind` does not match an ElementKind variant
/// - Any relationship `kind` does not match a RelationKind variant
/// - Any relationship references a non-existent element
/// - Any element `id` is duplicated
/// - Any `id` is empty or contains spaces
pub fn parse_yaml(input: &str) -> ParseResult<Model> {
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

    // Build set of valid global relationship IDs for viewpoint reference validation.
    let rel_ids_valid: HashSet<String> = relationships.iter().map(|r| r.id.clone()).collect();

    // Validate viewpoint element and relationship id-references (after global elements are created)
    for vp_def in viewpoints.iter() {
        // Validate viewpoint element IDs — must reference existing global elements.
        let mut vp_seen_ids: HashSet<String> = HashSet::new();
        for elem_id in &vp_def.elements {
            if elem_id.is_empty() || elem_id.contains(' ') {
                errors.push(SchemaError::InvalidId);
            } else if !vp_seen_ids.insert(elem_id.clone()) {
                errors.push(SchemaError::DuplicateId);
            }
            if !elem_id.is_empty() && !id_to_index.contains_key(elem_id) {
                errors.push(SchemaError::UndefinedId);
            }
        }

        // Validate viewpoint relationship IDs — must reference existing global relationships.
        let mut vp_seen_rels: HashSet<String> = HashSet::new();
        for rel_id in &vp_def.relationships {
            if rel_id.is_empty() || rel_id.contains(' ') {
                errors.push(SchemaError::InvalidId);
            } else if !vp_seen_rels.insert(rel_id.clone()) {
                errors.push(SchemaError::DuplicateId);
            }
            // Viewpoint relationship id must reference an existing global relationship.
            if !rel_id.is_empty() && !rel_ids_valid.contains(rel_id) {
                errors.push(SchemaError::UndefinedId);
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    // Build the Model. Elements carry their original ids so serialization
    // round-trips faithfully; the string-id maps below are local resolution
    // details, not part of the interface.
    let mut model = Model::new(name);
    let mut str_to_elem: HashMap<String, ElementId> = HashMap::new();
    for elem in &elements {
        let kind = ElementKind::from_name(&elem.kind).expect("validated above");
        let id = model.add_element_with_id(&elem.id, &elem.name, kind);
        str_to_elem.insert(elem.id.clone(), id);
    }

    let mut str_to_rel: HashMap<String, RelationId> = HashMap::new();
    for rel in &relationships {
        let kind = RelationKind::from_name(&rel.kind).expect("validated above");
        let source = str_to_elem[&rel.source];
        let target = str_to_elem[&rel.target];
        let rel_id = model.link_with_id(&rel.id, source, target, kind);
        str_to_rel.insert(rel.id.clone(), rel_id);
    }


    // Attach viewpoint definitions to the model so serialization can emit one
    // diagram per viewpoint.
    model.set_viewpoints(viewpoints);
    Ok(model)
}

/// Serializes an ArchiMate Model back to YAML.
///
/// Original ids and viewpoint definitions are emitted verbatim from the
/// Model, so YAML → Model → YAML round trips are faithful.
pub fn model_to_yaml(model: &Model) -> String {
    let inner = YamlModelInner {
        name: model.name.clone(),
        elements: model
            .iter_elements()
            .map(|elem| YamlElement {
                id: elem.original_id.clone(),
                name: elem.name.clone(),
                kind: elem.kind.to_string(),
            })
            .collect(),
        relationships: model
            .iter_relations()
            .map(|rel| YamlRelationship {
                id: rel.original_id.clone(),
                source: model.element(rel.source).original_id.clone(),
                target: model.element(rel.target).original_id.clone(),
                kind: rel.kind.to_string(),
            })
            .collect(),
        viewpoints: model.viewpoints().to_vec(),
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

        // Original ids survive the round trip verbatim.
        let elem_ids: Vec<&str> = reparsed.iter_elements().map(|e| e.original_id.as_str()).collect();
        assert_eq!(elem_ids, vec!["e1", "e2"]);
        let rel = reparsed.iter_relations().next().unwrap();
        assert_eq!(rel.original_id, "r1");
        assert_eq!(model.element(reparsed.iter_relations().next().unwrap().source).original_id, "e1");
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
    #[test]
    fn test_viewpoint_round_trip() {
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
  viewpoints:
    - id: vp1
      name: Business view
      kind: business
      elements: [e1]
      relationships: [r1]
"#;

        let model = parse_yaml(original).unwrap();
        let serialized = model_to_yaml(&model);
        let reparsed = parse_yaml(&serialized).unwrap();

        assert_eq!(reparsed.viewpoints().len(), 1, "viewpoints must survive YAML round trip");
        let vp = &reparsed.viewpoints()[0];
        assert_eq!(vp.id, "vp1");
        assert_eq!(vp.name, "Business view");
        assert_eq!(vp.kind, crate::model::ViewpointKind::Business);
        assert_eq!(vp.elements, vec!["e1"]);
        assert_eq!(vp.relationships, vec!["r1"]);
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
