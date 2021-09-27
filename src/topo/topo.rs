use crate::keys::{ComponentId, ChannelId};
use crate::component::{ComponentBuilder};
use petgraph::graph::UnGraph;
use crate::sim::Simulation;
use crate::channel::ChannelBuilder;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct TopoNode {
    pub component_id: Option<ComponentId>,
    pub point: Point,
    // component_data: Box<dyn Component>
}

#[derive(Debug, Default)]
pub struct TopoEdge {
    pub channle_id: Option<ChannelId>,
    pub weight: f64,
    // delay: std::time::Duration,
}

pub(super) type TopoGraph = UnGraph<TopoNode, TopoEdge> ;

pub struct Topology {
    pub(super) g: TopoGraph,
}

impl<CB: ChannelBuilder> Simulation<CB> {

    pub fn build_from_topo(&mut self,
                           topo: &mut Topology,
                           component_builder: &mut dyn ComponentBuilder,
                           channel_builder: &mut CB,
    ) {

        let g = &mut topo.g;

        for idx in  g.node_indices() {
            let component_id = self.add_component(component_builder);
            g[idx].component_id = Some(component_id);

        }

        for idx in  g.edge_indices() {
            let (n0, n1) = g.edge_endpoints(idx).unwrap();
            let c0 = g[n0].component_id.unwrap();
            let c1 = g[n1].component_id.unwrap();
            let channel_id = self.add_channel(channel_builder, c0, c1);
            g[idx].channle_id = Some(channel_id);
        }

    }
}