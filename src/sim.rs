use crate::channel::Channel;
use crate::channel::ChannelBuilder;
use crate::component::{ChannelLabel, Component, ComponentBuilder};
use crate::keys::{ChannelId, ComponentId};
use crate::scheduler::EventType;
use crate::simtime::SimTime;
use crate::simvars::{SIM, sim_sched};

pub type Components = Vec<Box<dyn Component>>;

pub struct Simulation<ChannelT>
    where ChannelT: Channel
{
    components: Components,
    channels: Vec<ChannelT>,
    // scheduler: Scheduler,
}

impl<ChannelT: Channel> Default for Simulation<ChannelT> {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            channels: Vec::new(),
            // scheduler: Scheduler::new(),
        }
    }
}

impl<ChannelT: Channel> Simulation<ChannelT> {

    pub fn add_component(&mut self, builder: &mut dyn ComponentBuilder) -> ComponentId {
        let id = self.components.len();
        let id = ComponentId::new(id);

        self.components.push(builder.build_component(id));
        id
    }

    pub fn add_channel<ChannelBuilderT>(&mut self, builder: &mut ChannelBuilderT, left: ComponentId, right: ComponentId) -> ChannelId
        where ChannelBuilderT: ChannelBuilder<C = ChannelT>
    {
        let channel_id = self.channels.len();
        let channel_id = ChannelId::new(channel_id);
        self.channels.push(builder.build_channel(channel_id, left, right));
        let p_left = &mut self.components[left.as_idx()];
        p_left.add_channel(channel_id, ChannelLabel::Left);
        let p_right = &mut self.components[right.as_idx()];
        p_right.add_channel(channel_id, ChannelLabel::Right);
        channel_id
    }

    pub fn call_init(&mut self) {
        println!("\nInitializing simulation: #components {}", self.components.len());

        unsafe {
            SIM.init();
        };

        for p in self.components.iter_mut() {
            // debug(p);
            p.init();
        }
    }

    pub fn step(&mut self) -> bool {

        let event = sim_sched().next_event();

        // let event2 = sim_sched_mut().next_event();
        // eprintln!("event2 = {:?}", event2);

        match event {
            EventType::ProcessEvent(ev_data) => {
                let component = &mut self.components[ev_data.receiver.as_idx()];
                component.process_event(ev_data.sender, ev_data.event);
            },
            EventType::MsgSendEvent(ev_data) => {
                let channel = &mut self.channels[ev_data.channel.as_idx()];
                channel.accept_message_from(ev_data.sender, ev_data.message);
            },
            EventType::MsgRcvEvent(ev_data) => {
                // println!("event at time: {}", self.scheduler.curr_time);
                let component = &mut self.components[ev_data.receiver.as_idx()];
                component.receive_msg(ev_data.channel, ev_data.message);
            }
            EventType::EndSimulation => {return false;}
        }

        true
    }

    pub fn run(&mut self) -> Result<(), ()> {
        // eprintln!("self.scheduler.events = {:?}", self.scheduler.events);
        println!("\nRunning simulation");
        while self.step() {}

        sim_sched().sim_status()
    }

    pub fn run_until(&mut self, time: SimTime) -> Result<(), ()> {
        println!("\nRunning simulation until {:?}", time);
        while self.step() {
            // TODO: compare with first event on the queue
            if sim_sched().get_curr_time() > &time {
                break;
            }
        }

        sim_sched().sim_status()
    }

    // TODO: validate accepts immutable iterator for map
    pub fn call_terminate(&mut self) {
        println!("\nSimulation completed in {:?} time units", sim_sched().get_curr_time());
        for p in self.components.iter_mut() {
            p.terminate();
        }
    }

    // TODO:
    pub fn validate(&self, validate: fn(&[Box<dyn Component>]) -> bool) {
        assert!(validate(&self.components.iter().as_slice()));
    }
}