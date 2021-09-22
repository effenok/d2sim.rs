use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
// use std::cmp::Reverse;
use std::fmt::Debug;

use crate::uid::UIdGenRandom;
use crate::uid::UniqueId;
use crate::sim::ProcessState::{Unknown, Leader, Terminated};
use crate::sim::Message::Terminate;

/*
TODO: this macro suppresses all outputs, it can be used to
 create binaries for performance testing once i figure out how to
 turn stdout on and off in Cargo.toml file
macro_rules! println {
($($rest:tt)*) => {
#[cfg(feature = "stdout")]
std::println!($($rest)*)
}
}
future simulation base ---------------------------------
*/

static ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
fn generate_next_id() -> usize {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub type ComponentsMap = HashMap<usize, Box<Process>>;
type ChannelMap  = HashMap<usize, Box<Channel>>;
// type ChannelBox = Box<Channel>;

pub struct Simulation {
    components: ComponentsMap,
    channels: ChannelMap,
    scheduler: Scheduler,
    env: Environment,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            channels: HashMap::new(),
            scheduler: Scheduler::new(),
            env: Environment::default()
        }
    }
}

impl Simulation {

    // TODO: pass process_builder function with some parameters set

    pub fn add_process(&mut self,  builder: &mut ProcessBuilder) -> ProcessId {
        let id = generate_next_id();

        self.components.insert(id, Box::new(builder.create_process(id.clone())));
        id
    }

    pub fn add_channel(&mut self, p0: ProcessId, p1: ProcessId) -> ChannelId {
        let channel_id = generate_next_id();
        self.channels.insert(channel_id, Box::new(Channel::new(p0, p1)));
        let  p0 = self.components.get_mut(&p0).unwrap();
        p0.channels.push(channel_id);
        let p1 = self.components.get_mut(&p1).unwrap();
        p1.channels.push(channel_id);
        channel_id
    }

    pub fn call_init(&mut self) {
        println!("\nInitializing simulation: #components {}", self.components.len());
        for p in self.components.values_mut() {
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
            SimEvent::WakeUpProcess(ev_data) => {
                let process = self.components.get_mut(&ev_data.process).unwrap();
                process.start(&mut self.scheduler);
            },
            SimEvent::SendOnChannel(ev_data) => {
                let channel = self.channels.get_mut(&ev_data.channel).unwrap();
                channel.message_from(ev_data.sender, ev_data.channel, ev_data.message, &mut self.scheduler);
            },
            SimEvent::ReceiveByProcess(ev_data) => {
                println!("event at time: {}", self.scheduler.curr_time);
                let process = self.components.get_mut(&ev_data.receiver).unwrap();
                process.receive_msg(ev_data.message, ev_data.channel, &mut self.scheduler);
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
    pub fn call_terminate(&mut self, validate: fn(&ComponentsMap) -> bool) {
        println!("\nSimulation completed in {} time units", self.scheduler.curr_time);
        for p in self.components.values_mut() {
            // debug(p);
            p.terminate(&mut self.env);
        }

        assert!(validate(&self.components));
    }
}

// TODO: impl Debug for Simulation {
// 	println!("{:?}");
// 	println!();
// }

#[derive(Debug)]
struct WakeUpEvent {
    process: ProcessId,
}

#[derive(Debug)]
struct ChannelEvent {
    sender:ProcessId,
    channel:ChannelId,
    message: Box<Message>,
}

#[derive(Debug)]
struct ProcessEvent {
    channel:ChannelId,
    receiver:ProcessId,
    message: Box<Message>,
}

#[derive(Debug)]
enum SimEvent {
    WakeUpProcess(WakeUpEvent),
    SendOnChannel(ChannelEvent),
    ReceiveByProcess(ProcessEvent)
}

#[derive(Debug)]
struct Event {
    time: usize,
    event: SimEvent,
    // sender: Option<ProcessId>,
    // channel: ChannelId,
    // receiver: Option<ProcessId>,
    // message: Box<Message>,
}

// impl Debug for Event {
// 	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
// 		write!(f, "Event {{ time: {:?}, sender: {:?}, channel: {:?}, receiver: {:?}, message: {:p} }}"
// 			   , self.time, self.sender, self.channel, self.receiver, self.message)
// 	}
// }

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time.cmp(&other.time).reverse())
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

const NO_DELTA: usize = 0;
const ROUND_DELTA: usize = 1;

pub struct Scheduler {
    events: BinaryHeap<Event>,
    curr_time: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler{ events: BinaryHeap::default(), curr_time: 0}
    }

    fn schedule_channel_event(&mut self, timedelta: usize, sender: ProcessId, channel: ChannelId, message: Box<Message>) {
        let time = self.curr_time + timedelta;
        let event = Event{ time, event: SimEvent::SendOnChannel(
            ChannelEvent { sender, channel, message }
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }

    fn schedule_process_event(&mut self, timedelta: usize, receiver: ProcessId, channel: ChannelId, message: Box<Message>) {
        let time = self.curr_time + timedelta;
        let event = Event{ time, event: SimEvent::ReceiveByProcess(
                ProcessEvent {channel, receiver, message}
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }

    fn schedule_process_start(&mut self, timedelta: usize, process: ProcessId) {
        assert_eq!(self.curr_time, 0);

        let time = self.curr_time + timedelta;
        let event = Event{ time, event: SimEvent::WakeUpProcess(
            WakeUpEvent { process }
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }
}

#[derive(Debug,Default)]
struct Environment {

    // TODO: variable storage goes here
}

// impl Default for Environment {
//     fn default() -> Self {
//         // Environment {uid_gen: UIdGenRandom::new(100)}
//     }
// }

// future simulation asynch-nw ---------------------------------

pub type ProcessId = usize;
pub type ChannelId = usize;

pub struct ProcessBuilder {
    uid_gen: UIdGenRandom,
}

impl ProcessBuilder {
    pub(crate) fn new(max_uid: usize) -> Self {
        ProcessBuilder {uid_gen: UIdGenRandom::new(max_uid)}
    }

    fn create_process(&mut self, pid: ProcessId) -> Process {
        Process {
            process_id: pid,
            channels: Vec::new(),
            uid: self.uid_gen.generate_uid(),
            state: ProcessState::Unknown
        }
    }
}

#[derive(Debug)]
pub enum ProcessState {
    Unknown, Leader, Terminated(UniqueId),
}

#[derive(Debug)]
pub struct Process {
    process_id: ProcessId,
    channels: Vec<ChannelId>, // TODO: refactor in left and right
    //--------
    pub uid: UniqueId,
    pub state: ProcessState,
}

// impl Default for Process {
//     fn default() -> Self {
//         Process {
//             process_id: None,
//             channels: Vec::new(),
//             uid: UniqueId(0),
//             state: Unknown
//         }
//     }
// }

// TODO: use start, or have a special schedule wake-up ?
// special simulation channel ?

impl Process {
    fn init(&mut self, scheduler: &mut Scheduler, _env: &mut Environment) {
        // assert correct variables
        assert_eq!(self.channels.len(), 2);

        //FIXME: first process on the ring gets its channels in opposite order
        let pid = self.process_id;
        if pid == 1 {
            let right = self.channels[0];
            self.channels[0] = self.channels[1];
            self.channels[1] = right;
        }

        println!{"initialized process {:?}", self}
        scheduler.schedule_process_start(NO_DELTA, pid);
    }

    fn start(&mut self, scheduler: &mut Scheduler) {
        println!{"starting process {:?}", self}
        let pid = self.process_id;

        let channel = self.left();
        let msg = Box::new(Message::new_msg(self.uid));
        println!{"\t sending message {:?} on channel {:?}", msg, channel}
        scheduler.schedule_channel_event(NO_DELTA, pid, channel, msg);
    }

    fn receive_msg(&mut self, msg: Box<Message>, channel: ChannelId, scheduler: &mut Scheduler) {
        println!{"executing receive_msg {:?} from channel {:?} process {:?}",msg, channel, self}
        let pid = self.process_id;

        assert_eq!(channel, self.right());

        if let Leader = self.state {
            if let Terminate(leader) = *msg {
                assert_eq!(leader, self.uid, "")
            }
            else {assert!(false)}
            return;
        }

        // TODO: can i somehow <cast> Box<Message> to the value of enum?

        match *msg {
            Message::SendUId(sender) => {
                if sender > self.uid {
                    let channel = self.left();
                    println!("\tsender={:?} > myuid={:?}, sending message to the left on channel {:?}"
                             , sender, self.uid, channel);

                    scheduler.schedule_channel_event(NO_DELTA, pid, channel, msg);
                } else if sender < self.uid {
                    println!("\tsender={:?} < myuid={:?}, discarding message", sender, self.uid)
                } else {
                    let channel = self.left();
                    println!("\tsender={:?} = myuid={:?}, i am leader, sending terminate to the left on channel {:?}"
                             , sender, self.uid, channel);
                    self.state = Leader;
                    let msg = Box::new(Message::new_terminate(self.uid));
                    scheduler.schedule_channel_event(NO_DELTA, pid, channel, msg);
                }
            }
            Terminate(leader) => {
                let channel = self.left();
                println!("\tleader = {:?}, sending terminate to the left on channel {:?}", leader, channel);
                self.state = Terminated(leader);
                scheduler.schedule_channel_event(NO_DELTA, pid, channel, msg);
            }
        }
    }

    fn terminate(&mut self, _env: &mut Environment) {
        println!{"terminating process {:?}", self}
        match self.state {
            Unknown => {assert!(false)}
            // Leader => {
            //     println!("\tI am the leader")
            // }
            _ => {}
        }
    }

    fn left(&self) -> ChannelId {
        self.channels[0]
    }

    fn right(&self) -> ChannelId {
        self.channels[1]
    }
}

// enum ChannelType {
//     In, Out, InOut
// }

#[derive(Debug)]
struct Channel {
    left: ProcessId,
    right: ProcessId,
}

impl Channel {
    pub fn new(p0: ProcessId, p1: ProcessId) -> Self {
        Channel{ left: p0, right: p1 }
    }

    pub fn message_from(&mut self,
                        source: ProcessId,
                        channel: ChannelId,
                        message: Box<Message>,
                        scheduler: &mut Scheduler) {

        let dst : ProcessId;

        if source == self.left {
            dst = self.right;
        } else if source == self.right {
            dst = self.left;
        } else {
            panic! ("unknown source {:?} for channel {:?}", source, self);
        }

        scheduler.schedule_process_event(ROUND_DELTA, dst, channel, message);
    }
}

#[derive(Debug)]
enum Message {
    SendUId(UniqueId),
    Terminate(UniqueId),
}

impl Message {
    fn new_msg(uid: UniqueId) -> Self {Self::SendUId(uid)}
    fn new_terminate(leader: UniqueId) -> Self {Self::Terminate (leader)}
}
