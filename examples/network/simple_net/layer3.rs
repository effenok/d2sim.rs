use d2simrs::keys::ComponentId;
use d2simrs::util::internalref::InternalRef;

use crate::layer2::Layer2;
use crate::packet::Packet;
use crate::router::SimHelper;
use crate::types::InterfaceId;

pub struct Layer3 {
    control_plane: Layer3ControlPlane,

    pub layer2: InternalRef<Layer2>,
    pub sim: InternalRef<SimHelper>, // TODO: this is probably onle needed in control plane
}

impl Layer3 {
    pub fn new(parent_id: ComponentId) -> Self {
        Layer3 {
            control_plane: Layer3ControlPlane::new(),
            layer2: InternalRef::new(),
            sim: InternalRef::new(),
        }
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

    pub fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>) {
        todo!();
    }

    pub fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        todo!();
    }

    pub fn timeout(&mut self, ev: Box<Packet>) {
        todo!();
    }

    pub fn terminate(&mut self) {
        self.control_plane.terminate();
        todo!();
    }
}

//-----------------------------------------------------

pub struct Layer3ControlPlane {
    //TODO
}

impl Layer3ControlPlane {
    fn new() -> Self {
        Layer3ControlPlane {}
    }

    pub fn add_interface(&self, if_id: InterfaceId) {
        todo!()
    }

    pub fn start(&mut self) {
        todo!()
    }

    pub fn on_interface_up(&mut self, if_id: InterfaceId) {
        todo!()
    }

    pub fn timeout(&mut self, ev: Box<Packet>) {
        todo!();
    }

    pub fn terminate(&mut self) {
        todo!();
    }
}
