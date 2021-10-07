use std::any::Any;
use std::collections::HashMap;

use d2simrs::basicnet::{SimBase, SimpleLayer2};
use d2simrs::basicnet::packet::Packet;
use d2simrs::basicnet::types::InterfaceId;
use d2simrs::component::{ChannelLabel, Component, ComponentBuilder};
use d2simrs::keys::{ChannelId, ComponentId, DUMMY_COMPONENT};
use d2simrs::simtime::NO_DELTA;
use d2simrs::simvars::{sim_sched};
use d2simrs::util::uid::UniqueId;

use crate::simpledv::config::HostAddr;
use crate::types::{L2NextHeader, Layer2, Layer3};

#[derive(Default)]
pub struct RouterBuilder {
    counter: usize,
}

impl ComponentBuilder for RouterBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        self.counter += 1;

        let mut router = Box::new(Router {
            sim_helper: SimBase::new(id),
            channel_map: HashMap::new(),
            layer2: SimpleLayer2::new(),
            layer3: Layer3::new(UniqueId(self.counter)),
        });


        let ptr = &mut router.layer3 as *mut Layer3;
        let sim = &mut router.sim_helper;

        router.layer3.control_plane.set_refs(ptr, sim);
        router.layer3.set_refs(&mut router.layer2, sim);
        router.layer2.set_refs(sim);

        if self.counter == 1 {
            println!("adding config to router 1");
            let if_id = InterfaceId::from(0);
            router.layer3.control_plane.config.add_interface(if_id, HostAddr {
                router_id: UniqueId(self.counter),
                interface_id: if_id,
            })
        }

        router
    }
}

//-------------------------------------------------------------------

#[derive(Debug)]
pub struct InterfaceEvent {
    pub(crate) interface: InterfaceId,
    pub(crate) down: bool,
}

pub enum InternalEvent {
    RouterStartEvent,
    L3TimerEvent(Box<dyn Any>),
    InterfaceEvent(InterfaceEvent)
}

impl InternalEvent {
    fn new_router_start() -> Box<Self> {
        Box::new(InternalEvent::RouterStartEvent)
    }
}

struct Router {
    sim_helper: SimBase,
    channel_map: HashMap<ChannelId, InterfaceId>,
    layer2: Layer2,
    layer3: Layer3,
}

impl Component for Router {
    fn sim_id(&self) -> ComponentId {
        self.sim_helper.sim_id()
    }

    fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        let if_id = self.layer2.create_p2p_interface(channel_id);
        self.layer3.add_interface(if_id);
        self.channel_map.insert(channel_id, if_id);
    }

    fn init(&mut self) {
        println!("initializing router {:?}", self.sim_id());
        sim_sched().sched_self_event_with_data(NO_DELTA, self.sim_id(),
                                               InternalEvent::new_router_start());
    }

    fn process_event(&mut self, sender: ComponentId, event: Box<dyn Any>) {
        let event = event.downcast::<InternalEvent>().unwrap();

        assert!(sender == self.sim_id() || sender == DUMMY_COMPONENT);

        match *event {
            InternalEvent::RouterStartEvent => {
                self.layer2.start();
                self.layer3.start();

                self.layer2.bring_up_interfaces(&mut self.layer3);
            }
            InternalEvent::L3TimerEvent(l3_timer) => {
                self.layer3.on_timeout(l3_timer)
            }
            InternalEvent::InterfaceEvent(event) => {
                eprintln!("event = {:?}", event);
                self.layer2.bring_down_interface(event.interface);
                self.layer3.on_interface_down(event.interface);
            }
        }
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>) {
        let if_id = *self.channel_map.get(&incoming_channel).unwrap();
        let packet = msg.downcast::<Packet>().unwrap();

        // 1. send message to layer 2
        let next_layer = self.layer2.receive_packet(if_id, &*packet);

        if next_layer.is_none() {
            // drop packet because interface is down
            return;
        }


        // 2. send message to layer 3
        assert!(matches!(next_layer, Some(L2NextHeader::Layer3)));
        self.layer3.receive_packet(if_id, &*packet);

        // there are no layers on this device
    }

    fn terminate(&mut self) {
        self.layer2.terminate();
        self.layer3.terminate();
    }
}