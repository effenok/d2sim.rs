use d2simrs::component::{Component, ComponentBuilder};
use d2simrs::dummycomponent::DummyComponentBuilder;
use d2simrs::keys::ComponentId;

use crate::router::RouterBuilder;

pub enum ComponentType {
    Host,
    Router,
}

pub struct NetworkComponentBuilder {
    next_component: ComponentType,
    dummy_builder: DummyComponentBuilder,
    router_builder: RouterBuilder,
}

impl NetworkComponentBuilder {
    pub(crate) fn new() -> Self {
        NetworkComponentBuilder {
            next_component: ComponentType::Host,
            dummy_builder: DummyComponentBuilder::default(),
            router_builder: RouterBuilder::default(),
        }
    }

    pub fn set_next_component(&mut self, next_component: ComponentType) {
        self.next_component = next_component;
    }
}

impl ComponentBuilder for NetworkComponentBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        return match self.next_component {
            ComponentType::Host => {
                self.dummy_builder.build_component(id)
            }
            ComponentType::Router => {
                self.router_builder.build_component(id)
            }
        }
    }
}



