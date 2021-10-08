use petgraph::graph::UnGraph;
use crate::keys::{ChannelId, ComponentId};

#[derive(Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct TopoNode<NodeData> {
    pub component_id: Option<ComponentId>,
    pub position: Point,
    pub data: NodeData,
}

#[derive(Debug, Default)]
pub struct TopoEdge<EdgeData> {
    pub channle_id: Option<ChannelId>,
    pub distance: f64,
    pub data: EdgeData
}

pub(super) type TopoGraph<N, E> = UnGraph<TopoNode<N>, TopoEdge<E>> ;
// pub(super) type TopoGraph<N, E> = UnGraph<TopoNode<N>, TopoEdge<E>> ; TODO: directed graph
