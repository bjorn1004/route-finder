use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::resource::MatrixID;

#[derive(Debug, Clone)]
pub struct Route{
    pub linked_vector: CompactLinkedVector<MatrixID>,
    // there are a few variables missing here.
    // If we add new variables here, we also need to add them to the "constructor"
}


impl Route{
    /// Construct an empty route
    pub fn new() -> Self{
        Route{
            linked_vector: CompactLinkedVector::<MatrixID>::new(),
        }
    }
}