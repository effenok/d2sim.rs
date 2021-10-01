use d2simrs::util::uid::UniqueId;

use crate::router::RouterId;
use crate::types::InterfaceId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterfaceAddress {
    pub router_id: RouterId,
    pub interface_id: InterfaceId,
}

#[derive(Debug)]
pub enum SimpleAddress {
    UnicastAddress(InterfaceAddress),
    MulticastAddress,
}
