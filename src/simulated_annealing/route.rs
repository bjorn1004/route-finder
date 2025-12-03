use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::resource::MatrixID;

#[derive(Debug, Clone)]
pub struct Route{
    pub route: CompactLinkedVector<MatrixID>,
    // there are a few variables missing here.
    // If we add new variables here, we also need to add them to the "constructor"
}


impl Route{
    
    // This is the constructor, the new function.
    /// Construct an empty route
    pub fn new() -> Self{
        Route{
            route: CompactLinkedVector::<MatrixID>::new(),
        }
    }
}