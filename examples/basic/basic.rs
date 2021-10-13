
// messages -------------------

use std::any::Any;
use rand::{Rng, thread_rng};
use d2simrs::*;

struct Message {}
struct ACK {}

// sender -------------------

struct Sender {
    sim_id: ComponentId,
    to_receiver: ChannelId,
}

impl Component for Sender {

    fn sim_id(&self) -> ComponentId {
        self.sim_id
    }

    fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        // check that this component is only attached to this channel
        assert!(!self.to_receiver.is_initialized());
        self.to_receiver = channel_id;
    }

    fn init(&mut self) {
        // send a message after somewhere between 0 and 10 seconds
        let sec = thread_rng().gen_range(0..10);
        let sim_delay = SimTimeDelta::from(std::time::Duration::from_secs(sec));

        sim_sched().sched_self_event(sim_delay, self.sim_id);
    }

    fn process_event(&mut self, sender: ComponentId, _event: Box<dyn Any>) {
        // this is event that we have sent ourselves in init
        assert_eq!(sender, self.sim_id);

        println!("[{}s][sender] sending message to receiver", sim_time().as_secs());

        let msg = Box::new(Message {});
        sim_sched().send_msg(self.sim_id, self.to_receiver, msg);
    }

    fn receive_msg(&mut self, _incoming_channel: ChannelId, msg: Box<dyn Any>) {
        if let Ok(_) = msg.downcast::<ACK>() {
            println!("[{}s][sender] received ACK", sim_time().as_secs());
        } else {
            assert!(false);
        }
    }

    fn terminate(&mut self) {
        println!("bye");
    }
}

pub struct SenderBuilder{}

impl SenderBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl ComponentBuilder for SenderBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        Box::new(Sender {
            sim_id: id,
            to_receiver: ChannelId::default()
        })
    }
}

// receiver -------------------

struct Receiver {
    sim_id: ComponentId,
    to_sender: ChannelId,
}

impl Component for Receiver {

    fn sim_id(&self) -> ComponentId {
        self.sim_id
    }

    fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        assert!(!self.to_sender.is_initialized());
        self.to_sender = channel_id;
    }

    fn init(&mut self) {
        // do nothing
    }

    fn process_event(&mut self, _sender: ComponentId, _event: Box<dyn Any>) {
        // do nothing
    }

    fn receive_msg(&mut self, _incoming_channel: ChannelId, msg: Box<dyn Any>) {
        if let Ok(_) = msg.downcast::<Message>() {
            println!("[{}s][receiver] received message, sending ACK", sim_time().as_secs());

            let ack = Box::new(ACK {});
            sim_sched().send_msg(self.sim_id, self.to_sender, ack);
        } else {
            assert!(false);
        }
    }

    fn terminate(&mut self) {
        // do nothing
    }
}

pub struct ReceiverBuilder{}

impl ReceiverBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl ComponentBuilder for ReceiverBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        Box::new(Receiver {
            sim_id: id,
            to_sender: ChannelId::default()
        })
    }
}