use crate::environment::Environment;
use crate::keys::{ComponentId, ChannelId};
use crate::scheduler::Scheduler;
use std::any::Any;

#[derive(Debug)]
pub enum ChannelLabel {
    Left, Right
}

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

    pub fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        self.channels.push(channel_id);
    }
}

pub trait Component {

    fn get_sim_base(&self) -> &ComponentBase {
        todo!("implement me or implement add_channel() function")
    }

    fn get_sim_base_mut(&mut self) -> &mut ComponentBase {
        todo!("implement me or implement add_channel() function")
    }

    fn sim_id(&self) -> ComponentId {
        return self.get_sim_base().component_id;
    }

    fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
        self.get_sim_base_mut().add_channel(channel_id, label);
    }

    fn init(&mut self, scheduler: &mut Scheduler);

    fn process_event(&mut self,
                     sender: ComponentId,
                     event: Box::<dyn Any>,
                     scheduler: &mut Scheduler,
    );

    fn receive_msg(&mut self,
                   incoming_channel: ChannelId,
                   msg: Box<dyn Any>,
                   scheduler: &mut Scheduler,
    );

    fn terminate(&mut self, env: &mut Environment );
}

pub trait StaticComponentBuilder {
    type C : Component;

    fn build_component(&mut self, pid: ComponentId, env: &mut Environment) -> Self::C;
}

pub trait ComponentBuilder {
    fn build_component(&mut self, pid: ComponentId, env: &mut Environment) -> Box<dyn Component>;
}