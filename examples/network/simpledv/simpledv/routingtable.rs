use d2simrs::basicnet::types::InterfaceId;
use std::collections::HashMap;
use std::fmt;

use crate::simpledv::addr::HostAddr;
use crate::simpledv::metric::Metric;

#[derive(Debug)]
pub struct RoutingTableEntry {
    // this is a map Interface -> Metric, with Interface being used as index
    distances: Vec<Metric>,
    preferred_neighbor: InterfaceId,
    my_distance: Metric,
}

#[derive(Debug)]
pub struct RoutingTable {
    storage: HashMap<HostAddr, RoutingTableEntry>,
    num_interfaces: usize,
}

impl RoutingTable {}

impl RoutingTableEntry {

    // pub fn my_distance(&self) -> &Metric {
    //     &self.my_distance
    // }
}

impl RoutingTable {
    pub(crate) fn new() -> Self {
        RoutingTable { storage: HashMap::new(), num_interfaces: 0 }
    }

    pub fn set_num_interfaces(&mut self, num_interfaces: usize) {
        assert_eq!(self.storage.len(), 0, "changing the number of interfaces is not supported");
        self.num_interfaces = num_interfaces;
    }

    fn add_entry(&mut self, addr: HostAddr, if_id: InterfaceId, metric: Metric) -> Metric {
        let my_metric = metric + Metric::ONE_HOP;
        let mut entry = RoutingTableEntry {
            distances: vec![Metric::default(); self.num_interfaces],
            preferred_neighbor: if_id,
            my_distance: my_metric,
        };

        entry.distances[if_id.as_idx()] = metric;

        let old_entry = self.storage.insert(addr, entry);
        assert!(old_entry.is_none());

        my_metric
    }

    pub fn add_local_entry(&mut self, addr: HostAddr, if_id: InterfaceId) {
        let mut entry = RoutingTableEntry {
            distances: vec![Metric::default(); self.num_interfaces],
            preferred_neighbor: if_id,
            my_distance: Metric::new_zero() + Metric::ONE_HOP,
        };

        entry.distances[if_id.as_idx()] = Metric::new_zero();

        let old_entry = self.storage.insert(addr, entry);

        assert!(old_entry.is_none());
    }

    pub fn update_route(&mut self, nb_interface: InterfaceId, adv_addr: HostAddr, nb_metric: Metric) -> Option<(InterfaceId, Metric)> {
        let entry1 = self.storage.get_mut(&adv_addr);

        match entry1 {
            None => {
                let my_metric = self.add_entry(adv_addr, nb_interface, nb_metric);
                return Some((nb_interface, my_metric));
            }
            Some(entry) => {
                // let old_metric = entry.distances[nb_interface.as_idx()];
                //
                // if old_metric <= nb_metric {
                //     todo!("increased metric is not implemented");
                // }

                entry.distances[nb_interface.as_idx()] = nb_metric;
                let new_min_distance = RoutingTable::calculate_my_distance(entry);

                if new_min_distance != entry.my_distance {
                    entry.my_distance = new_min_distance;
                    entry.preferred_neighbor = nb_interface;
                    return Some((entry.preferred_neighbor, entry.my_distance));
                } else {
                    return None;
                }
            }
        }
    }

    // TODO: return an iterator over changed entries
    pub fn remove_entries_for_interface(&mut self, nb_interface: InterfaceId) -> Option<(HostAddr, Metric)> {
        assert!(self.storage.len() <= 1, "having multiple entries is not yet implemented");

        if self.storage.len() == 0 {return None;};

        for (addr, entry) in &mut self.storage {
            entry.distances[nb_interface.as_idx()] = Metric::INFINITY;

            // entry.distances.iter().min().unwrap().clone();
            let (min_if_id, new_min) = entry.distances.iter().enumerate().min_by_key(|&(_, item)| item).unwrap();

            eprintln!("if_id = {:?}", min_if_id);
            eprintln!("if_id = {:?}", new_min);

            if min_if_id != entry.preferred_neighbor.as_idx() {
                entry.preferred_neighbor = InterfaceId::from(min_if_id);
                entry.my_distance = *new_min + Metric::ONE_HOP;
                return Some((*addr, entry.my_distance))
            } else if new_min.is_infinity() {
                entry.my_distance = Metric::INFINITY;
                return Some((*addr, Metric::INFINITY))
            } else {
                assert_eq!( entry.my_distance , *new_min + Metric::ONE_HOP);
                return None;
            }
        }

        todo!()
    }

    pub fn get_item(&self) -> (&HostAddr, Metric) {
        assert!(self.storage.len() == 1, "having multiple entries is not yet implemented");

        for (key, val) in &self.storage {
            return (key, val.my_distance);
        }

        todo!()
    }

    // pub fn iter(&self) -> Iter<'_, HostAddr, RoutingTableEntry>{
    //     self.storage.iter()
    // }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    fn calculate_my_distance(entry: &mut RoutingTableEntry) -> Metric {
        // this only works in metric is hop_count
        // otherwise needs to add interface cost to distances
        let min_dst = entry.distances.iter().min().unwrap().clone();
        let my_dst = min_dst + Metric::ONE_HOP;
        my_dst
    }
}

impl fmt::Display for RoutingTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tRouting Table:\n")?;
        write!(f, "\t\t\t\t\t")?;
        for idx in 0..self.num_interfaces {
            write!(f, "\tif_{}", idx)?;
        }
        write!(f, "\tmy distance/over")?;
        write!(f, "\n")?;
        for (addr, entry) in &self.storage {
            write!(f, "\t\t{} => ", addr)?;
            for (if_id, metric) in entry.distances.iter().enumerate() {
                if if_id == entry.preferred_neighbor.as_idx() {
                    write!(f, "\t\t{}*", metric)?;
                } else {
                    write!(f, "\t\t{}", metric)?;
                }
            }
            write!(f, "\t\t{}/{:?}", entry.my_distance, entry.preferred_neighbor)?;
        }
        write!(f, "\n")
    }
}