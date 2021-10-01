use d2simrs::component::{Component, ComponentBuilder};
use d2simrs::dummycomponent::DummyComponentBuilder;
use d2simrs::keys::ComponentId;

use crate::router::RouterBuilder;

pub enum ComponentType {
    Host,
    Switch,
    Router,
}

pub struct NetworkComponentBuilder {
    next_component: ComponentType,
    dummy_builder: DummyComponentBuilder,
    router_builder: RouterBuilder,
}

impl NetworkComponentBuilder {
    pub(crate) fn new(type_: ComponentType) -> Self {
        NetworkComponentBuilder {
            next_component: type_,
            dummy_builder: DummyComponentBuilder::default(),
            router_builder: RouterBuilder {},
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
            ComponentType::Switch => {
                self.dummy_builder.build_component(id)
            }
            ComponentType::Router => {
                self.router_builder.build_component(id)
            }
        }
    }
}



