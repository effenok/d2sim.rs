
pub type ComponentId = usize;
pub type ChannelId = usize;

// TODO
// macro_rules! key_type {
//     ($name:ident, $inner:ty, $doc:literal) => {
//         #[doc = $doc]
//         #[derive(Debug, PartialEq, Eq, Hash)]
//         pub struct $name<V> {
//             pub(crate) id: $inner,
//             _marker: PhantomData<V>,
//         }
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
//         impl<T> Copy for $name<T> {}
//     };
// }