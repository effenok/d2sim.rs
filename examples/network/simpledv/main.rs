use std::time::Duration;

use d2simrs::basicnet::InterfaceId;
use d2simrs::channels::delay_channel::DelayChannel;
use d2simrs::keys::{ComponentId, DUMMY_COMPONENT};
use d2simrs::sim::Simulation;
use d2simrs::simtime::SimTime;
use d2simrs::simtime::SimTimeDelta;
use d2simrs::simvars::sim_sched;

use crate::builder::NetworkBuilder;
use crate::router::{InterfaceEvent, InternalEvent};

mod simpledv;
mod router;
mod builder;
mod types;

fn main() {
    println!("Simple Distance-Vector Algorithm");

    let mut simulation = Simulation::<DelayChannel>::default();
    let mut network_builder = NetworkBuilder::new();

    const NUM_NODES: usize = 8;

    // first node is a host
    network_builder.add_host();

    // next NUM_NODES - 2 are routers
    for _ in 1..NUM_NODES {
        network_builder.add_router();
    }

    // edges:
    network_builder.add_link(0, 1);
    network_builder.add_link(1, 2);
    network_builder.add_link(2, 3);
    // simulation.add_channel(&mut channel_builder, nodes[2], nodes[3]);
    // simulation.add_channel(&mut channel_builder, nodes[2], nodes[4]);
    // simulation.add_channel(&mut channel_builder, nodes[3], nodes[4]);
    // // simulation.add_channel(&mut channel_builder, nodes[3], nodes[5]);
    // // simulation.add_channel(&mut channel_builder, nodes[3], nodes[6]);
    // // simulation.add_channel(&mut channel_builder, nodes[4], nodes[7]);
    // // simulation.add_channel(&mut channel_builder, nodes[6], nodes[7]);


    network_builder.debug();
    network_builder.build_sim(&mut simulation);

    // eprintln!("network_builder = {:?}", network_builder);
    // 
    // let mut component_builder = NetworkComponentBuilder::new();
    // let mut channel_builder = DelayChannelBuilder::default();
    // let delay1ms = std::time::Duration::from_millis(1);
    // channel_builder.with_delay(delay1ms);
    // 
    // const NUM_NODES: usize = 8;
    // 
    // let mut nodes: Vec<ComponentId> = Vec::with_capacity(NUM_NODES);
    // 
    // // create one host
    // {
    //     let node = simulation.add_component(&mut component_builder);
    //     nodes.push(node);
    // }
    // 
    // component_builder.set_next_component(ComponentType::Router);
    // 
    // // create nodes
    // for _ in 1..NUM_NODES {
    //     let node = simulation.add_component(&mut component_builder);
    //     nodes.push(node);
    // }
    // 
    // /*
    // h -- r1
    // r1 -- r2
    // r2 -- r3
    // r2 -- r4
    // r3 -- r4
    // r3 -- r5
    // r3 -- r6
    // r4 -- r7
    // r6 -- r7
    //  */
    // 
    // simulation.add_channel(&mut channel_builder, nodes[0], nodes[1]);
    // simulation.add_channel(&mut channel_builder, nodes[1], nodes[2]);
    // simulation.add_channel(&mut channel_builder, nodes[2], nodes[3]);
    // simulation.add_channel(&mut channel_builder, nodes[2], nodes[4]);
    // simulation.add_channel(&mut channel_builder, nodes[3], nodes[4]);
    // // simulation.add_channel(&mut channel_builder, nodes[3], nodes[5]);
    // // simulation.add_channel(&mut channel_builder, nodes[3], nodes[6]);
    // // simulation.add_channel(&mut channel_builder, nodes[4], nodes[7]);
    // // simulation.add_channel(&mut channel_builder, nodes[6], nodes[7]);
    // 
    simulation.call_init();

    // turn off interface fi_1 at router 2 at 30 sec
    const DUR_30SEC: SimTimeDelta = SimTimeDelta::from(Duration::from_secs(30));
    const DUR_1SEC: SimTimeDelta = SimTimeDelta::from(Duration::from_secs(1));

    // this can be used to test hello
    // send_interface_down_event(nodes[2], 1, DUR_30SEC);
    // this should cause count-to-infinity without poison reverse
    // send_interface_down_event(nodes[2], 0, DUR_30SEC);

    // stop simulation after one minute simulation time
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

fn send_interface_down_event(to: ComponentId, which: usize, at: SimTimeDelta) {
    let ev = InterfaceEvent { interface: InterfaceId::from(which), down: true };

    sim_sched().sched_component_event(at,
                                      DUMMY_COMPONENT,
                                      to,
                                      Box::new(InternalEvent::InterfaceEvent(ev)));
}
