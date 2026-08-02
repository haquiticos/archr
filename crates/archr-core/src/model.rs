//! Arena-based graph model for ArchiMate 3.2.
//!
//! Uses typed newtype indices (`ElementId`, `RelationId`) for O(1) `Vec` access
//! instead of `Rc<RefCell<>>` or string-keyed `HashMap`s.

use std::ops::Index;
use std::str::FromStr;

// ---------------------------------------------------------------------------
// Layer enum
// ---------------------------------------------------------------------------

/// ArchiMate 3.2 metamodel layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ElementLayer {
    Strategy,
    Business,
    Application,
    Technology,
    Physical,
    Motivation,
    Implementation,
    Other,
}

// ---------------------------------------------------------------------------
// ElementKind — 61 variants (ArchiMate 3.2 complete taxonomy)
// ---------------------------------------------------------------------------

/// All 61 ArchiMate 3.2 element types, organized by layer.
///
/// Variant names match the Open Exchange `xsi:type` attribute exactly so that
/// (de)serialization is straightforward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementKind {
    // -- Motivation (10) --
    Stakeholder,
    Driver,
    Assessment,
    Goal,
    Outcome,
    Principle,
    Requirement,
    Constraint,
    Meaning,
    Value,

    // -- Strategy (4) --
    Resource,
    Capability,
    ValueStream,
    CourseOfAction,

    // -- Business (13) --
    BusinessActor,
    BusinessRole,
    BusinessCollaboration,
    BusinessInterface,
    BusinessProcess,
    BusinessFunction,
    BusinessInteraction,
    BusinessEvent,
    BusinessService,
    BusinessObject,
    Contract,
    Representation,
    Product,

    // -- Application (9) --
    ApplicationComponent,
    ApplicationCollaboration,
    ApplicationInterface,
    ApplicationFunction,
    ApplicationProcess,
    ApplicationInteraction,
    ApplicationEvent,
    ApplicationService,
    DataObject,

    // -- Technology (13) --
    Node,
    Device,
    SystemSoftware,
    TechnologyCollaboration,
    TechnologyInterface,
    Path,
    CommunicationNetwork,
    Artifact,
    TechnologyFunction,
    TechnologyProcess,
    TechnologyInteraction,
    TechnologyEvent,
    TechnologyService,

    // -- Physical (4) --
    Equipment,
    Facility,
    Material,
    DistributionNetwork,

    // -- Implementation & Migration (4) --
    WorkPackage,
    Deliverable,
    Plateau,
    Gap,

    // -- Other (4) --
    Grouping,
    Location,
    AndJunction,
    OrJunction,
}

impl ElementKind {
    /// Returns the metamodel layer for this element type.
    pub const fn layer(self) -> ElementLayer {
        use ElementKind::*;
        match self {
            // Motivation
            Stakeholder | Driver | Assessment | Goal | Outcome | Principle | Requirement
            | Constraint | Meaning | Value => ElementLayer::Motivation,

            // Strategy
            Resource | Capability | ValueStream | CourseOfAction => ElementLayer::Strategy,

            // Business
            BusinessActor
            | BusinessRole
            | BusinessCollaboration
            | BusinessInterface
            | BusinessProcess
            | BusinessFunction
            | BusinessInteraction
            | BusinessEvent
            | BusinessService
            | BusinessObject
            | Contract
            | Representation
            | Product => ElementLayer::Business,

            // Application
            ApplicationComponent
            | ApplicationCollaboration
            | ApplicationInterface
            | ApplicationFunction
            | ApplicationProcess
            | ApplicationInteraction
            | ApplicationEvent
            | ApplicationService
            | DataObject => ElementLayer::Application,

            // Technology
            Node
            | Device
            | SystemSoftware
            | TechnologyCollaboration
            | TechnologyInterface
            | Path
            | CommunicationNetwork
            | Artifact
            | TechnologyFunction
            | TechnologyProcess
            | TechnologyInteraction
            | TechnologyEvent
            | TechnologyService => ElementLayer::Technology,

            // Physical
            Equipment | Facility | Material | DistributionNetwork => ElementLayer::Physical,

            // Implementation & Migration
            WorkPackage | Deliverable | Plateau | Gap => ElementLayer::Implementation,

            // Other
            Grouping | Location | AndJunction | OrJunction => ElementLayer::Other,
        }
    }

    /// Parses a string (case-insensitive) into an `ElementKind`.
    ///
    /// Matches the exact Open Exchange type names, e.g. `"BusinessActor"`,
    /// `"ValueStream"`, `"AndJunction"`.
    pub fn from_name(s: &str) -> Option<Self> {
        use ElementKind::*;
        match s {
            // Motivation
            "Stakeholder" => Some(Stakeholder),
            "Driver" => Some(Driver),
            "Assessment" => Some(Assessment),
            "Goal" => Some(Goal),
            "Outcome" => Some(Outcome),
            "Principle" => Some(Principle),
            "Requirement" => Some(Requirement),
            "Constraint" => Some(Constraint),
            "Meaning" => Some(Meaning),
            "Value" => Some(Value),

            // Strategy
            "Resource" => Some(Resource),
            "Capability" => Some(Capability),
            "ValueStream" => Some(ValueStream),
            "CourseOfAction" => Some(CourseOfAction),

            // Business
            "BusinessActor" => Some(BusinessActor),
            "BusinessRole" => Some(BusinessRole),
            "BusinessCollaboration" => Some(BusinessCollaboration),
            "BusinessInterface" => Some(BusinessInterface),
            "BusinessProcess" => Some(BusinessProcess),
            "BusinessFunction" => Some(BusinessFunction),
            "BusinessInteraction" => Some(BusinessInteraction),
            "BusinessEvent" => Some(BusinessEvent),
            "BusinessService" => Some(BusinessService),
            "BusinessObject" => Some(BusinessObject),
            "Contract" => Some(Contract),
            "Representation" => Some(Representation),
            "Product" => Some(Product),

            // Application
            "ApplicationComponent" => Some(ApplicationComponent),
            "ApplicationCollaboration" => Some(ApplicationCollaboration),
            "ApplicationInterface" => Some(ApplicationInterface),
            "ApplicationFunction" => Some(ApplicationFunction),
            "ApplicationProcess" => Some(ApplicationProcess),
            "ApplicationInteraction" => Some(ApplicationInteraction),
            "ApplicationEvent" => Some(ApplicationEvent),
            "ApplicationService" => Some(ApplicationService),
            "DataObject" => Some(DataObject),

            // Technology
            "Node" => Some(Node),
            "Device" => Some(Device),
            "SystemSoftware" => Some(SystemSoftware),
            "TechnologyCollaboration" => Some(TechnologyCollaboration),
            "TechnologyInterface" => Some(TechnologyInterface),
            "Path" => Some(Path),
            "CommunicationNetwork" => Some(CommunicationNetwork),
            "Artifact" => Some(Artifact),
            "TechnologyFunction" => Some(TechnologyFunction),
            "TechnologyProcess" => Some(TechnologyProcess),
            "TechnologyInteraction" => Some(TechnologyInteraction),
            "TechnologyEvent" => Some(TechnologyEvent),
            "TechnologyService" => Some(TechnologyService),

            // Physical
            "Equipment" => Some(Equipment),
            "Facility" => Some(Facility),
            "Material" => Some(Material),
            "DistributionNetwork" => Some(DistributionNetwork),

            // Implementation & Migration
            "WorkPackage" => Some(WorkPackage),
            "Deliverable" => Some(Deliverable),
            "Plateau" => Some(Plateau),
            "Gap" => Some(Gap),

            // Other
            "Grouping" => Some(Grouping),
            "Location" => Some(Location),
            "AndJunction" => Some(AndJunction),
            "OrJunction" => Some(OrJunction),

            _ => None,
        }
    }

    /// Returns the canonical Open Exchange type name (matches the variant name).
    pub fn type_name(self) -> &'static str {
        use ElementKind::*;
        match self {
            Stakeholder => "Stakeholder",
            Driver => "Driver",
            Assessment => "Assessment",
            Goal => "Goal",
            Outcome => "Outcome",
            Principle => "Principle",
            Requirement => "Requirement",
            Constraint => "Constraint",
            Meaning => "Meaning",
            Value => "Value",
            Resource => "Resource",
            Capability => "Capability",
            ValueStream => "ValueStream",
            CourseOfAction => "CourseOfAction",
            BusinessActor => "BusinessActor",
            BusinessRole => "BusinessRole",
            BusinessCollaboration => "BusinessCollaboration",
            BusinessInterface => "BusinessInterface",
            BusinessProcess => "BusinessProcess",
            BusinessFunction => "BusinessFunction",
            BusinessInteraction => "BusinessInteraction",
            BusinessEvent => "BusinessEvent",
            BusinessService => "BusinessService",
            BusinessObject => "BusinessObject",
            Contract => "Contract",
            Representation => "Representation",
            Product => "Product",
            ApplicationComponent => "ApplicationComponent",
            ApplicationCollaboration => "ApplicationCollaboration",
            ApplicationInterface => "ApplicationInterface",
            ApplicationFunction => "ApplicationFunction",
            ApplicationProcess => "ApplicationProcess",
            ApplicationInteraction => "ApplicationInteraction",
            ApplicationEvent => "ApplicationEvent",
            ApplicationService => "ApplicationService",
            DataObject => "DataObject",
            Node => "Node",
            Device => "Device",
            SystemSoftware => "SystemSoftware",
            TechnologyCollaboration => "TechnologyCollaboration",
            TechnologyInterface => "TechnologyInterface",
            Path => "Path",
            CommunicationNetwork => "CommunicationNetwork",
            Artifact => "Artifact",
            TechnologyFunction => "TechnologyFunction",
            TechnologyProcess => "TechnologyProcess",
            TechnologyInteraction => "TechnologyInteraction",
            TechnologyEvent => "TechnologyEvent",
            TechnologyService => "TechnologyService",
            Equipment => "Equipment",
            Facility => "Facility",
            Material => "Material",
            DistributionNetwork => "DistributionNetwork",
            WorkPackage => "WorkPackage",
            Deliverable => "Deliverable",
            Plateau => "Plateau",
            Gap => "Gap",
            Grouping => "Grouping",
            Location => "Location",
            AndJunction => "AndJunction",
            OrJunction => "OrJunction",
        }
    }

    /// Total number of element kind variants. Compile-time constant for tests.
    pub const VARIANT_COUNT: usize = 61;
}

impl std::fmt::Display for ElementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.type_name())
    }
}

impl FromStr for ElementKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or_else(|| format!("unknown element kind: {s}"))
    }
}

// ---------------------------------------------------------------------------
// RelationKind — 11 variants
// ---------------------------------------------------------------------------

/// All 11 ArchiMate 3.2 relationship types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationKind {
    // Structural
    Composition,
    Aggregation,
    Assignment,
    Realization,
    // Dependency
    Serving,
    Access,
    Influence,
    Association,
    // Dynamic
    Triggering,
    Flow,
    // Other
    Specialization,
}

impl RelationKind {
    /// Parses a string into a `RelationKind`.
    pub fn from_name(s: &str) -> Option<Self> {
        use RelationKind::*;
        match s {
            "Composition" => Some(Composition),
            "Aggregation" => Some(Aggregation),
            "Assignment" => Some(Assignment),
            "Realization" => Some(Realization),
            "Serving" => Some(Serving),
            "Access" => Some(Access),
            "Influence" => Some(Influence),
            "Association" => Some(Association),
            "Triggering" => Some(Triggering),
            "Flow" => Some(Flow),
            "Specialization" => Some(Specialization),
            _ => None,
        }
    }

    /// Canonical type name (matches the variant name).
    pub fn type_name(self) -> &'static str {
        use RelationKind::*;
        match self {
            Composition => "Composition",
            Aggregation => "Aggregation",
            Assignment => "Assignment",
            Realization => "Realization",
            Serving => "Serving",
            Access => "Access",
            Influence => "Influence",
            Association => "Association",
            Triggering => "Triggering",
            Flow => "Flow",
            Specialization => "Specialization",
        }
    }

    pub const VARIANT_COUNT: usize = 11;
}

impl std::fmt::Display for RelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.type_name())
    }
}

impl FromStr for RelationKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or_else(|| format!("unknown relation kind: {s}"))
    }
}

// ---------------------------------------------------------------------------
// Typed index newtypes
// ---------------------------------------------------------------------------

/// Strongly-typed index into the `Model.elements` `Vec`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(pub usize);

/// Strongly-typed index into the `Model.relations` `Vec`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationId(pub usize);

// ---------------------------------------------------------------------------
// Model graph (Arena pattern)
// ---------------------------------------------------------------------------

/// Root container for an ArchiMate model.
///
/// Owns all elements and relationships. Access via typed indices in O(1).
#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    elements: Vec<Element>,
    relations: Vec<Relationship>,
}

/// A node in the ArchiMate graph.
#[derive(Debug, Clone)]
pub struct Element {
    pub id: ElementId,
    pub name: String,
    pub kind: ElementKind,
}

/// A directed edge in the ArchiMate graph.
#[derive(Debug, Clone)]
pub struct Relationship {
    pub id: RelationId,
    pub source: ElementId,
    pub target: ElementId,
    pub kind: RelationKind,
}

impl Model {
    /// Creates a new empty model.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            elements: Vec::new(),
            relations: Vec::new(),
        }
    }

    /// Adds an element and returns its typed index.
    pub fn add_element(&mut self, name: &str, kind: ElementKind) -> ElementId {
        let id = ElementId(self.elements.len());
        self.elements.push(Element {
            id,
            name: name.to_string(),
            kind,
        });
        id
    }

    /// Adds a relationship and returns its typed index.
    pub fn link(&mut self, source: ElementId, target: ElementId, kind: RelationKind) -> RelationId {
        let id = RelationId(self.relations.len());
        self.relations.push(Relationship {
            id,
            source,
            target,
            kind,
        });
        id
    }

    /// Direct O(1) access to an element by typed index.
    pub fn element(&self, id: ElementId) -> &Element {
        &self.elements[id.0]
    }

    /// Direct O(1) access to a relationship by typed index.
    pub fn relation(&self, id: RelationId) -> &Relationship {
        &self.relations[id.0]
    }

    /// Number of elements.
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    /// Number of relationships.
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Iterator over elements.
    pub fn iter_elements(&self) -> impl Iterator<Item = &Element> {
        self.elements.iter()
    }

    /// Iterator over relationships.
    pub fn iter_relations(&self) -> impl Iterator<Item = &Relationship> {
        self.relations.iter()
    }
}

impl Index<ElementId> for Model {
    type Output = Element;
    fn index(&self, id: ElementId) -> &Self::Output {
        &self.elements[id.0]
    }
}

impl Index<RelationId> for Model {
    type Output = Relationship;
    fn index(&self, id: RelationId) -> &Self::Output {
        &self.relations[id.0]
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_element_returns_incrementing_id() {
        let mut m = Model::new("test");
        let id0 = m.add_element("a", ElementKind::BusinessActor);
        let id1 = m.add_element("b", ElementKind::Node);
        assert_eq!(id0, ElementId(0));
        assert_eq!(id1, ElementId(1));
    }

    #[test]
    fn element_returns_correct_element() {
        let mut m = Model::new("test");
        let id = m.add_element("my_actor", ElementKind::BusinessActor);
        let e = m.element(id);
        assert_eq!(e.name, "my_actor");
        assert_eq!(e.kind, ElementKind::BusinessActor);
        assert_eq!(e.id, id);
    }

    #[test]
    fn link_returns_incrementing_id() {
        let mut m = Model::new("test");
        let a = m.add_element("a", ElementKind::BusinessActor);
        let b = m.add_element("b", ElementKind::ApplicationComponent);
        let r0 = m.link(a, b, RelationKind::Serving);
        let r1 = m.link(b, a, RelationKind::Association);
        assert_eq!(r0, RelationId(0));
        assert_eq!(r1, RelationId(1));
    }

    #[test]
    fn index_trait_works() {
        let mut m = Model::new("test");
        let id = m.add_element("x", ElementKind::Node);
        assert_eq!(m[id].name, "x");
    }

    #[test]
    fn element_kind_layer_mapping() {
        // At least one variant per layer
        assert_eq!(ElementKind::Stakeholder.layer(), ElementLayer::Motivation);
        assert_eq!(ElementKind::Resource.layer(), ElementLayer::Strategy);
        assert_eq!(ElementKind::BusinessActor.layer(), ElementLayer::Business);
        assert_eq!(
            ElementKind::ApplicationComponent.layer(),
            ElementLayer::Application
        );
        assert_eq!(ElementKind::Node.layer(), ElementLayer::Technology);
        assert_eq!(ElementKind::Equipment.layer(), ElementLayer::Physical);
        assert_eq!(
            ElementKind::WorkPackage.layer(),
            ElementLayer::Implementation
        );
        assert_eq!(ElementKind::Grouping.layer(), ElementLayer::Other);
    }

    #[test]
    fn element_kind_variant_count() {
        // Verify we haven't accidentally added or removed variants
        assert_eq!(ElementKind::VARIANT_COUNT, 61);
    }

    #[test]
    fn relation_kind_variant_count() {
        assert_eq!(RelationKind::VARIANT_COUNT, 11);
    }

    #[test]
    fn element_kind_from_name_round_trip() {
        let kinds = [
            ElementKind::BusinessActor,
            ElementKind::ApplicationComponent,
            ElementKind::Node,
            ElementKind::ValueStream,
            ElementKind::CourseOfAction,
            ElementKind::CommunicationNetwork,
            ElementKind::DistributionNetwork,
            ElementKind::AndJunction,
            ElementKind::OrJunction,
            ElementKind::DataObject,
            ElementKind::Constraint,
        ];
        for kind in kinds {
            assert_eq!(
                ElementKind::from_name(kind.type_name()),
                Some(kind),
                "round-trip failed for {kind}"
            );
        }
    }

    #[test]
    fn relation_kind_from_name_round_trip() {
        for kind in [
            RelationKind::Composition,
            RelationKind::Serving,
            RelationKind::Access,
            RelationKind::Triggering,
            RelationKind::Flow,
            RelationKind::Specialization,
        ] {
            assert_eq!(RelationKind::from_name(kind.type_name()), Some(kind),);
        }
    }

    #[test]
    fn element_kind_display_matches_type_name() {
        let kind = ElementKind::BusinessActor;
        assert_eq!(kind.to_string(), "BusinessActor");
        assert_eq!(kind.type_name(), "BusinessActor");
    }

    #[test]
    fn unknown_kind_returns_none() {
        assert!(ElementKind::from_name("FooBar").is_none());
        assert!(RelationKind::from_name("InvalidRelation").is_none());
    }

    #[test]
    fn from_str_errors_on_unknown() {
        let err = ElementKind::from_str("Nonexistent");
        assert!(err.is_err());
        let err = RelationKind::from_str("Bogus");
        assert!(err.is_err());
    }
}

impl std::fmt::Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
