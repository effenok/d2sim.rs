use crate::keys::{ComponentId, ChannelId};
use std::collections::BinaryHeap;
use std::any::Any;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct ProcessEvent {
    pub sender: ComponentId,
    pub receiver: ComponentId,
    pub event: Box<dyn Any>,
}

#[derive(Debug)]
pub struct MessageSendEvent {
    pub sender: ComponentId,
    pub channel:ChannelId,
    pub message: Box<dyn Any>,
}


#[derive(Debug)]
pub struct MessageRcvEvent {
    pub channel:ChannelId,
    pub receiver: ComponentId,
    pub message: Box<dyn Any>,
}

#[derive(Debug)]
pub enum EventType {
    ProcessEvent(ProcessEvent),
    MsgSendEvent(MessageSendEvent),
    MsgRcvEvent(MessageRcvEvent)
}

#[derive(Debug)]
pub struct ScheduledEvent {
    pub time: usize,
    pub event: EventType,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time.cmp(&other.time).reverse())
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for ScheduledEvent {}

// impl Debug for Event {
// 	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
// 		write!(f, "Event {{ time: {:?}, sender: {:?}, channel: {:?}, receiver: {:?}, message: {:p} }}"
// 			   , self.time, self.sender, self.channel, self.receiver, self.message)
// 	}
// }

pub const NO_DELTA: usize = 0;
pub const ROUND_DELTA: usize = 1;

pub struct RoundScheduler {
    pub events: BinaryHeap<ScheduledEvent>,
    pub curr_time: usize,
}

impl RoundScheduler {
    pub fn new() -> Self {
        RoundScheduler { events: BinaryHeap::default(), curr_time: 0}
    }

    pub fn sched_send_msg(&mut self, timedelta: usize, sender: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent { time, event: EventType::MsgSendEvent(
            MessageSendEvent { sender, channel, message }
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }

    pub fn sched_receive_msg(&mut self, timedelta: usize, receiver: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent { time, event: EventType::MsgRcvEvent(
            MessageRcvEvent {channel, receiver, message}
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }

    pub fn sched_self_event(&mut self, timedelta: usize, process: ComponentId) {
        assert_eq!(self.curr_time, 0);

        let time = self.curr_time + timedelta;
        let event = ScheduledEvent { time, event: EventType::ProcessEvent(
            ProcessEvent {
                sender: process,
                receiver: process,
                event: Box::new(std::ptr::null::<usize>())
            }
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }
}
