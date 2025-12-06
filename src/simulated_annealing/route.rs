use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::resource::MatrixID;

#[derive(Debug, Clone)]
pub struct Route{
    pub linked_vector: CompactLinkedVector<OrderIndex>,
    pub capacity: u64,
    pub time: f32,
}
pub type OrderIndex = usize;

impl Route{
    /// Construct an empty route
    /// We should maybe add the dropoff location as the first and last element of this list?
    pub fn new() -> Self{
        Route{
            linked_vector: CompactLinkedVector::<OrderIndex>::new(),
            capacity: 0,
            time: 0f32,
        }
    }

    pub fn check_correctness(&self){
        let dist_matrix = crate::get_orders();
        assert_eq!(self.linked_vector
                       .iter()
                       .map(|(_, matrix_id)| dist_matrix[*matrix_id as usize].container_volume as u64)
                       .sum::<u64>(),
                   0u64, "The currently stored trash volume is incorrect")

    }


}