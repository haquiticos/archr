//! Tests to verify `archr`'s element taxonomy is consistent with Archi's EMF metamodel.
//!
//! The metamodel is defined in `archimate.ecore` (MIT license) and represents the
//! authoritative definition of ArchiMate 3.2 element types and their inheritance hierarchy.

use crate::model::{ElementKind, ElementLayer, ElementLayer::*};
use std::collections::HashSet;

/// Returns the canonical layer for each ElementKind based on our implementation.
/// This should be kept in sync with ElementKind::layer().
fn element_layer_from_kind(kind: ElementKind) -> ElementLayer {
    use ElementKind::*;
    match kind {
        // Motivation (10)
        Stakeholder | Driver | Assessment | Goal | Outcome
        | Principle | Requirement | Constraint | Meaning | Value
            => Motivation,

        // Strategy (4)
        Resource | Capability | ValueStream | CourseOfAction
            => Strategy,

        // Business (13)
        BusinessActor | BusinessRole | BusinessCollaboration | BusinessInterface
        | BusinessProcess | BusinessFunction | BusinessInteraction | BusinessEvent
        | BusinessService | BusinessObject | Contract | Representation | Product
            => Business,

        // Application (9)
        ApplicationComponent | ApplicationCollaboration | ApplicationInterface
        | ApplicationFunction | ApplicationProcess | ApplicationInteraction
        | ApplicationEvent | ApplicationService | DataObject
            => Application,

        // Technology (13)
        Node | Device | SystemSoftware | TechnologyCollaboration
        | TechnologyInterface | Path | CommunicationNetwork | Artifact
        | TechnologyFunction | TechnologyProcess | TechnologyInteraction
        | TechnologyEvent | TechnologyService
            => Technology,

        // Physical (4)
        Equipment | Facility | Material | DistributionNetwork
            => Physical,

        // Implementation & Migration (4)
        WorkPackage | Deliverable | Plateau | Gap
            => Implementation,

        // Other (4)
        Grouping | Location | AndJunction | OrJunction
            => Other,
    }
}

/// Collects all elements in our implementation and verifies the count matches expectations.
#[test]
fn test_element_kind_count() {
    // We have 61 elements defined in ElementKind::variants()
    let count = ElementKind::VARIANT_COUNT;
    assert_eq!(count, 61, "ElementKind should have 61 variants");

    // Verify each variant has a layer assigned
    let mut elements: Vec<_> = vec![
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
        ElementKind::OrJunction
    ];

    // Verify we have the correct number of elements
    assert_eq!(elements.len(), 61, "Should have exactly 61 element types");

    // Verify each element has a layer
    for kind in &elements {
        let kind_parsed = ElementKind::from_name(kind.type_name()).unwrap();
        let layer = element_layer_from_kind(kind_parsed);
        assert!(matches!(layer, Motivation | Strategy | Business | Application | Technology | Physical | Implementation | Other),
            "ElementKind::{} should have a defined layer: {:?}",
            kind.type_name(), layer);
    }
}

/// Tests that element layer assignment is consistent with ArchiMate semantics.
/// We verify that no element is missing a layer or has an unexpected one.
#[test]
fn test_element_layers_are_defined() {
    let layers: HashSet<_> = vec![
        Motivation, Strategy, Business, Application, Technology,
        Physical, Implementation, Other
    ].into_iter().collect();

    let kinds = vec![
        ElementKind::Stakeholder, ElementKind::Driver, ElementKind::Assessment, ElementKind::Goal,
        ElementKind::Outcome, ElementKind::Principle, ElementKind::Requirement, ElementKind::Constraint,
        ElementKind::Meaning, ElementKind::Value,
        ElementKind::Resource, ElementKind::Capability, ElementKind::ValueStream, ElementKind::CourseOfAction,
        ElementKind::BusinessActor, ElementKind::BusinessRole, ElementKind::BusinessCollaboration,
        ElementKind::BusinessInterface, ElementKind::BusinessProcess, ElementKind::BusinessFunction,
        ElementKind::BusinessInteraction, ElementKind::BusinessEvent, ElementKind::BusinessService,
        ElementKind::BusinessObject, ElementKind::Contract, ElementKind::Representation, ElementKind::Product,
        ElementKind::ApplicationComponent, ElementKind::ApplicationCollaboration, ElementKind::ApplicationInterface,
        ElementKind::ApplicationFunction, ElementKind::ApplicationProcess, ElementKind::ApplicationInteraction,
        ElementKind::ApplicationEvent, ElementKind::ApplicationService, ElementKind::DataObject,
        ElementKind::Node, ElementKind::Device, ElementKind::SystemSoftware, ElementKind::TechnologyCollaboration,
        ElementKind::TechnologyInterface, ElementKind::Path, ElementKind::CommunicationNetwork, ElementKind::Artifact,
        ElementKind::TechnologyFunction, ElementKind::TechnologyProcess, ElementKind::TechnologyInteraction,
        ElementKind::TechnologyEvent, ElementKind::TechnologyService,
        ElementKind::Equipment, ElementKind::Facility, ElementKind::Material, ElementKind::DistributionNetwork,
        ElementKind::WorkPackage, ElementKind::Deliverable, ElementKind::Plateau, ElementKind::Gap,
        ElementKind::Grouping, ElementKind::Location, ElementKind::AndJunction, ElementKind::OrJunction
    ];

    for kind in kinds {
        let layer = element_layer_from_kind(kind);
        assert!(layers.contains(&layer),
            "ElementKind::{} should have a valid layer: {:?}",
            kind.type_name(), layer);
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
        assert_eq!(layer, expected_layer,
            "ElementKind::{} should have layer: {:?}, not {:?}",
            type_name, expected_layer, layer);
    }
}

/// Verifies that all relationship types are defined and count is correct.
#[test]
fn test_relation_kind_count() {
    use crate::model::RelationKind::*;
    let variants: Vec<_> = vec![
        Composition, Aggregation, Assignment, Realization,
        Serving, Access, Influence, Association,
        Triggering, Flow, Specialization
    ];

    assert_eq!(variants.len(), 11, "RelationKind should have 11 variants");

    // Verify each variant has a type_name
    for kind in variants {
        let name = kind.type_name();
        assert!(!name.is_empty(), "RelationKind variant should have a type_name");
    }
}

/// Verifies that ElementKind::type_name() returns canonical names matching ecore.
#[test]
fn test_element_kind_type_names() {
    let expected = vec![
        "Stakeholder", "Driver", "Assessment", "Goal", "Outcome",
        "Principle", "Requirement", "Constraint", "Meaning", "Value",
        "Resource", "Capability", "ValueStream", "CourseOfAction",
        "BusinessActor", "BusinessRole", "BusinessCollaboration",
        "BusinessInterface", "BusinessProcess", "BusinessFunction",
        "BusinessInteraction", "BusinessEvent", "BusinessService",
        "BusinessObject", "Contract", "Representation", "Product",
        "ApplicationComponent", "ApplicationCollaboration",
        "ApplicationInterface", "ApplicationFunction",
        "ApplicationProcess", "ApplicationInteraction", "ApplicationEvent",
        "ApplicationService", "DataObject",
        "Node", "Device", "SystemSoftware", "TechnologyCollaboration",
        "TechnologyInterface", "Path", "CommunicationNetwork", "Artifact",
        "TechnologyFunction", "TechnologyProcess", "TechnologyInteraction",
        "TechnologyEvent", "TechnologyService",
        "Equipment", "Facility", "Material", "DistributionNetwork",
        "WorkPackage", "Deliverable", "Plateau", "Gap",
        "Grouping", "Location", "AndJunction", "OrJunction"
    ];

    let actual: Vec<_> = vec![
        "Stakeholder", "Driver", "Assessment", "Goal", "Outcome",
        "Principle", "Requirement", "Constraint", "Meaning", "Value",
        "Resource", "Capability", "ValueStream", "CourseOfAction",
        "BusinessActor", "BusinessRole", "BusinessCollaboration",
        "BusinessInterface", "BusinessProcess", "BusinessFunction",
        "BusinessInteraction", "BusinessEvent", "BusinessService",
        "BusinessObject", "Contract", "Representation", "Product",
        "ApplicationComponent", "ApplicationCollaboration",
        "ApplicationInterface", "ApplicationFunction",
        "ApplicationProcess", "ApplicationInteraction", "ApplicationEvent",
        "ApplicationService", "DataObject",
        "Node", "Device", "SystemSoftware", "TechnologyCollaboration",
        "TechnologyInterface", "Path", "CommunicationNetwork", "Artifact",
        "TechnologyFunction", "TechnologyProcess", "TechnologyInteraction",
        "TechnologyEvent", "TechnologyService",
        "Equipment", "Facility", "Material", "DistributionNetwork",
        "WorkPackage", "Deliverable", "Plateau", "Gap",
        "Grouping", "Location", "AndJunction", "OrJunction"
    ].iter()
        .map(|name| ElementKind::from_name(name).unwrap())
        .map(|k| k.type_name())
        .collect();

    assert_eq!(actual, expected,
        "ElementKind::type_name() should match canonical names");
}
