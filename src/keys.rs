use crate::component::Component;
use std::marker::PhantomData;
use crate::channel::Channel;

// ChannelId ---------------------------------------------------

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct ChannelId {
    id: usize,
    _marker: PhantomData<dyn Channel>
}

impl Default for ChannelId {
    fn default() -> Self {
        ChannelId {
            id: std::usize::MAX,
            _marker: PhantomData,
        }
    }
}

impl ChannelId {
    pub fn as_idx(&self) -> usize {
        self.id
    }

    // TODO: FROM index
    pub fn new(id: usize) -> Self {
        ChannelId {
            id,
            _marker: PhantomData,
        }
    }

    pub fn is_initialized (&self) -> bool {
        self.id != std::usize::MAX
    }
}

impl std::fmt::Debug for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.id != std::usize::MAX {
            write!(f, "({})", self.id)
        } else {
            write!(f, "(uninitialized)")
        }
    }
}

// ComponentId ---------------------------------------------------

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ComponentId {
    id: usize,
    _marker: PhantomData<dyn Component>
}

impl ComponentId {
    pub fn as_idx(&self) -> usize {
        self.id
    }

    // TODO: FROM index
    pub fn new(id: usize) -> Self {
        ComponentId {
            id,
            _marker: PhantomData,
        }
    }
}

impl std::fmt::Debug for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}

