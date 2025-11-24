use rand::{Rng, RngCore};

pub trait LinkedVector<T> {
    fn get_random<R>(&self, rng: &mut R) -> Option<&Node<T>> where R:Rng + ?Sized;
    fn get_mut_random<R>(&mut self, rng: &mut R) -> Option<&mut Node<T>> where R:RngCore;
    fn get_at_index(&self) -> &Option<&Node<T>>;
    fn get_mut_at_index(&mut self) -> &mut Option<Node<T>>;
    fn insert_after(&mut self, node: Node<T>, value: T) -> &Node<T>;
    fn insert_before (&mut self, node: Node<T>, value: T) -> &Node<T>;
    fn remove(&mut self, node: Node<T>, value: T) -> &Node<T>;
}
pub struct Node<T>{
    value: T,
    next: usize,
    prev: usize,
    
    // This number is used to check whether this Node is part of the LinkedVector.
    // We will only check this in debugmode, in release mode, panic??
    #[cfg(debug_assertions)]
    list_id: usize,
}