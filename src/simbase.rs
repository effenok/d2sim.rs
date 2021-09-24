use crate::scheduler::{EventType, RoundScheduler};
use crate::component::{Component, ComponentBuilder};
use crate::channel::synch::Channel;
use crate::environment::Environment;
use crate::keys::{ComponentId, ChannelId};
use std::collections::HashMap;
use crate::channel::ChannelTrait;

static ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
fn generate_next_id() -> usize {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub type Components = Vec<Box<dyn Component>>;
type ChannelMap  = HashMap<usize, Box<Channel>>;

pub struct Simulation {
    components: Components,
    channels: Vec<Channel>,
    scheduler: RoundScheduler,
    env: Environment,
    // sim_keys: SimKeys,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            channels: Vec::new(),
            scheduler: RoundScheduler::new(),
            env: Environment::default(),
            // sim_keys: SimKeys::default()
        }
    }
}

impl Simulation {

    pub fn add_component(&mut self, builder: &mut dyn ComponentBuilder) -> ComponentId {
        let id = self.components.len();
        let id = ComponentId::new(id);

        self.components.push(builder.build_component(id, &mut self.env));
        id
    }

    pub fn add_channel(&mut self, p0: ComponentId, p1: ComponentId) -> ChannelId {
        let channel_id = self.channels.len();
        self.channels.push(Channel::new(channel_id, p0, p1));
        let  p0 = &mut self.components[p0.as_idx()];
        p0.add_channel(channel_id);
        let p1 = &mut self.components[p1.as_idx()];
        p1.add_channel(channel_id);
        channel_id
    }

    pub fn call_init(&mut self) {
        println!("\nInitializing simulation: #components {}", self.components.len());
        for p in self.components.iter_mut() {
            // debug(p);
            p.init(&mut self.scheduler, &mut self.env);
        }
    }

    pub fn step(&mut self) -> bool {
        // eprintln!("self.scheduler.events = {:?}", self.scheduler.events);
        let event = self.scheduler.events.pop();

        if event.is_none() {
            return false;
        }

        let event = event.unwrap();

        // updaate time
        if self.scheduler.curr_time > event.time {
            eprintln!("processing event = {:?}", event);
            eprintln!("self.scheduler.events = {:?}", self.scheduler.events);
            assert!(self.scheduler.curr_time <= event.time, "time mismatch: {} {}", self.scheduler.curr_time, event.time);
        }

        if self.scheduler.curr_time < event.time {
            self.scheduler.curr_time = event.time;
        }

        match event.event {
            EventType::ProcessEvent(ev_data) => {
                let component = &mut self.components[ev_data.receiver.as_idx()];
                component.process_event(ev_data.sender, ev_data.event, &mut self.scheduler, &mut self.env);
            },
            EventType::MsgSendEvent(ev_data) => {
                let channel = &mut self.channels[ev_data.channel];
                channel.accept_message_from(ev_data.sender, ev_data.message, &mut self.scheduler);
            },
            EventType::MsgRcvEvent(ev_data) => {
                // println!("event at time: {}", self.scheduler.curr_time);
                let component = &mut self.components[ev_data.receiver.as_idx()];
                component.receive_msg(ev_data.channel, ev_data.message, &mut self.scheduler, &mut self.env);
            }
        }

        true
    }

    pub fn run(&mut self)  {
        // eprintln!("self.scheduler.events = {:?}", self.scheduler.events);
        println!("\nRunning simulation");
        while self.step(){}
    }

    // TODO: validate accepts immutable iterator for map
    pub fn call_terminate(&mut self, validate: fn(&Components) -> bool) {
        println!("\nSimulation completed in {} time units", self.scheduler.curr_time);
        for p in self.components.iter_mut() {
            // debug(p);
            p.terminate(&mut self.env);
        }

        assert!(validate(&self.components));
    }
}