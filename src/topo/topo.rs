use crate::topo::topodecl::TopoGraph;

pub struct Topology<V=(), E=()> {
    pub(super) g: TopoGraph<V, E>,
}

impl<V, E> Topology<V, E> {
    pub(crate) fn new() -> Self {
        Self {g: TopoGraph::new_undirected()}
    }
}

// impl<V, E> Topology<V, E>{
    // pub fn edge()
// }

// impl<ChannelT: Channel> Simulation<ChannelT> {

    // pub fn build_from_topo<CB>(&mut self,
    //                        topo: &mut Topology,
    //                        component_builder: &mut dyn ComponentBuilder,
    //                        channel_builder: &mut CB,
    // )
    //     where CB: ChannelBuilder<C = ChannelT>
    // {
    //
    //     let g = &mut topo.g;
    //
    //     for idx in  g.node_indices() {
    //         let component_id = self.add_component(component_builder);
    //         g[idx].component_id = Some(component_id);
    //
    //     }
    //
    //     for idx in  g.edge_indices() {
    //         let (n0, n1) = g.edge_endpoints(idx).unwrap();
    //         let c0 = g[n0].component_id.unwrap();
    //         let c1 = g[n1].component_id.unwrap();
    //         let channel_id = self.add_channel(channel_builder, c0, c1);
    //         g[idx].channle_id = Some(channel_id);
    //     }
    //
    // }
// }