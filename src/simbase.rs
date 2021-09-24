use crate::scheduler::{EventType, Scheduler};
use crate::component::{Component, ComponentBuilder};
use crate::environment::Environment;
use crate::keys::{ComponentId, ChannelId};

use crate::channel::ChannelTrait;
use crate::channel::ChannelBuilder;

pub type Components = Vec<Box<dyn Component>>;

pub struct Simulation<CB>
    where CB : ChannelBuilder
{
    components: Components,
    channels: Vec<CB::C>,
    scheduler: Scheduler,
    env: Environment,
    // sim_keys: SimKeys,
}

impl<CB: ChannelBuilder> Default for Simulation<CB> {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            channels: Vec::new(),
            scheduler: Scheduler::new(),
            env: Environment::default(),
            // sim_keys: SimKeys::default()
        }
    }
}

impl<CB: ChannelBuilder> Simulation<CB> {

    pub fn add_component(&mut self, builder: &mut dyn ComponentBuilder) -> ComponentId {
        let id = self.components.len();
        let id = ComponentId::new(id);

        self.components.push(builder.build_component(id, &mut self.env));
        id
    }

    pub fn add_channel(&mut self, builder: &mut CB, p0: ComponentId, p1: ComponentId) -> ChannelId {
        let channel_id = self.channels.len();
        self.channels.push(builder.build_channel(channel_id, p0, p1));
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

        let event = self.scheduler.next_event();

        match event {
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
            EventType::EndSimulation => {return false;}
        }

        true
    }

    pub fn run(&mut self)  {
        // eprintln!("self.scheduler.events = {:?}", self.scheduler.events);
        println!("\nRunning simulation");
        while self.step(){}
    }

    // TODO: validate accepts immutable iterator for map
    pub fn call_terminate(&mut self) {
        println!("\nSimulation completed in {:?} time units", self.scheduler.curr_time);
        for p in self.components.iter_mut() {
            // debug(p);
            p.terminate(&mut self.env);
        }
    }

    // TODO:
    pub fn validate(&self, validate: fn(&[Box<dyn Component>]) -> bool) {
        assert!(validate(&self.components.iter().as_slice()));
    }
}