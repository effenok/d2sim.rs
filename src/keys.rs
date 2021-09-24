use crate::component::Component;
use std::marker::PhantomData;

// pub type ComponentId = usize;
pub type ChannelId = usize;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

// TODO
// macro_rules! sim_key_type {
//     ($name:ident, $inner:ty, $doc:literal) => {
//         #[doc = $doc]
//         #[derive(Debug, PartialEq, Eq, Hash)]
//         pub struct $name<V> {
//             pub(crate) id: $inner,
//             _marker: PhantomData<V>,
//         }
//
//         impl<T> $name<T> {
//             pub(crate) fn new(id: $inner) -> Self {
//                 $name {
//                     id,
//                     _marker: PhantomData,
//                 }
//             }
//         }
//         impl<T> Clone for $name<T> {
//             fn clone(&self) -> Self {
//                 Self::new(self.id)
//             }
//         }
//
//         impl<T> Copy for $name<T> {}
//     };
// }
