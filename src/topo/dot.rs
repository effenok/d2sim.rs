use crate::topo::topo::{Topology};
use petgraph::dot::Dot;
use petgraph::dot::Config::{EdgeNoLabel, NodeNoLabel};


impl Topology {

    pub fn dot(topo: &Topology) {
        let dot = Dot::with_attr_getters(
            &topo.g,
            &[NodeNoLabel, EdgeNoLabel],
            &|_, e| {
                String::from(format!("weigh={:.3}", e.weight().weight))
            },
            &|_, n| {
                return match n.1.component_id {
                    None => {String::new()},
                    Some(id) => { String::from(format!("label=\"{:?}\"", id))}
                }

            }
        );

        eprintln!("dot = {:?}", dot);
    }
}