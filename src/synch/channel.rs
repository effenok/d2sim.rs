use crate::keys::{ChannelId, ComponentId};
use crate::scheduler::{Scheduler, ROUND_DELTA};
use std::any::Any;
use crate::channel::{ChannelTrait, ChannelBuilder};

#[derive(Debug)]
pub struct BasicChannel {
    pub id: ChannelId,
    pub left: ComponentId,
    pub right: ComponentId,
}

impl ChannelTrait for BasicChannel {

    fn accept_message_from(&mut self,
                           source: ComponentId,
                           message: Box<dyn Any>,
                           scheduler: &mut Scheduler) {

        let dst : ComponentId;

        if source == self.left {
            dst = self.right;
        } else if source == self.right {
            dst = self.left;
        } else {
            panic! ("unknown source {:?} for channel {:?}", source, self);
        }

        // TODO: this channel only works for synchronous networks
        scheduler.sched_receive_msg(ROUND_DELTA, dst, self.id, message);
    }
}


pub struct BasicChannelBuilder {}

impl ChannelBuilder for BasicChannelBuilder {
    type C = BasicChannel;

    fn build_channel(&self, id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C {
        BasicChannel { id, left: p0, right: p1, }
    }
}