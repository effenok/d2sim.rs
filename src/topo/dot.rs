use crate::topo::topo::{Topology};
use petgraph::dot::Dot;
use petgraph::dot::Config::{EdgeNoLabel, NodeNoLabel};


impl<V, E> Topology<V, E>
    where V: std::fmt::Debug, E: std::fmt::Debug
{
    pub fn dot(topo: &Topology<V, E>) {
        let dot = Dot::with_attr_getters(
            &topo.g,
            &[NodeNoLabel, EdgeNoLabel],
            &|_, e| {
                String::from(format!("weigh={:.3}", e.weight().distance))
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