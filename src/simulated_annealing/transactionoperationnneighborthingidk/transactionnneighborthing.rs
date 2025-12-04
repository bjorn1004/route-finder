use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use crate::resource::MatrixID;
use super::super::week::Week;

pub trait TransactionNeighborThing{
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self, truck1: &Week, truck2: &Week);
    // this would perform the thing on the schedules.
    fn execute(&self, truck1: &mut Week, truck2: &mut Week);
}

pub struct Swap2RandomValuesInSameRoute {
    truck: usize,
    day: usize,
    route_of_day: usize,
    index1: NodeIndex,
    index2: NodeIndex,
    matrix_id1: MatrixID,
    matrix_id2: MatrixID,

}

/// This tihng will change nothing, it is purely here to find what variables we would need in the trait above.
impl TransactionNeighborThing for Swap2RandomValuesInSameRoute {
    fn evaluate(&self, truck1: &Week, truck2: &Week) {
        todo!()
    }

    fn execute(&self, truck1: &mut Week, truck2: &mut Week) {
        todo!()
    }
}

impl Swap2RandomValuesInSameRoute{
    pub fn new() -> Self{
        todo!()
    }
}