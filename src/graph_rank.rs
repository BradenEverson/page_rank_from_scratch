//! Given a graph of connections and probabilities that each node will move to a connected node,
//! can find the steady state of the system and "rank" nodes

use std::collections::HashMap;

use slotmap::{new_key_type, SlotMap};

use crate::{
    matrix::{Matrix, Stochastic},
    vector::{Probability, Vector},
};

new_key_type! {
    pub struct GraphKey;
}

/// A graph holding connected nodes. Each node has a chance to move to another node or stay where
/// it is, which can be represented as a stochastic matrix
#[derive(Default)]
pub struct ConnectionGraph {
    pub nodes: SlotMap<GraphKey, Node>,
}

impl ConnectionGraph {
    /// Registers a new empty node to the graph
    pub fn register(&mut self) -> GraphKey {
        self.nodes.insert(Node::default())
    }

    /// Connects a node to another node with a given probability that the node will travel
    pub fn connect(&mut self, from: GraphKey, to: GraphKey, prob: f32) {
        self.nodes[from].connections.push((to, prob));
    }

    pub fn matrix_representation<const NODES: usize>(
        &self,
    ) -> Option<Matrix<NODES, NODES, Stochastic>> {
        // Register all nodes to an ID
        let mut res = [Vector::default(); NODES];
        let mut indexes = HashMap::new();
        let mut curr = 0;
        for (key, _) in &self.nodes {
            indexes.insert(key, curr);
            curr += 1
        }

        for (key, node) in &self.nodes {
            let curr_idx = indexes[&key];
            let curr_vector = &mut res[curr_idx];

            for (conn_key, prob) in &node.connections {
                curr_vector[indexes[&conn_key]] = *prob
            }
        }

        Matrix::from_vectors(res).stochastic_matrix()
    }

    pub fn get_rank<const NODES: usize>(&self) -> Option<Vector<NODES, Probability>> {
        let matrix = self.matrix_representation::<NODES>()?;
        matrix.steady_state_solution()
    }
}

/// A node in the graph containing probabilities that it moves to another node
#[derive(Default)]
pub struct Node {
    pub connections: Vec<(GraphKey, f32)>,
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, vector::Vector};

    use super::ConnectionGraph;

    #[test]
    fn graph_generates_proper_stochastic() {
        let mut graph = ConnectionGraph::default();

        let a = graph.register();
        let b = graph.register();
        let c = graph.register();

        graph.connect(a, a, 0.5);
        graph.connect(a, b, 0.25);
        graph.connect(a, c, 0.25);

        graph.connect(b, b, 0.8);
        graph.connect(b, c, 0.2);

        graph.connect(c, a, 0.35);
        graph.connect(c, b, 0.65);

        let stochastic_representation = graph
            .matrix_representation::<3>()
            .expect("Create stochastic matrix from graph");

        let expected = Matrix::from_vectors([
            Vector::from_data([0.5, 0.25, 0.25]),
            Vector::from_data([0.0, 0.8, 0.2]),
            Vector::from_data([0.35, 0.65, 0.0]),
        ])
        .stochastic_matrix()
        .expect("Stochastic");

        assert_eq!(stochastic_representation, expected);
    }

    #[test]
    fn get_rank_from_graph() {
        let mut graph = ConnectionGraph::default();

        let a = graph.register();
        let b = graph.register();
        let c = graph.register();

        graph.connect(a, a, 0.5);
        graph.connect(a, b, 0.25);
        graph.connect(a, c, 0.25);

        graph.connect(b, b, 0.8);
        graph.connect(b, c, 0.2);

        graph.connect(c, a, 0.35);
        graph.connect(c, b, 0.65);

        let rank = graph
            .get_rank::<3>()
            .expect("Create stochastic matrix from graph");

        let expected = [0.12017166, 0.7081545, 0.1716738];
        assert_eq!(rank.data, expected)
    }
}
