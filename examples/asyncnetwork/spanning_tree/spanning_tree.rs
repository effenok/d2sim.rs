use std::fmt::Debug;
use std::any::Any;

use d2simrs::util::uid::UIdGenRandom;
use d2simrs::util::uid::UniqueId;
use d2simrs::component::{ComponentBuilder, Component, ComponentBase};
use d2simrs::keys::{ComponentId, ChannelId};
use d2simrs::channel::ChannelBuilder;
use d2simrs::channels::delay_channel::DelayChannel;
use rand::Rng;
use std::fmt;
use d2simrs::simtime::{SimTimeDelta, NO_DELTA};
use d2simrs::simvars::{sim_sched, sim_time};

// random delay channel builder ----------

pub struct RandomDelayChannelBuilder {
}

impl Default for RandomDelayChannelBuilder {
    fn default() -> Self {
        RandomDelayChannelBuilder { }
    }
}

impl ChannelBuilder for RandomDelayChannelBuilder {
    type C = DelayChannel;

    fn build_channel(&self, id: ChannelId, left: ComponentId, right: ComponentId) -> Self::C {
        let delay_ms = rand::thread_rng().gen_range(1..11);
        let delay = std::time::Duration::from_millis(delay_ms);

        DelayChannel { id, left, right, delay: SimTimeDelta::from(delay)}
    }
}

// process builder -------------------
pub struct ProcessBuilder {
    uid_gen: UIdGenRandom,
    has_root: bool,
}

impl ProcessBuilder {
    pub fn new(max_uid: usize) -> Self {
        ProcessBuilder {uid_gen: UIdGenRandom::new(max_uid), has_root: false }
    }
}

impl ComponentBuilder for ProcessBuilder {

    fn build_component(&mut self, pid: ComponentId) -> Box<dyn Component> {
        let state;

        if self.has_root {
            state = State::Unmarked;
        } else {
            state = State::Root;
            self.has_root = true;
        }

        Box::new( Process {
            base: ComponentBase::new(pid),
            uid: self.uid_gen.generate_uid(),
            state
        })
    }
}

// end process builder -------------------

// process  -------------------

#[derive(Debug)]
pub struct TreeInfo {
    parent: UniqueId,
    root: UniqueId
}

#[derive(Debug)]
pub enum State {
    Root, Unmarked, Marked(TreeInfo),
}

#[derive(Debug)]
pub struct Process {
    base: ComponentBase,
    //--------
    pub uid: UniqueId,
    pub state: State,
}

impl Component for Process {

    fn sim_id(&self) -> ComponentId {
        self.base.component_id
    }

    fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        self.base.channels.push(channel_id);
    }

    fn init(&mut self) {
        println!{"initialized process {:?}", self}

        if let State::Root = self.state {
            sim_sched().sched_self_event(NO_DELTA, self.sim_id());
        }
    }

    fn process_event(&mut self, sender: ComponentId, _event: Box<dyn Any>) {
        assert_eq!(sender, self.sim_id());
        println!("[time {}ms] starting process {:?}", sim_time().as_millis(), self);

        for channel in &self.base.channels {
            let msg = Box::new(Message::new(
                self.uid, self.uid
            ));
            println!{"\t sending message {:?} on channel {:?}", msg, channel}
            sim_sched().send_msg(self.sim_id(), *channel, msg);
        }
    }

    fn receive_msg(&mut self,
                   incoming_channel: ChannelId,
                   msg: Box<dyn Any>
    ) {
        let msg = msg.downcast::<Message>().unwrap();
        println!{"[time {}ms] process {} received msg {:?} on channel {:?}",
                 sim_time().as_millis(), self, msg, incoming_channel};

        match &self.state {
            State::Unmarked => {
                for channel in &self.base.channels {
                    if incoming_channel != *channel {
                        let my_msg = Box::new(Message::new(
                            msg.root, self.uid
                        ));
                        println!{"\t sending message {:?} on channel {:?}", msg, channel}
                        sim_sched().send_msg(self.sim_id(), *channel, my_msg);
                    }
                }

                self.state = State::Marked(TreeInfo{ parent: msg.sender, root: msg.root })
            }
            State::Root => { assert!(false);}
            State::Marked(ti) => {
                println!{"\t ignoring message, node is already marked with {:?}", ti}
            }
        };
    }

    fn terminate(&mut self) {
        println!{"terminating process {:?}", self}

        if let State::Unmarked = self.state {
            assert!(false);
        }
    }
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Process {{ process_id {:?}, uid {:?} }}", self.sim_id(), self.uid)
    }
}

// end process  -------------------

// message
#[derive(Debug)]
struct  Message {
    root: UniqueId,
    sender: UniqueId
}

impl Message {
    fn new(root: UniqueId, sender: UniqueId) -> Self {
        Message{ root, sender }
    }
}
// end message -------------------