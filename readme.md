# d2sim.rs

'double-d' sim or Discrete event SIMulator for Distributed systems in RuSt.

similar to OmNet++ simulator (only the core part))

currently, in development

## How To Use

[see the most basic exampe here](examples/basic/readme.md)

## Progress

currently, can run simple algorithms for asynchronous networks and synchronous networks where processes only awake 
on incoming messages. There is a basic code for simulating basic networks, but it is under construction as it is 
currently not idiomatic Rust.

See examples of how to use the simulator:

## Examples: 

 * asynchnetwork - general asynchronous networks
   * lcr_leader_election => example asynchronous process
   * spanning_tree (basic spanning tree with pre-defined root) => asynchronous process and random topology
 * synch - synchronous networks
   * lcr_leader_election (LCR algorithm for leader election in synchronous rings) => example synchronous process
