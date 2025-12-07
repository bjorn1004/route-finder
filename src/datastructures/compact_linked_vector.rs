use crate::datastructures::linked_vectors::{LinkedVector, Node, LVNodeIndex};
use rand::Rng;
use rand::prelude::IndexedRandom;

/// This is a linkedlist inside a Vector. It works with Node<t>
/// Each Node contains its own index in the vector, and the index of the next and previous Node
/// If you want to make use of the get_random function, make sure to use it in this order.
/// ```rust
/// let mut lv = CompactLinkedVector::new();
/// let mut rng = rand::rng();
/// loop {
///     let (index, value) = lv.get_random(rng).unwrap();
///     // do your stuff with the values you get from random positions in the list
///     lv.insert_after(index, new_value);
///     // Don't remove the same index twice
///     // The code won't immediatly panic, but the linkedlist might get corrupted
///     lv.remove(index);
///
///     // once you have done everything with the random indexes and values, compact the lv
///     lv.compact();
///     // After compacting the list, all indexes you have gotten previously have become invalid.
///     // Do not try to use any indexes you have gotten before the last time you have compacted.
/// }
/// ```
#[derive(Clone, Debug)]
pub struct CompactLinkedVector<T> {
    list: Vec<Node<T>>,
    head: Option<LVNodeIndex>, // the index of the head in our list
    tail: Option<LVNodeIndex>, // the index of the tail in our list
    empty_indices: Vec<LVNodeIndex>,
}
impl<T> LinkedVector<T> for CompactLinkedVector<T> {
    fn get_random<R>(&self, rng: &mut R) -> Option<(LVNodeIndex, &T)>
    where
        R: Rng + ?Sized,
    {
        if !self.empty_indices.is_empty() {
            panic!("plz run compact before getting random values")
        }

        if self.list.is_empty() {
            return None;
        }

        if self.list.len() == 1 {
            return Some((0, &self.list[0].value));
        }

        while let Some(node) = self.list.choose(rng) {
            if node.next.is_none() && node.prev.is_none() {
                continue;
            }
            return Some((node.index, &node.value));
        }
        panic!("something went wrong in the random function");
    }

    fn get_value(&self, index: LVNodeIndex) -> Option<&T> {
        if index < self.list.len() {
            let node = &self.list[index];
            Some(&node.value)
        } else {
            None
        }
    }

    fn get_mut_value(&mut self, index: LVNodeIndex) -> Option<&mut T> {
        if index < self.list.len() {
            let node = &mut self.list[index];
            Some(&mut node.value)
        } else {
            None
        }
    }

    fn get_head_index(&self) -> Option<LVNodeIndex> {
        self.head?;
        let node = &self.list[self.head.unwrap()];
        Some(node.index)
    }

    fn get_tail_index(&self) -> Option<LVNodeIndex> {
        self.tail?;
        let node = &self.list[self.tail.unwrap()];
        Some(node.index)
    }
    fn insert_after(&mut self, index: LVNodeIndex, value: T) -> LVNodeIndex {
        if index >= self.list.len() {
            panic!("tried to index out of range")
        }
        let node = &self.list[index];

        if let Some(next_index) = node.next {
            self.insert_(Some(next_index), value)
        } else {
            self.insert_(None, value)
        }
    }

    fn insert_before(&mut self, index: LVNodeIndex, value: T) -> LVNodeIndex {
        if index >= self.list.len() {
            panic!("tried to index out of range")
        }
        let node = &self.list[index];

        self.insert_(Some(node.index), value)
    }

    fn push_front(&mut self, value: T) -> LVNodeIndex {
        self.insert_(self.head, value)
    }

    fn push_back(&mut self, value: T) -> LVNodeIndex {
        self.insert_(None, value)
    }

    fn remove(&mut self, index: LVNodeIndex) {
        if index >= self.list.len() {
            panic!("index out of range")
        }

        let node = &self.list[index];
        if let Some(prev) = node.prev {
            self.list[prev].next = node.next;
        } else {
            // there is no previous index
            // we are at the start of the linkedlist
            self.head = node.next;
        }

        let node = &self.list[index];
        if let Some(next) = node.next {
            self.list[next].prev = node.prev;
        } else {
            // there is no next index
            // we are at the end of the list
            self.tail = node.prev;
        }

        let node = &mut self.list[index];
        node.next = None;
        node.prev = None;
        self.empty_indices.push(node.index);
    }

    fn set_value_at_index(&mut self, index: LVNodeIndex, value: T) {
        self.list[index].value = value;
    }

    fn get_next(&self, index: LVNodeIndex) -> Option<LVNodeIndex> {
        self.list[index].next
    }
    fn get_prev(&self, index: LVNodeIndex) -> Option<LVNodeIndex> {
        self.list[index].prev
    }

    fn get_next_value(&self, index: LVNodeIndex) -> Option<&T> {
        if let Some(next_index) = self.list[index].next{
            self.get_value(next_index)
        } else {
            None
        }
    }
    fn get_prev_value(&self, index: LVNodeIndex) -> Option<&T> {
        if let Some(prev_index) = self.list[index].prev{
            self.get_value(prev_index)
        } else {
            None
        }
    }
}
impl<T> CompactLinkedVector<T> {
    pub fn new() -> Self {
        CompactLinkedVector {
            list: vec![],
            head: None,
            tail: None,
            empty_indices: vec![],
        }
    }
    /// This function will contain all the logic for swapping prev and next indices.
    /// If the list is empty, we add it as the first element of the list.
    /// If node is Some(node) we add the value in front of the node in the linkedlist.
    /// If node is None, we add the value to the end of the linked list
    /// The new node will be placed in an empty spot or at the end of the list.
    fn insert_(&mut self, node_index: Option<usize>, value: T) -> LVNodeIndex {
        let new_index = self.get_valid_empty_index();
        let new_node: Node<T>;
        if self.list.is_empty() {
            new_node = Node {
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
            if let Some(prev_index) = prev {
                let prev = &mut self.list[prev_index];
                prev.next = Some(new_index);
            } else {
                // if None, we are at the front of the linkedlist
                self.head = Some(new_index);
            }
            new_node = Node {
                value,
                index: new_index,
                prev,
                next: Some(node),
            };
        } else {
            // if node is None
            if let Some(tail) = self.tail {
                let tail = &mut self.list[tail];
                tail.next = Some(new_index);
            } else {
                panic!(
                    "not sure in which we would insert a value at the end of the list without a tail"
                )
            }
            new_node = Node {
                value,
                index: new_index,
                prev: self.tail,
                next: None,
            };

            self.tail = Some(new_index);
        }

        if new_index == self.list.len() {
            self.list.push(new_node);
        } else {
            self.list[new_index] = new_node;
        }
        new_index
    }
    fn get_valid_empty_index(&mut self) -> LVNodeIndex {
        if let Some(index) = self.empty_indices.pop() {
            index
        } else {
            self.list.len()
        }
    }

    /// Please don't use this function often,
    /// it takes O(nlogn) where n is the length of the empty indices
    pub fn compact(&mut self) {
        self.empty_indices.sort();
        while let Some(index) = self.empty_indices.pop() {
            self.move_back_to_new_index(index);
        }
    }

    fn move_back_to_new_index(&mut self, new_i: LVNodeIndex) {
        let new_node_pos = &self.list[new_i];
        if new_node_pos.next.is_some() || new_node_pos.prev.is_some() || new_node_pos.index != new_i
        {
            panic!("AAAAa")
        }

        let last_i = self.list.len() - 1;
        if new_i == last_i {
            // should only happen if last_i is empty
            self.list.pop();
            return;
        }

        let mut node = self.list.pop().unwrap();

        // if the node is not the tail or head
        // and the node also doesn't have a next or pev value, we don't have to move it.
        if self.is_empty(&node) {
            // We can't return here, because we still need to fill the new_i with our new thing
        }

        // if the node is the head, update head to the new position
        if self.head == Some(node.index) {
            self.head = Some(new_i);
        }

        // if the node is the tail, update tail to the new position
        if self.tail == Some(node.index) {
            self.tail = Some(new_i)
        }

        // if node has a next value, update the prev in the next node to new_i
        if let Some(next) = node.next {
            self.list[next].prev = Some(new_i)
        }
        // if node has a prev value, update the next in the prev node to new_i
        if let Some(prev) = node.prev {
            self.list[prev].next = Some(new_i)
        }

        // we don't have to change the next or prev values in node
        // assuming they pointed to correct values, they will still point to correct values.
        // update the index of the new index
        node.index = new_i;
        self.list[new_i] = node;
    }
    fn is_empty(&self, node: &Node<T>) -> bool {
        node.index != self.head.unwrap()
            && node.index != self.tail.unwrap()
            && node.next.is_none()
            && node.prev.is_none()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

// made this iterator using chatgpt, I am not competent enough with lifetimes to do this myself.
pub struct Iter<'a, T> {
    list: &'a CompactLinkedVector<T>,
    current: Option<LVNodeIndex>,
}

impl<T> CompactLinkedVector<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            current: self.head,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (LVNodeIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.current?;
        let node = &self.list.list[idx];
        self.current = node.next;
        Some((idx, &node.value))
    }
}
#[cfg(test)]
mod tests {
    fn check_list_integrity(lv: &CompactLinkedVector<usize>) {
        if lv.list.is_empty() {
            assert!(lv.head.is_none());
            assert!(lv.tail.is_none());
            return;
        }

        // Walk forward
        let mut count_forward = 0;
        let mut curr = lv.get_head_index();
        let mut last = None;

        while let Some(i) = curr {
            let node = &lv.list[i];
            if let Some(prev) = node.prev {
                assert_eq!(lv.list[prev].next, Some(i));
            }
            last = Some(i);
            curr = node.next;
            count_forward += 1;
        }

        assert_eq!(last, lv.get_tail_index());

        // Walk backward
        let mut count_backward = 0;
        curr = lv.get_tail_index();
        while let Some(i) = curr {
            let node = &lv.list[i];
            if let Some(next) = node.next {
                assert_eq!(lv.list[next].prev, Some(i));
            }
            curr = node.prev;
            count_backward += 1;
        }

        assert_eq!(count_forward, count_backward);
    }

    fn is_compacted<T>(ls: &CompactLinkedVector<T>) {
        if ls.list.is_empty() {
            assert_eq!(ls.head, None);
            assert_eq!(ls.tail, None);
            return;
        }

        if ls.list.len() == 1 {
            assert_eq!(ls.head, Some(0));
            assert_eq!(ls.tail, Some(0));
            assert_eq!(ls.list[0].next, None);
            assert_eq!(ls.list[0].prev, None);
            assert!(ls.empty_indices.is_empty());
            return;
        }

        let mut count = 0;
        assert_ne!(ls.head, ls.tail);
        for node in &ls.list {
            count += 1;
            if node.next.is_none() && node.prev.is_none() {
                panic!("empty node");
            }
        }
        let mut iter_count = 0;
        for _ in ls.iter() {
            iter_count += 1;
        }

        assert_eq!(count, iter_count)
    }

    use rand::rngs::SmallRng;

    use super::*;
    use std::collections::HashSet;
    #[test]
    fn push_back_to_empty_list() {
        let mut lv = CompactLinkedVector::new();
        lv.push_back(1);
        assert_eq!(lv.head.unwrap(), 0, "yay");
        assert_eq!(lv.list[lv.head.unwrap()].value, 1, "yay");
        assert_eq!(lv.list[lv.tail.unwrap()].value, 1, "yay");
    }
    #[test]
    fn push_front_to_empty_list() {
        let mut lv = CompactLinkedVector::new();
        lv.push_front(1);
        assert_eq!(lv.head.unwrap(), 0, "yay");
        assert_eq!(lv.list[lv.head.unwrap()].value, 1, "yay");
        assert_eq!(lv.list[lv.tail.unwrap()].value, 1, "yay");
    }
    #[test]
    fn push_back_and_remove() {
        let mut lv = CompactLinkedVector::new();
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
        assert_eq!(lv.empty_indices.len(), 2)
    }

    // From this point on, the tests are written by chatgpt
    #[test]
    fn insert_front_and_back_multiple() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(1);
        let b = lv.push_back(2);
        let c = lv.push_back(3);

        assert_eq!(lv.get_head_index(), Some(a));
        assert_eq!(lv.get_tail_index(), Some(c));

        assert_eq!(lv.list[a].next, Some(b));
        assert_eq!(lv.list[b].next, Some(c));
        assert_eq!(lv.list[c].prev, Some(b));
    }

    #[test]
    fn insert_before_middle() {
        let mut lv = CompactLinkedVector::new();
        lv.push_back(1);
        let b = lv.push_back(3);
        lv.push_back(4);

        let inserted = lv.insert_before(b, 2);

        assert_eq!(lv.list[inserted].value, 2);
        assert_eq!(lv.list[inserted].next, Some(b));
        assert_eq!(lv.list[b].prev, Some(inserted));
    }

    #[test]
    fn insert_after_middle() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(1);
        let b = lv.push_back(3);

        let inserted = lv.insert_after(a, 2);

        assert_eq!(lv.list[inserted].value, 2);
        assert_eq!(lv.list[inserted].prev, Some(a));
        assert_eq!(lv.list[inserted].next, Some(b));
        assert_eq!(lv.list[a].next, Some(inserted));
        assert_eq!(lv.list[b].prev, Some(inserted));
    }

    #[test]
    fn remove_head() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(1);
        let b = lv.push_back(2);

        lv.remove(a);

        assert_eq!(lv.get_head_index(), Some(b));
        assert_eq!(lv.list[b].prev, None);
        assert_eq!(lv.list[b].value, 2);
    }

    #[test]
    fn remove_tail() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(1);
        let b = lv.push_back(2);

        lv.remove(b);

        assert_eq!(lv.get_tail_index(), Some(a));
        assert_eq!(lv.list[a].next, None);
    }

    #[test]
    fn remove_middle() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(1);
        let b = lv.push_back(2);
        let c = lv.push_back(3);

        lv.remove(b);

        assert_eq!(lv.list[a].next, Some(c));
        assert_eq!(lv.list[c].prev, Some(a));
    }

    #[test]
    fn reuse_empty_nodes() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(10);
        let b = lv.push_back(20);

        lv.remove(a);

        // the next insert should reuse the index 'a'
        let c = lv.push_back(30);

        assert_eq!(c, a, "Empty slot was not reused");
        assert_eq!(lv.list[c].value, 30);
        assert_eq!(lv.list[b].next, Some(c));
    }

    #[test]
    fn insert_after_tail() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(1);
        let b = lv.insert_after(a, 2);

        assert_eq!(lv.get_tail_index(), Some(b));
        assert_eq!(lv.list[a].next, Some(b));
        assert_eq!(lv.list[b].prev, Some(a));
    }

    #[test]
    fn insert_before_head() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(2);
        let b = lv.insert_before(a, 1);

        assert_eq!(lv.get_head_index(), Some(b));
        assert_eq!(lv.list[b].next, Some(a));
    }

    #[test]
    fn random_insertions_and_removals_stay_consistent() {
        let mut lv = CompactLinkedVector::new();
        let mut nodes = vec![];

        // insert 10 nodes
        for i in 0..10 {
            nodes.push(lv.push_back(i));
        }

        // remove even indices
        for &n in &nodes {
            if lv.list[n].value % 2 == 0 {
                lv.remove(n);
            }
        }

        // check linked list integrity
        // forward traversal
        let mut curr = lv.get_head_index();
        let mut last = None;

        while let Some(i) = curr {
            let node = &lv.list[i];
            if let Some(prev) = node.prev {
                assert_eq!(lv.list[prev].next, Some(i));
            }
            last = Some(i);
            curr = node.next;
        }

        assert_eq!(last, lv.get_tail_index());
    }

    #[test]
    fn get_random_never_returns_empty_nodes() {
        let mut lv = CompactLinkedVector::new();
        let a = lv.push_back(10);
        lv.push_back(20);

        lv.remove(a);

        lv.compact();
        let mut rng = rand::rng();
        lv.compact();
        for _ in 0..100 {
            let (_, value) = lv.get_random(&mut rng).unwrap();
            assert_eq!(*value, 20);
        }
    }
    #[test]
    fn compact() {
        let mut lv = CompactLinkedVector::new();
        let mut nodes = vec![];

        // insert 10 nodes
        for i in 0..10 {
            nodes.push(lv.push_back(i));
        }

        // remove even indices
        for &n in &nodes {
            if lv.list[n].value % 2 == 0 {
                lv.remove(n);
            }
        }
        lv.compact();
        is_compacted(&lv);
    }
    #[test]
    fn compact_empty_list() {
        let mut lv = CompactLinkedVector::new();
        let mut nodes = vec![];
        for i in 0..2 {
            nodes.push(lv.push_back(i))
        }

        for &n in &nodes {
            lv.remove(n);
        }
        lv.compact();
        is_compacted(&lv);
    }

    fn fill_linked_vector(indices: usize) -> CompactLinkedVector<usize> {
        let mut lv = CompactLinkedVector::new();
        for i in 0..indices {
            lv.push_back(i);
        }
        lv
    }
    #[test]
    fn compact_creates_no_self_loops_bug4() {
        let mut lv = CompactLinkedVector::new();

        // Build a list: 0 <-> 1 <-> 2 <-> 3
        // Indices:       0    1    2    3
        lv.push_back(10); // 0
        let b = lv.push_back(20); // 1
        let c = lv.push_back(30); // 2
        lv.push_back(40); // 3

        lv.remove(b);
        lv.remove(c);
        lv.compact();

        // Detect self-loops
        for node in &lv.list {
            if let Some(next) = node.next {
                assert_ne!(
                    next, node.index,
                    "BUG: compact created a self-loop at index {}",
                    node.index
                );
            }
        }
    }
    #[test]
    fn compact_on_middle_removals() {
        let mut lv = CompactLinkedVector::new();

        lv.push_back(1);
        let b = lv.push_back(2);
        let c = lv.push_back(3);
        lv.push_back(4);

        // Remove b and c
        lv.remove(b);
        lv.remove(c);

        // Now compact — current code will corrupt node links
        // Node a's next still points to b
        // Node d's prev still points to c
        lv.compact();

        let mut curr = lv.get_head_index();
        while let Some(idx) = curr {
            let node = &lv.list[idx];
            curr = node.next;
        }
        is_compacted(&lv);
    }
    #[test]
    fn compact_when_head_tail_removed() {
        let mut lv = CompactLinkedVector::new();

        let a = lv.push_back(1);
        lv.push_back(2);
        let c = lv.push_back(3);

        // Remove head and tail
        lv.remove(a);
        lv.remove(c);

        // Only b remains
        // Compact should be safe, but current code may panic on index mismatch
        lv.compact();

        assert_eq!(lv.get_head_index(), Some(0));
        assert_eq!(lv.get_tail_index(), Some(0));
        is_compacted(&lv);
    }

    #[test]
    fn stress_test_with_continuous_correctness_checks() {
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};
        let mut rng = SmallRng::seed_from_u64(12345);
        let mut lv = fill_linked_vector(10_000);

        let mut random_var_val = 10_000;
        let mut random_indices = HashSet::new();

        for _ in 0..10_000 {
            random_indices.clear();
            let action_count: u8 = Rng::random(&mut rng);

            for _ in 0..(action_count % 30) {
                random_indices.insert(lv.get_random(&mut rng).unwrap().0);
            }

            for j in &random_indices {
                let action: u8 = Rng::random(&mut rng);

                assert_eq!(*j, lv.list[*j].index);
                match action % 4 {
                    0 => {
                        lv.insert_after(*j, random_var_val);
                        random_var_val += 1;
                    }
                    1 => {
                        lv.insert_before(*j, random_var_val);
                        random_var_val += 1;
                    }
                    _ => {
                        lv.remove(*j);
                    }
                }
            }
            check_list_integrity(&lv);
            lv.compact();
            check_list_integrity(&lv);
            is_compacted(&lv);
        }
    }
    #[test]
    fn stress_test_only_check_at_the_end() {
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};
        let mut rng = SmallRng::seed_from_u64(12345);
        let mut lv = fill_linked_vector(100_000);

        let mut random_var_val = 10_000;
        let mut random_indices = HashSet::new();

        #[cfg(debug_assertions)]
        let iterations = 100_000;

        #[cfg(not(debug_assertions))]
        let iterations = 100_000_000;

        for _ in 0..iterations {
            random_indices.clear();

            for _ in 0..5 {
                random_indices.insert(lv.get_random(&mut rng).unwrap().0);
            }

            for j in &random_indices {
                let action: u8 = Rng::random(&mut rng);

                assert_eq!(*j, lv.list[*j].index);
                match action % 4 {
                    0 => {
                        lv.insert_after(*j, random_var_val);
                        random_var_val += 1;
                    }
                    1 => {
                        lv.insert_before(*j, random_var_val);
                        random_var_val += 1;
                    }
                    _ => {
                        lv.remove(*j);
                    }
                }
            }
            lv.compact();
        }
        check_list_integrity(&lv);
        lv.compact();
        assert_eq!(lv.iter().count(), lv.list.len());
        check_list_integrity(&lv);
        is_compacted(&lv);
    }
}
