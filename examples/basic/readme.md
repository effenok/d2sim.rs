# Basic Usage of d2sim.rs

d2sim is an event-driven simulator. It simulates components, which are interconnected by channels and can exchange 
messages with each other using channels. This is a simple example to explain main concepts of d2sim.rs.

In this example, there are two components. One component - `Sender`, after sleeping for some time 
sends another compoment a message. Another component - `Receiver` receives a message, and sends back an acknowledgement.

## Creating and Starting Simulation

This simple code snippet shows how to crete a simulation and then run it.

```rust
fn main() {
    println!("Simple Simulation");
    
    // create a simulation
    let mut simulation = Simulation::default();
    
    // explained later
    let mut sender_builder = SenderBuilder::new();
    let mut receiver_builder = ReceiverBuilder::new();
    
    // delay channel delivers message to another proces
    // after a given delay
    let mut channel_builder = DelayChannelBuilder::new();
    
    // add two components, sender and receiver
    // sender_id and receiver_id are component Ids assigned 
    // to the created components. They are needed to create a
    // channel between them
    let sender_id = simulation.add_component(&mut sender_builder);
    let receiver_id = simulation.add_component(&mut receiver_builder);

    // connect components with channel, delay or "travel time" for messages
    // between the components is 1sec 
    simulation.add_channel(channel_builder.delay_sec(1), sender_id, receiver_id);
    
    // initializes the simulation and all compoments 
    // (corresponding method on components will be called)
    simulation.call_init();
    
    // runs the simulation
    simulation.run();
    
    // terminates the components
    simulation.call_terminate();
}
```
The basic pattern is:
- create a simulation instance
- add components to the simulation
- interconnect components with channels
- initialize simulation, which calls corresponding method on all components
- run the simulation
- then terminate simulation, and all components

## Programming Components

This time I will start with an import.

```rust
use std::any::Any;
use rand::{Rng, thread_rng}; // from crate "rand"
use d2simrs::*;
```

### Messages

Let's start the simulation by creating messages. There are no special classes for messages, so it can be anything.

```rust
struct Message { }

struct ACK { }
```

You could have declared an enum if there should be no distinction between a sender and a receiver.

```rust
enum Message {
    Message,
    ACK
}
```

### Sender

Now, let's create a sender. Each component must implement `Component` trait and have to store its component id as 
well as "pointers" to channels. These pointers store ```ChannelId``` of each channel.

```rust
struct Sender {
    // store component id assigned to this component
    sim_id: ComponentId,
    
    // store channel id of one connected channel
    to_receiver: ChannelId
}
```

Variable `sim_id` should be set when the component is created. In the main we used a component builder, which is 
described below. The builder gets the component id and creates the component. 

Channels are added after component is created.

Usually there will be several channels. Component could for example store them in a vector. 

Here is a snippet for component methods. Next we will walk over them and implement them.
```rust
impl Component for Sender {
    
    fn sim_id(&self) -> ComponentId {
        todo!()
    }

    fn add_channel(&mut self, channel_id: ChannelId, label: ChannelLabel) {
        todo!()
    }

    fn init(&mut self) {
        todo!()
    }

    fn process_event(&mut self, sender: ComponentId, event: Box<dyn Any>) {
        todo!()
    }

    fn receive_msg(&mut self, incoming_channel: ChannelId, msg: Box<dyn Any>) {
        todo!()
    }

    fn terminate(&mut self) {
        todo!()
    }
}
```

**(1)** `sim_id` returns component id. it is simple

```rust
    fn sim_id(&self) -> ComponentId {
        self.sim_id
    }
```

**(2)** `add_channel` is called after channel is created (whe we called `create_channel` from main). ChannelLabel is 
either 
`left` or `right`, where left will be the first compoenet passed to `create_channel` method and `right` will be the 
second. You can use this label, if the order is important, but it usually is not. Let's ignore it by changing the 
parameter to `_label`. 

```rust
    fn add_channel(&mut self, channel_id: ChannelId, _label: ChannelLabel) {
        // check that this component is only attached to this channel
        assert!(!self.to_receiver.is_initialized());
        self.to_receiver = channel_id;
    }
```

**(3)** `init` is called from `simulation.call_init()`. Common usage is to wake component up at some time in the 
simulation. Here, the component can also do anything that needs to be done after all channels are created. Our 
sender wants to "wake up" and send one message at some random time.

```rust
    fn init(&mut self) {
        // generate a random duration (this is done using `rand` crate)
        let sec = thread_rng().gen_range(0..10);
        // convert it to SimTimeDelta, which is a wrapper around std::time::Duration
        let sim_delay = SimTimeDelta::from(std::time::Duration::from_secs(sec));
        // add event to the simulation scheduler
        sim_sched().sched_self_event(sim_delay, self.sim_id);
    }
```

Simulation scheduler is a singleton, that can be accessed using `sim_sched()` method. It contains methods to add new 
events (as in discrete *event* simulation). In this case we want to schedule an event that is delivered to this 
component after `sec` seconds. Events can also have objects, passed around, but in this case we do not need one.

If we wanted to start a component at '0' simulation time, we could use the `NO_DELTA` constant.
```rust 
sim_sched().sched_self_event(NO_DELTA, self.sim_id);
```

**(4)** `process_event` is called when an event needs to be delivered to the component. Here, we need to process the 
event, that we sent us in `init`. We did not send any associated object, so we do not need `_event` variable. 

Another common use-case for this method is to implement timers. 

```rust
    fn process_event(&mut self, sender: ComponentId, _event: Box<dyn Any>) {
        // this is event that we have sent ourselves in init
        // when it fires, send message to the receiver
    
        println!("[{}s][sender] sending message to receiver", sim_time().as_secs());
    
        let msg = Box::new(Message {});
        sim_sched().send_msg(self.sim_id, self.to_receiver, msg);
    }
```
Method `sim_time()` returns current simulation time. Logging is currently a bit ugly and will be improved in future 
versions.

**(5)** `receive_msg` is called when messages are delivered by channels. In our example, sender sends message to the 
receiver and receiver sends back an ACK. Thus, this method at the sender needs to receive an ACK.

```rust
    fn receive_msg(&mut self, _incoming_channel: ChannelId, msg: Box<dyn Any>) {
        if let Ok(_) = msg.downcast::<ACK>() {
            println!("[{}s][sender] received ACK", sim_time().as_secs());
        } else {
            assert!(false);
        }
    }
```

**(6)** `terminate` is called after simulation is completed. Well, it is called when `simulation.call_terminate()` 
is called, and simulation cannot run after this. In our example, this method has no use.

In more complex examples, this method can be use to verify that the component is in the correct state, or to gather 
statistics.

```rust
    fn terminate(&mut self) {
    }
```

#### SenderBuilder 

To create a component, simulation is passed a so called `SenderBuilder`. It is an object that implements trait 
`ComponentBuilder`, which creates a component with provided `ComponentId`. 

```rust
pub struct SenderBuilder{}

impl SenderBuilder {
    pub fn new() -> Self { Self {} }
}

impl ComponentBuilder for SenderBuilder {
    fn build_component(&mut self, id: ComponentId) -> Box<dyn Component> {
        Box::new(Sender {
            sim_id: id,
            to_receiver: ChannelId::default()
        })
    }
}
```

### Receiver

Receiver is mostly identical, so we will only show changed parts. 

Receiver does not need to wake up, because it is activated by sender when it receives a message. When it receives 
the message, it sends back an ACK. Here are the relevant parts.

```rust
impl Component for Receiver {
    
    fn init(&mut self) {
        // do nothing
    }

    fn process_event(&mut self, sender: ComponentId, event: Box<dyn Any>) {
        // do nothing
    }

    fn receive_msg(&mut self, _incoming_channel: ChannelId, msg: Box<dyn Any>) {
        if let Ok(_) = msg.downcast::<Message>() {
            println!("[{}s][receiver] received message, sending ACK", sim_time().as_secs());

            let ack = Box::new(ACK {});
            sim_sched().send_msg(self.sim_id, self.to_receiver, ack);
        } else {
            assert!(false);
        }
    }

// other methods
}
```

Receiver also needs a receiver builder created.

```rust
struct ReceiverBuilder{}

impl ComponentBuilder for ReceiverBuilder { 
    // code
}
```

## Running Simulation

Running main will produce something like this:

```
Simple Simulation

Initializing simulation: #components 2

Running simulation
[9s][sender] sending message to receiver
[10s][receiver] received message, sending ACK
[11s][sender] received ACK

Simulation completed in SimTime { time: 11s } time units
```