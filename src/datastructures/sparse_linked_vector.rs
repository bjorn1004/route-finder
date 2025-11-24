use crate::datastructures::linked_vectors::*;

pub struct SparseLinkedVector<T> {
    list: Vec<Node<T>>,
    head: Node<T>,

    // This field is used to detect foreign handles. If a handle's
    // 3rd field doesn't match this, it's foreign.
    #[cfg(debug_assertions)]
    uuid  : usize,

}