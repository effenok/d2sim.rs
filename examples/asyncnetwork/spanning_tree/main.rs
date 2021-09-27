mod spanning_tree;

use d2simrs::sim::Simulation;
use d2simrs::topo::anchoredrandomgraphbuilder::AnchoredRandomGraphGen;
use crate::spanning_tree::ProcessBuilder;
use crate::spanning_tree::RandomDelayChannelBuilder;


fn main() {
	println!("Spanning Tree in Asynchronous Networks");

    let mut simulation = Simulation::<RandomDelayChannelBuilder>::default();
	let mut process_builder = ProcessBuilder::new(100);
	let mut channel_builder = RandomDelayChannelBuilder::default();

	let mut gen = AnchoredRandomGraphGen::new(10, 0.2);
	let mut topo = gen.generate_connected_graph(10);

	simulation.build_from_topo(&mut topo, &mut process_builder, &mut channel_builder);

	// let mut builder = AsynchChannelBuilder::default();

	// let mut nb = MNetworkBuilder::new();
	// nb.tmp1();
	// nb.build_sim(&mut simulation);



	// const NUM_NODES: usize = 10;
	// let mut nodes: Vec<ComponentId> = Vec::with_capacity(NUM_NODES);
	//
	// // create nodes
	// for _ in 0..NUM_NODES {
	// 	let node = simulation.add_component(&mut process_builder);
	// 	nodes.push(node);
	// 	// println!("created node {:?}", nodes[n]);
	// }
	//
	// let mut builder = AsynchChannelBuilder::default();
	// let delay1ms = std::time::Duration::from_millis(1);
	//
	// // create random channels with
	//
	// // connect nodes by channels in a ring
	// for idx0 in 0..NUM_NODES {
	// 	let idx1  = (idx0 + 1) % NUM_NODES;
	//
	// 	let p0 = nodes[idx0];
	// 	let p1 = nodes[idx1];
	//
	// 	simulation.add_channel(builder.with_delay(delay1ms), p0, p1);
	// 	// println!("created channel {:?} between processes {:?} and {:?}", c, p0, p1);
	// }

	simulation.call_init();
	
	simulation.run();

	simulation.call_terminate();

	// TODO:
	// simulation.validate(validate);
}
