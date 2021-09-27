// TODO: separate utility if possible?

use crate::topo::topo::{TopoGraph, TopoNode, TopoEdge, Topology};
use petgraph::graph::NodeIndex;
use rand::Rng;
use petgraph::algo::connected_components;

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

    fn generate_nodes(&mut self, g: &mut TopoGraph) {
        for _ in 0..self.num_nodes {
            g.add_node(TopoNode{component_id: None });
        }
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

        eprintln!("g.node_count() = {:?}", g.node_count());
        eprintln!("g.edge_count() = {:?}", g.edge_count());
        eprintln!("edges_left = {:?}", edges_left);
    }

    pub fn generate_connected_graph(&mut self, max_iter: usize) -> Topology {
        let mut g = TopoGraph::with_capacity(self.num_nodes, self.num_edges);

        self.generate_nodes(&mut g);

        for _ in 0..max_iter {
            self.generate_edges(&mut g);

            if connected_components(&g) == 1 {
                return Topology {g: g};
            }

           g.clear_edges();
        }

        // TODO: return Result;
        panic!("could not generate topo in {} iterations", max_iter);
    }

    pub fn generate_graph(&mut self) -> Topology {
        let mut g = TopoGraph::with_capacity(self.num_nodes, self.num_edges);
        self.generate_nodes(&mut g);
        self.generate_edges(&mut g);
        return Topology {g: g};
    }
}
