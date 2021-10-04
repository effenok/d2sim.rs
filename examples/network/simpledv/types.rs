use d2simrs::basicnet::dummylayer3::DummyLayer3;
use d2simrs::basicnet::SimpleLayer2;
use d2simrs::basicnet::simplelayer2::SimpleL2NextHeader;

use crate::simpledv::SimpleDiv;

// layer2 definitions
pub type L2NextHeader = SimpleL2NextHeader;
pub type P2PPacket = d2simrs::basicnet::simplelayer2::P2PPacket<L2NextHeader>;
pub type Layer2 = SimpleLayer2<L2NextHeader>;

// layer3 definitions
pub type Layer3 = DummyLayer3<SimpleDiv, Layer2>;

// #[derive(Debug)]
// pub enum NextHeader3 {
//     SimpleDiv
// }
//
// #[derive(Debug)]
// pub struct Layer3Packet {
//     next_header: NextHeader3,
// }