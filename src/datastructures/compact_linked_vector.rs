use crate::datastructures::linked_vectors::{LinkedVector, Node};
use rand::{Rng, RngCore};
use rand::prelude::{IndexedMutRandom, IndexedRandom};

pub struct CompactLinkedVector<T> {
    list: Vec<Node<T>>,
    head: Node<T>,

    // This field is used to detect foreign handles. If a handle's
    // 3rd field doesn't match this, it's foreign.
    #[cfg(debug_assertions)]
    uuid  : usize,

}
impl<T> LinkedVector<T> for CompactLinkedVector<T>{
    fn get_random<R>(&self, rng: &mut R) -> Option<&Node<T>>
    where
        R: Rng + ?Sized
    {
        self.list.choose(rng)
    }

    fn get_mut_random<R>(&mut self, rng: &mut R) -> Option<&mut Node<T>>
    where
        R: RngCore
    {
        self.list.choose_mut(rng)
    }

    fn get_at_index(&self, i: usize) -> Option<&Node<T>> {
        if i < self.list.len(){
            return Some(&self.list[i]);
        }
        None
    }

    fn get_mut_at_index(&mut self, i: usize) -> Option<&mut Node<T>> {
        if i < self.list.len(){
            return Some(&mut self.list[i]);
        }
        None
    }

    fn insert_after(&mut self, node: Node<T>, value: T) -> &Node<T> {
        todo!()
    }

    fn insert_before(&mut self, node: Node<T>, value: T) -> &Node<T> {
        todo!()
    }

    fn remove(&mut self, node: Node<T>, value: T) -> &Node<T> {
        todo!()
    }
}
