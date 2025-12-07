use std::fmt;
use std::fmt::Formatter;
use rand::{Rng};

pub trait LinkedVector<T>{
    fn get_random<R>(&self, rng: &mut R) -> Option<(LVNodeIndex, &T)> where R:Rng + ?Sized;
    fn get_value(&self, index: LVNodeIndex) -> Option<&T>;
    fn get_mut_value(&mut self, index: LVNodeIndex) -> Option<&mut T>;
    fn get_head_index(&self) -> Option<LVNodeIndex>;
    fn get_tail_index(&self) -> Option<LVNodeIndex>;
    fn insert_after(&mut self, index: LVNodeIndex, value: T) -> LVNodeIndex;
    fn insert_before (&mut self, index: LVNodeIndex, value: T) -> LVNodeIndex;
    fn push_front(&mut self, value: T) -> LVNodeIndex;
    fn push_back(&mut self, value: T) -> LVNodeIndex;
    fn remove(&mut self, index: LVNodeIndex);
    fn set_value_at_index(&mut self, index: LVNodeIndex, value: T);
    fn get_next(&self, index: LVNodeIndex) -> Option<LVNodeIndex>;
    fn get_next_value(&self, index: LVNodeIndex) -> Option<&T>;
    fn get_prev(&self, index: LVNodeIndex) -> Option<LVNodeIndex>;
    fn get_prev_value(&self, index: LVNodeIndex) -> Option<&T>;
}
#[derive(Clone)]
pub struct Node<T>{
    pub value: T,
    pub index: LVNodeIndex,
    pub prev: Option<LVNodeIndex>,
    pub next: Option<LVNodeIndex>,
    }
pub type LVNodeIndex = usize;
impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("index", &self.index)
            .field("prev", &self.prev)
            .field("next", &self.next)
            .finish()
    }
}
