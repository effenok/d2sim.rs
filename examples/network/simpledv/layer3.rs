use d2simrs::keys::ComponentId;
use d2simrs::util::internalref::InternalRef;
use std::any::Any;

use crate::layer2::Layer2;
use crate::packet::Packet;
use crate::router::{InternalEvent, RouterId, SimHelper};
use crate::simpledv::SimpleDiv;
use crate::types::InterfaceId;

#[derive(Debug)]
pub enum NextHeader3 {
    SimpleDiv
}

#[derive(Debug)]
pub struct Layer3Packet {
    next_header: NextHeader3,
}

pub struct Layer3 {
    parent_id: ComponentId,
    control_plane: SimpleDiv,

    layer2: InternalRef<Layer2>,
}

impl Layer3 {
    pub fn new(parent_id: ComponentId, router_id: RouterId) -> Self {
        Layer3 {
            parent_id,
            control_plane: SimpleDiv::new(router_id),
            layer2: InternalRef::new(),
        }
    }

    pub fn set_ptrs(&mut self, l2: &mut Layer2, sim: &mut SimHelper) {
        let ptr = self as *mut Layer3;
        self.control_plane.layer3.set_ptr(ptr);
        self.control_plane.sim.set(sim);
        self.layer2.set(l2);
    }

    pub fn add_interface(&mut self, if_id: InterfaceId) {
        self.control_plane.add_interface(if_id);
    }

    pub fn start(&mut self) {
        self.control_plane.start();
    }

    pub fn on_interface_up(&mut self, if_id: InterfaceId) {
        self.control_plane.on_interface_up(if_id)
    }

    pub fn timeout(&mut self, ev: Box<dyn Any>) {
        self.control_plane.timeout(ev);
    }

    pub fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>) {
        // add layer2 next header
        self.layer2.send_packet(if_id, packet);
    }

    pub fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        // here one needs to check for appropriate header
        self.control_plane.receive_packet(if_id, packet);
    }

    pub fn terminate(&mut self) {
        self.control_plane.terminate();
    }
}

//-----------------------------------------------------

