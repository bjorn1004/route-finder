use crate::datastructures::linked_vectors::{LinkedVector, Node, NodeIndex};
use rand::{Rng};
use rand::prelude::{IndexedRandom};

pub struct LinkedVectorWIthErrorValuesAsEmptyNodes<T> {
    list: Vec<Node<T>>,
    head: Option<NodeIndex>, // the index of the head in our list
    tail: Option<NodeIndex>, // the index of the tail in our list
    empty_indexes: Vec<NodeIndex>,
}
impl<T> LinkedVector<T> for LinkedVectorWIthErrorValuesAsEmptyNodes<T>{
    fn get_random<R>(&self, rng: &mut R) -> Option<(NodeIndex, &T)>
    where
        R: Rng + ?Sized
    {
         if let Some(node) = self.list.choose(rng){
             Some((node.index, &node.value))
         } else {
             None
         }
    }

    fn get_value(&self, index: NodeIndex) -> Option<&T> {
        if index < self.list.len(){
            let node = &self.list[index];
            Some(&node.value)
        } else {
            None
        }
    }

    fn get_mut_value(&mut self, index: NodeIndex) -> Option<&mut T> {
        if index < self.list.len(){
            let node = &mut self.list[index];
            Some(&mut node.value)
        } else {
            None
        }
    }

    fn get_head_index(&self) -> Option<NodeIndex> {
        if self.head.is_none(){
            return None;
        }
        let node = &self.list[self.head.unwrap()];
        Some(node.index)
    }

    fn get_tail_index(&self) -> Option<NodeIndex> {
        if self.tail.is_none(){
            return None;
        }
        let node = &self.list[self.tail.unwrap()];
        Some(node.index)
    }
    fn insert_after(&mut self, index: NodeIndex, value: T) -> NodeIndex {
        if index >= self.list.len() { panic!("tried to index out of range")}
        let node = &self.list[index];

        if let Some(next_index) = node.next{
            self.insert_(Some(next_index), value)
        } else {
            self.insert_(None, value)
        }
    }

    fn insert_before(&mut self, index: NodeIndex, value: T) -> NodeIndex {
        if index >= self.list.len() { panic!("tried to index out of range")}
        let node = &self.list[index];
        self.insert_(Some(node.index), value)
    }

    fn push_front(&mut self, value: T) -> NodeIndex {
        self.insert_(self.head, value)
    }

    fn push_back(&mut self, value: T) -> NodeIndex{
        self.insert_(None, value)
    }

    fn remove(&mut self, index: NodeIndex) {
        if index >= self.list.len() {
            panic!("index out of range")
        }

        let node = &self.list[index];
        if let Some(prev) = node.prev{
            self.list[prev].next = node.next;
        } else {
            // there is no previous index
            // we are at the start of the linkedlist
            self.head = node.next;
        }

        let node = &self.list[index];
        if let Some(next) = node.next{
            self.list[next].prev = node.prev;
        } else {
            // there is no next index
            // we are at the end of the list
            self.tail = node.prev;
        }

        let node = &mut self.list[index];
        node.next = None;
        node.prev = None;
        self.empty_indexes.push(node.index);
    }
}
impl<T> LinkedVectorWIthErrorValuesAsEmptyNodes<T> {
    fn new() -> Self{
        LinkedVectorWIthErrorValuesAsEmptyNodes {
            list: vec![],
            head: None,
            tail: None,
            empty_indexes: vec![],
        }
    }
    /// This function will contain all the logic for swapping prev and next indices.
    /// If the list is empty, we add it as the first element of the list.
    /// If node is Some(node) we add the value in front of the node in the linkedlist.
    /// If node is None, we add the value to the end of the linked list
    /// In all cases, the value will be put at the end of the vector
    fn insert_(&mut self, node_index: Option<usize>, value: T) -> NodeIndex{
        let new_index = self.get_valid_empty_index();
        let new_node: Node<T>;
        if self.list.is_empty(){
            #[cfg(debug_assertions)]
            assert!(node_index.is_none());
            new_node = Node{
                value,
                index: new_index,
                prev: None,
                next: None,
            };
            self.head = Some(0);
            self.tail = Some(0);
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
            new_node = Node{
                value,
                index: new_index,
                prev,
                next: Some(node),
            };
        } else { // if node is None
            if let Some(tail) = self.tail{
                let tail = &mut self.list[tail];
                tail.next = Some(new_index);
            } else if let None = self.tail{
                panic!("not sure in which we would insert a value at the end of the list without a tail")
            }
            new_node = Node{
                value,
                index: new_index,
                prev: self.tail,
                next: None,
            };

            self.tail = Some(new_index);
        }

        if new_index == self.list.len(){
            self.list.push(new_node);
        } else {
            self.list[new_index] = new_node;
        }
        new_index
    }
    fn get_valid_empty_index(&mut self) -> NodeIndex{
        if let Some(index) = self.empty_indexes.pop(){
            index
        } else {
            self.list.len()
        }

    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn push_back_to_empty_list(){
        let mut lv = LinkedVectorWIthErrorValuesAsEmptyNodes::new();
        lv.push_back(1);
        assert_eq!(lv.head.unwrap(), 0, "yay");
        assert_eq!(lv.list[lv.head.unwrap()].value, 1, "yay");
        assert_eq!(lv.list[lv.tail.unwrap()].value, 1, "yay");
    }
    #[test]
    fn push_front_to_empty_list(){
        let mut lv = LinkedVectorWIthErrorValuesAsEmptyNodes::new();
        lv.push_front(1);
        assert_eq!(lv.head.unwrap(), 0, "yay");
        assert_eq!(lv.list[lv.head.unwrap()].value, 1, "yay");
        assert_eq!(lv.list[lv.tail.unwrap()].value, 1, "yay");
    }
    #[test]
    fn push_back_and_remove(){
        let mut lv = LinkedVectorWIthErrorValuesAsEmptyNodes::new();
        let node1 = lv.push_back(1);
        let node2 = lv.push_back(2);
        assert_eq!(lv.list[lv.head.unwrap()].value, 1);
        assert_eq!(lv.list[lv.tail.unwrap()].value, 2);
        lv.remove(node2);
        assert_eq!(lv.list[lv.head.unwrap()].value, 1);
        assert_eq!(lv.list[lv.tail.unwrap()].value, 1);
        lv.remove(node1);
        assert!(lv.head.is_none());
        assert!(lv.tail.is_none());
        assert_eq!(lv.empty_indexes.len(), 2)
    }
}
