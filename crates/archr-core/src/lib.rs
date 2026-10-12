pub mod model;
pub mod io {
    pub mod xml;
    pub mod yaml;
}
pub mod diff;
pub mod command;
pub mod layout;
pub mod query;
#[cfg(test)]
mod test_ecore;
pub mod validate;
pub mod viewpoint;

// Re-export the mutation seam (commands + connection validation)
pub use command::{apply, can_connect, ApplyError, Command, ConnError};
// Re-export the query engine
pub use query::{dependencies, impact, orphans, path, QueryError};
// Re-export viewpoint projection
pub use viewpoint::{project, project_view, ProjectError};
// Re-export key I/O functions for tests
pub use io::xml::{model_to_xml, xml_to_model};
pub use io::yaml::{model_to_yaml, parse_yaml};
pub use model::Model;
pub use model::RefId;
// Re-export core enums
pub use model::ElementKind;
pub use model::ElementLayer;
pub use model::RelationKind;
pub use model::Viewpoint;
pub use model::{ViewpointDefinition, ViewpointKind};
// Re-export core types
pub use model::ElementId;
pub use model::RelationId;
