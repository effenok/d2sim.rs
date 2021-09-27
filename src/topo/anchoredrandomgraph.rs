// TODO: separate utility if possible?
// TODO: trait - need check connectivity?

use crate::topo::topo::{TopoGraph, TopoEdge};
use petgraph::graph::NodeIndex;
use rand::Rng;
use crate::topo::topogen::EdgesGenerator;

pub struct AnchoredRandomGraphGen {
    num_nodes: usize,
    minimum_degree: usize,
    num_edges: usize,
}

impl AnchoredRandomGraphGen {
    pub fn new(num_nodes: usize, connectivity: f64) -> Self {
        AnchoredRandomGraphGen::new1(num_nodes, connectivity, 1)
    }

    pub fn new1(num_nodes: usize, connectivity: f64, minimum_degree: usize) -> Self {
        let num_nodes_f = num_nodes as f64;

        let num_edges = std::cmp::max(
            (connectivity * num_nodes_f * num_nodes_f / 2.0).ceil() as usize,
            ((minimum_degree as f64) * num_nodes_f / 2.0).ceil() as usize
        );

        AnchoredRandomGraphGen {
            num_nodes,
            minimum_degree,
            num_edges,
        }
    }
}

impl EdgesGenerator for AnchoredRandomGraphGen {

    fn estimated_edges_count(&mut self) -> usize {
        self.num_edges
    }

    fn generate_edges(&mut self, g: &mut TopoGraph) {
        let mut rng = rand::thread_rng();

        let indices: Vec<NodeIndex> = g.node_indices().collect();
        
        let mut edges_left = self.num_edges;
        
        // add edges to each node until minimum_degree requirement is satisfied
        for head_idx in g.node_indices() {
            while g.edges(head_idx).count() < self.minimum_degree {
                loop {
                    let tail_idx = indices[rng.gen_range(0..self.num_nodes)];
                    if tail_idx != head_idx && g.find_edge(head_idx, tail_idx).is_none() {
                        g.add_edge(head_idx, tail_idx, TopoEdge::default());
                        edges_left-= 1;
                        break;
                    }
                }
            }
        }

        // add rest of the edges
        while edges_left > 0 {

            let head_idx = indices[rng.gen_range(0..self.num_nodes)];
            let tail_idx = indices[rng.gen_range(0..self.num_nodes)];

            if tail_idx != head_idx && g.find_edge(head_idx, tail_idx).is_none() {
                g.add_edge(head_idx, tail_idx, TopoEdge::default());
                edges_left-= 1;
            }

        }
    }
}
