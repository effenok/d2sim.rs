use std::fmt::Debug;
use std::any::Any;

use d2simrs::util::uid::UIdGenRandom;
use d2simrs::util::uid::UniqueId;
use d2simrs::component::{ComponentBuilder, Component, ChannelLabel};
use d2simrs::keys::{ComponentId, ChannelId};
use d2simrs::scheduler::{Scheduler};
use d2simrs::simvars::{sim_sched, sim_time};
use d2simrs::simtime::NO_DELTA;

// process builder -------------------
pub struct ProcessBuilder {
    uid_gen: UIdGenRandom,
}

impl ProcessBuilder {
    pub fn new(max_uid: usize) -> Self {
        ProcessBuilder {uid_gen: UIdGenRandom::new(max_uid)}
    }
}

impl ComponentBuilder for ProcessBuilder {

    fn build_component(&mut self, pid: ComponentId) -> Box<dyn Component> {
        Box::new( Process {
            component_id: pid,
            left: ChannelId::default(),
            right: ChannelId::default(),
            uid: self.uid_gen.generate_uid(),
            state: ProcessState::Unknown
        })
    }
}

// end process builder -------------------

// process  -------------------

#[derive(Debug)]
pub enum ProcessState {
    Unknown, Leader, Terminated(UniqueId),
}

#[derive(Debug)]
pub struct Process {
    pub component_id: ComponentId,
    pub left: ChannelId,
    pub right: ChannelId,
    //--------
    pub uid: UniqueId,
    pub state: ProcessState,
}

impl Component for Process {

    fn sim_id(&self) -> ComponentId {
        self.component_id
    }

    fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
        match label {
            ChannelLabel::Left => {self.left =  channel_id}
            ChannelLabel::Right => {self.right = channel_id}
        }
    }

    fn init(&mut self) {
        // assert correct variables
        assert!(self.left.is_initialized());
        assert!(self.right.is_initialized());

        println!{"initialized process {:?}", self}
        sim_sched().sched_self_event(NO_DELTA, self.id());
    }

    fn process_event(&mut self, sender: ComponentId, _event: Box<dyn Any>) {
        println!{"[time {}ms] starting process {:?}", sim_time().as_millis(), self}
        // this is call to start function
        assert_eq!(self.id(), sender);
        if let Some((channel, msg)) = self.round0() {
            sim_sched().send_msg(self.id(), channel, msg);
        }
    }

    fn receive_msg(&mut self,
                   incoming_channel: ChannelId,
                   msg: Box<dyn Any>,
    ) {
        let msg = msg.downcast::<Message>().unwrap();
        println!{"[time {}ms] process {:?} received msg {:?} on channel {:?}",
                 sim_time().as_millis(), self, msg, incoming_channel}
        if let Some((channel, msg))  = self.round(incoming_channel, msg) {
            sim_sched().send_msg(self.id(), channel, msg);
        }
    }

    fn terminate(&mut self) {
        println!{"terminating process {:?}", self}
        match self.state {
            ProcessState::Unknown => {assert!(false)}
            // Leader => {
            //     println!("\tI am the leader")
            // }
            _ => {}
        }
    }
}

impl Process {

    fn round0(&mut self) -> Option<(ChannelId, Box<Message>)>{

        let channel = self.left();
        let msg = Box::new(Message::new_msg(self.uid));
        println!{"\t sending message {:?} on channel {:?}", msg, channel}
        return Some((channel, msg));
    }

    fn round(&mut self, incoming_channel: ChannelId, msg: Box<Message>) -> Option<(ChannelId, Box<Message>)> {

        assert_eq!(incoming_channel, self.right());

        return match self.state {
            ProcessState::Unknown => {
                match *msg {
                    Message::SendUId(sender) => {
                        if sender > self.uid {
                            let channel = self.left();
                            println!("\tsender={:?} > myuid={:?}, sending message to the left on channel {:?}"
                                     , sender, self.uid, channel);

                            Some((channel, msg))
                        } else if sender < self.uid {
                            println!("\tsender={:?} < myuid={:?}, discarding message", sender, self.uid);
                            None
                        } else {
                            let channel = self.left();
                            println!("\tsender={:?} = myuid={:?}, i am leader, sending terminate to the left on channel {:?}"
                                     , sender, self.uid, channel);
                            self.state = ProcessState::Leader;

                            let msg = Box::new(Message::new_terminate(self.uid));
                            Some((channel, msg))
                        }
                    }
                    Message::Terminate(leader) => {
                        let channel = self.left();
                        println!("\tleader = {:?}, sending terminate to the left on channel {:?}", leader, channel);
                        self.state = ProcessState::Terminated(leader);
                        Some((channel, msg))
                    }
                }
            }
            ProcessState::Leader => {
                if let Message::Terminate(leader) = *msg {
                    assert_eq!(leader, self.uid, "")
                } else { assert!(false) }

                None
            }
            ProcessState::Terminated(_) => {
                // TODO: some kind of runtime exception
                assert!(false, "Unexpected message");
                None
            }
        }
    }

    fn id(&self) -> ComponentId { self.component_id}

    fn left(&self) -> ChannelId {
        self.left
    }

    fn right(&self) -> ChannelId { self.right }
}

// end process  -------------------

// message

#[derive(Debug)]
enum Message {
    SendUId(UniqueId),
    Terminate(UniqueId),
}

impl Message {
    fn new_msg(uid: UniqueId) -> Self {Self::SendUId(uid)}
    fn new_terminate(leader: UniqueId) -> Self {Self::Terminate (leader)}
}
// end message -------------------