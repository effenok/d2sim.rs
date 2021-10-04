use std::any::Any;
use std::cell::Cell;

#[derive(Debug)]
pub struct Packet {
    pub stack: Vec<Box<dyn Any>>,
    pub next_header: Cell<Option<usize>>, // TODO: pass mutable references when parsing ?
}

impl Packet {
    pub fn new_box() -> Box<Self> {
        Box::new(Packet {
            stack: Vec::new(),
            next_header: Cell::new(None),
        })
    }

    pub fn create_and_wrap(packet: Box<dyn Any>) -> Box<Self> {
        let mut b = Packet::new_box();
        b.add_packet(packet);
        b
    }

    pub fn add_packet(&mut self, packet: Box<dyn Any>) {
        self.stack.push(packet);
    }

    pub fn unwrap_first<T: 'static>(&self) -> &T {
        assert!(self.stack.len() >= 1);

        if self.stack.len() >= 2 {
            self.next_header.replace(Some(self.stack.len() - 2));
        }

        self.stack[self.stack.len() - 1].downcast_ref::<T>().unwrap()
    }

    pub fn has_next(&self) -> bool {
        self.next_header.get().is_some()
    }

    pub fn unwrap_next<T: 'static>(&self) -> Option<&T> {
        let next_header = self.next_header.get()?;

        // set next header
        if next_header > 0 {
            self.next_header.replace(Some(next_header - 1));
        } else {
            self.next_header.replace(None);
        }

        Some(self.stack[next_header].downcast_ref::<T>()?)
    }

    // TODO: only in debug?
    pub fn is_first_of_type<T: 'static>(&self) -> bool {
        if self.stack.len() == 0 { return false };

        self.stack[self.stack.len() - 1].is::<T>()
    }
}