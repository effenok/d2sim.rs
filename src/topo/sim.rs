use crate::channel::{Channel, ChannelBuilder};
use crate::component::ComponentBuilder;
use crate::sim::Simulation;
use crate::topo::topo::Topology;

pub trait FromGraphBuilder {
    type Node;
    type Edge;

    fn node_cfg(&mut self, node: &Self::Node) -> &mut Self;
    fn edge_cfg(&mut self, edge: &Self::Edge) -> &mut Self;
}

impl<ChannelT: Channel> Simulation<ChannelT> {

    pub fn build_from_topo<V, E, NB>(&mut self,
                                     mut topo: Topology<V, E>,
                                     builder: &mut NB
    )
        where NB: ComponentBuilder + ChannelBuilder<C = ChannelT>
        + FromGraphBuilder<Node= V, Edge = E>
    {
        let g = &mut topo.g;

        for idx in  g.node_indices() {
            let component_id = self.add_component(builder.node_cfg(&g[idx].data));
            g[idx].component_id = Some(component_id);

        }

        for idx in  g.edge_indices() {
            let (n0, n1) = g.edge_endpoints(idx).unwrap();
            let c0 = g[n0].component_id.unwrap();
            let c1 = g[n1].component_id.unwrap();
            let channel_id = self.add_channel(builder.edge_cfg(&g[idx].data), c0, c1);
            g[idx].channle_id = Some(channel_id);
        }

        // TODO: add value in env
        // sim_env().add_value(String::from("topology"), Box::new(topo));
    }
}