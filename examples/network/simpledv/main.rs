use d2simrs::channels::delay_channel::DelayChannelBuilder;
use d2simrs::keys::ComponentId;
use d2simrs::sim::Simulation;
use d2simrs::simtime::SimTime;
use d2simrs::simtime::SimTimeDelta;
use std::time::Duration;

use crate::builder::{ComponentType, NetworkComponentBuilder};

mod simpledv;

mod layer3;
mod router;
mod builder;
mod types;

fn main() {
    println!("Simple Distance-Vector Algorithm");

    let mut simulation = Simulation::<DelayChannelBuilder>::default();
    let mut component_builder = NetworkComponentBuilder::new();
    let mut channel_builder = DelayChannelBuilder::default();
    let delay1ms = std::time::Duration::from_millis(1);
    channel_builder.with_delay(delay1ms);

    const NUM_NODES: usize = 7;

    let mut nodes: Vec<ComponentId> = Vec::with_capacity(NUM_NODES);

    // create one host
    {
        let node = simulation.add_component(&mut component_builder);
        nodes.push(node);
    }

    component_builder.set_next_component(ComponentType::Router);

    // create nodes
    for _ in 1..NUM_NODES {
        let node = simulation.add_component(&mut component_builder);
        nodes.push(node);
    }

    /*
    h -- r1
    r1 -- r2
    r1 -- r3
    r2 -- r3
    r3 -- r4
    r3 -- r5
    r2 -- r6
    r4 -- r6
     */

    simulation.add_channel(&mut channel_builder, nodes[0], nodes[1]);
    simulation.add_channel(&mut channel_builder, nodes[1], nodes[2]);
    simulation.add_channel(&mut channel_builder, nodes[1], nodes[3]);
    simulation.add_channel(&mut channel_builder, nodes[2], nodes[3]);
    simulation.add_channel(&mut channel_builder, nodes[3], nodes[4]);
    simulation.add_channel(&mut channel_builder, nodes[3], nodes[5]);
    simulation.add_channel(&mut channel_builder, nodes[2], nodes[6]);
    simulation.add_channel(&mut channel_builder, nodes[4], nodes[6]);

    simulation.call_init();

    const DUR_1MIN: SimTimeDelta = SimTimeDelta::from(Duration::from_secs(60));

    let sim_result = simulation.run_until(SimTime::default() + DUR_1MIN);

    simulation.call_terminate();

    match sim_result {
        Result::Ok(_) => {
            println!("Simulation completed successfully");
        },
        Result::Err(_) => {
            eprintln!("Simulation was aborted due to error");
        }
    }
}
