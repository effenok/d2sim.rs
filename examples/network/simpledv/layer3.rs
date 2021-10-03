use d2simrs::basicnet::{Layer, Packet, SimBase};
use d2simrs::basicnet::layertraits::ControlPlane;
use d2simrs::basicnet::types::{InterfaceId, RouterId};
use d2simrs::util::internalref::InternalRef;
use std::any::Any;

use crate::simpledv::SimpleDiv;
use crate::types::{Layer2, Layer2Ref};

pub struct Layer3 {
    pub control_plane: SimpleDiv,

    layer2: Layer2Ref,
}

impl Layer for Layer3 {
    fn on_interface_up(&mut self, interface_id: InterfaceId) {
        self.control_plane.on_interface_up(interface_id)
    }

    fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        self.control_plane.receive_packet(if_id, packet);
    }
}

impl Layer3 {
    pub fn new(router_id: RouterId) -> Self {
        Layer3 {
            control_plane: SimpleDiv::new(router_id),
            layer2: InternalRef::new(),
        }
    }

    pub fn set_refs(&mut self, l2: &mut Layer2, _sim: &mut SimBase) {
        self.layer2.set(l2);
    }

    pub fn add_interface(&mut self, if_id: InterfaceId) {
        self.control_plane.add_interface(if_id);
    }

    pub fn start(&mut self) {
        self.control_plane.start();
    }

    pub fn timeout(&mut self, ev: Box<dyn Any>) {
        self.control_plane.on_timeout(ev);
    }

    pub fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>) {
        // add layer2 next header
        self.layer2.send_packet(if_id, packet);
    }
    pub fn terminate(&mut self) {
        self.control_plane.terminate();
    }
}
