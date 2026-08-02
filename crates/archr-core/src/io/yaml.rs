//! YAML I/O for ArchiMate models.
//!
//! Handles (de)serialization to/from YAML, with schema-level validation.

use crate::model::{ElementId, ElementKind, Model, RelationKind};
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlElement {
    id: String,
    name: String,
    kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlRelationship {
    id: String,
    source: String,
    target: String,
    kind: String,
}

/// Schema validation errors returned during parse.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SchemaError {
    UnknownKind,
    UndefinedId,
    DuplicateId,
    InvalidId,
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
        serde_yaml::from_str(input).map_err(|_| vec![SchemaError::InvalidId])?;

    let YamlModelInner {
        name,
        elements,
        relationships,
    } = yaml_model.model;

    let mut errors = Vec::new();

    // Validate element IDs (empty / spaces / duplicates) and element kinds.
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

    // Build id -> index map (only valid ids resolve; undefined refs flagged below).
    let mut id_to_index = std::collections::HashMap::new();
    for (idx, elem) in elements.iter().enumerate() {
        id_to_index.insert(elem.id.clone(), idx);
    }

    // Validate relationships: reference resolution + relation kind.
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

    if !errors.is_empty() {
        return Err(errors);
    }

    // Build the Model (all prior checks guarantee success here).
    let mut model = Model::new(name);
    let mut elem_ids: Vec<ElementId> = Vec::with_capacity(elements.len());
    for elem in &elements {
        let kind = ElementKind::from_name(&elem.kind).expect("validated above");
        let id = model.add_element(&elem.name, kind);
        elem_ids.push(id);
    }

    // Map original string ids to internal ElementId handles.
    let mut str_to_elem: std::collections::HashMap<String, ElementId> =
        std::collections::HashMap::new();
    for (elem, id) in elements.iter().zip(elem_ids.iter()) {
        str_to_elem.insert(elem.id.clone(), *id);
    }

    for rel in &relationships {
        let kind = RelationKind::from_name(&rel.kind).expect("validated above");
        let source = str_to_elem[&rel.source];
        let target = str_to_elem[&rel.target];
        model.link(source, target, kind);
    }

    Ok(model)
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
        );
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
        );
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
        );
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
        );
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
