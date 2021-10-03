use crate::simpledv::packets::{Route, SimpleDVPacket};
use crate::simpledv::SimpleDiv;
use crate::types::InterfaceId;

impl SimpleDiv {
    pub(super) fn set_num_interfaces(&mut self, num_interfaces: usize) {
        self.routing_table.set_num_interfaces(num_interfaces);
    }

    pub(super) fn add_local_route(&mut self, interface_id: InterfaceId) {
        let host_addr = self.config.get_config(interface_id).advertise_addr;
        self.routing_table.add_local_entry(host_addr, interface_id);
    }

    pub(super) fn on_new_neighbor(&mut self, interface_id: InterfaceId) {
        // if there are no routes in the table, do nothing
        if self.routing_table.len() == 0 {
            return;
        }

        let entry = &self.neighbor_table[interface_id];
        let (adv_addr, metric) = self.routing_table.get_item();

        let update = SimpleDVPacket::new_update(&entry.my_addr, entry.other_addr.unwrap(),
                                                adv_addr, metric);

        self.wrap_and_send_packet(interface_id, Box::new(update));
    }

    pub(super) fn receive_update(&mut self, interface_id: InterfaceId, route: &Route) {
        println!("\treceived new route = {:?} from neighbor {:?}", route, interface_id);

        // check that this interface is participating in SimpleDV calculation
        if !self.neighbor_table[interface_id].is_active_simpledv_interface() {
            // here the router should ignore the update,
            // however for the simulation we will raise an error
            // and abort simulation
            eprintln!("\t received update from non-active interface, aborting");
            eprintln!("\t\t neighbor entry = {:?}", self.neighbor_table[interface_id]);
            self.sim.stop_simulation();
            return;
        }

        let upd_res = self.routing_table.update_route(interface_id, route.addr, route.metric);

        match upd_res {
            None => {
                println!("\tI already have a better route");
                return;
            }
            Some(my_new_metric) => {
                println!("\tMy route has changed, sending update on all other interfaces");

                println!("{}", self.routing_table);

                for entry in self.neighbor_table.iter() {
                    if !entry.is_active_simpledv_interface() {
                        continue;
                    }

                    // todo: for now send my entry, we will implement poison reverse later

                    let update = SimpleDVPacket::new_update(&entry.my_addr, entry.other_addr.unwrap(),
                                                            &route.addr, my_new_metric);

                    self.wrap_and_send_packet(entry.interface_id, Box::new(update));
                }
            }
        }
    }
}