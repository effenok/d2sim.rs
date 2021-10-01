use std::any::Any;
use crate::component::{Component, ChannelLabel, ComponentBuilder};
use crate::keys::{ComponentId, ChannelId};

#[derive(Debug)]
pub struct DummyComponent {
    component_id: ComponentId
}

impl Component for DummyComponent {
    
    fn sim_id(&self) -> ComponentId {
        self.component_id
    }

    fn add_channel(&mut self, _channel_id: ChannelId, _label: ChannelLabel) {
        // do nothing
    }

    fn init(&mut self) {
        // do nothing
    }

    fn process_event(&mut self, _sender: ComponentId, _event: Box<dyn Any>) {
        assert!(false, "dummy component is not supposed to receive any events");
    }

    fn receive_msg(&mut self, _incoming_channel: ChannelId, _msg: Box<dyn Any>) {
        assert!(false, "dummy component is not supposed to receive any messages");
    }

    fn terminate(&mut self) {
    }
}

#[derive(Default)]
pub struct DummyComponentBuilder {
}

impl ComponentBuilder for DummyComponentBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        Box::new(DummyComponent{ component_id: id })
    }
}

