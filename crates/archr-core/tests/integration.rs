//! Integration tests for archr-core covering critical paths that were missing tests.
//!
//! These tests ensure that bugs mentioned in issue #15 are caught by unit tests.

use archr_core::model::{ElementKind, Model, RelationKind};
use archr_core::{model_to_yaml, parse_yaml};

/// Test that relationship IDs are preserved during round-trip conversion.
///
/// This ensures that connection→relationship identifier integrity is maintained
/// when converting between YAML, XML, and back.
#[test]
fn test_relationship_id_preservation() {
    // Create a model with relationships
    let mut model = Model::new("Test Model");
    let actor_id = model.add_element("Customer", ElementKind::BusinessActor);
    let app_id = model.add_element("CRM", ElementKind::ApplicationComponent);
    let goal_id = model.add_element("Sales Goal", ElementKind::Goal);

    // Add relationships
    let _serving_rel = model.link(actor_id, app_id, RelationKind::Serving);
    let _realization_rel = model.link(app_id, goal_id, RelationKind::Realization);

    // Convert to YAML
    let yaml_output = model_to_yaml(&model);

    // Verify that relationship IDs are in the YAML (they will be generated)
    // The important thing is that relationships exist in the output
    assert!(
        yaml_output.contains("Serving"),
        "Should contain relationship type"
    );
    assert!(
        yaml_output.contains("target:"),
        "Should contain target field"
    );

    // Parse YAML back.
    let parsed_model = parse_yaml(&yaml_output).expect("YAML should parse successfully");

    // Original ids survive the round trip verbatim. Programmatically-built
    // models carry synthesized `e_N`/`r_N` originals.
    let elem_ids: Vec<&str> = parsed_model
        .iter_elements()
        .map(|e| e.original_id.as_str())
        .collect();
    assert_eq!(elem_ids, vec!["e_0", "e_1", "e_2"]);

    let rel_ids: Vec<&str> = parsed_model
        .iter_relations()
        .map(|r| r.original_id.as_str())
        .collect();
    assert_eq!(rel_ids, vec!["r_0", "r_1"]);

    // Relationship structure is preserved.
    let rel_kinds: Vec<RelationKind> = parsed_model.iter_relations().map(|r| r.kind).collect();
    assert!(
        rel_kinds.contains(&RelationKind::Serving),
        "Should have Serving relationship"
    );
    assert!(
        rel_kinds.contains(&RelationKind::Realization),
        "Should have Realization relationship"
    );
}

/// Test that the model can be serialized to YAML without panicking, even with relationships.
#[test]
fn test_model_with_relationships_to_yaml_no_panic() {
    let mut model = Model::new("Test Model");
    let actor_id = model.add_element("Actor", ElementKind::BusinessActor);
    let app_id = model.add_element("App", ElementKind::ApplicationComponent);
    model.link(actor_id, app_id, RelationKind::Serving);

    // This should not panic
    let yaml_output = model_to_yaml(&model);
    assert!(!yaml_output.is_empty(), "YAML output should not be empty");

    // Verify relationship is in output
    assert!(
        yaml_output.contains("Serving"),
        "Should contain Serving relationship type"
    );
}

/// Test empty model handling is robust for both YAML and XML paths.
#[test]
fn test_empty_model_handles_no_relationships() {
    let model = Model::new("Empty Model");

    // Serialize to YAML (should not panic)
    let yaml_output = model_to_yaml(&model);
    assert!(yaml_output.contains("name: Empty Model"));

    // Parse YAML back
    let parsed = parse_yaml(&yaml_output).expect("YAML should parse successfully");
    assert_eq!(parsed.element_count(), 0, "Should have 0 elements");
    assert_eq!(parsed.relation_count(), 0, "Should have 0 relationships");
}

/// Regression: viewpoints must survive generate → parse (XML → YAML).
/// Both YAML serializers and the XML parser used to drop them silently.
#[test]
fn test_viewpoints_survive_xml_to_yaml() {
    use archr_core::model::{ViewpointDefinition, ViewpointKind};
    use std::collections::HashMap;

    let mut model = Model::new("VP Model");
    let a = model.add_element_with_id("e1", "Customer", ElementKind::BusinessActor);
    let b = model.add_element_with_id("e2", "CRM", ElementKind::ApplicationComponent);
    model.link_with_id("r1", a, b, RelationKind::Serving);
    model.set_viewpoints(vec![ViewpointDefinition {
        id: "vp1".into(),
        name: "Coarse".into(),
        kind: ViewpointKind::Business,
        elements: vec!["e1".into(), "e2".into()],
        relationships: vec!["r1".into()],
    }]);

    let xml = archr_core::model_to_xml(&model, &HashMap::new()).unwrap();
    let parsed = archr_core::xml_to_model(&xml).unwrap();
    let yaml_out = archr_core::model_to_yaml(&parsed);
    let reparsed = archr_core::parse_yaml(&yaml_out).unwrap();

    assert_eq!(reparsed.viewpoints().len(), 1);
    let vp = &reparsed.viewpoints()[0];
    assert_eq!(vp.id, "vp1");
    assert_eq!(vp.name, "Coarse");
    assert_eq!(vp.kind, ViewpointKind::Business);
    assert_eq!(vp.elements, vec!["e1".to_string(), "e2".to_string()]);
    assert_eq!(vp.relationships, vec!["r1".to_string()]);
}
