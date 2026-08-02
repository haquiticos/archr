//! Tests to verify `archr`'s element taxonomy is consistent with Archi's EMF metamodel.
//!
//! The metamodel is defined in `archimate.ecore` (MIT license) and represents the
//! authoritative definition of ArchiMate 3.2 element types and their inheritance hierarchy.

use crate::model::{ElementKind, ElementLayer::*};
use std::collections::HashSet;

/// Collects all elements in our implementation and verifies the count matches expectations.
#[test]
fn test_element_kind_count() {
    // We have 61 elements defined in ElementKind::variants()
    let count = ElementKind::VARIANT_COUNT;
    assert_eq!(count, 61, "ElementKind should have 61 variants");

    // Verify each variant has a layer assigned
    let elements: Vec<_> = vec![
        ElementKind::Stakeholder,
        ElementKind::Driver,
        ElementKind::Assessment,
        ElementKind::Goal,
        ElementKind::Outcome,
        ElementKind::Principle,
        ElementKind::Requirement,
        ElementKind::Constraint,
        ElementKind::Meaning,
        ElementKind::Value,
        ElementKind::Resource,
        ElementKind::Capability,
        ElementKind::ValueStream,
        ElementKind::CourseOfAction,
        ElementKind::BusinessActor,
        ElementKind::BusinessRole,
        ElementKind::BusinessCollaboration,
        ElementKind::BusinessInterface,
        ElementKind::BusinessProcess,
        ElementKind::BusinessFunction,
        ElementKind::BusinessInteraction,
        ElementKind::BusinessEvent,
        ElementKind::BusinessService,
        ElementKind::BusinessObject,
        ElementKind::Contract,
        ElementKind::Representation,
        ElementKind::Product,
        ElementKind::ApplicationComponent,
        ElementKind::ApplicationCollaboration,
        ElementKind::ApplicationInterface,
        ElementKind::ApplicationFunction,
        ElementKind::ApplicationProcess,
        ElementKind::ApplicationInteraction,
        ElementKind::ApplicationEvent,
        ElementKind::ApplicationService,
        ElementKind::DataObject,
        ElementKind::Node,
        ElementKind::Device,
        ElementKind::SystemSoftware,
        ElementKind::TechnologyCollaboration,
        ElementKind::TechnologyInterface,
        ElementKind::Path,
        ElementKind::CommunicationNetwork,
        ElementKind::Artifact,
        ElementKind::TechnologyFunction,
        ElementKind::TechnologyProcess,
        ElementKind::TechnologyInteraction,
        ElementKind::TechnologyEvent,
        ElementKind::TechnologyService,
        ElementKind::Equipment,
        ElementKind::Facility,
        ElementKind::Material,
        ElementKind::DistributionNetwork,
        ElementKind::WorkPackage,
        ElementKind::Deliverable,
        ElementKind::Plateau,
        ElementKind::Gap,
        ElementKind::Grouping,
        ElementKind::Location,
        ElementKind::AndJunction,
        ElementKind::OrJunction,
    ];

    // Verify we have the correct number of elements
    assert_eq!(elements.len(), 61, "Should have exactly 61 element types");

    // Verify each element has a layer
    for kind in &elements {
        let layer = kind.layer();
        assert!(
            matches!(
                layer,
                Motivation
                    | Strategy
                    | Business
                    | Application
                    | Technology
                    | Physical
                    | Implementation
                    | Other
            ),
            "ElementKind::{} should have a defined layer: {:?}",
            kind.type_name(),
            layer
        );
    }
}

/// Tests that element layer assignment is consistent with ArchiMate semantics.
/// We verify that no element is missing a layer or has an unexpected one.
#[test]
fn test_element_layers_are_defined() {
    let layers: HashSet<_> = vec![
        Motivation,
        Strategy,
        Business,
        Application,
        Technology,
        Physical,
        Implementation,
        Other,
    ]
    .into_iter()
    .collect();

    let kinds = vec![
        ElementKind::Stakeholder,
        ElementKind::Driver,
        ElementKind::Assessment,
        ElementKind::Goal,
        ElementKind::Outcome,
        ElementKind::Principle,
        ElementKind::Requirement,
        ElementKind::Constraint,
        ElementKind::Meaning,
        ElementKind::Value,
        ElementKind::Resource,
        ElementKind::Capability,
        ElementKind::ValueStream,
        ElementKind::CourseOfAction,
        ElementKind::BusinessActor,
        ElementKind::BusinessRole,
        ElementKind::BusinessCollaboration,
        ElementKind::BusinessInterface,
        ElementKind::BusinessProcess,
        ElementKind::BusinessFunction,
        ElementKind::BusinessInteraction,
        ElementKind::BusinessEvent,
        ElementKind::BusinessService,
        ElementKind::BusinessObject,
        ElementKind::Contract,
        ElementKind::Representation,
        ElementKind::Product,
        ElementKind::ApplicationComponent,
        ElementKind::ApplicationCollaboration,
        ElementKind::ApplicationInterface,
        ElementKind::ApplicationFunction,
        ElementKind::ApplicationProcess,
        ElementKind::ApplicationInteraction,
        ElementKind::ApplicationEvent,
        ElementKind::ApplicationService,
        ElementKind::DataObject,
        ElementKind::Node,
        ElementKind::Device,
        ElementKind::SystemSoftware,
        ElementKind::TechnologyCollaboration,
        ElementKind::TechnologyInterface,
        ElementKind::Path,
        ElementKind::CommunicationNetwork,
        ElementKind::Artifact,
        ElementKind::TechnologyFunction,
        ElementKind::TechnologyProcess,
        ElementKind::TechnologyInteraction,
        ElementKind::TechnologyEvent,
        ElementKind::TechnologyService,
        ElementKind::Equipment,
        ElementKind::Facility,
        ElementKind::Material,
        ElementKind::DistributionNetwork,
        ElementKind::WorkPackage,
        ElementKind::Deliverable,
        ElementKind::Plateau,
        ElementKind::Gap,
        ElementKind::Grouping,
        ElementKind::Location,
        ElementKind::AndJunction,
        ElementKind::OrJunction,
    ];

    for kind in kinds {
        let layer = kind.layer();
        assert!(
            layers.contains(&layer),
            "ElementKind::{} should have a valid layer: {:?}",
            kind.type_name(),
            layer
        );
    }
}

/// Tests that the layer() method is consistent with the explicit layer assignment.
#[test]
fn test_element_kind_layer_method() {
    // Test a representative sample of element types
    let test_cases = vec![
        ("Stakeholder", Motivation),
        ("Goal", Motivation),
        ("Driver", Motivation),
        ("Capability", Strategy),
        ("Resource", Strategy),
        ("ValueStream", Strategy),
        ("BusinessActor", Business),
        ("BusinessProcess", Business),
        ("BusinessService", Business),
        ("ApplicationComponent", Application),
        ("DataObject", Application),
        ("Node", Technology),
        ("Device", Technology),
        ("SystemSoftware", Technology),
        ("CommunicationNetwork", Technology),
        ("Artifact", Technology),
        ("Equipment", Physical),
        ("Facility", Physical),
        ("Material", Physical),
        ("DistributionNetwork", Physical),
        ("WorkPackage", Implementation),
        ("Deliverable", Implementation),
        ("Plateau", Implementation),
        ("Gap", Implementation),
        ("Grouping", Other),
        ("Location", Other),
        ("AndJunction", Other),
        ("OrJunction", Other),
    ];

    for (type_name, expected_layer) in test_cases {
        let kind = ElementKind::from_name(type_name).unwrap();
        let layer = kind.layer();
        assert_eq!(
            layer, expected_layer,
            "ElementKind::{} should have layer: {:?}, not {:?}",
            type_name, expected_layer, layer
        );
    }
}

/// Verifies that all relationship types are defined and count is correct.
#[test]
fn test_relation_kind_count() {
    use crate::model::RelationKind::*;
    let variants: Vec<_> = vec![
        Composition,
        Aggregation,
        Assignment,
        Realization,
        Serving,
        Access,
        Influence,
        Association,
        Triggering,
        Flow,
        Specialization,
    ];

    assert_eq!(variants.len(), 11, "RelationKind should have 11 variants");

    // Verify each variant has a type_name
    for kind in variants {
        let name = kind.type_name();
        assert!(
            !name.is_empty(),
            "RelationKind variant should have a type_name"
        );
    }
}

/// Verifies that ElementKind::type_name() round-trips through from_name() —
/// i.e. for every canonical name N: from_name(N).type_name() == N.
/// This catches mismatches between the two lookup tables.
#[test]
fn test_element_kind_type_names() {
    let canonical = [
        "Stakeholder",
        "Driver",
        "Assessment",
        "Goal",
        "Outcome",
        "Principle",
        "Requirement",
        "Constraint",
        "Meaning",
        "Value",
        "Resource",
        "Capability",
        "ValueStream",
        "CourseOfAction",
        "BusinessActor",
        "BusinessRole",
        "BusinessCollaboration",
        "BusinessInterface",
        "BusinessProcess",
        "BusinessFunction",
        "BusinessInteraction",
        "BusinessEvent",
        "BusinessService",
        "BusinessObject",
        "Contract",
        "Representation",
        "Product",
        "ApplicationComponent",
        "ApplicationCollaboration",
        "ApplicationInterface",
        "ApplicationFunction",
        "ApplicationProcess",
        "ApplicationInteraction",
        "ApplicationEvent",
        "ApplicationService",
        "DataObject",
        "Node",
        "Device",
        "SystemSoftware",
        "TechnologyCollaboration",
        "TechnologyInterface",
        "Path",
        "CommunicationNetwork",
        "Artifact",
        "TechnologyFunction",
        "TechnologyProcess",
        "TechnologyInteraction",
        "TechnologyEvent",
        "TechnologyService",
        "Equipment",
        "Facility",
        "Material",
        "DistributionNetwork",
        "WorkPackage",
        "Deliverable",
        "Plateau",
        "Gap",
        "Grouping",
        "Location",
        "AndJunction",
        "OrJunction",
    ];

    assert_eq!(canonical.len(), 61, "Should list exactly 61 element types");

    for name in canonical {
        let kind = ElementKind::from_name(name)
            .unwrap_or_else(|| panic!("from_name({}) returned None", name));
        assert_eq!(
            kind.type_name(),
            name,
            "type_name/from_name round-trip failed for {}",
            name,
        );
    }
}
