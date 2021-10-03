use d2simrs::basicnet::types::{InterfaceId, RouterId};
use d2simrs::simtime::SimTime;
use std::fmt;
// use rand::seq::index::IndexVec;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::Iter;

use crate::simpledv::addr::InterfaceAddress;

#[derive(Debug, PartialEq)]
pub enum InterfaceType {
    EndSystem,
    SimpleDV,
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

    pub fn is_simpledv_interface(&self) -> bool {
        self.interface_type == InterfaceType::SimpleDV
    }

    pub fn is_active_simpledv_interface(&self) -> bool {
        self.interface_type == InterfaceType::SimpleDV && self.other_addr.is_some() && self.is_up
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

    pub fn iter(&self) -> Iter<NeighborTableEntry> {
        return self.table.iter();
    }
}

// todo: https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct

impl fmt::Display for NeighborTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tNeighbor Table:\n")?;
        write!(f, "\t\t\t\t\ttype \t is_up \t my_addr \t\t other_addr \tlast hello\n")?;
        for entry in &self.table {
            write!(f, "\t\t{:?} => ", entry.interface_id)?;
            write!(f, "\t{:?}", entry.interface_type)?;
            write!(f, "\t{}", entry.is_up)?;
            write!(f, "\t{:?}", entry.my_addr)?;
            match entry.other_addr {
                Some(addr) => {
                    write!(f, "\t{:?}", addr)?;
                    write!(f, "\t{}ms", entry.last_hello_received.as_millis())?;
                }
                None => {
                    write!(f, "\t no neighbor known")?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}