use std::fmt;

use crate::basicnet::{InterfaceId, Packet, SimBase};
use crate::basicnet::nettraits::{BottomLayer, InterfaceEventListener};
use crate::keys::ChannelId;
use crate::util::internalref::InternalRef;

#[derive(Debug, Copy, Clone)]
pub enum SimpleL2NextHeader {
    Layer3
}

#[derive(Debug, Copy, Clone)]
pub struct P2PPacket<L2NextHeaderT> {
    pub next_header: L2NextHeaderT,
}

/// simple layer 2 for only point-to-point links
/// each channel is associated with an interface
/// packets should be accosted with interfaces
/// there is no control plane
///
/// packets have only one field - next header
/// there are no addresses for point-to-point links
pub struct SimpleLayer2<NextHeaderT>
    where NextHeaderT: 'static + std::fmt::Debug + Copy
{
    interfaces: Vec<P2PLayer2Instance>,

    pub layer3: InternalRef<NextHeaderT>,
    pub sim: InternalRef<SimBase>,

    marker: std::marker::PhantomData<NextHeaderT>,
}

impl<NextHeaderT> BottomLayer for SimpleLayer2<NextHeaderT>
    where NextHeaderT: 'static + std::fmt::Debug + Copy
{
    fn send_packet(&self, if_id: InterfaceId, packet: Box<Packet>) {
        assert!(packet.is_first_of_type::<P2PPacket<NextHeaderT>>());

        let if_ = &self.interfaces[if_id.as_idx()];
        if !if_.is_up {
            assert!(false); // router should not send packets on interface that is not up
            // ignore packet
            return;
        }
        let channel = if_.channel_id;
        self.sim.send_msg_on_channel(channel, packet);
    }
}

impl<NextHeaderT> SimpleLayer2<NextHeaderT>
    where NextHeaderT: 'static + std::fmt::Debug + Copy
{
    pub fn new() -> Self {
        SimpleLayer2 {
            interfaces: Vec::new(),

            layer3: InternalRef::new(),
            sim: InternalRef::new(),
            marker: Default::default(),
        }
    }

    pub fn set_refs(&mut self, sim: &mut SimBase) {
        // self.layer3.set(l3);
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

    pub fn bring_up_interfaces<L3: InterfaceEventListener>(&mut self, layer3: &mut L3) {
        for if_ in &mut self.interfaces {
            if_.set_up();
            layer3.on_interface_up(if_.interface_id);
        }
    }

    pub fn bring_down_interface(&mut self, if_id: InterfaceId) {
        let mut interface = &mut self.interfaces[if_id.as_idx()];
        assert!(interface.is_up);
        interface.is_up = false;
    }

    pub fn is_interface_up(&self, if_id: InterfaceId) -> bool {
        self.interfaces[if_id.as_idx()].is_up
    }

    pub fn receive_packet(&mut self, if_id: InterfaceId, packet: &Packet) -> Option<NextHeaderT> {
        let if_ = &self.interfaces[if_id.as_idx()];
        if !if_.is_up {
            // ignore packet
            return None;
        }

        let l2header = packet.unwrap_first::<P2PPacket<NextHeaderT>>();
        Some(l2header.next_header)
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