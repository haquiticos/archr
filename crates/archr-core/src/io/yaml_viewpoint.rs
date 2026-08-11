//! Viewpoint helper functions for YAML I/O
//!
//! Maps viewpoint names to their allowed ElementKind sets based on Archi's viewpoints.xml

use crate::model::ElementKind;

/// Returns the set of ElementKind allowed by a viewpoint.
///
/// Returns None for viewpoints that cannot be parsed or for Layered (which allows all).
pub fn allowed_elements_for_viewpoint(viewpoint_name: &str) -> Option<Vec<ElementKind>> {
    use ElementKind::*;
    
    // Base layer-based viewpoints
    let layers = match viewpoint_name {
        "Motivation" => vec![Stakeholder, Driver, Assessment, Goal, Outcome, Principle, 
                            Requirement, Constraint, Meaning, Value],
        "Strategy" => vec![Resource, Capability, ValueStream, CourseOfAction],
        "Business" => vec![BusinessActor, BusinessCollaboration, BusinessInterface, 
                          BusinessRole, BusinessProcess, BusinessFunction, BusinessInteraction, 
                          BusinessEvent, BusinessService, BusinessObject, Contract, Representation],
        "Application" => vec![ApplicationComponent, ApplicationCollaboration, ApplicationInterface,
                              ApplicationFunction, ApplicationProcess, ApplicationInteraction,
                              ApplicationEvent, ApplicationService, DataObject],
        "Technology" => vec![Node, Device, SystemSoftware, TechnologyCollaboration, TechnologyInterface,
                             Path, CommunicationNetwork, Artifact, TechnologyFunction, TechnologyProcess,
                             TechnologyInteraction, TechnologyEvent, TechnologyService],
        "Physical" => vec![Node, Device, SystemSoftware, TechnologyCollaboration, TechnologyInterface,
                           Path, CommunicationNetwork, Artifact, TechnologyFunction, TechnologyProcess,
                           TechnologyInteraction, TechnologyEvent, TechnologyService, Equipment,
                           Facility, Material, DistributionNetwork],
        "Implementation" => vec![ApplicationComponent, ApplicationCollaboration, ApplicationInterface,
                                 ApplicationFunction, ApplicationProcess, ApplicationInteraction,
                                 ApplicationEvent, ApplicationService, DataObject, Artifact, Path,
                                 SystemSoftware, TechnologyFunction, TechnologyInteraction,
                                 TechnologyInterface, TechnologyProcess, TechnologyService],
        "Other" => vec![Grouping, Location, AndJunction, OrJunction],
        _ => return None, // Unknown viewpoint
    };
    
    Some(layers)
}
