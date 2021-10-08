use d2simrs::basicnet::{InterfaceId, RouterId};
use d2simrs::component::{Component, ComponentBuilder};
use d2simrs::dummycomponent::DummyComponentBuilder;
use d2simrs::keys::{ChannelId, ComponentId};
use d2simrs::util::uid::UIdGenSequential;
use d2simrs::topo::topobuilder::{TopologyBuilder};

use crate::router::RouterBuilder;
use crate::simpledv::config::Config;
use crate::simpledv::metric::Metric;

pub struct NetworkComponentBuilder {
    next_node: NodeData,
    dummy_builder: DummyComponentBuilder,
    router_builder: RouterBuilder,
    channel_builder: DelayChannelBuilder,
}

impl NetworkComponentBuilder {
    pub(crate) fn new() -> Self {
        let delay1ms = std::time::Duration::from_millis(1);

        NetworkComponentBuilder {
            next_node: NodeData::Host,
            dummy_builder: DummyComponentBuilder::default(),
            router_builder: RouterBuilder::default(),
            channel_builder: DelayChannelBuilder::with_delay(delay1ms),
        }
    }
}

impl ComponentBuilder for NetworkComponentBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        return match &self.next_node {
            NodeData::Host => {
                self.dummy_builder.build_component(id)
            }
            NodeData::Router(cfg) => {
                // unimplemented!();
                self.router_builder.build_router(id, cfg.router_id, &cfg.router_config)
            }
        }
    }
}

impl ChannelBuilder for NetworkComponentBuilder {
    type C = DelayChannel;

    fn build_channel(&self, c: ChannelId, from: ComponentId, to: ComponentId) -> Self::C {
        self.channel_builder.build_channel(c, from, to)
    }
}

impl FromGraphBuilder for NetworkComponentBuilder {
    type Node = NodeData;
    type Edge = EdgeData;

    fn node_cfg(&mut self, node: &Self::Node) -> &mut Self {
        self.next_node = node.clone();
        self
    }

    fn edge_cfg(&mut self, _edge: &Self::Edge) -> &mut Self {
        //do nothing
        self
    }
}

#[derive(Debug,Clone)]
pub struct RouterConfig {
    router_id: RouterId,
    router_config: Config,
}

#[derive(Clone)]
pub enum NodeData {
    Host,
    Router(RouterConfig),
}

#[derive(Debug)]
pub struct EdgeData {
    metric: Metric
}

type TopologyBuilder1 = TopologyBuilder<NodeData, EdgeData>;

pub struct NetworkBuilder {
    topo_builder: TopologyBuilder1,
    uid_gen: UIdGenSequential,
}

impl NetworkBuilder {
    pub(crate) fn new() -> Self {
        Self { topo_builder: TopologyBuilder::new(), uid_gen: UIdGenSequential::new(1) }
    }
    
    pub fn add_host(&mut self) {
        let node_data = NodeData::Host;
        self.topo_builder.add_node(node_data);
    }

    pub fn add_router(&mut self,) {
        let node_data = NodeData::Router(RouterConfig{
            router_id: self.uid_gen.generate_uid(),
            router_config: Config::default() });
        self.topo_builder.add_node(node_data);
    }

    pub fn add_link(&mut self, from: usize, to: usize) {
        let edge_data = EdgeData{ metric: Metric::ONE_HOP };
        self.topo_builder.add_edge(from, to, edge_data);

        // now if from or to was a host, router needs to be configured with its id
        
        let mut router_idx = None;

        let from_node = self.topo_builder.node_ref(from);
        let to_node = self.topo_builder.node_ref(to);
        if matches!(from_node, NodeData::Host) {
            router_idx = Some(to);
        } else if matches!(to_node, NodeData::Host) {
            router_idx = Some(from);
        }

        if router_idx.is_some() {
            let router_idx = router_idx.unwrap();
            let edges_num = self.topo_builder.count_edges(router_idx);
            let interface = InterfaceId::from(edges_num-1);
            
            if let NodeData::Router(cfg) = self.topo_builder.node_ref_mut(router_idx){
                cfg.router_config.add_interface(interface, HostAddr {
                    router_id: cfg.router_id, interface_id: interface } );
            }
        }
    }

    pub(crate) fn build_sim(self, sim: &mut Simulation<DelayChannel>) {
        let topo = self.topo_builder.build_topo();
        let mut nc = NetworkComponentBuilder::new();
        sim.build_from_topo(*topo, &mut nc);
    }

    pub fn debug(&self) {
        self.topo_builder.dump();
    }
}

use std::fmt;
use d2simrs::channel::ChannelBuilder;
use d2simrs::channels::delay_channel::{DelayChannel, DelayChannelBuilder};
use d2simrs::sim::Simulation;
use d2simrs::topo::FromGraphBuilder;
use crate::simpledv::addr::HostAddr;

impl fmt::Debug for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            NodeData::Host => {
                f.debug_struct("Host").finish()
            },
            NodeData::Router(router_config) =>  {
                f.debug_struct("Router")
                    .field("router_id", &router_config.router_id.0)
                    .field("config", &router_config.router_config)
                    .finish()
            }
        }
    }
}



