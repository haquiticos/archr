//! Viewpoint projection: a valid subgraph for a metamodel viewpoint or a
//! saved diagram view.
//!
//! Projection keeps only the elements the scope allows and the relationships
//! whose both endpoints survive, so the result is always a valid [`Model`]
//! (passes [`crate::validate`], round-trips through YAML/XML).

use std::collections::{HashMap, HashSet};

use crate::model::{ElementId, ElementKind, ElementLayer, Model, RefId, Relationship, Viewpoint};

/// Why a model could not be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectError {
    /// The viewpoint's exact element composition is not defined in this
    /// codebase, so projecting would silently filter wrong. Refused instead.
    UnsupportedViewpoint(Viewpoint),
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::UnsupportedViewpoint(vp) => {
                write!(f, "viewpoint `{vp}` has no exact element filter defined")
            }
        }
    }
}

impl std::error::Error for ProjectError {}

/// The element-kind predicate for a viewpoint, or `None` when its exact
/// composition is not defined here.
///
/// Exact where defined: the 8 layer viewpoints via
/// [`Viewpoint::layer_filter`], `Layered` (everything), and the three mixins
/// documented in [`Viewpoint`]'s docs (Organization = Business + Location,
/// Product = Business + Application + Artifact + TechnologyService + Value,
/// ValueStream = Strategy + Outcome).
// ponytail: EnterpriseStructure, BusinessProcessCooperation, ApplicationCooperation
// and ApplicationUsage need their exact $-collections from Archi's
// viewpoints.xml; refusing beats shipping a wrong filter. Add their arms here
// once the lists are verified.
fn kind_filter(vp: Viewpoint) -> Option<Box<dyn Fn(ElementKind) -> bool>> {
    use ElementKind as K;
    use ElementLayer as L;
    use Viewpoint as V;
    if let Some(layer) = vp.layer_filter() {
        return Some(Box::new(move |kind| kind.layer() == layer));
    }
    match vp {
        V::NONE => Some(Box::new(|_| true)),
        V::Organization => Some(Box::new(|kind| {
            kind == K::Location || kind.layer() == L::Business
        })),
        V::Product => Some(Box::new(|kind| {
            matches!(kind.layer(), L::Business | L::Application)
                || matches!(kind, K::Artifact | K::TechnologyService | K::Value)
        })),
        V::ValueStream => Some(Box::new(|kind| {
            kind == K::Outcome || kind.layer() == L::Strategy
        })),
        // The remaining arms (EnterpriseStructure, BusinessProcessCooperation,
        // ApplicationCooperation, ApplicationUsage and the layer viewpoints,
        // which never reach here) have no exact filter defined.
        _ => None,
    }
}

impl std::fmt::Display for Viewpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_yaml_viewpoint_name())
    }
}

/// Projects the model onto a metamodel viewpoint: only elements the viewpoint
/// allows survive, and only relationships whose both endpoints survive. The
/// model name is preserved; viewpoint definitions are dropped (a projection
/// of a projection is not defined).
pub fn project(model: &Model, vp: Viewpoint) -> Result<Model, ProjectError> {
    let keep_kind = kind_filter(vp).ok_or(ProjectError::UnsupportedViewpoint(vp))?;
    let kept: HashSet<ElementId> = model
        .iter_elements()
        .filter(|e| keep_kind(e.kind))
        .map(|e| e.id)
        .collect();
    Ok(subgraph(model, &kept, |_| true))
}

/// Projects onto a saved diagram view: its explicit element list plus the
/// listed relationships whose endpoints both survive. Returns `None` when no
/// view carries that id.
pub fn project_view(model: &Model, view: &RefId) -> Option<Model> {
    let def = model
        .viewpoints()
        .iter()
        .find(|d| RefId::new(&d.id) == *view)?;
    let kept: HashSet<ElementId> = model
        .iter_elements()
        .filter(|e| def.elements.iter().any(|id| id == &e.original_id))
        .map(|e| e.id)
        .collect();
    let listed: HashSet<&str> = def.relationships.iter().map(|s| s.as_str()).collect();
    Some(subgraph(model, &kept, |r| listed.contains(r.original_id.as_str())))
}

/// Builds a new model from kept elements and a predicate over relationships.
/// Original ids are carried over verbatim; arena indices are fresh.
fn subgraph(model: &Model, kept: &HashSet<ElementId>, keep_rel: impl Fn(&Relationship) -> bool) -> Model {
    let mut out = Model::new(&model.name);
    let mut remap: HashMap<ElementId, ElementId> = HashMap::new();
    for e in model.iter_elements() {
        if kept.contains(&e.id) {
            let new = out.add_element_with_id(&e.original_id, &e.name, e.kind);
            remap.insert(e.id, new);
        }
    }
    for r in model.iter_relations() {
        if !keep_rel(r) {
            continue;
        }
        if let (Some(&s), Some(&t)) = (remap.get(&r.source), remap.get(&r.target)) {
            out.link_with_id(&r.original_id, s, t, r.kind);
        }
    }
    out
}
