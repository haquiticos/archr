//! ArchiMate 3.2 validation — data-driven derivability matrix.

use crate::model::{Element, ElementKind, ElementLayer, Model, RelationKind, Relationship};
use serde::Serialize;

/// A single validation error, serializable to JSON.
#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub element_source: String,
    pub suggestion: String,
}

/// Full validation report.
#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<ValidationError>,
}

/// Data-driven derivability matrix: `(source_layer, rel_kind, target_layer) -> true`.
///
/// The matrix is a const slice for zero-cost lookups.
const ALLOWED: &[(ElementLayer, RelationKind, ElementLayer)] = &[
    // -----------------------------------------------------------------------
    // Structural relationships: Composition, Aggregation, Assignment, Realization
    // -----------------------------------------------------------------------

    // Same-layer structural rules (all 8 layers × 4 = 32 entries)
    (ElementLayer::Strategy, RelationKind::Composition, ElementLayer::Strategy),
    (ElementLayer::Strategy, RelationKind::Aggregation, ElementLayer::Strategy),
    (ElementLayer::Strategy, RelationKind::Assignment, ElementLayer::Strategy),
    (ElementLayer::Strategy, RelationKind::Realization, ElementLayer::Strategy),
    (ElementLayer::Business, RelationKind::Composition, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Aggregation, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Assignment, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Realization, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Composition, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Aggregation, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Assignment, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Realization, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Composition, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Aggregation, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Assignment, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Realization, ElementLayer::Technology),
    (ElementLayer::Physical, RelationKind::Composition, ElementLayer::Physical),
    (ElementLayer::Physical, RelationKind::Aggregation, ElementLayer::Physical),
    (ElementLayer::Physical, RelationKind::Assignment, ElementLayer::Physical),
    (ElementLayer::Physical, RelationKind::Realization, ElementLayer::Physical),
    (ElementLayer::Motivation, RelationKind::Composition, ElementLayer::Motivation),
    (ElementLayer::Motivation, RelationKind::Aggregation, ElementLayer::Motivation),
    (ElementLayer::Motivation, RelationKind::Assignment, ElementLayer::Motivation),
    (ElementLayer::Motivation, RelationKind::Realization, ElementLayer::Motivation),
    (ElementLayer::Implementation, RelationKind::Composition, ElementLayer::Implementation),
    (ElementLayer::Implementation, RelationKind::Aggregation, ElementLayer::Implementation),
    (ElementLayer::Implementation, RelationKind::Assignment, ElementLayer::Implementation),
    (ElementLayer::Implementation, RelationKind::Realization, ElementLayer::Implementation),
    (ElementLayer::Other, RelationKind::Composition, ElementLayer::Other),
    (ElementLayer::Other, RelationKind::Aggregation, ElementLayer::Other),
    (ElementLayer::Other, RelationKind::Assignment, ElementLayer::Other),
    (ElementLayer::Other, RelationKind::Realization, ElementLayer::Other),

    // Realization: upward crossing (lower to higher layers)
    (ElementLayer::Implementation, RelationKind::Realization, ElementLayer::Strategy),
    (ElementLayer::Implementation, RelationKind::Realization, ElementLayer::Business),
    (ElementLayer::Implementation, RelationKind::Realization, ElementLayer::Application),
    (ElementLayer::Implementation, RelationKind::Realization, ElementLayer::Technology),
    (ElementLayer::Implementation, RelationKind::Realization, ElementLayer::Physical),

    (ElementLayer::Technology, RelationKind::Realization, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Realization, ElementLayer::Business),

    (ElementLayer::Application, RelationKind::Realization, ElementLayer::Business),

    // -----------------------------------------------------------------------
    // Serving: directional descending (higher layer serves lower layer)
    // -----------------------------------------------------------------------
    // Descending chain (active structure serves the behavior above it):
    //   Strategy ← Business ← Application ← Technology ← Physical
    // Permitted skips documented in the README:
    //   Physical→Technology, Technology→Application, Technology→Business,
    //   Application→Business, Application→Strategy, Business→Strategy.
    (ElementLayer::Physical, RelationKind::Serving, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Serving, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Serving, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Serving, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Serving, ElementLayer::Strategy),
    (ElementLayer::Business, RelationKind::Serving, ElementLayer::Strategy),

    // -----------------------------------------------------------------------
    // Access: Application↔Technology, Application↔Business, Application↔DataObject
    // -----------------------------------------------------------------------
    (ElementLayer::Application, RelationKind::Access, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Access, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Access, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Access, ElementLayer::Application),

    // Access to/from DataObject (Application layer)
    (ElementLayer::Application, RelationKind::Access, ElementLayer::Application),

    // -----------------------------------------------------------------------
    // Influence: any layer to any layer (fully permissive)
    // -----------------------------------------------------------------------
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Strategy, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Business, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Application, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Technology, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Physical, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Motivation, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Implementation, RelationKind::Influence, ElementLayer::Other),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Strategy),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Business),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Application),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Technology),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Physical),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Motivation),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Implementation),
    (ElementLayer::Other, RelationKind::Influence, ElementLayer::Other),

    // -----------------------------------------------------------------------
    // Association: any layer to any layer (fully permissive)
    // -----------------------------------------------------------------------
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Strategy, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Business, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Application, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Technology, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Physical, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Motivation, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Implementation, RelationKind::Association, ElementLayer::Other),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Strategy),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Business),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Application),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Technology),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Physical),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Motivation),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Implementation),
    (ElementLayer::Other, RelationKind::Association, ElementLayer::Other),

    // -----------------------------------------------------------------------
    // Triggering / Flow: within same layer only
    // -----------------------------------------------------------------------
    (ElementLayer::Strategy, RelationKind::Triggering, ElementLayer::Strategy),
    (ElementLayer::Strategy, RelationKind::Flow, ElementLayer::Strategy),
    (ElementLayer::Business, RelationKind::Triggering, ElementLayer::Business),
    (ElementLayer::Business, RelationKind::Flow, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Triggering, ElementLayer::Application),
    (ElementLayer::Application, RelationKind::Flow, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Triggering, ElementLayer::Technology),
    (ElementLayer::Technology, RelationKind::Flow, ElementLayer::Technology),
    (ElementLayer::Physical, RelationKind::Triggering, ElementLayer::Physical),
    (ElementLayer::Physical, RelationKind::Flow, ElementLayer::Physical),
    (ElementLayer::Motivation, RelationKind::Triggering, ElementLayer::Motivation),
    (ElementLayer::Motivation, RelationKind::Flow, ElementLayer::Motivation),
    (ElementLayer::Implementation, RelationKind::Triggering, ElementLayer::Implementation),
    (ElementLayer::Implementation, RelationKind::Flow, ElementLayer::Implementation),
    (ElementLayer::Other, RelationKind::Triggering, ElementLayer::Other),
    (ElementLayer::Other, RelationKind::Flow, ElementLayer::Other),

    // -----------------------------------------------------------------------
    // Specialization: within same layer only
    // -----------------------------------------------------------------------
    (ElementLayer::Strategy, RelationKind::Specialization, ElementLayer::Strategy),
    (ElementLayer::Business, RelationKind::Specialization, ElementLayer::Business),
    (ElementLayer::Application, RelationKind::Specialization, ElementLayer::Application),
    (ElementLayer::Technology, RelationKind::Specialization, ElementLayer::Technology),
    (ElementLayer::Physical, RelationKind::Specialization, ElementLayer::Physical),
    (ElementLayer::Motivation, RelationKind::Specialization, ElementLayer::Motivation),
    (ElementLayer::Implementation, RelationKind::Specialization, ElementLayer::Implementation),
    (ElementLayer::Other, RelationKind::Specialization, ElementLayer::Other),
];

/// Find an entry in the derivability matrix.
///
/// Returns `true` if the triple `(source_layer, rel_kind, target_layer)` is allowed.
fn is_allowed(source_layer: ElementLayer, rel_kind: RelationKind, target_layer: ElementLayer) -> bool {
    ALLOWED
        .iter()
        .any(|&(s, k, t)| s == source_layer && k == rel_kind && t == target_layer)
}

/// Validate a single relationship given the source and target element kinds.
pub fn validate_relationship(
    source_kind: ElementKind,
    target_kind: ElementKind,
    rel_kind: RelationKind,
) -> Result<(), ValidationError> {
    let source_layer = source_kind.layer();
    let target_layer = target_kind.layer();

    if !is_allowed(source_layer, rel_kind, target_layer) {
        Err(ValidationError {
            code: "INVALID_RELATIONSHIP".to_string(),
            message: format!("{} cannot {} {}", source_kind, rel_kind, target_kind),
            element_source: source_kind.type_name().to_string(),
            suggestion: format!("Change relation type or ensure source/target are compatible"),
        })
    } else {
        Ok(())
    }
}

/// Validate an entire model.
pub fn validate_model(model: &Model) -> ValidationResult {
    let mut errors = Vec::new();

    for rel in model.iter_relations() {
        if let Err(err) = validate_relationship(
            model.element(rel.source).kind,
            model.element(rel.target).kind,
            rel.kind,
        ) {
            errors.push(err);
        }
    }

    ValidationResult {
        success: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Model, ElementKind, RelationKind};

    fn create_model() -> Model {
        Model::new("Test Model")
    }

    #[test]
    fn test_valid_serving() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let component = model.add_element("ApplicationComponent", ElementKind::ApplicationComponent);
        model.link(component, actor, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success);
    }

    #[test]
    fn test_invalid_realization_crosslayer() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let node = model.add_element("Node", ElementKind::Node);
        model.link(actor, node, RelationKind::Realization);
        let result = validate_model(&model);
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "INVALID_RELATIONSHIP");
        assert_eq!(
            result.errors[0].element_source,
            "BusinessActor"
        );
    }

    #[test]
    fn test_association_any_layer() {
        let mut model = create_model();
        let stakeholder = model.add_element("Stakeholder", ElementKind::Stakeholder);
        let node = model.add_element("Node", ElementKind::Node);
        model.link(stakeholder, node, RelationKind::Association);
        let result = validate_model(&model);
        assert!(result.success);
    }

    #[test]
    fn test_specialization_within_layer() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let role = model.add_element("BusinessRole", ElementKind::BusinessRole);
        model.link(actor, role, RelationKind::Specialization);
        let result = validate_model(&model);
        assert!(result.success);
    }

    #[test]
    fn test_specialization_cross_layer_fails() {
        let mut model = create_model();
        let stakeholder = model.add_element("Stakeholder", ElementKind::Stakeholder);
        let node = model.add_element("Node", ElementKind::Node);
        model.link(stakeholder, node, RelationKind::Specialization);
        let result = validate_model(&model);
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "INVALID_RELATIONSHIP");
    }

    #[test]
    fn test_validate_model_clean() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let component = model.add_element("ApplicationComponent", ElementKind::ApplicationComponent);
        model.link(component, actor, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_model_with_errors() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let node = model.add_element("Node", ElementKind::Node);
        model.link(actor, node, RelationKind::Realization);
        let result = validate_model(&model);
        assert!(!result.success);
        assert!(!result.errors.is_empty());
    }
    // --- Regression tests for Serving block fixes (issue #4) ---
    // Allowed descending directions (README rule "Tech→App→Business→Strategy"):
    //   Physical→Technology, Technology→Application, Technology→Business,
    //   Application→Business, Application→Strategy, Business→Strategy.




    #[test]
    fn test_serving_valid_tech_app() {
        let mut model = create_model();
        let node = model.add_element("Node", ElementKind::Node);
        let component = model.add_element("ApplicationComponent", ElementKind::ApplicationComponent);
        model.link(node, component, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success, "Technology→Application Serving should be valid");
    }

    #[test]
    fn test_serving_valid_tech_business() {
        let mut model = create_model();
        let node = model.add_element("Node", ElementKind::Node);
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        model.link(node, actor, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success, "Technology→Business Serving should be valid");
    }

    #[test]
    fn test_serving_valid_app_business() {
        let mut model = create_model();
        let component = model.add_element("ApplicationComponent", ElementKind::ApplicationComponent);
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        model.link(component, actor, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success, "Application→Business Serving should be valid");
    }

    #[test]
    fn test_serving_valid_app_strategy() {
        let mut model = create_model();
        let process = model.add_element("BusinessProcess", ElementKind::BusinessProcess);
        let value_stream = model.add_element("ValueStream", ElementKind::ValueStream);
        model.link(process, value_stream, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success, "Application→Strategy Serving should be valid");
    }

    #[test]
    fn test_serving_valid_business_strategy() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let value_stream = model.add_element("ValueStream", ElementKind::ValueStream);
        model.link(actor, value_stream, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(result.success, "Business→Strategy Serving should be valid");
    }

    #[test]
    fn test_serving_invalid_business_app() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let component = model.add_element("ApplicationComponent", ElementKind::ApplicationComponent);
        model.link(actor, component, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(!result.success, "Business→Application Serving should be invalid");
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "INVALID_RELATIONSHIP");
    }

    #[test]
    fn test_serving_invalid_app_tech() {
        let mut model = create_model();
        let component = model.add_element("ApplicationComponent", ElementKind::ApplicationComponent);
        let node = model.add_element("Node", ElementKind::Node);
        model.link(component, node, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(!result.success, "Application→Technology Serving should be invalid");
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "INVALID_RELATIONSHIP");
    }


    #[test]
    fn test_serving_invalid_same_layer_tech() {
        let mut model = create_model();
        let node1 = model.add_element("Node", ElementKind::Node);
        let node2 = model.add_element("Node", ElementKind::Node);
        model.link(node1, node2, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(!result.success, "Technology→Technology Serving should be invalid");
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "INVALID_RELATIONSHIP");
    }

    #[test]
    fn test_serving_invalid_same_layer_business() {
        let mut model = create_model();
        let actor = model.add_element("BusinessActor", ElementKind::BusinessActor);
        let role = model.add_element("BusinessRole", ElementKind::BusinessRole);
        model.link(actor, role, RelationKind::Serving);
        let result = validate_model(&model);
        assert!(!result.success, "Business→Business Serving should be invalid");
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "INVALID_RELATIONSHIP");
    }
}
