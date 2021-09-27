use crate::topo::topo::{Topology};
use petgraph::dot::Dot;
use petgraph::dot::Config::{EdgeNoLabel, NodeNoLabel};


impl Topology {

    pub fn dot(topo: &Topology) {
        let dot = Dot::with_attr_getters(
            &topo.g,
            &[NodeNoLabel, EdgeNoLabel],
            &|_, e| {
                String::from(format!("label=\"weigh: {:.3}\"", e.weight().weight))
            },
            &|_, n| {
                //TODO: unwrap some
                String::from(format!("label=\"{:?}\"", n.1.component_id))
            }
        );

        eprintln!("dot = {:?}", dot);
    }
}