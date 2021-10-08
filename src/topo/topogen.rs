use crate::topo::topo::{Topology};
use petgraph::algo::connected_components;
use rand::{thread_rng, Rng};
use crate::topo::topodecl::{Point, TopoGraph, TopoNode};

pub trait EdgesGenerator {
    fn estimated_edges_count(&mut self) -> usize;
    fn generate_edges(&mut self, g: &mut TopoGraph<(),()>);
}

pub struct TopologyGenerator<EdgesGeneratorT>
    where EdgesGeneratorT: EdgesGenerator
{
    edge_strategy: EdgesGeneratorT,
    num_nodes: usize,
}

impl<EdgesGeneratorT: EdgesGenerator>  TopologyGenerator<EdgesGeneratorT> {
    pub fn new(num_nodes: usize, edge_strategy: EdgesGeneratorT) -> TopologyGenerator<EdgesGeneratorT>{
        TopologyGenerator{ edge_strategy, num_nodes }
    }

    pub fn try_build_connected_network(&mut self) -> Topology {
        const MAX_ITER: usize = 10;
        let mut g = TopoGraph::with_capacity(self.num_nodes, self.edge_strategy.estimated_edges_count());

        self.generate_nodes(&mut g);

        for _ in 0..MAX_ITER {
            self.edge_strategy.generate_edges(&mut g);

            if connected_components(&g) == 1 {
                self.set_edge_weight(&mut g);
                return Topology {g: g};
            }

            g.clear_edges();
        }

        // TODO: return Result;
        panic!("could not generate topo in {} iterations", MAX_ITER);
    }

    pub fn build_network(&mut self) -> Topology {
        let mut g = TopoGraph::with_capacity(self.num_nodes, self.edge_strategy.estimated_edges_count());
        self.generate_nodes(&mut g);
        self.edge_strategy.generate_edges(&mut g);
        self.set_edge_weight(&mut g);
        return Topology {g: g};
    }

    //---------------------------------------------------------------------------------

    fn generate_nodes(&mut self, g: &mut TopoGraph<(), ()>) {
        let mut rng = thread_rng();

        for _ in 0..self.num_nodes {
            let x = rng.gen();
            let y = rng.gen();
            g.add_node(TopoNode{component_id: None, position: Point {x, y}, data: () });
        }
    }

    fn set_edge_weight(&mut self, g: &mut TopoGraph<(), ()>) {
        for idx in  g.edge_indices() {
            let (a, b) = g.edge_endpoints(idx).unwrap();
            let p_a = &g[a].position;
            let p_b = &g[b].position;
            let d = ((p_a.x - p_b.x).powi(2) + (p_a.y - p_b.y).powi(2)).sqrt();
            g[idx].distance = d;
        }
    }

}