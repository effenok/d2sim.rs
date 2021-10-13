use d2simrs::*;

use crate::basic::ReceiverBuilder;
use crate::basic::SenderBuilder;

mod basic;

fn main() {
	println!("Simple Simulation");

	let mut simulation = Simulation::default();

	let mut sender_builder = SenderBuilder::new();
	let mut receiver_builder = ReceiverBuilder::new();
	let mut channel_builder = DelayChannelBuilder::new();

	let sender_id = simulation.add_component(&mut sender_builder);
	let receiver_id = simulation.add_component(&mut receiver_builder);

	simulation.add_channel(channel_builder.delay_sec(1), sender_id, receiver_id);

	simulation.call_init();

	simulation.run();

	simulation.call_terminate();
}
