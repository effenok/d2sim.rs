use d2simrs::component::{ChannelLabel, Component, ComponentBuilder};
use d2simrs::keys::{ChannelId, ComponentId};
use d2simrs::simtime::{NO_DELTA, SimTimeDelta};
use d2simrs::simvars::sim_sched;
use std::any::Any;
use std::collections::HashMap;

use crate::layer2::Layer2;
use crate::layer3::Layer3;
use crate::packet::Packet;
use crate::types::InterfaceId;

pub struct RouterBuilder {
    // todo: assign router id's
}

impl ComponentBuilder for RouterBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        let mut router = Box::new(Router {
            sim_helper: SimHelper { sim_id: id },
            channel_map: HashMap::new(),
            layer2: Layer2::new(id),
            layer3: Layer3::new(id),
        });

        router.layer3.layer2.set(&mut router.layer2);
        router.layer3.sim.set(&mut router.sim_helper);

        router.layer2.layer3.set(&mut router.layer3);
        router.layer2.sim.set(&mut router.sim_helper);


        router
    }
}

//-------------------------------------------------------------------

pub enum InternalEvent {
    RouterStartEvent,
    //TODO: add timer here
}

impl InternalEvent {
    fn new_router_start() -> Box<Self> {
        Box::new(InternalEvent::RouterStartEvent)
    }
}

//-------------------------------------------------------------------

pub struct SimHelper {
    sim_id: ComponentId,
}

impl SimHelper {
    pub fn send_msg_on_channel(&self, channel: ChannelId, msg: Box<dyn Any>) {
        sim_sched().send_msg(self.sim_id, channel, msg);
    }

    pub fn timer(&self, timeout: SimTimeDelta, timer: Box<InternalEvent>) {
        sim_sched().sched_self_event1(timeout, self.sim_id, timer);
    }
}

//-------------------------------------------------------------------

struct Router {
    sim_helper: SimHelper,
    channel_map: HashMap<ChannelId, InterfaceId>,
    layer2: Layer2,
    layer3: Layer3,
}

impl Component for Router {
    fn sim_id(&self) -> ComponentId {
        self.sim_helper.sim_id
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

        // todo!()
    }

    fn process_event(&mut self, sender: ComponentId, event: Box<dyn Any>) {
        let event = event.downcast::<InternalEvent>().unwrap();

        match *event {
            InternalEvent::RouterStartEvent => {
                println! {"[time {}ms] starting component {:?}",
                          sim_sched().get_curr_time().as_millis(), self.sim_id()};

                self.layer2.start();
                self.layer3.start();

                self.layer2.bring_up_interfaces();
            }
            // _ => {
            //     todo!();
            // }
        }
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>) {
        let if_id = self.channel_map.get(&incoming_channel).unwrap();
        let packet = msg.downcast::<Packet>().unwrap();
        self.layer2.receive_packet(*if_id, packet);
    }

    fn terminate(&mut self) {
        todo!()
    }
}

