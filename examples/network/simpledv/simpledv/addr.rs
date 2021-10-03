use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use crate::router::RouterId;
use crate::types::InterfaceId;

#[derive(Clone, Copy, PartialEq)]
pub struct InterfaceAddress {
    pub router_id: RouterId,
    pub interface_id: InterfaceId,
}

#[derive(Debug)]
pub enum SimpleAddress {
    UnicastAddress(InterfaceAddress),
    MulticastAddress,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HostAddr {
    pub router_id: RouterId,
    pub interface_id: InterfaceId,
}

impl Hash for HostAddr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.router_id.0.hash(state);
        self.interface_id.as_idx().hash(state);
    }
}

impl fmt::Debug for InterfaceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "__::r_{}::if_{}", self.router_id.0, self.interface_id.as_idx())
    }
}

impl fmt::Debug for HostAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "__::r_{}::if_{}", self.router_id.0, self.interface_id.as_idx())
    }
}

impl fmt::Display for HostAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "**::{}::{}", self.router_id.0, self.interface_id.as_idx())
    }
}


