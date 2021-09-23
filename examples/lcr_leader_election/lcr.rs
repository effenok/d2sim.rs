use std::fmt::Debug;
use std::any::Any;

use grappy::environment::Environment;
use grappy::uid::UIdGenRandom;
use grappy::uid::UniqueId;
use grappy::component::{ComponentBuilder, Component, ComponentBase};
use grappy::keys::{ComponentId, ChannelId};
use grappy::scheduler::{RoundScheduler, NO_DELTA};

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

    fn build_component(&mut self, pid: ComponentId, _env: &mut Environment) -> Box<dyn Component> {
        Box::new( Process {
            base: ComponentBase::new(pid),
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
    //TODO: make private
    pub(crate) base: ComponentBase,
    //--------
    pub uid: UniqueId,
    pub state: ProcessState,
}

impl Component for Process {
    fn get_sim_base(&mut self) -> &mut ComponentBase {
        return &mut self.base;
    }

    fn init(&mut self, scheduler: &mut RoundScheduler, _env: &mut Environment) {
        // assert correct variables
        assert_eq!(self.base.channels.len(), 2);

        //FIXME: first process on the ring gets its channels in opposite order
        let pid = self.base.component_id;
        if pid == 1 {
            let right = self.base.channels[0];
            self.base.channels[0] = self.base.channels[1];
            self.base.channels[1] = right;
        }

        println!{"initialized process {:?}", self}
        scheduler.sched_self_event(NO_DELTA, pid);
    }

    fn process_event(&mut self, sender: ComponentId, _event: Box<dyn Any>, scheduler: &mut RoundScheduler, _env: &mut Environment) {
        println!{"[round {}] starting process {:?}", scheduler.curr_time, self}
        // this is call to start function
        assert_eq!(self.id(), sender);
        if let Some((channel, msg)) = self.round0() {
            scheduler.sched_send_msg(NO_DELTA, self.id(), channel, msg);
        }
    }

    fn receive_msg(&mut self,
                   incoming_channel: ChannelId,
                   msg: Box<dyn Any>,
                   scheduler: &mut RoundScheduler,
                   _env: &mut Environment
    ) {
        let msg = msg.downcast::<Message>().unwrap();
        println!{"[round {}] process {:?} received msg {:?} on channel {:?}",
                 scheduler.curr_time, self, msg, incoming_channel}
        if let Some((channel, msg))  = self.round(incoming_channel, msg) {
            scheduler.sched_send_msg(NO_DELTA, self.id(), channel, msg);
        }
    }

    fn terminate(&mut self, _env: &mut Environment) {
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

    fn id(&self) -> ComponentId { self.base.component_id}

    fn left(&self) -> ChannelId {
        self.base.channels[0]
    }

    fn right(&self) -> ChannelId { self.base.channels[1] }
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