use d2simrs::keys::{ChannelId, ComponentId};
use d2simrs::util::internalref::InternalRef;
use std::collections::HashMap;
use std::fmt;

use crate::layer3::Layer3;
use crate::packet::Packet;
use crate::router::SimHelper;
use crate::types::InterfaceId;

//--------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub enum NextHeader2 {
    Layer3
}

#[derive(Debug, Copy, Clone)]
pub struct P2PPacket {
    pub next_header: NextHeader2,
}

pub struct Layer2 {
    parent_id: ComponentId,
    interfaces: Vec<P2PLayer2Instance>,
    control_plane: Layer2ControlPlane,

    // TODO: accessible to network builder only?
    pub layer3: InternalRef<Layer3>,
    pub sim: InternalRef<SimHelper>,
}

impl Layer2 {
    pub fn new(parent_id: ComponentId) -> Self {
        Layer2 {
            parent_id,
            interfaces: Vec::new(),
            control_plane: Layer2ControlPlane::new(),

            layer3: InternalRef::new(),
            sim: InternalRef::new(),
        }
    }

    pub fn start(&mut self) {
        self.control_plane.start();
    }

    pub fn bring_up_interfaces(&mut self) {
        for if_ in &mut self.interfaces {
            if_.bring_up();
            self.layer3.on_interface_up(if_.interface_id);
        }
    }

    pub fn create_p2p_interface(&mut self, channel_id: ChannelId) -> InterfaceId {
        let if_id = InterfaceId::from(self.interfaces.len());

        let mut if_ = P2PLayer2Instance {
            parent_id: self.parent_id,
            interface_id: if_id,
            channel_id,
            is_up: false,
        };
        self.interfaces.push(if_);

        if_id
    }

    pub fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>) {
        let if_ = &self.interfaces[if_id.as_idx()];
        if !if_.is_up {
            todo!()
        }
        let channel = if_.channel_id;
        self.sim.send_msg_on_channel(channel, packet);
    }

    pub fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        let if_ = &self.interfaces[if_id.as_idx()];
        if !if_.is_up {
            todo!()
        }
        // here i should check for the next layer header, but since there is only one
        self.layer3.receive_packet(if_id, packet);
    }

    pub fn terminate(&mut self) {
        // do nothing
    }
}


//---------------------------------------------------------

struct P2PLayer2Instance {
    parent_id: ComponentId,
    interface_id: InterfaceId,
    channel_id: ChannelId,
    is_up: bool,
}

impl P2PLayer2Instance {
    pub fn bring_up(&mut self) {
        self.is_up = true;
    }
}


impl fmt::Debug for P2PLayer2Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("P2PLayer2Instance")
            .field("parent_id", &self.parent_id)
            .field("interface_id", &self.interface_id)
            .field("channel_id", &self.channel_id)
            .finish()
    }
}

// -----------------------------------------------------------

pub struct Layer2ControlPlane {
    //TODO
}

impl Layer2ControlPlane {
    fn new() -> Self {
        Layer2ControlPlane {}
    }

    fn start(&mut self) {
        // do nothing
    }
}
