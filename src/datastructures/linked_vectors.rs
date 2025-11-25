use rand::{Rng, RngCore};

pub trait LinkedVector<T> {
    fn get_random<R>(&self, rng: &mut R) -> Option<&Node<T>> where R:Rng + ?Sized;
    fn get_at_index(&self, index: usize) -> Option<&Node<T>>;
    fn get_head(&self) -> Option<&Node<T>>;
    fn get_tail(&self) -> Option<&Node<T>>;
    fn insert_after(&mut self, index: usize, value: T) -> &Node<T>;
    fn insert_before (&mut self, index: usize, value: T) -> &Node<T>;
    fn push_front(&mut self, value: T) -> &Node<T>;
    fn push_back(&mut self, value: T) -> &Node<T>;
    fn remove(&mut self, node: Node<T>, value: T) -> &Node<T>;
}
#[derive(Copy, Clone)]
pub struct Node<T>{
    pub value: T,
    pub index: usize,
    pub prev: Option<usize>,
    pub next: Option<usize>,

    // This number is used to check whether this Node is part of the LinkedVector.
    // We will only check this in debugmode, in release mode, panic??
    #[cfg(debug_assertions)]
    pub(super) list_id: usize,
}