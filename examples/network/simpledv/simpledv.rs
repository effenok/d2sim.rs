use std::any::Any;

use d2simrs::basicnet::{Packet, SimBase};
use d2simrs::basicnet::nettraits::ControlPlane;
use d2simrs::basicnet::types::{InterfaceId, RouterId};
use d2simrs::simvars::sim_time;
use d2simrs::util::internalref::InternalRef;

// use crate::layer3::Layer3;
use crate::simpledv::config::Config;
use crate::simpledv::neighbortable::NeighborTable;
use crate::simpledv::packets::{SimpleDVPacket, SimpleDVPacketType};
use crate::simpledv::routingtable::RoutingTable;
use crate::simpledv::timer::{HelloTimer, NeighborHoldTimer};
use crate::types::{L2NextHeader, Layer3, P2PPacket};

pub mod addr;
mod constants;
mod packets;
mod timer;
mod neighbordiscovery;
mod neighbortable;
pub mod config;
mod routingtable;
pub mod metric;
pub(crate) mod routedissemination;

pub(super) const DEBUG_PERIODIC_HELLOS: bool = false;

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
        self.log_msg("starting SimpleDV on router");
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

    fn on_interface_down(&mut self, interface_id: InterfaceId) {
        // TODO: logln macro
        println!("[time {}ms][router {}] interface {:?} is down", sim_time().as_millis(), self.router_id, interface_id);
        let entry = &mut self.neighbor_table[interface_id];

        if entry.is_simpledv_interface() {
            self.on_simpledv_interface_down(interface_id);
        } else {
            todo!()
        }
    }

    fn receive_packet(&mut self, interface: InterfaceId, packet: &Packet) {
        let packet = packet.unwrap_next::<SimpleDVPacket>().unwrap();
        self.log_packet_receive(packet, interface);

        match &packet.content {
            SimpleDVPacketType::Hello => {
                self.receive_hello(interface, packet.source)
            }
            SimpleDVPacketType::Update(route) => {
                self.receive_update(interface, route);
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
        if self.neighbor_table.len() == 0 {return;}

        println!("[router {}] terminating", self.router_id);
        println!("{}", self.neighbor_table);
        println!("{}", self.routing_table);

        // if self.router_id.0 == 1 {
        //     eprintln!("self.neighbor_table = {:?}", self.neighbor_table);
        // }
    }
}

impl SimpleDiv {
    pub fn set_refs(&mut self, l3: *mut Layer3, sim: &mut SimBase) {
        self.layer3.set_ptr(l3);
        self.sim.set(sim);
    }

    fn wrap_and_send_packet(&self, interface: InterfaceId, dv_packet: Box<SimpleDVPacket>) {
        self.log_send_packet(&dv_packet, interface);

        static L2_HEADER: P2PPacket = P2PPacket {
            next_header: L2NextHeader::Layer3
        };

        let mut packet = Packet::create_and_wrap(dv_packet);
        // todo: create next header for IP
        packet.add_packet(Box::new(L2_HEADER));

        self.layer3.send_packet(interface, packet);
    }

    fn log_msg(&self, msg: &str) {
        println!("[time {}ms][router {}] {}", sim_time().as_millis(), self.router_id, msg);
    }

    fn log_packet_receive(&self, packet: &SimpleDVPacket, interface: InterfaceId) {
        if DEBUG_PERIODIC_HELLOS || !matches!(packet.content, SimpleDVPacketType::Hello) {
            println!("[time {}ms][router {}] received packet {:?} from interface {:?}", sim_time().as_millis(), self.router_id, packet, interface);
        }
    }

    fn log_send_packet(&self, packet: &SimpleDVPacket, interface: InterfaceId) {
        if DEBUG_PERIODIC_HELLOS || !matches!(packet.content, SimpleDVPacketType::Hello) {
            println!("\tsending packet {:?} on interface {:?}", packet, interface);
        }
    }
}
