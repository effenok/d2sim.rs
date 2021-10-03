use d2simrs::simtime::SimTime;
use d2simrs::simvars::sim_time;

use crate::router::InternalEvent;
use crate::simpledv::{constants, SimpleDiv};
use crate::simpledv::addr::InterfaceAddress;
use crate::simpledv::constants::{HELLO_INTERVAL, HOLD_TIME};
use crate::simpledv::neighbortable::InterfaceType;
use crate::simpledv::packets::SimpleDVPacket;
use crate::simpledv::timer::{HelloTimer, NeighborHoldTimer};
use crate::types::InterfaceId;

impl SimpleDiv {

    pub fn add_interface(&mut self, interface_id: InterfaceId) {
        let interface_type = if self.config.has_interface(interface_id)
        { InterfaceType::EndSystem } else { InterfaceType::SimpleDV };

        self.neighbor_table.add_entry_for_interface(self.router_id, interface_id, interface_type);
    }

    pub fn on_simpledv_interface_up(&mut self, interface_id: InterfaceId) {
        let mut entry = &mut self.neighbor_table[interface_id];
        let if_id = entry.interface_id;

        println!("\tstarting HELLOs on interface {:?}", if_id);

        // send hello
        let hello = Box::new(SimpleDVPacket::new_hello(&entry.my_addr));
        self.wrap_and_send_packet(if_id, hello);


        // start hello timer
        let hello_timer = InternalEvent::new_hello_timer(if_id);
        // TODO:
        // self.sim.timer(HELLO_INTERVAL, hello_timer);
    }

    pub(super) fn receive_hello(&mut self, if_id: InterfaceId, neighbor_addr: InterfaceAddress) {
        if !self.neighbor_table[if_id].is_simpledv_interface() {
            // here the router should ignore the hello,
            // actually I have no idea what to do here
            // however for the simulation we will raise an error
            // and abort simulation
            eprintln!("\t[error!] received hello from a peer that is not eirgp neighbor");
            eprintln!("\tneighbor entry = {:?}", self.neighbor_table[if_id]);
            self.sim.stop_simulation();
            return;
        }


        let other_peer = &self.neighbor_table[if_id].other_addr;

        match other_peer {
            None => {
                // add neighbor to neighbor table
                println!("\tNew neighbor {:?} on interface {:?}", neighbor_addr, if_id);
                self.neighbor_table[if_id].other_addr = Some(neighbor_addr);
                self.neighbor_table[if_id].last_hello_received = sim_time().clone();
                println!("\tupdated neighbor entry to = {:?}", self.neighbor_table[if_id]);

                // start hold timer
                let hold_timer = InternalEvent::new_hold_timer(if_id);
                // TODO:
                // self.sim.timer(HOLD_TIME, hold_timer);

                self.on_new_neighbor(if_id);
            }
            Some(addr) => {
                if neighbor_addr != *addr {
                    // this should not happen
                    todo!();
                }

                self.neighbor_table[if_id].last_hello_received = sim_time().clone();
            }
        }
    }

    pub(super) fn timeout_hello(&mut self, timer: Box<HelloTimer>) {
        let if_id = timer.interface_id;
        println!("\t hello timer for interface {:?}", if_id);
        let mut neighbor = &mut self.neighbor_table[if_id];

        let hello = Box::new(SimpleDVPacket::new_hello(&neighbor.my_addr));
        self.wrap_and_send_packet(if_id, hello);

        // 3. start another timer
        let hello_timer = InternalEvent::from_hello_timer(timer);
        self.sim.timer(HELLO_INTERVAL, hello_timer);
    }

    pub(super) fn timeout_hold(&mut self, timer: Box<NeighborHoldTimer>) {
        let if_id = timer.interface_id;
        let mut neighbor = &mut self.neighbor_table[if_id];

        // println!("\t hello timer for interface {:?}", if_id);

        //  validate entry
        let delete_time = neighbor.last_hello_received + constants::HOLD_TIME;
        let curr_time = sim_time();

        if curr_time < delete_time {
            // neighbor is still up, restart timer with the remaining time
            let delta = delete_time - curr_time;

            let hold_timer = InternalEvent::from_hold_timer(timer);
            self.sim.timer(delta, hold_timer);
        } else {
            println!("\tneighbor timeout {:?} on interface {:?}", neighbor.other_addr, timer.interface_id);
            neighbor.other_addr = None;
            neighbor.last_hello_received = SimTime::default();
            println!("\tupdated neighbor entry to = {:?}", self.neighbor_table[if_id]);
            //todo!("react to neighbor down");
        }
    }
}