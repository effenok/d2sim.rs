use std::any::Any;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::collections::BinaryHeap;

use crate::environment::Environment;
use crate::keys::{ChannelId, ComponentId};
use crate::simtime::{NO_DELTA, SimTime, SimTimeDelta};

#[derive(Debug)]
pub struct ComponentEvent {
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
    ProcessEvent(ComponentEvent),
    MsgSendEvent(MessageSendEvent),
    MsgRcvEvent(MessageRcvEvent),
    EndSimulation,
}

#[derive(Debug)]
struct ScheduledEvent
{
    time: SimTime,
    index: usize,
    event: EventType,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering
    {
        let mut ord = self.time.cmp(&other.time);
        if let Ordering::Equal = ord {
            ord = self.index.cmp(&other.index);
        }

        ord.reverse()
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.index == other.index
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
    next_event: usize,
}

impl Scheduler
{
    pub fn new() -> Self {
        Scheduler {
            events: BinaryHeap::default(),
            curr_time: SimTime::default(),
            env: Environment::default(),
            sim_status: SimStatus::Ok,
            next_event: 0,
        }
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
        self.curr_time.advance_to(event.time);

        return event.event;
    }

    pub fn sim_status(&self) -> Result<(), ()> {
        match self.sim_status {
            SimStatus::Ok => Result::Ok(()),
            SimStatus::Failure => Result::Err(())
        }
    }

    pub fn send_msg_delayed(&mut self, timedelta: SimTimeDelta, sender: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent {
            time,
            index: self.next_event,
            event: EventType::MsgSendEvent(
                MessageSendEvent { sender, channel, message }
            ),
        };
        self.push_event(event);
    }

    pub fn send_msg(&mut self, sender: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        self.send_msg_delayed(NO_DELTA, sender, channel, message);
    }

    pub fn sched_receive_msg(&mut self, timedelta: SimTimeDelta, receiver: ComponentId, channel: ChannelId, message: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent {
            time,
            index: self.next_event,
            event: EventType::MsgRcvEvent(
                MessageRcvEvent { channel, receiver, message }
            ),
        };
        self.push_event(event);
    }

    pub fn sched_self_event(&mut self, timedelta: SimTimeDelta, component: ComponentId) {
        self.sched_component_event(timedelta, component,  component,Box::new(()));
    }

    pub fn sched_self_event_with_data(&mut self, timedelta: SimTimeDelta, component: ComponentId, event: Box<dyn Any>) {
        self.sched_component_event(timedelta, component,  component,event);
    }

    pub fn sched_component_event(&mut self, timedelta: SimTimeDelta, sender: ComponentId, receiver: ComponentId, event: Box<dyn Any>) {
        let time = self.curr_time + timedelta;
        let event = ScheduledEvent {
            time,
            index: self.next_event,
            event: EventType::ProcessEvent(
                ComponentEvent {
                    sender: sender,
                    receiver: receiver,
                    event: event,
                }
            ),
        };
        self.push_event(event);
    }

    pub fn sim_error(&mut self) {
        self.sim_status = SimStatus::Failure;
    }

    fn push_event(&mut self, event: ScheduledEvent) {
        self.events.push(event);
        self.next_event += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_order1() {
        const NUM_EV: usize = 5; // 3 does not work

        let time = SimTime::default();
        let process = ComponentId::new(1);
        let mut sched = Scheduler::new();

        let idx_to_val = |idx| { 2 * idx };

        for idx in 0..NUM_EV {
            let event1 = crete_event(time, process, idx, idx_to_val(idx));
            sched.events.push(event1);
        }

        // ----------------------------------------

        for idx in 0..NUM_EV {
            // peak the next item and assert its index
            let peaked_ev = sched.events.peek().unwrap();
            assert_eq!(idx, peaked_ev.index);
            eprintln!("peaked_ev = {:?}", peaked_ev);

            // get the next item and assert its value
            let popped_ev = sched.next_event();
            assert!(matches!(popped_ev, EventType::ProcessEvent(_)));
            let popped_num = unwrap_process_event(popped_ev);
            assert_eq!(idx_to_val(idx), popped_num);
            eprintln!("popped_ev = {:?}", popped_num);
        }

        assert!(matches!(sched.next_event(), EventType::EndSimulation));
    }

    fn unwrap_process_event(popped_ev: EventType) -> usize {
        let ev = match popped_ev {
            EventType::ProcessEvent(ev) => { Some(ev) },
            _ => { None }
        }.unwrap();
        let num = ev.event.downcast::<usize>().unwrap();
        *num
    }

    fn crete_event(time: SimTime, process: ComponentId, ev_idx: usize, ev_value: usize) -> ScheduledEvent {
        ScheduledEvent {
            time,
            index: ev_idx,
            event: EventType::ProcessEvent(
                ComponentEvent {
                    sender: process,
                    receiver: process,
                    event: Box::new(ev_value),
                }
            ),
        }
    }
}