use crate::simpledv::addr::{InterfaceAddress, SimpleAddress};

#[derive(Debug)]
pub enum SimpleDVPacketType {
    Hello
}

#[derive(Debug)]
pub struct SimpleDVPacket {
    pub source: InterfaceAddress,
    pub destination: SimpleAddress,
    pub content: SimpleDVPacketType,
}

impl SimpleDVPacket {
    pub(super) fn new_hello(my_addr: InterfaceAddress) -> Self {
        SimpleDVPacket {
            source: my_addr.clone(),
            destination: SimpleAddress::MulticastAddress,
            content: SimpleDVPacketType::Hello,
        }
    }
}
