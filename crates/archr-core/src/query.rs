//! Query engine: graph questions over the model, headless.
//!
//! Edge semantics: a relationship points from provider to consumer
//! (`Serving`, `Realization`, …). `impact` follows edges *out* (what breaks if
//! the element changes); `dependencies` follows edges *in* (what the element
//! relies on). All results are sorted by id for stable output.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::model::{ElementId, Model, RefId};

/// Why a query could not run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    /// The id resolves to no element in the model.
    UnknownElement(RefId),
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::UnknownElement(id) => write!(f, "unknown element `{id}`"),
        }
    }
}

impl std::error::Error for QueryError {}

/// Adjacency lists keyed by arena index.
fn adjacencies(model: &Model) -> (HashMap<ElementId, Vec<ElementId>>, HashMap<ElementId, Vec<ElementId>>) {
    let mut incoming: HashMap<ElementId, Vec<ElementId>> = HashMap::new();
    let mut outgoing: HashMap<ElementId, Vec<ElementId>> = HashMap::new();
    for r in model.iter_relations() {
        outgoing.entry(r.source).or_default().push(r.target);
        incoming.entry(r.target).or_default().push(r.source);
    }
    (incoming, outgoing)
}

/// Transitive closure from `start` over `adj`, excluding `start` itself,
/// sorted by id.
fn reachable(model: &Model, start: ElementId, adj: &HashMap<ElementId, Vec<ElementId>>) -> Vec<RefId> {
    let mut visited: HashSet<ElementId> = HashSet::from([start]);
    let mut queue: VecDeque<ElementId> = VecDeque::from([start]);
    let mut ids: Vec<RefId> = Vec::new();
    while let Some(cur) = queue.pop_front() {
        for &next in adj.get(&cur).into_iter().flatten() {
            if visited.insert(next) {
                ids.push(RefId::new(&model[next].original_id));
                queue.push_back(next);
            }
        }
    }
    ids.sort();
    ids
}

/// Elements transitively affected if `id` changes or is removed (transitive
/// closure over outgoing edges).
pub fn impact(model: &Model, id: &RefId) -> Result<Vec<RefId>, QueryError> {
    let start = model
        .resolve_element(id)
        .ok_or_else(|| QueryError::UnknownElement(id.clone()))?;
    let (_, outgoing) = adjacencies(model);
    Ok(reachable(model, start, &outgoing))
}

/// Elements `id` transitively relies on (transitive closure over incoming
/// edges).
pub fn dependencies(model: &Model, id: &RefId) -> Result<Vec<RefId>, QueryError> {
    let start = model
        .resolve_element(id)
        .ok_or_else(|| QueryError::UnknownElement(id.clone()))?;
    let (incoming, _) = adjacencies(model);
    Ok(reachable(model, start, &incoming))
}

/// Shortest directed path from `from` to `to` (BFS), inclusive of both
/// endpoints; `None` when unreachable. `path(m, x, x)` is `Some([x])`.
pub fn path(model: &Model, from: &RefId, to: &RefId) -> Result<Option<Vec<RefId>>, QueryError> {
    let src = model
        .resolve_element(from)
        .ok_or_else(|| QueryError::UnknownElement(from.clone()))?;
    let dst = model
        .resolve_element(to)
        .ok_or_else(|| QueryError::UnknownElement(to.clone()))?;
    let (_, outgoing) = adjacencies(model);

    let mut prev: HashMap<ElementId, ElementId> = HashMap::new();
    let mut visited: HashSet<ElementId> = HashSet::from([src]);
    let mut queue: VecDeque<ElementId> = VecDeque::from([src]);
    while let Some(cur) = queue.pop_front() {
        if cur == dst {
            break;
        }
        for &next in outgoing.get(&cur).into_iter().flatten() {
            if visited.insert(next) {
                prev.insert(next, cur);
                queue.push_back(next);
            }
        }
    }
    if !visited.contains(&dst) {
        return Ok(None);
    }
    let mut step = dst;
    let mut ids = vec![RefId::new(&model[dst].original_id)];
    while let Some(&p) = prev.get(&step) {
        ids.push(RefId::new(&model[p].original_id));
        step = p;
    }
    ids.reverse();
    Ok(Some(ids))
}

/// Elements without any relationship.
pub fn orphans(model: &Model) -> Vec<RefId> {
    let mut connected: HashSet<ElementId> = HashSet::new();
    for r in model.iter_relations() {
        connected.insert(r.source);
        connected.insert(r.target);
    }
    let mut ids: Vec<RefId> = model
        .iter_elements()
        .filter(|e| !connected.contains(&e.id))
        .map(|e| RefId::new(&e.original_id))
        .collect();
    ids.sort();
    ids
}
