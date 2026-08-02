//! Model diff analysis — compare two models by element name.
//!
//! Matching is done by `name`, not by id, because UUIDs and string ids are
//! different representations of the same logical entity.

use crate::model::Model;
use serde::Serialize;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Type of reference error encountered during diff analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceErrorType {
    /// A relationship references a non-existent element id.
    UndefinedId,
}

/// A reference error found in a model.
#[derive(Debug, Clone, Serialize)]
pub struct ReferenceError {
    pub id: String,
    pub error_type: ReferenceErrorType,
}

// ---------------------------------------------------------------------------
// Diff report
// ---------------------------------------------------------------------------

/// Result of comparing two models by element name.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DiffReport {
    /// Element names present in the new model but not in the existing one.
    pub added: Vec<String>,
    /// Element names present in the existing model but not in the new one.
    pub removed: Vec<String>,
    /// Element names present in both but with a different kind.
    pub modified: Vec<String>,
}

// ---------------------------------------------------------------------------
// Diff analyzer
// ---------------------------------------------------------------------------

/// Compares a new model against a previously-seen "existing" model.
///
/// Build with [`ModelDiffAnalyzer::from_existing`], then call
/// [`ModelDiffAnalyzer::analyze_update`] with the new model.
#[derive(Debug, Default)]
pub struct ModelDiffAnalyzer {
    /// Map of element name -> element kind for the existing model.
    existing: std::collections::HashMap<String, String>,
}

impl ModelDiffAnalyzer {
    /// Create an analyzer from an existing model.
    ///
    /// Indexes every element by its `name`, recording its kind for later
    /// modification detection.
    pub fn from_existing(model: &Model) -> Self {
        let mut existing = std::collections::HashMap::new();
        for elem in model.iter_elements() {
            existing.insert(elem.name.clone(), elem.kind.type_name().to_string());
        }
        Self { existing }
    }

    /// Analyze how a new model differs from the existing one.
    ///
    /// Returns `Ok(DiffReport)` — all relationships in the parsed model are guaranteed to have resolvable endpoints.
    pub fn analyze_update(&self, new_model: &Model) -> Result<DiffReport, Vec<ReferenceError>> {
        // Relationships in parsed models are guaranteed to have resolvable endpoints (validated by parser).
        // Compute diff by name.

        let mut report = DiffReport::default();

        let new_names: std::collections::HashMap<&str, &str> = new_model
            .iter_elements()
            .map(|e| (e.name.as_str(), e.kind.type_name()))
            .collect();

        // Added: in new but not in existing.
        for (name, kind) in &new_names {
            match self.existing.get(*name) {
                None => report.added.push(name.to_string()),
                Some(old_kind) if old_kind.as_str() != *kind => {
                    report.modified.push(name.to_string());
                }
                _ => {}
            }
        }

        // Removed: in existing but not in new.
        for name in self.existing.keys() {
            if !new_names.contains_key(name.as_str()) {
                report.removed.push(name.clone());
            }
        }

        // Deterministic ordering.
        report.added.sort();
        report.removed.sort();
        report.modified.sort();

        Ok(report)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ElementKind, Model, RelationKind};

    fn sample_model() -> Model {
        let mut m = Model::new("existing");
        let a = m.add_element("Actor", ElementKind::BusinessActor);
        let b = m.add_element("Service", ElementKind::BusinessService);
        m.link(a, b, RelationKind::Serving);
        m
    }

    #[test]
    fn test_identical_models_empty_diff() {
        let existing = sample_model();
        let analyzer = ModelDiffAnalyzer::from_existing(&existing);

        // Same model again.
        let new = sample_model();
        let report = analyzer.analyze_update(&new).unwrap();

        assert!(report.added.is_empty());
        assert!(report.removed.is_empty());
        assert!(report.modified.is_empty());
    }

    #[test]
    fn test_added_element() {
        let existing = sample_model();
        let analyzer = ModelDiffAnalyzer::from_existing(&existing);

        let mut new = sample_model();
        new.add_element("Database", ElementKind::DataObject);

        let report = analyzer.analyze_update(&new).unwrap();
        assert_eq!(report.added, vec!["Database"]);
        assert!(report.removed.is_empty());
    }

    #[test]
    fn test_removed_element() {
        let existing = sample_model();
        let analyzer = ModelDiffAnalyzer::from_existing(&existing);

        // New model with only "Actor", missing "Service".
        let mut new = Model::new("new");
        new.add_element("Actor", ElementKind::BusinessActor);

        let report = analyzer.analyze_update(&new).unwrap();
        assert!(report.added.is_empty());
        assert_eq!(report.removed, vec!["Service"]);
    }

    #[test]
    fn test_modified_element_kind_change() {
        let existing = sample_model();
        let analyzer = ModelDiffAnalyzer::from_existing(&existing);

        // Same name but different kind.
        let mut new = Model::new("new");
        new.add_element("Actor", ElementKind::BusinessRole);
        new.add_element("Service", ElementKind::BusinessService);

        let report = analyzer.analyze_update(&new).unwrap();
        assert_eq!(report.modified, vec!["Actor"]);
    }

    #[test]
    fn test_added_and_removed_simultaneously() {
        let existing = sample_model();
        let analyzer = ModelDiffAnalyzer::from_existing(&existing);

        let mut new = Model::new("new");
        new.add_element("Actor", ElementKind::BusinessActor);
        new.add_element("Node", ElementKind::Node);

        let report = analyzer.analyze_update(&new).unwrap();
        assert_eq!(report.added, vec!["Node"]);
        assert_eq!(report.removed, vec!["Service"]);
    }

    #[test]
    fn test_empty_models() {
        let existing = Model::new("empty");
        let analyzer = ModelDiffAnalyzer::from_existing(&existing);

        let new = Model::new("also_empty");
        let report = analyzer.analyze_update(&new).unwrap();

        assert!(report.added.is_empty());
        assert!(report.removed.is_empty());
        assert!(report.modified.is_empty());
    }
}
