use rand::{Rng, RngCore};

pub trait LinkedVector<T>{
    fn get_random<R>(&self, rng: &mut R) -> Option<(NodeIndex, &T)> where R:Rng + ?Sized;
    fn get_value(&self, index: NodeIndex) -> Option<&T>;
    fn get_mut_value(&mut self, index: NodeIndex) -> Option<&mut T>;
    fn get_head_index(&self) -> Option<NodeIndex>;
    fn get_tail_index(&self) -> Option<NodeIndex>;
    fn insert_after(&mut self, index: NodeIndex, value: T) -> NodeIndex;
    fn insert_before (&mut self, index: NodeIndex, value: T) -> NodeIndex;
    fn push_front(&mut self, value: T) -> NodeIndex;
    fn push_back(&mut self, value: T) -> NodeIndex;
    fn remove(&mut self, index: NodeIndex);
}
pub struct Node<T>{
    pub value: T,
    pub index: NodeIndex,
    pub prev: Option<NodeIndex>,
    pub next: Option<NodeIndex>,
    }
pub type NodeIndex = usize;