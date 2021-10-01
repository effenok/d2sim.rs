use std::fmt::Debug;
use std::any::Any;

use d2simrs::util::uid::UIdGenRandom;
use d2simrs::util::uid::UniqueId;
use d2simrs::component::{ComponentBuilder, Component, ChannelLabel};
use d2simrs::keys::{ComponentId, ChannelId};
use d2simrs::synch::process::{SynchProcess, ProcessId};
use d2simrs::simtime::SimTime;
use std::fmt;
use d2simrs::simvars::sim_sched;

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
            process_id: pid,
            left: ChannelId::default(),
            right: ChannelId::default(),
            curr_round: SimTime::default(),
            uid: self.uid_gen.generate_uid(),
            state: State::Unknown,
        })
    }
}

// end process builder -------------------

// process  -------------------

#[derive(Debug)]
pub enum State {
    Unknown, Leader, NonLeader(UniqueId),
}

#[derive(Debug)]
pub struct Process {
    pub process_id: ProcessId,
    // TODO: uninitialized data
    pub left: ChannelId,
    pub right: ChannelId,
    curr_round: SimTime,
    //--------
    pub uid: UniqueId,
    pub state: State,
}

impl SynchProcess for Process {

    fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
        match label {
            ChannelLabel::Left => {self.left =  channel_id}
            ChannelLabel::Right => {self.right = channel_id}
        }
    }

    fn id(&self) -> ProcessId {
        self.process_id
    }

    fn get_curr_round(&self) -> &SimTime {
        &self.curr_round
    }

    fn set_curr_round(&mut self, round: SimTime) {
        self.curr_round = round;
    }

    //-------------------------------------

    fn init(&mut self) {
        assert!(self.left.is_initialized());
        assert!(self.right.is_initialized());
    }


    fn round_zero(&mut self) {
        println!{"[round {}] starting process {}", self.curr_round.as_rounds(), self}
        let channel = self.left();
        let msg = Box::new(Message::new_send_uid(self.uid));
        println!{"\t sending message {:?} on channel {:?}", msg, channel}
        sim_sched().send_msg(self.id(), channel, msg);
    }

    fn start_new_round(&mut self) {
        // do nothing
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>) {

        let msg = msg.downcast::<Message>().unwrap();
        println!{"[round {}] process {} received msg {:?} on channel {:?}",
                 self.curr_round.as_rounds(), self, msg, incoming_channel}
        if let Some((channel, msg))  = self.round(incoming_channel, msg) {
            sim_sched().send_msg(self.id(), channel, msg);
        }
    }

    fn terminate(&mut self) {
        println!{"terminating process {:?}", self}
        match self.state {
            State::Unknown => {assert!(false)}
            // Leader => {
            //     println!("\tI am the leader")
            // }
            _ => {}
        }
    }
}

impl Process {

    fn round(&mut self, incoming_channel: ChannelId, msg: Box<Message>) -> Option<(ChannelId, Box<Message>)> {

        assert_eq!(incoming_channel, self.right());

        return match self.state {
            State::Unknown => {
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
                            self.state = State::Leader;

                            let msg = Box::new(Message::new_terminate(self.uid));
                            Some((channel, msg))
                        }
                    }
                    Message::Terminate(leader) => {
                        let channel = self.left();
                        println!("\tleader = {:?}, sending terminate to the left on channel {:?}", leader, channel);
                        self.state = State::NonLeader(leader);
                        Some((channel, msg))
                    }
                }
            }
            State::Leader => {
                if let Message::Terminate(leader) = *msg {
                    assert_eq!(leader, self.uid, "")
                } else { assert!(false) }

                None
            }
            State::NonLeader(_) => {
                // TODO: some kind of runtime exception
                assert!(false, "Unexpected message");
                None
            }
        }
    }

    fn left(&self) -> ChannelId { self.left }

    fn right(&self) -> ChannelId { self.right}
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Process {{ process_id {:?}, uid {:?} }}", self.id(), self.uid)
    }
}

// end process  -------------------

// message

#[derive(Debug)]
enum Message {
    SendUId(UniqueId),
    Terminate(UniqueId),
}

impl Message {
    fn new_send_uid(uid: UniqueId) -> Self {Self::SendUId(uid)}
    fn new_terminate(leader: UniqueId) -> Self {Self::Terminate (leader)}
}
// end message -------------------