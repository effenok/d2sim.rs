use std::any::Any;

use crate::basicnet::{InterfaceId, Layer, Packet};
use crate::basicnet::layertraits::{BottomLayer, ControlPlane};
use crate::basicnet::types::RouterId;
use crate::util::internalref::InternalRef;

/// dummy layer3
///
/// represents a non-working layer 3 with a control plane
/// packets can only be forwarded to and from the control plane
pub struct DummyLayer3<CP, Layer2T>
    where CP: ControlPlane, Layer2T: BottomLayer
{
    pub control_plane: CP,

    layer2: InternalRef<Layer2T>,
}

impl<CP, Layer2T> Layer for DummyLayer3<CP, Layer2T>
    where CP: ControlPlane, Layer2T: BottomLayer
{
    fn on_interface_up(&mut self, interface_id: InterfaceId) {
        self.control_plane.on_interface_up(interface_id)
    }

    fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        self.control_plane.receive_packet(if_id, packet);
    }
}

impl<CP, Layer2T> DummyLayer3<CP, Layer2T>
    where CP: ControlPlane, Layer2T: BottomLayer
{
    pub fn new(router_id: RouterId) -> Self {
        DummyLayer3 {
            control_plane: CP::new(router_id),
            layer2: InternalRef::new(),
        }
    }

    // pub fn set_refs(&mut self, l2: &mut dyn Any, sim: &mut SimBase) {
    //     let ptr = self as *mut dyn Any;
    //     self.control_plane.set_refs(ptr, sim);
    //     // self.layer2.set(l2);
    // }

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
