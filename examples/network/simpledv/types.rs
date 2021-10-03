use d2simrs::basicnet::SimpleLayer2;
use d2simrs::util::internalref::InternalRef;

use crate::layer3::Layer3;

pub type Layer2 = SimpleLayer2<Layer3>;
pub type Layer2Ref = InternalRef<Layer2>;

// pub type Layer3 = DummyLayer3<SimpleDiv, SimpleLayer2<Layer3>>;
// pub type Layer3Ref = InternalRef<Layer3>;

#[derive(Debug, Copy, Clone)]
pub enum L2NextHeader {
    Layer3
}

#[derive(Debug, Copy, Clone)]
pub struct P2PPacket {
    pub next_header: L2NextHeader,
}

#[derive(Debug)]
pub enum NextHeader3 {
    SimpleDiv
}

#[derive(Debug)]
pub struct Layer3Packet {
    next_header: NextHeader3,
}