use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex};
use crate::topo::topo::Topology;
use crate::topo::topodecl::{Point, TopoEdge, TopoNode};

pub struct TopologyBuilder<V, E>
{
    topo: Box<Topology<V, E>>,
    pub indices: Vec<NodeIndex>,
}

impl<V, E> TopologyBuilder<V, E> {
    pub fn new() -> Self {
        Self {
            topo: Box::new(Topology::new()),
            indices: vec![],
        }
    }

    pub fn add_node(&mut self, data: V) {
        let node = TopoNode { component_id: None, position: Point::default(), data };
        let idx = self.topo.g.add_node(node);
        self.indices.push(idx);
    }

    pub fn node_ref(&self, idx: usize) -> &V {
        &self.topo.g[self.indices[idx]].data
    }

    pub fn node_ref_mut(&mut self, idx: usize) -> &mut V {
        &mut self.topo.g[self.indices[idx]].data
    }

    pub fn add_edge(&mut self, from: usize, to: usize, data: E) {
        let edge = TopoEdge {
            channle_id: None,
            distance: 0.0,
            data: data,
        };

        self.topo.g.add_edge(self.indices[from], self.indices[to], edge);
    }

    pub fn count_edges(&self, idx: usize) -> usize {
        let node_idx = self.indices[idx];
        self.topo.g.neighbors(node_idx).count()
    }

    pub fn build_topo(self) -> Box<Topology<V, E>> {
        self.topo
    }
}

impl<V, E> TopologyBuilder<V, E>
    where V: std::fmt::Debug,
          E: std::fmt::Debug
{
    pub fn dump(&self) {
        let g = &self.topo.g;
        let dot = Dot::new(g);
        eprintln!("dot = {:?}", dot);
    }
}

