use crate::keys::{ChannelId, ComponentId};
use std::any::Any;
use crate::channel::{Channel, ChannelBuilder};
use crate::simtime::{SimTimeDelta, NO_DELTA};
use crate::simvars::sim_sched;


#[derive(Debug)]
pub struct DelayChannel {
    pub id: ChannelId,
    pub left: ComponentId,
    pub right: ComponentId,
    pub delay: SimTimeDelta,
}

impl Channel for DelayChannel {

    fn accept_message_from(&mut self,
                           source: ComponentId,
                           message: Box<dyn Any>,
    ) {

        let dst : ComponentId;

        if source == self.left {
            dst = self.right;
        } else if source == self.right {
            dst = self.left;
        } else {
            panic! ("unknown source {:?} for channel {:?}", source, self);
        }

        sim_sched().sched_receive_msg(self.delay, dst, self.id, message);
    }
}

pub struct DelayChannelBuilder {
    delay: SimTimeDelta
}

impl Default for DelayChannelBuilder {
    fn default() -> Self {
        DelayChannelBuilder { delay: NO_DELTA }
    }
}

impl ChannelBuilder for DelayChannelBuilder {
    type C = DelayChannel;

    fn build_channel(&self, id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C {
        DelayChannel { id, left: p0, right: p1, delay: self.delay}
    }
}

impl DelayChannelBuilder {

    pub fn with_delay(delay: std::time::Duration) -> Self {
        DelayChannelBuilder { delay: SimTimeDelta::from(delay) }
    }

    pub fn delay(&mut self, delay: std::time::Duration) -> &mut Self {
        self.delay =  SimTimeDelta::from(delay);
        self
    }
}
