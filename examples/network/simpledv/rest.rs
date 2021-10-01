use d2simrs::component::{ChannelLabel, Component, ComponentBase, ComponentBuilder};
use d2simrs::dummycomponent::DummyComponentBuilder;
use d2simrs::environment::Environment;
use d2simrs::keys::{ChannelId, ComponentId};
use d2simrs::scheduler::Scheduler;
use d2simrs::simtime::NO_DELTA;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::simpledv::{Layer3, SimpleDVPacket};

//---------------------------------------------------------

impl P2PLayer2Instance {
    pub(crate) fn start(&self, sched: &mut Scheduler) {
        self.layer3.borrow_mut().l3.on_interface_up(self.idx, sched);
        // do nothing
    }

    pub(crate) fn send_msg(&self, interface_id: usize, msg: Box<SimpleDVPacket>, scheduler: &mut Scheduler) {
        let msg = Box::new(P2PPacket { next_header: 0, packet: msg });
        // eprintln!("msg = {:?}", msg);
        scheduler.send_msg(self.sim_id, self.channel_id, msg);
    }

    pub(crate) fn receive_msg1(&mut self, msg: Box<dyn Any>, scheduler: &mut Scheduler) {
        let msg = msg.downcast::<P2PPacket>().unwrap();
        let l3ind = msg.next_header;
        let msg = msg.packet;
        self.layer3.borrow_mut().receive_msg1(l3ind, self.idx, msg, scheduler);
    }
}

#[derive(Debug)]
pub(crate) struct Layer2Composite {
    instances: Vec<P2PLayer2Instance>,
    channel_map: HashMap<ChannelId, usize>, // TODO: references, not index
    // names: Vec<String>
}

impl Layer2Composite {
    pub(crate) fn receive_msg1(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>, scheduler: &mut Scheduler) {
        let idx = *self.channel_map.get(&incoming_channel).unwrap();
        assert_eq!(incoming_channel, self.instances[idx].channel_id);
        self.instances[idx].receive_msg1(msg, scheduler);
    }

    pub(crate) fn send_msg(&self, interface_id: usize, msg: Box<SimpleDVPacket>, scheduler: &mut Scheduler) {
        assert!(interface_id < self.instances.len());

        self.instances[interface_id].send_msg(interface_id, msg, scheduler);
    }

    pub(crate) fn start(&mut self, scheduler: &mut Scheduler) {
        for (idx, instance) in self.instances.iter_mut().enumerate() {
            println!("\tbringing up interface {}", idx);
            instance.start(scheduler);
        }
    }
}


//---------------------------------------------------------


//---------------------------------------------------------

#[derive(Debug)]
struct Layer3Composite {
    l3: Layer3,
}

impl Layer3Composite {
    pub(crate) fn receive_msg1(&mut self, l3ind: usize, l2ind: usize, msg: Box<dyn Any>, scheduler: &mut Scheduler) {
        self.l3.receive_msg1(l2ind, msg, scheduler);
    }
}

pub enum LCEvent {
    RouterStart,
    L3TimerEvent((usize, Box<dyn Any>)),
}

impl LCEvent {
    fn new_router_start() -> Box<Self> {
        Box::new(LCEvent::RouterStart)
    }
}

#[derive(Debug)]
struct Router {
    sim_id: ComponentId,
    layer2: Rc<RefCell<Layer2Composite>>,
    layer3: Rc<RefCell<Layer3Composite>>, // TODO: weak references
}

// TODO: tx-queue -> l2 adds messages to tx_queue,

impl Component for Router {
    // fn sim_id(&self) -> ComponentId {
    //     self.sim_id
    // }
    //
    // fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
    //     let if_id = self.layer2.borrow_mut().instances.len();
    //     let i = P2PLayer2Instance {
    //         sim_id: self.sim_id,
    //         idx: if_id,
    //         channel_id,
    //         layer3: Rc::clone(&self.layer3)
    //     };
    //
    //     self.layer2.borrow_mut().instances.push(i);
    //     self.layer2.borrow_mut().channel_map.insert(channel_id, if_id);
    //     self.layer3.borrow_mut().l3.new_interface(if_id);
    // }

    fn init(&mut self, scheduler: &mut Scheduler) {
        // TODO: call init from everyone else to read configs
        // TODO: random start
        scheduler.sched_self_event1(NO_DELTA, self.sim_id(), LCEvent::new_router_start());
        // todo!()
    }

    fn process_event(&mut self, sender: ComponentId, event: Box<dyn Any>, scheduler: &mut Scheduler) {
        let event = event.downcast::<LCEvent>().unwrap();

        match *event {
            LCEvent::RouterStart => {
                println! {"[time {}ms] starting component {:?}",
                          scheduler.get_curr_time().as_millis(), self.sim_id};

                self.layer2.borrow_mut().start(scheduler);
                // self.layer3.borrow_mut().l3.start(&self.layer2.borrow_mut(), scheduler);
            },
            LCEvent::L3TimerEvent((_, ev)) => {
                println! {"[time {}ms] l3 timer fired at {:?}",
                          scheduler.get_curr_time().as_millis(), self.sim_id};
                self.layer3.borrow_mut().l3.process_timer(ev, &self.layer2.borrow_mut(), scheduler);
            }
            // currently only support process start
            // go and start each layer from bottom to top
        }


        // todo!()
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>, scheduler: &mut Scheduler) {
        println! {"[time {}ms] process {:?} received msg {:?} on channel {:?}",
                  scheduler.get_curr_time().as_millis(), self.sim_id, msg, incoming_channel};

        self.layer2.borrow_mut().receive_msg1(incoming_channel, msg, scheduler);
    }

    fn terminate(&mut self, env: &mut Environment) {
        self.layer3.borrow_mut().l3.terminate1();
    }
}
