use std::any::Any;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::collections::BinaryHeap;

use crate::environment::Environment;
use crate::keys::{ChannelId, ComponentId};
use crate::simtime::{SimTime, SimTimeDelta, NO_DELTA};

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
    MsgRcvEvent(MessageRcvEvent),
    EndSimulation,
}

#[derive(Debug)]
struct ScheduledEvent
{
    time: SimTime,
    event: EventType,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering
    {
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

pub enum SimStatus {
    Ok,  Failure
}

pub struct Scheduler
{
    events: BinaryHeap<ScheduledEvent>,
    curr_time: SimTime,
    pub(crate) env: Environment,
    sim_status: SimStatus,
}

impl Scheduler
{
    pub fn new() -> Self {
        Scheduler { events: BinaryHeap::default(), curr_time: SimTime::default(), env: Environment::default(), sim_status: SimStatus::Ok}
    }

    pub fn get_curr_time(&self) -> &SimTime {
        return &self.curr_time;
    }

    pub fn next_event(&mut self) -> EventType {

        if let SimStatus::Failure = self.sim_status {
            return EventType::EndSimulation;
        }

        let event = self.events.pop();

        if event.is_none() {
            return EventType::EndSimulation;
        }

        let event = event.unwrap();

        // updaate time
        self.curr_time.advance_to (event.time);

        return event.event;
    }

    pub fn send_msg_delayed(&mut self, timedelta: SimTimeDelta, sender: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent { time, event: EventType::MsgSendEvent(
            MessageSendEvent { sender, channel, message }
        )};
        self.events.push(event);
    }

    pub fn send_msg(&mut self, sender: ComponentId, channel: ChannelId, message: Box<dyn Any>){
        self.send_msg_delayed(NO_DELTA, sender, channel, message);
    }

    pub fn sched_receive_msg(&mut self, timedelta: SimTimeDelta, receiver: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent { time, event: EventType::MsgRcvEvent(
            MessageRcvEvent {channel, receiver, message}
        )};
        self.events.push(event);
    }

    pub fn sched_self_event(&mut self, timedelta: SimTimeDelta, process: ComponentId) {
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

    pub fn sched_self_event1(&mut self, timedelta: SimTimeDelta, process: ComponentId, event: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent { time, event: EventType::ProcessEvent(
            ProcessEvent {
                sender: process,
                receiver: process,
                event: event,
            }
        )};
        // eprintln!("\t\t\tcreated event = {:?}", event);
        self.events.push(event);
    }

    pub fn sim_error(&mut self) {
        self.sim_status = SimStatus::Failure;
    }
}