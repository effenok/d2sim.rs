use std::any::Any;

use crate::keys::{ChannelId, ComponentId};
use crate::simtime::SimTimeDelta;
use crate::simvars::sim_sched;

pub struct SimBase {
    sim_id: ComponentId,
}

impl SimBase {
    pub fn new(sim_id: ComponentId) -> Self {
        SimBase { sim_id }
    }

    pub fn sim_id(&self) -> ComponentId {
        self.sim_id
    }

    pub fn send_msg_on_channel(&self, channel: ChannelId, msg: Box<dyn Any>) {
        sim_sched().send_msg(self.sim_id, channel, msg);
    }

    pub fn timer(&self, timeout: SimTimeDelta, timer: Box<dyn Any>) {
        sim_sched().sched_self_event_with_data(timeout, self.sim_id, timer);
    }

    pub fn stop_simulation(&self) {
        sim_sched().sim_error();
    }
}