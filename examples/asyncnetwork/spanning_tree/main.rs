mod spanning_tree;

use d2simrs::sim::Simulation;
use d2simrs::topo::anchoredrandomgraph::AnchoredRandomGraphGen;
use crate::spanning_tree::ProcessBuilder;
use crate::spanning_tree::RandomDelayChannelBuilder;
use d2simrs::topo::topogen::TopologyGenerator;
use d2simrs::topo::topo::Topology;


fn main() {
	println!("Spanning Tree in Asynchronous Networks");

    let mut simulation = Simulation::<RandomDelayChannelBuilder>::default();
	let mut process_builder = ProcessBuilder::new(100);
	let mut channel_builder = RandomDelayChannelBuilder::default();

	const NUM_NODES: usize = 10;
	let gen = AnchoredRandomGraphGen::new(NUM_NODES, 0.2);
	let mut topo = TopologyGenerator::new(NUM_NODES, gen).try_build_connected_network();

	simulation.build_from_topo(&mut topo, &mut process_builder, &mut channel_builder);

	simulation.call_init();
	
	simulation.run();

	simulation.call_terminate();

	Topology::dot(&mut topo);
}
