use crate::keys::{ChannelId, ComponentId};
use std::any::Any;
use crate::channel::{Channel, ChannelBuilder};
use crate::simtime::SimTimeDelta;
use std::time::Duration;
use crate::simvars::sim_sched;

pub const ROUND_DELTA: SimTimeDelta = SimTimeDelta::from(Duration::from_secs(1));

#[derive(Debug)]
pub struct BasicChannel {
    pub id: ChannelId,
    pub left: ComponentId,
    pub right: ComponentId,
}

impl Channel for BasicChannel {
    fn accept_message_from(&mut self,
                           source: ComponentId,
                           message: Box<dyn Any>,
                           ) {
        let dst: ComponentId;

        if source == self.left {
            dst = self.right;
        } else if source == self.right {
            dst = self.left;
        } else {
            panic!("unknown source {:?} for channel {:?}", source, self);
        }

        // TODO: this channel only works for synchronous networks
        sim_sched().sched_receive_msg(ROUND_DELTA, dst, self.id, message);
    }
}


pub struct BasicChannelBuilder {}

impl ChannelBuilder for BasicChannelBuilder {
    type C = BasicChannel;

    fn build_channel(&self, id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C {
        BasicChannel { id, left: p0, right: p1 }
    }
}