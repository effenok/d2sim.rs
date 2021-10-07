use std::any::Any;

use crate::basicnet::{InterfaceId, Packet};
use crate::basicnet::types::RouterId;

pub trait InterfaceEventListener {
    fn on_interface_up(&mut self, interface_id: InterfaceId);
}

pub trait BottomLayer {
    fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>);
}

/// represents control plane of the component
pub trait ControlPlane {
    fn new(router_id: RouterId) -> Self;


    /// adds interface to the router
    /// this function is called from Component::add_channel() method
    /// that is before component is initialized
    ///
    /// # Arguments
    ///
    /// * `if_id`: interface identifier local to the router
    fn add_interface(&mut self, interface: InterfaceId);

    fn start(&mut self);

    fn on_interface_up(&mut self, interface_id: InterfaceId);

    fn on_interface_down(&mut self, interface_id: InterfaceId);

    /// process control plane messages
    fn receive_packet(&mut self, if_id: InterfaceId, packet: &Packet);

    /// processes timeout events
    fn on_timeout(&mut self, ev: Box<dyn Any>);

    fn terminate(&mut self);
}