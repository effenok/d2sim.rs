use std::any::Any;

use crate::basicnet::{InterfaceId, Packet, SimBase};
use crate::basicnet::types::RouterId;

pub trait Layer {
    fn on_interface_up(&mut self, interface_id: InterfaceId);
    fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>);
}

// TODO: rename
pub trait Ref1<LayerT: Layer> {
    fn set_refs(&mut self, l3_ptr: *mut dyn Any, sim: &mut SimBase);
}

pub trait BottomLayer {
    fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>);
}

pub trait ControlPlane {
    fn new(router_id: RouterId) -> Self;

    fn add_interface(&mut self, if_id: InterfaceId);
    fn start(&mut self);

    fn on_interface_up(&mut self, interface_id: InterfaceId);

    fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>);
    fn on_timeout(&mut self, ev: Box<dyn Any>);

    fn terminate(&mut self);
}