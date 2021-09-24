mod asynchlcr;

use grappy::simbase::Simulation;
use grappy::simbase::Components;
use grappy::keys::ComponentId;
use grappy::channel::asynch::AsynchChannelBuilder;
use crate::asynchlcr::ProcessBuilder;

// fn validate<'a>(components: &Components) -> bool {
// 	true
// 	// false
// }

fn main() {
	println!("LCR Algorithm for Leader Election in Rings");

    let mut simulation = Simulation::default();
	let mut process_builder = ProcessBuilder::new(100);

	const NUM_NODES: usize = 10;
	let mut nodes: Vec<ComponentId> = Vec::with_capacity(NUM_NODES);

	// create nodes
	for _ in 0..NUM_NODES {
		let node = simulation.add_component(&mut process_builder);
		nodes.push(node);
		// println!("created node {:?}", nodes[n]);
	}

	let mut builder = AsynchChannelBuilder::default();
	let delay1ms = std::time::Duration::from_millis(1);

	// connect nodes by channels in a ring
	for idx0 in 0..NUM_NODES {
		let idx1  = (idx0 + 1) % NUM_NODES;

		let p0 = nodes[idx0];
		let p1 = nodes[idx1];

		simulation.add_channel(builder.with_delay(delay1ms), p0, p1);
		// println!("created channel {:?} between processes {:?} and {:?}", c, p0, p1);
	}

	simulation.call_init();
	
	// simulation.start();
	simulation.run();

	simulation.call_terminate();

	// TODO:
	// simulation.validate(validate);
}
