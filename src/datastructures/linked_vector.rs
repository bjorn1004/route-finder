use crate::datastructures::linked_vectors::{LinkedVector, Node, NodeIndex};
use rand::{Rng};
use rand::prelude::{IndexedRandom};

pub struct CompactLinkedVector<T> {
    list: Vec<Node<T>>,
    head: Option<NodeIndex>, // the index of the head in our list
    tail: Option<NodeIndex>, // the index of the tail in our list
}
impl<T> LinkedVector<T> for CompactLinkedVector<T>{
    fn get_random<R>(&self, rng: &mut R) -> Option<&Node<T>>
    where
        R: Rng + ?Sized
    {
        self.list.choose(rng)
    }

    fn get_at_index(&self, i: usize) -> Option<&Node<T>> {
        if i < self.list.len(){
            return Some(&self.list[i]);
        }
        None
    }

    fn get_head(&self) -> Option<&Node<T>> {
        Some(&self.list[self.head?])
    }

    fn get_tail(&self) -> Option<&Node<T>> {
        Some(&self.list[self.tail?])
    }
    fn insert_after(&mut self, index: NodeIndex, value: T) -> &Node<T> {
        if index > self.list.len() { panic!("tried to index out of range")}
        let node = self.get_at_index(index).unwrap();

        if let Some(next_index) = node.next{
            self.insert_(Some(next_index), value)
        } else {
            self.insert_(None, value)
        }
    }

    fn insert_before(&mut self, index: NodeIndex, value: T) -> &Node<T> {
        if index > self.list.len() { panic!("tried to index out of range")}
        let node = self.get_at_index(index).unwrap();
        self.insert_(Some(node.index), value)
    }

    fn push_front(&mut self, value: T) -> &Node<T> {
        self.insert_(self.head, value)
    }

    fn push_back(&mut self, value: T) -> &Node<T> {
        self.insert_(None, value)
    }

    fn remove(&mut self, node: Node<T>, value: T) -> &Node<T> {
        todo!()
    }
}
impl<T> CompactLinkedVector<T> {
    fn new() -> Self{
        CompactLinkedVector{
            list: vec![],
            head: None,
            tail: None,
        }
    }
    /// This function will contain all the logic for swapping prev and next indices.
    /// If the list is empty, we add it as the first element of the list.
    /// If node is Some(node) we add the value in front of the node in the linkedlist.
    /// If node is None, we add the value to the end of the linked list
    /// In all cases, the value will be put at the end of the vector
    fn insert_(&mut self, node_index: Option<usize>, value: T) -> &Node<T>{
        let new_index = self.list.len();
        if self.list.is_empty(){
            #[cfg(debug_assertions)]
            assert!(node_index.is_none());
            let new_node = Node{
                value,
                index: new_index,
                prev: None,
                next: None,
            };
            self.head = Some(0);
            self.tail = Some(0);
            self.list.push(new_node);

            &self.list[0]
        } else if let Some(node) = node_index {
            let prev = self.list[node].prev;
            self.list[node].prev = Some(new_index);
            if let Some(prev_index) = prev{
                let prev = &mut self.list[prev_index];
                prev.next = Some(new_index);
            } else {
                // if None, we are at the front of the linkedlist
                self.head = Some(new_index);
            }
            let new_node = Node{
                value,
                index: new_index,
                prev,
                next: Some(node),
            };
            self.list.push(new_node);
            &self.list[new_index]
        } else { // if node is None
            if let Some(tail) = self.tail{
                let tail = &mut self.list[tail];
                tail.next = Some(new_index);
            } else if let None = self.tail{
                panic!("not sure in which we would insert a value at the end of the list without a tail")
            }
            let new_node = Node{
                value,
                index: new_index,
                prev: self.tail,
                next: None,
            };

            self.tail = Some(new_index);
            self.list.push(new_node);
            &self.list[new_index]
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn push_back_to_empty_list(){
        let mut lv = CompactLinkedVector::new();
        lv.push_back(1);
        assert_eq!(lv.head.unwrap(), 0, "yay");
        assert_eq!(lv.get_head().unwrap().value, 1, "yay");
    }
    #[test]
    fn push_front_to_empty_list(){
        let mut lv = CompactLinkedVector::new();
        lv.push_front(1);
        assert_eq!(lv.head.unwrap(), 0, "yay");
        assert_eq!(lv.get_head().unwrap().value, 1, "yay");
    }
    #[test]
    fn insert(){
        let mut lv = CompactLinkedVector::new();
        let node1 = lv.push_back(1);
        let node2 = lv.push_back(2);

        let before2 = *lv.insert_before(node2.index, 2);

        lv.insert_before(node1.index, 100);
        assert_eq!(lv.get_head().unwrap().value, 100);
        assert_eq!(lv.get_head().unwrap().next.unwrap(), before2.index);
        assert_eq!(lv.get_tail().unwrap().value, 5);

    }
}
