use crate::component::{Component, ChannelLabel};
use crate::keys::{ComponentId, ChannelId};
use crate::scheduler::{SimTime, Scheduler, NO_DELTA};
use crate::environment::Environment;
use std::any::Any;

pub type ProcessId = ComponentId;

#[derive(Debug)]
pub struct ProcessBase {
    pub component_id: ComponentId,
    pub channels: Vec<ChannelId>,
    pub curr_round: SimTime,
}

impl ProcessBase {
    pub fn new(id: ComponentId) -> Self {
        ProcessBase {
            component_id: id,
            channels: Vec::new(),
            curr_round: SimTime::default(),
        }
    }

    pub fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        self.channels.push(channel_id);
    }
}

pub trait SynchProcess: Component {

    fn get_sim_base(&self) -> &ProcessBase {
        todo!("implement me or implement id() and get_curr_round() function")
    }

    fn get_sim_base_mut(&mut self) -> &mut ProcessBase {
        todo!("implement me or implement add_channel() and set_curr_round() function")
    }

    fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
        SynchProcess::get_sim_base_mut(self).add_channel(channel_id, label);
    }

    fn id(&self) -> ProcessId {
        SynchProcess::get_sim_base(self).component_id
    }

    fn get_curr_round(&self) -> &SimTime {
        &SynchProcess::get_sim_base(self).curr_round
    }

    fn set_curr_round(&mut self, round: SimTime) {
        SynchProcess::get_sim_base_mut(self).curr_round = round;
    }

    //-----------------------------------------------------------------------

    fn init(&mut self, _env: &mut Environment) {}

    fn round_zero(&mut self, scheduler: &mut Scheduler);

    fn start_new_round(&mut self, scheduler: &mut Scheduler);

    fn receive_msg(&mut self,
                         incoming_channel: ChannelId,
                         msg: Box<dyn Any>,
                         scheduler: &mut Scheduler
    );

    fn terminate(&mut self, env: &mut Environment);
}

impl<P: SynchProcess> Component for P {

    fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
        SynchProcess::add_channel(self, channel_id, label);
    }

    fn init(&mut self, scheduler: &mut Scheduler) {
        SynchProcess::init(self, &mut scheduler.env);

        //println!{"initialized process {:?}", self}
        scheduler.sched_self_event(NO_DELTA, self.id());
    }

    fn process_event(&mut self, sender: ComponentId, _event: Box<dyn Any>, scheduler: &mut Scheduler) {
        //TODO: what about the components that wake up on no input in rounds...

        assert_eq!(self.id(), sender);
        assert!(self.get_curr_round().is_zero());
        assert!(scheduler.get_curr_time().is_zero());

        self.round_zero(scheduler);
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>, scheduler: &mut Scheduler) {
        if self.get_curr_round().as_rounds() != scheduler.get_curr_time().as_rounds() {
            self.set_curr_round(scheduler.get_curr_time().clone());
            self.start_new_round(scheduler);
        }

        SynchProcess::receive_msg(self, incoming_channel, msg, scheduler);

    }

    fn terminate(&mut self, env: &mut Environment) {
        SynchProcess::terminate(self, env);
    }
}
