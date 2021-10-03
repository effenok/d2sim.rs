use std::any::Any;

#[derive(Debug)]
pub struct Packet {
    pub stack: Vec<Box<dyn Any>>,
}

impl Packet {
    pub fn new_box() -> Box<Self> {
        Box::new(Packet {
            stack: Vec::new()
        })
    }

    pub fn create_and_wrap(packet: Box<dyn Any>) -> Box<Self> {
        let mut b = Box::new(Packet {
            stack: Vec::new()
        });
        b.add_packet(packet);
        b
    }

    pub fn add_packet(&mut self, packet: Box<dyn Any>) {
        self.stack.push(packet);
    }

    pub fn unwrap<T: 'static>(&self, idx: usize) -> &T {
        assert!(idx < self.stack.len());

        self.stack[idx].downcast_ref::<T>().unwrap()
    }
}