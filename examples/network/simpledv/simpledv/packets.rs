use crate::simpledv::addr::{HostAddr, InterfaceAddress, SimpleAddress};
use crate::simpledv::metric::Metric;

#[derive(Debug)]
pub struct Route {
    pub addr: HostAddr,
    pub metric: Metric,
}

#[derive(Debug)]
pub enum SimpleDVPacketType {
    Hello,
    Update(Route),
}

pub struct SimpleDVPacket {
    pub source: InterfaceAddress,
    pub destination: SimpleAddress,
    pub content: SimpleDVPacketType,
}

impl SimpleDVPacket {
    pub(super) fn new_hello(my_addr: &InterfaceAddress) -> Self {
        SimpleDVPacket {
            source: my_addr.clone(),
            destination: SimpleAddress::MulticastAddress,
            content: SimpleDVPacketType::Hello,
        }
    }

    pub(super) fn new_update(my_addr: &InterfaceAddress, other_addr: InterfaceAddress, adv_addr: &HostAddr, metric: Metric) -> Self {
        SimpleDVPacket {
            source: my_addr.clone(),
            destination: SimpleAddress::UnicastAddress(other_addr.clone()),
            content: SimpleDVPacketType::Update(Route {
                addr: adv_addr.clone(),
                metric,
            }),
        }
    }
}

use std::fmt;

impl fmt::Debug for SimpleDVPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            SimpleDVPacketType::Hello => {
                f.debug_struct("Hello")
                    .field("src", &self.source)
                    .finish()
            },
            SimpleDVPacketType::Update(route) => {
                f.debug_struct("Update")
                    .field("from", &self.source)
                    .field("to", &self.destination)
                    .field("route-dst", &route.addr)
                    .field("route-metric", &route.metric)
                    .finish()
            }
        }
    }
}