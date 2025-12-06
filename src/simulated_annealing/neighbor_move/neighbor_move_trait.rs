use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use crate::resource::MatrixID;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use super::super::week::Week;

pub trait NeighborMove {
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Option<Evaluation>;
    // this would perform the thing on the schedules.
    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags);
}

pub struct Evaluation{
    pub cost: f32,
}
