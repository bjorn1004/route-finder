use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use crate::resource::MatrixID;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use super::super::week::Week;

pub trait NeighborMove {
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self, truck1: &Week, truck2: &Week);
    // this would perform the thing on the schedules.
    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &OrderFlags);
}

pub struct Swap2RandomValuesInSameRoute {
}

/// This tihng will change nothing, it is purely here to find what variables we would need in the trait above.
impl NeighborMove for Swap2RandomValuesInSameRoute {
    fn evaluate(&self, truck1: &Week, truck2: &Week) {
        todo!()
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &OrderFlags) {
        todo!()
    }
}

impl Swap2RandomValuesInSameRoute{
    pub fn new(truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Self{
        todo!()
    }
}