//! Seam tests for viewpoint projection and the query engine.

use archr_core::validate::validate_model;
use archr_core::{
    dependencies, impact, model_to_yaml, orphans, parse_yaml, path, project, project_view,
    ProjectError, QueryError, RefId, Viewpoint, ViewpointDefinition, ViewpointKind,
};
use archr_core::{ElementKind, Model, RelationKind};

/// goal -Association→ actor? No: actor -Serving← app; chain: e1→e2→e3 with
/// Association (any layer), plus an isolated element.
fn chain_model() -> Model {
    let mut m = Model::new("chain");
    m.add_element_with_id("a", "A", ElementKind::BusinessActor);
    m.add_element_with_id("b", "B", ElementKind::ApplicationComponent);
    m.add_element_with_id("c", "C", ElementKind::Node);
    m.add_element_with_id("iso", "Isolated", ElementKind::Goal);
    let (a, b, c) = (
        m.resolve_element(&RefId::new("a")).unwrap(),
        m.resolve_element(&RefId::new("b")).unwrap(),
        m.resolve_element(&RefId::new("c")).unwrap(),
    );
    m.link_with_id("r1", a, b, RelationKind::Association);
    m.link_with_id("r2", b, c, RelationKind::Association);
    m
}

fn ids(v: Vec<RefId>) -> Vec<String> {
    v.iter().map(|r| r.0.clone()).collect()
}

// ---------------------------------------------------------------------------
// projection
// ---------------------------------------------------------------------------

#[test]
fn project_filters_to_the_viewpoint_layer() {
    let m = chain_model();
    let p = project(&m, Viewpoint::Business).unwrap();
    // Only the Business actor survives; both its relations point outside.
    assert_eq!(p.element_count(), 1);
    assert_eq!(p.relation_count(), 0);
    assert_eq!(p.name, "chain");
    let a = p.resolve_element(&RefId::new("a")).unwrap();
    assert_eq!(p[a].kind, ElementKind::BusinessActor);
    assert!(validate_model(&p).success);
    // And it is a real model: round-trips.
    assert_eq!(parse_yaml(&model_to_yaml(&p)).unwrap().element_count(), 1);
}

#[test]
fn project_layered_keeps_everything() {
    let m = chain_model();
    let p = project(&m, Viewpoint::NONE).unwrap();
    assert_eq!(p.element_count(), 4);
    assert_eq!(p.relation_count(), 2);
}

#[test]
fn project_refuses_undefined_viewpoints() {
    let m = chain_model();
    assert_eq!(
        project(&m, Viewpoint::EnterpriseStructure),
        Err(ProjectError::UnsupportedViewpoint(Viewpoint::EnterpriseStructure))
    );
}

#[test]
fn project_view_follows_the_saved_element_list() {
    let mut m = chain_model();
    m.set_viewpoints(vec![ViewpointDefinition {
        id: "vp1".into(),
        name: "App view".into(),
        kind: ViewpointKind::Application,
        elements: vec!["b".into(), "c".into(), "ghost".into()],
        relationships: vec!["r2".into(), "r1".into()],
    }]);
    let p = project_view(&m, &RefId::new("vp1")).unwrap();
    // Ghost element never existed; r1's endpoints (a) are out of scope, so it drops.
    assert_eq!(p.element_count(), 2);
    assert_eq!(p.relation_count(), 1);
    assert!(validate_model(&p).success);
    assert!(project_view(&m, &RefId::new("nope")).is_none());
}

// ---------------------------------------------------------------------------
// queries
// ---------------------------------------------------------------------------

#[test]
fn impact_follows_outgoing_and_deps_incoming() {
    let m = chain_model();
    // Edges point provider → consumer: a → b → c.
    assert_eq!(ids(impact(&m, &RefId::new("a")).unwrap()), vec!["b", "c"]);
    assert_eq!(ids(impact(&m, &RefId::new("c")).unwrap()), Vec::<String>::new());
    assert_eq!(
        ids(dependencies(&m, &RefId::new("c")).unwrap()),
        vec!["a", "b"]
    );
    assert_eq!(
        impact(&m, &RefId::new("ghost")),
        Err(QueryError::UnknownElement(RefId::new("ghost")))
    );
}

#[test]
fn path_finds_shortest_and_reports_unreachable() {
    let m = chain_model();
    let (a, c) = (RefId::new("a"), RefId::new("c"));
    assert_eq!(
        path(&m, &a, &c).unwrap().map(ids),
        Some(vec!["a".into(), "b".into(), "c".into()])
    );
    // No directed edge backwards.
    assert_eq!(path(&m, &c, &a).unwrap(), None);
    assert_eq!(path(&m, &a, &a).unwrap().map(ids), Some(vec!["a".into()]));
    assert_eq!(
        path(&m, &a, &RefId::new("ghost")),
        Err(QueryError::UnknownElement(RefId::new("ghost")))
    );
}

#[test]
fn orphans_lists_disconnected_elements() {
    let m = chain_model();
    assert_eq!(ids(orphans(&m)), vec!["iso"]);
    // Cycles must not loop forever: close the chain.
    let mut m = m;
    let (a, c) = (
        m.resolve_element(&RefId::new("a")).unwrap(),
        m.resolve_element(&RefId::new("c")).unwrap(),
    );
    m.link_with_id("r3", c, a, RelationKind::Association);
    assert_eq!(ids(impact(&m, &RefId::new("a")).unwrap()), vec!["b", "c"]);
    assert_eq!(ids(path(&m, &RefId::new("c"), &RefId::new("a")).unwrap().unwrap()), vec!["c", "a"]);
}
