use crate::component::Component;
use std::marker::PhantomData;

// pub type ComponentId = usize;
pub type ChannelId = usize;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ComponentId {
    id: usize,
    _marker: PhantomData<dyn Component>
}

impl ComponentId {
    pub fn as_idx(&self) -> usize {
        self.id
    }

    // TODO: from index
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

