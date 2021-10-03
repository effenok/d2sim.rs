use std::fmt;

use crate::basicnet::{InterfaceId, Packet, SimBase};
use crate::basicnet::layertraits::Layer;
use crate::keys::ChannelId;
use crate::util::internalref::InternalRef;

/// simple layer 2 for only point-to-point links
/// each channel is associated with an interface
/// packets should be accosted with interfaces
/// there is no control plane
///
/// packets have only one field - next header
/// there are no addresses for point-to-point links
pub struct SimpleLayer2<Layer3T>
    where Layer3T: Layer
{
    interfaces: Vec<P2PLayer2Instance>,

    pub layer3: InternalRef<Layer3T>,
    pub sim: InternalRef<SimBase>,
}

impl<Layer3T: Layer> SimpleLayer2<Layer3T> {
    pub fn new() -> Self {
        SimpleLayer2 {
            interfaces: Vec::new(),

            layer3: InternalRef::new(),
            sim: InternalRef::new(),
        }
    }

    pub fn set_refs(&mut self, l3: &mut Layer3T, sim: &mut SimBase) {
        self.layer3.set(l3);
        self.sim.set(sim);
    }

    /// creates interface accosted with the channel
    pub fn create_p2p_interface(&mut self, channel_id: ChannelId) -> InterfaceId {
        let if_id = InterfaceId::from(self.interfaces.len());

        let if_ = P2PLayer2Instance {
            interface_id: if_id,
            channel_id,
            is_up: false,
        };
        self.interfaces.push(if_);

        if_id
    }

    pub fn start(&mut self) {
        // do nothing
    }

    pub fn bring_up_interfaces(&mut self) {
        for if_ in &mut self.interfaces {
            if_.set_up();
            self.layer3.on_interface_up(if_.interface_id);
        }
    }

    pub fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>) {
        // TODO: check that last header is layer2 header with next_header set
        let if_ = &self.interfaces[if_id.as_idx()];
        if !if_.is_up {
            // ignore packet
            return;
        }
        let channel = if_.channel_id;
        self.sim.send_msg_on_channel(channel, packet);
    }

    pub fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        // TODO: parse next header and pass next header / next header id to layer 3
        let if_ = &self.interfaces[if_id.as_idx()];
        if !if_.is_up {
            // ignore packet
            return;
        }
        // here i should check for the next layer header, but since there is only one
        self.layer3.receive_packet(if_id, packet);
    }

    pub fn terminate(&mut self) {
        // do nothing
    }
}

//-------------------------------------------------------------

struct P2PLayer2Instance {
    interface_id: InterfaceId,
    channel_id: ChannelId,
    is_up: bool,
}

impl P2PLayer2Instance {
    pub fn set_up(&mut self) {
        self.is_up = true;
    }
}

impl fmt::Debug for P2PLayer2Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("P2PLayer2Instance")
            .field("interface_id", &self.interface_id)
            .field("channel_id", &self.channel_id)
            .finish()
    }
}
