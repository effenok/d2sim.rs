use grappy::channel::synch::SynchChannelBuilder;
use grappy::component::Component;
use grappy::keys::ComponentId;
use grappy::simbase::{Simulation};
// use std::any::Any;

use crate::lcr::{ProcessBuilder};

mod lcr;

fn validate<'a>(components: &[Box<dyn Component>]) -> bool {
	// checking that there is a leader
	// there is a unique leader
	// leader is largest id
	// let mut process_in_state_leader: usize = 0;
	// let mut leader_var_in_followers: usize = 0;
	// let mut max_uid : usize = 0;

	for _c in components{
		// eprintln!("c.type_id() = {:?}", (*c).type_id());

		// let process = (*c).downcast::<Process>();

		//
		// // eprintln!("iterval = {:?}", process);
		// max_uid = cmp::max(max_uid, process.uid.0);
		//
		// match process.state {
		// 	ProcessState::Leader => {
		// 		if process_in_state_leader != 0 {
		// 			assert!(false, "found second leader {:?}, previous leader uid {}", process, process_in_state_leader);
		// 			return false;
		// 		}
		// 		process_in_state_leader = process.uid.0;
		//
		// 		if leader_var_in_followers != 0 && leader_var_in_followers != process_in_state_leader {
		// 			println!("other process has a different leader  {}", leader_var_in_followers);
		// 			return false;
		// 		}
		// 	},
		// 	ProcessState::Terminated(leader) => {
		// 		if leader_var_in_followers != 0 && leader_var_in_followers != leader.0 {
		// 			println!("other process has a different leader  {}", leader_var_in_followers);
		// 			return false;
		// 		}
		//
		// 		leader_var_in_followers = leader.0;
		//
		// 		if process_in_state_leader != 0 && process_in_state_leader != leader_var_in_followers {
		// 			assert!(false, "other process has declared itself leader  {} {} {:?}"
		// 					, process_in_state_leader, leader_var_in_followers, leader);
		// 			return false;
		// 		}
		// 	},
		// 	_ => { return false;}
		// }
	}

	// assert_eq!(max_uid, process_in_state_leader);

	true
	// false
}

// fn find_min<'a, I>(vals: I) -> Option<&'a u32>
// 	where
// 		I: Iterator<Item = &'a u32>,
// {
// 	//todo
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

	// connect nodes by channels in a ring

	let mut channel_builder = SynchChannelBuilder {};

	for idx0 in 0..NUM_NODES {
		let idx1  = (idx0 + 1) % NUM_NODES;

		let p0 = nodes[idx0];
		let p1 = nodes[idx1];

		simulation.add_channel(&mut channel_builder, p0, p1);
		// println!("created channel {:?} between processes {:?} and {:?}", c, p0, p1);
	}

	simulation.call_init();
	
	// simulation.start();
	simulation.run();

	simulation.call_terminate();

	// TODO:
	// simulation.validate(validate);
}