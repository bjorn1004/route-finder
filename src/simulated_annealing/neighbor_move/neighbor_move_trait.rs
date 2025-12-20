use crate::resource::Time;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::solution::Solution;
use super::super::week::Week;

pub trait NeighborMove {
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self, solution: &Solution, order_flags: &OrderFlags) -> CostChange;
    // this would perform the thing on the schedules.
    fn apply(&self, solution: &mut Solution, order_flags: &mut OrderFlags) -> ScoreChange;
}

pub type CostChange = Time;
pub type ScoreChange = Time;
