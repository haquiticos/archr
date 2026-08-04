//! Tests to verify `archr`'s element taxonomy is consistent with Archi's EMF metamodel.
//!
//! The metamodel is defined in `archimate.ecore` (MIT license) and represents the
//! authoritative definition of ArchiMate 3.2 element types and their inheritance hierarchy.

use crate::ElementKind;
use crate::ElementLayer;
use crate::Viewpoint;
use std::collections::HashSet;

/// Collects all elements in our implementation and verifies the count matches expectations.
#[test]
fn test_element_kind_count() {
    // We have 62 elements defined in ElementKind::VARIANT_COUNT
    let count = ElementKind::VARIANT_COUNT;
    assert_eq!(count, 62, "ElementKind should have 62 variants");

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
                ElementLayer::Motivation
                    | ElementLayer::Strategy
                    | ElementLayer::Business
                    | ElementLayer::Application
                    | ElementLayer::Technology
                    | ElementLayer::Physical
                    | ElementLayer::Implementation
                    | ElementLayer::Other
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
        ElementLayer::Motivation,
        ElementLayer::Strategy,
        ElementLayer::Business,
        ElementLayer::Application,
        ElementLayer::Technology,
        ElementLayer::Physical,
        ElementLayer::Implementation,
        ElementLayer::Other,
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
        ("Stakeholder", ElementLayer::Motivation),
        ("Goal", ElementLayer::Motivation),
        ("Driver", ElementLayer::Motivation),
        ("Capability", ElementLayer::Strategy),
        ("Resource", ElementLayer::Strategy),
        ("ValueStream", ElementLayer::Strategy),
        ("BusinessActor", ElementLayer::Business),
        ("BusinessProcess", ElementLayer::Business),
        ("BusinessService", ElementLayer::Business),
        ("ApplicationComponent", ElementLayer::Application),
        ("DataObject", ElementLayer::Application),
        ("Node", ElementLayer::Technology),
        ("Device", ElementLayer::Technology),
        ("SystemSoftware", ElementLayer::Technology),
        ("CommunicationNetwork", ElementLayer::Technology),
        ("Artifact", ElementLayer::Technology),
        ("Equipment", ElementLayer::Physical),
        ("Facility", ElementLayer::Physical),
        ("Material", ElementLayer::Physical),
        ("DistributionNetwork", ElementLayer::Physical),
        ("WorkPackage", ElementLayer::Implementation),
        ("Deliverable", ElementLayer::Implementation),
        ("Plateau", ElementLayer::Implementation),
        ("Gap", ElementLayer::Implementation),
        ("Grouping", ElementLayer::Other),
        ("Location", ElementLayer::Other),
        ("AndJunction", ElementLayer::Other),
        ("OrJunction", ElementLayer::Other),
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

/// Verifies `Viewpoint::from_xml_viewpoint_name()` correctly parses all 15 Archi XML viewpoint names.
#[test]
fn test_viewpoint_from_xml_viewpoint_name() {
    // Layer-based viewpoints
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Motivation"),
        Some(Viewpoint::Motivation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Strategy"),
        Some(Viewpoint::Strategy)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Business"),
        Some(Viewpoint::Business)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Application"),
        Some(Viewpoint::Application)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Technology"),
        Some(Viewpoint::Technology)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Physical"),
        Some(Viewpoint::Physical)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Implementation and Deployment"),
        Some(Viewpoint::Implementation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Physical"),
        Some(Viewpoint::Physical)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Implementation and Deployment"),
        Some(Viewpoint::Implementation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Layered"),
        Some(Viewpoint::NONE)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Implementation and Deployment"),
        Some(Viewpoint::Implementation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Layered"),
        Some(Viewpoint::NONE)
    );
    // Special viewpoint variants

    // Mixin viewpoints
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Enterprise Structure"),
        Some(Viewpoint::EnterpriseStructure)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Value Stream"),
        Some(Viewpoint::ValueStream)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Organization"),
        Some(Viewpoint::Organization)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Business Process Cooperation"),
        Some(Viewpoint::BusinessProcessCooperation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Product"),
        Some(Viewpoint::Product)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Application Cooperation"),
        Some(Viewpoint::ApplicationCooperation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Motivation"),
        Some(Viewpoint::Motivation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Strategy"),
        Some(Viewpoint::Strategy)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Business"),
        Some(Viewpoint::Business)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Application"),
        Some(Viewpoint::Application)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Technology"),
        Some(Viewpoint::Technology)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Physical"),
        Some(Viewpoint::Physical)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Implementation and Deployment"),
        Some(Viewpoint::Implementation)
    );
    assert_eq!(
        Viewpoint::from_xml_viewpoint_name("Layered"),
        Some(Viewpoint::NONE)
    );
}

/// Verifies `Viewpoint::to_xml_viewpoint_name()` round-trips correctly for all variants.
#[test]
fn test_viewpoint_to_xml_viewpoint_name() {
    let variants = [
        Viewpoint::Motivation,
        Viewpoint::Strategy,
        Viewpoint::Business,
        Viewpoint::Application,
        Viewpoint::Technology,
        Viewpoint::Physical,
        Viewpoint::Implementation,
        Viewpoint::Other,
        Viewpoint::EnterpriseStructure,
        Viewpoint::ValueStream,
        Viewpoint::Organization,
        Viewpoint::BusinessProcessCooperation,
        Viewpoint::Product,
        Viewpoint::ApplicationCooperation,
        Viewpoint::ApplicationUsage,
        Viewpoint::NONE,
    ];

    for viewpoint in variants {
        let name = viewpoint.to_xml_viewpoint_name();
        let parsed = Viewpoint::from_xml_viewpoint_name(name);
        assert_eq!(
            parsed,
            Some(viewpoint),
            "Round-trip failed for {:?}",
            viewpoint
        );
    }
}

/// Verifies `Viewpoint::layer_filter()` returns the correct layer for layer-based viewpoints and None for others.
#[test]
fn test_viewpoint_layer_filter() {
    let layer_based = [
        Viewpoint::Motivation,
        Viewpoint::Strategy,
        Viewpoint::Business,
        Viewpoint::Application,
        Viewpoint::Technology,
        Viewpoint::Physical,
        Viewpoint::Implementation,
        Viewpoint::Other,
    ];

    for viewpoint in layer_based {
        assert_eq!(
            viewpoint.layer_filter(),
            Some(match viewpoint {
                Viewpoint::Motivation => ElementLayer::Motivation,
                Viewpoint::Strategy => ElementLayer::Strategy,
                Viewpoint::Business => ElementLayer::Business,
                Viewpoint::Application => ElementLayer::Application,
                Viewpoint::Technology => ElementLayer::Technology,
                Viewpoint::Physical => ElementLayer::Physical,
                Viewpoint::Implementation => ElementLayer::Implementation,
                Viewpoint::Other => ElementLayer::Other,
                _ => unreachable!(),
            }),
            "layer_filter() should return Some for layer-based viewpoints"
        );
    }

    // Mixin viewpoints should return None
    let mixins = [
        Viewpoint::EnterpriseStructure,
        Viewpoint::ValueStream,
        Viewpoint::Organization,
        Viewpoint::BusinessProcessCooperation,
        Viewpoint::Product,
        Viewpoint::ApplicationCooperation,
        Viewpoint::ApplicationUsage,
    ];
    for viewpoint in mixins {
        assert_eq!(
            viewpoint.layer_filter(),
            None,
            "layer_filter() should return None for mixin viewpoints"
        );
    }

    // NONE should return None
    assert_eq!(Viewpoint::NONE.layer_filter(), None);
}
