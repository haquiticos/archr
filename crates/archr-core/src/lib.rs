pub mod model;
pub mod io {
    pub mod xml;
    pub mod yaml;
}
pub mod diff;
pub mod layout;
#[cfg(test)]
mod test_ecore;
pub mod validate;

// Re-export key I/O functions for tests
pub use io::xml::{model_to_xml, xml_to_model, xml_to_model_preserving_ids};
pub use io::yaml::{model_to_yaml, parse_yaml, parse_yaml_with_ids};
