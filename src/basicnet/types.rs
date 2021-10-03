use crate::util::uid::UniqueId;

pub type RouterId = UniqueId;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct InterfaceId {
    idx: usize,
}

impl From<usize> for InterfaceId {
    fn from(idx: usize) -> Self {
        InterfaceId { idx }
    }
}

impl PartialEq<usize> for InterfaceId {
    fn eq(&self, other: &usize) -> bool {
        self.idx == *other
    }
}

impl InterfaceId {
    pub fn as_idx(&self) -> usize {
        self.idx
    }
}

impl std::fmt::Debug for InterfaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "if_{}", self.idx)
    }
}

impl std::fmt::Display for InterfaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "if_{}", self.idx)
    }
}