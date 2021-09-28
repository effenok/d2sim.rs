# d2sim.rs

'double-d' sim or Discrete event SIMulator for Distributed systems in RuSt.

similar to OmNet++ simulator (only the core part))

currently, in development

## Progress

currently, can run simple algorithms for asynchronous networks and synchronous networks where processes only awake 
on incoming messages. 

See examples of how to use the simulator:

#### Examples: 

 * synch - synchronous networks
   * lcr_leader_election (LCR algorithm for leader election in synchronous rings) => example synchronous process
 * asynchnetwork - general asynchronous networks
   * lcr_leader_election => example asynchronous process
   * spanning_tree (basic spanning tree with pre-defined root) => asynchronous process and random topology 