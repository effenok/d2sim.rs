use d2simrs::basicnet::{Packet, SimBase};
use d2simrs::basicnet::layertraits::ControlPlane;
use d2simrs::basicnet::types::{InterfaceId, RouterId};
use d2simrs::simvars::sim_time;
use d2simrs::util::internalref::InternalRef;
use std::any::Any;

use crate::layer3::Layer3;
use crate::simpledv::config::Config;
use crate::simpledv::neighbortable::NeighborTable;
use crate::simpledv::packets::{SimpleDVPacket, SimpleDVPacketType};
use crate::simpledv::routingtable::RoutingTable;
use crate::simpledv::timer::{HelloTimer, NeighborHoldTimer};
use crate::types::{L2NextHeader, P2PPacket};

pub mod addr;
mod constants;
mod packets;
mod timer;
mod neighbordiscovery;
mod neighbortable;
pub mod config;
mod routingtable;
mod metric;
mod routedissemination;

pub struct SimpleDiv {
    router_id: RouterId,
    pub config: Config,
    neighbor_table: NeighborTable,
    routing_table: RoutingTable,

    pub(super) layer3: InternalRef<Layer3>,
    pub(super) sim: InternalRef<SimBase>,
}

impl ControlPlane for SimpleDiv {
    fn new(router_id: RouterId) -> Self {
        SimpleDiv {
            router_id,
            config: Config::new(),
            neighbor_table: NeighborTable::new(),
            routing_table: RoutingTable::new(),
            layer3: InternalRef::new(),
            sim: InternalRef::new(),
        }
    }

    fn add_interface(&mut self, interface_id: InterfaceId) {
        SimpleDiv::add_interface(self, interface_id);
    }

    fn start(&mut self) {
        println!("[time {}ms][router {}] starting SimpleDV on router", sim_time().as_millis(), self.router_id);
        self.set_num_interfaces(self.neighbor_table.len());
    }

    fn on_interface_up(&mut self, interface_id: InterfaceId) {
        let mut entry = &mut self.neighbor_table[interface_id];
        entry.is_up = true;

        if entry.is_simpledv_interface() {
            self.on_simpledv_interface_up(interface_id);
        } else {
            self.add_local_route(interface_id);
        }
    }

    fn receive_packet(&mut self, if_id: InterfaceId, packet: Box<Packet>) {
        let packet = packet.unwrap::<SimpleDVPacket>(0);
        println!("[time {}ms][router {}] received packet {:?}", sim_time().as_millis(), self.router_id, packet);

        match &packet.content {
            SimpleDVPacketType::Hello => {
                self.receive_hello(if_id, packet.source)
            }
            SimpleDVPacketType::Update(route) => {
                self.receive_update(if_id, route);
            }
        }
    }

    fn on_timeout(&mut self, ev: Box<dyn Any>) {
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

    fn terminate(&mut self) {
        println!("[router {}] terminating", self.router_id);
        println!("{}", self.neighbor_table);
        println!("{}", self.routing_table);

        if self.router_id.0 == 1 {
            eprintln!("self.neighbor_table = {:?}", self.neighbor_table);
        }
    }
}

impl SimpleDiv {
    pub fn set_refs(&mut self, l3: *mut Layer3, sim: &mut SimBase) {
        self.layer3.set_ptr(l3);
        self.sim.set(sim);
    }

    fn wrap_and_send_packet(&self, if_: InterfaceId, dv_packet: Box<SimpleDVPacket>) {
        println!("\tsending packet {:?} to neighbor {:?}", dv_packet, if_);
        static L2_HEADER: P2PPacket = P2PPacket {
            next_header: L2NextHeader::Layer3
        };

        let mut packet = Packet::create_and_wrap(dv_packet);
        // todo: create next header for IP
        packet.add_packet(Box::new(L2_HEADER));

        self.layer3.send_packet(if_, packet);
    }
}
