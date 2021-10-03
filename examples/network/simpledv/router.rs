use d2simrs::basicnet::{SimBase, SimpleLayer2};
use d2simrs::basicnet::packet::Packet;
use d2simrs::basicnet::types::InterfaceId;
use d2simrs::component::{ChannelLabel, Component, ComponentBuilder};
use d2simrs::keys::{ChannelId, ComponentId};
use d2simrs::simtime::NO_DELTA;
use d2simrs::simvars::{sim_sched, sim_time};
use d2simrs::util::uid::UniqueId;
use std::any::Any;
use std::collections::HashMap;

use crate::layer3::Layer3;
use crate::simpledv::config::HostAddr;

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
        router.layer2.layer3.set(&mut router.layer3);
        router.layer2.sim.set(sim);

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

pub enum InternalEvent {
    RouterStartEvent,
    L3TimerEvent(Box<dyn Any>),
}

impl InternalEvent {
    fn new_router_start() -> Box<Self> {
        Box::new(InternalEvent::RouterStartEvent)
    }
}

struct Router {
    sim_helper: SimBase,
    channel_map: HashMap<ChannelId, InterfaceId>,
    layer2: SimpleLayer2<Layer3>,
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
        sim_sched().sched_self_event1(NO_DELTA, self.sim_id(),
                                      InternalEvent::new_router_start());
    }

    fn process_event(&mut self, sender: ComponentId, event: Box<dyn Any>) {
        let event = event.downcast::<InternalEvent>().unwrap();

        assert_eq!(sender, self.sim_id());

        match *event {
            InternalEvent::RouterStartEvent => {
                self.layer2.start();
                self.layer3.start();

                self.layer2.bring_up_interfaces();
            }
            InternalEvent::L3TimerEvent(l3_timer) => {
                println! {"[time {}ms] timeout at component {:?}",
                          sim_time().as_millis(), self.sim_id()};
                self.layer3.timeout(l3_timer)
            }
        }
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>) {
        let if_id = self.channel_map.get(&incoming_channel).unwrap();
        let packet = msg.downcast::<Packet>().unwrap();
        self.layer2.receive_packet(*if_id, packet);
    }

    fn terminate(&mut self) {
        self.layer2.terminate();
        self.layer3.terminate();
    }
}