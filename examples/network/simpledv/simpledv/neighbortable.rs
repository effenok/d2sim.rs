use d2simrs::simtime::SimTime;
// use rand::seq::index::IndexVec;
use std::ops::Index;
use std::ops::IndexMut;

use crate::router::RouterId;
use crate::simpledv::addr::InterfaceAddress;
use crate::types::InterfaceId;

#[derive(Debug, PartialEq)]
pub enum InterfaceType {
    EndSystem,
    EIGRP,
}

#[derive(Debug)]
pub struct NeighborTableEntry {
    pub(super) interface_id: InterfaceId,
    pub(super) interface_type: InterfaceType,
    pub(super) my_addr: InterfaceAddress,
    pub(super) other_addr: Option<InterfaceAddress>,
    pub(super) last_hello_received: SimTime,
    pub(super) is_up: bool,
    // layer2 data
}

impl NeighborTableEntry {
    fn new(router_id: RouterId, interface_id: InterfaceId, interface_type: InterfaceType) -> Self {
        NeighborTableEntry {
            interface_id,
            interface_type,
            my_addr: InterfaceAddress { router_id, interface_id },
            other_addr: None,
            last_hello_received: SimTime::default(),
            is_up: false, // TODO: initially false, but send up from layer 2 on start
        }
    }

    pub fn up(&mut self) {
        self.is_up = true;
    }

    pub fn get_interface_id(&self) -> InterfaceId {
        self.interface_id
    }

    pub fn get_my_addr(&self) -> InterfaceAddress {
        self.my_addr
    }

    pub fn is_eigrp_interface(&self) -> bool {
        self.interface_type == InterfaceType::EIGRP
    }
}

#[derive(Debug)]
pub struct NeighborTable {
    table: Vec<NeighborTableEntry>,
}

impl Index<usize> for NeighborTable {
    type Output = NeighborTableEntry;

    fn index(&self, idx: usize) -> &NeighborTableEntry {
        &self.table[idx]
    }
}

impl IndexMut<usize> for NeighborTable {
    fn index_mut(&mut self, idx: usize) -> &mut NeighborTableEntry {
        &mut self.table[idx]
    }
}

impl Index<InterfaceId> for NeighborTable {
    type Output = NeighborTableEntry;

    fn index(&self, idx: InterfaceId) -> &NeighborTableEntry {
        &self.table[idx.as_idx()]
    }
}

impl IndexMut<InterfaceId> for NeighborTable {
    fn index_mut(&mut self, idx: InterfaceId) -> &mut NeighborTableEntry {
        &mut self.table[idx.as_idx()]
    }
}

impl NeighborTable {
    pub fn new() -> Self {
        NeighborTable { table: Vec::new() }
    }

    pub fn add_entry_for_interface(&mut self, router_id: RouterId, interface_id: InterfaceId, interface_type: InterfaceType) {
        assert_eq!(interface_id, self.table.len());
        self.table.push(NeighborTableEntry::new(router_id, interface_id, interface_type));
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}