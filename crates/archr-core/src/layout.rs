//! Automatic grid layout by topological layer.

use crate::model::{ElementId, Model, RelationKind};
use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
// ---------------------------------------------------------------------------
// Constants
const ROW_HEIGHT: f64 = 120.0;
const COL_WIDTH: f64 = 200.0;
const COMPONENT_OFFSET: f64 = 600.0;
// Error types
#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("layout calculation failed: {0}")]
    Calculation(String),
}
// Layout resolver
/// Computes (x, y) positions for all elements in a model.
#[derive(Debug, Default)]
pub struct LayoutResolver {
    positions: HashMap<ElementId, (f64, f64, f64, f64)>,
impl LayoutResolver {
    /// Calculate positions for all elements in the model.
    pub fn calculate_layout(&mut self, model: &Model) -> Result<(), LayoutError> {
        if model.element_count() == 0 {
            self.positions.clear();
            return Ok(());
        }
        // Build undirected graph from elements and relationships
        let mut graph = UnGraph::<ElementId, ()>::new_undirected();
        // Map from ElementId to NodeIndex in the graph
        let mut id_to_index: HashMap<ElementId, NodeIndex> = HashMap::new();
        // Add nodes for all elements
        for element in model.iter_elements() {
            let node = graph.add_node(element.id);
            id_to_index.insert(element.id, node);
        // Add edges for all relationships (undirected)
        for relation in model.iter_relations() {
            let source_node = id_to_index.get(&relation.source).ok_or_else(|| {
                LayoutError::Calculation(format!("Source element not found: {:?}", relation.source))
            })?;
            let target_node = id_to_index.get(&relation.target).ok_or_else(|| {
                LayoutError::Calculation(format!("Target element not found: {:?}", relation.target))
            // Only add edge if both nodes exist
            graph.add_edge(*source_node, *target_node, ());
        // Find connected components by BFS from each unvisited node
        let mut visited = HashSet::new();
        let mut components: Vec<Vec<NodeIndex>> = Vec::new();
        for node in graph.node_indices() {
            if !visited.contains(&node) {
                // BFS to find this component
                let mut queue = Vec::new();
                let mut component = Vec::new();
                queue.push(node);
                while let Some(current) = queue.pop() {
                    if visited.contains(&current) {
                        continue;
                    }
                    visited.insert(current);
                    component.push(current);
                    // Add unvisited neighbors to queue
                    for neighbor in graph.neighbors(current) {
                        if !visited.contains(&neighbor) {
                            queue.push(neighbor);
                        }
                }
                components.push(component);
            }
        // Process each component separately
        let mut component_index = 0;
        for component_nodes in components.iter() {
            // Create a directed version of the component
            let mut directed = DiGraph::<ElementId, ()>::new();
            // Map from original NodeIndex to new node in directed subgraph
            let mut sub_node_index: HashMap<NodeIndex, NodeIndex> = HashMap::new();
            for orig_idx in component_nodes {
                // Get ElementId from undirected graph
                let elem_id = graph[*orig_idx];
                let new_idx = directed.add_node(elem_id);
                sub_node_index.insert(*orig_idx, new_idx);
            // Add edges to directed subgraph
            for edge in graph.edge_indices() {
                if let Some((u, v)) = graph.edge_endpoints(edge) {
                    // Only add edges within this component
                    if component_nodes.contains(&u) && component_nodes.contains(&v) {
                        directed.add_edge(sub_node_index[&u], sub_node_index[&v], ());
            // Try topological sort; if cyclic, use BFS depth assignment
            let depths: HashMap<ElementId, usize> = match algo::toposort(&directed, None) {
                Ok(order) => {
                    // Graph is acyclic; assign depth based on longest path
                    let mut depth = HashMap::new();
                    let mut visited_order: HashSet<NodeIndex> = HashSet::new();
                    for node in order.into_iter() {
                        if visited_order.contains(&node) {
                            continue;
                        visited_order.insert(node);
                        let elem_id = directed[node];
                        // Find parents (incoming edges) and get their depth + 1
                        let parent_depth = directed
                            .neighbors_directed(node, petgraph::Incoming)
                            .filter_map(|neighbor| depth.get(&directed[neighbor]))
                            .map(|&d| d + 1)
                            .max()
                            .unwrap_or(0);
                        depth.insert(elem_id, parent_depth);
                    depth
                Err(_) => {
                    // Cyclic graph; use BFS from all roots
                    let mut depths = HashMap::new();
                    let mut queue: Vec<NodeIndex> = directed.node_indices().collect();
                    let mut visited: HashSet<NodeIndex> = HashSet::new();
                    while let Some(node) = queue.pop() {
                        if visited.contains(&node) {
                        visited.insert(node);
                        // Find parents (incoming edges)
                        let parent_depths: Vec<usize> = directed
                            .filter_map(|neighbor| depths.get(&directed[neighbor]))
                            .copied()
                            .collect();
                        // BFS level - use max parent depth + 1, or 0 if no parents
                        let new_depth = if let Some(max_val) = parent_depths.into_iter().max() {
                            max_val + 1
                        } else {
                            0
                        };
                        depths.insert(elem_id, new_depth);
                        // Add neighbors to queue (BFS)
                        for neighbor in directed.neighbors_directed(node, petgraph::Outgoing) {
                            if !visited.contains(&neighbor) {
                                queue.push(neighbor);
                            }
                    depths
            };
            // Assign grid positions
                let depth = depths[&elem_id];
                // Position: x = col * COL_WIDTH + component_x_offset
                // y = depth * ROW_HEIGHT
                let col = 0; // Simplified: all nodes in a row
                let x = col as f64 * COL_WIDTH + component_index as f64 * COMPONENT_OFFSET;
                let y = depth as f64 * ROW_HEIGHT;
                self.positions.insert(elem_id, (x, y, 120.0, 55.0));
            component_index += 1;
        Ok(())
    }
    /// Access the computed positions.
    pub fn positions(&self) -> &HashMap<ElementId, (f64, f64, f64, f64)> {
        &self.positions
// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ElementKind, RelationKind};
    fn create_linear_chain() -> Model {
        let mut model = Model::new("test");
        let a = model.add_element("A", ElementKind::BusinessActor);
        let b = model.add_element("B", ElementKind::BusinessActor);
        let c = model.add_element("C", ElementKind::BusinessActor);
        model.link(a, b, RelationKind::Serving);
        model.link(b, c, RelationKind::Serving);
        model
    fn create_cyclic() -> Model {
        model.link(b, a, RelationKind::Serving);
    fn create_disconnected() -> Model {
        let d = model.add_element("D", ElementKind::BusinessActor);
        model.link(c, d, RelationKind::Serving);
    fn create_isolated_element() -> Model {
        model.add_element("A", ElementKind::BusinessActor);
    fn create_empty_model() -> Model {
        Model::new("test")
    #[test]
    fn test_linear_chain() {
        let mut resolver = LayoutResolver::default();
        let model = create_linear_chain();
        resolver.calculate_layout(&model).unwrap();
        let positions = resolver.positions();
        assert_eq!(positions.len(), 3);
        let mut y_positions: Vec<f64> = positions.values().map(|(_, y, _, _)| *y).collect();
        y_positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // Y positions should be strictly increasing (A.y < B.y < C.y)
        assert!(y_positions[0] < y_positions[1]);
        assert!(y_positions[1] < y_positions[2]);
    fn test_cyclic() {
        let model = create_cyclic();
        // Should not hang even with cycle
        assert_eq!(positions.len(), 2);
        // Positions should be valid (finite)
        for pos in positions.values() {
            assert!(pos.0 >= 0.0);
            assert!(pos.1 >= 0.0);
            assert!(pos.2 >= 0.0);
            assert!(pos.3 >= 0.0);
    fn test_disconnected() {
        let model = create_disconnected();
        assert_eq!(positions.len(), 4);
        // Component A-B should be at lower X offset
        // Component C-D should be at higher X offset
        let min_x_comp1 = positions
            .values()
            .filter(|(_, x, _, _)| *x < COMPONENT_OFFSET / 2.0)
            .map(|(x, _, _, _)| *x)
            .fold(f64::INFINITY, f64::min);
        let min_x_comp2 = positions
            .filter(|(_, x, _, _)| *x >= COMPONENT_OFFSET / 2.0)
        // Check X offsets differ by at least COMPONENT_OFFSET
        assert!(min_x_comp2 - min_x_comp1 >= COMPONENT_OFFSET - 1e-6);
    fn test_isolated_element() {
        let model = create_isolated_element();
        assert_eq!(positions.len(), 1);
        // Single element should be near origin (0,0)
        let pos = positions.values().next().unwrap();
        // Allow small floating point tolerance
        assert!((pos.0 - 0.0).abs() < 1.0);
        assert!((pos.1 - 0.0).abs() < 1.0);
    fn test_empty_model() {
        let model = create_empty_model();
        assert!(positions.is_empty());
