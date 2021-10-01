use d2simrs::simtime::SimTime;
use d2simrs::simvars::sim_time;
use d2simrs::util::internalref::InternalRef;
use std::any::Any;

use crate::layer2::{NextHeader2, P2PPacket};
use crate::layer3::Layer3;
use crate::packet::Packet;
use crate::router::{InternalEvent, RouterId, SimHelper};
use crate::simpledv::addr::InterfaceAddress;
use crate::simpledv::constants::{HELLO_INTERVAL, HOLD_TIME};
use crate::simpledv::neighbortable::NeighborTable;
use crate::simpledv::packets::{SimpleDVPacket, SimpleDVPacketType};
use crate::simpledv::timer::{HelloTimer, NeighborHoldTimer};
use crate::types::InterfaceId;

mod addr;
mod constants;
mod packets;
mod timer;
mod neighbordiscovery;
mod neighbortable;

pub struct SimpleDiv {
    router_id: RouterId,

    neighbor_table: NeighborTable,

    pub(super) layer3: InternalRef<Layer3>,
    pub(super) sim: InternalRef<SimHelper>,
}

impl SimpleDiv {
    pub fn new(router_id: RouterId) -> Self {
        SimpleDiv {
            router_id,
            neighbor_table: NeighborTable::new(),
            layer3: InternalRef::new(),
            sim: InternalRef::new(),
        }
    }

    pub fn add_interface(&mut self, interface_id: InterfaceId) {
        self.neighbor_table.add_entry_for_interface(self.router_id, interface_id);
    }

    pub fn start(&mut self) {
        println!("starting SimpleDV on router {:?}", self.router_id);
        // as of now do nothing,
    }

    pub fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        let packet = packet.unwrap::<SimpleDVPacket>(0);

        match packet.content {
            SimpleDVPacketType::Hello => {
                self.receive_hello(if_id, packet.source)
            }
        }
    }


    pub fn timeout(&mut self, ev: Box<dyn Any>) {
        if ev.is::<HelloTimer>() {
            let hello_timer = ev.downcast::<HelloTimer>().unwrap();
            self.timeout_hello(hello_timer);
        } else if ev.is::<NeighborHoldTimer>() {
            let hold_timer = ev.downcast::<NeighborHoldTimer>().unwrap();
            self.timeout_hold(hold_timer);
        } else {
            assert!(false, "only hello timers are implemented");
            todo!()
        }
    }


    pub fn terminate(&mut self) {
        println!("\tInterface Table of Router {:?}", self.router_id);
        for idx in 0..self.neighbor_table.len() {
            println!("\t\t{:?}", self.neighbor_table[idx]);
        }
    }
}

impl SimpleDiv {
    fn wrap_and_send_packet(&self, if_: InterfaceId, dv_packet: Box<SimpleDVPacket>) {
        static L2_HEADER: P2PPacket = P2PPacket {
            next_header: NextHeader2::Layer3
        };

        let mut packet = Packet::create_and_wrap(dv_packet);
        // todo: create next header for IP
        packet.add_packet(Box::new(L2_HEADER));

        self.layer3.send_packet(if_, packet);
    }
}
