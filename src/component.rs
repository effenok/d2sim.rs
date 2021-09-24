use crate::environment::Environment;
use crate::keys::{ComponentId, ChannelId};
use crate::scheduler::Scheduler;
use std::any::Any;

#[derive(Debug)]
pub struct ComponentBase {
    pub component_id: ComponentId,
    pub channels: Vec<ChannelId>,
}

impl ComponentBase {
    pub fn new(id: ComponentId) -> Self {
        ComponentBase {
            component_id: id,
            channels: Vec::new()
        }
    }

    pub fn add_channel(&mut self, channel_id: ChannelId) {
        self.channels.push(channel_id);
    }
}

pub trait Component {

    fn get_sim_base(&mut self) -> &mut ComponentBase;

    fn add_channel(&mut self, channel_id: ChannelId) {
        self.get_sim_base().add_channel(channel_id);
    }

    fn init(&mut self, scheduler: &mut Scheduler, env: &mut Environment );

    fn process_event(&mut self,
                     sender: ComponentId,
                     event: Box::<dyn Any>,
                     scheduler: &mut Scheduler,
                     env: &mut Environment
    );

    fn receive_msg(&mut self,
                   incoming_channel: ChannelId,
                   msg: Box<dyn Any>,
                   scheduler: &mut Scheduler,
                   env: &mut Environment
    );

    fn terminate(&mut self, env: &mut Environment );
}


// fn start(&mut self, scheduler: &mut RoundScheduler, env: &mut Environment );

pub trait ComponentBuilder {
    fn build_component(&mut self, pid: ComponentId, env: &mut Environment) -> Box<dyn Component>;
}